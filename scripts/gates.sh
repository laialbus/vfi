#!/usr/bin/env bash
# Usage: scripts/gates.sh              — run every gate, then prove each one catches.
#        scripts/gates.sh --gates-only — run the gates and stop. This is the form a
#                                        proof runs inside its scratch copy.
#
# Exit codes: 0 every gate passed and every proof caught, 1 a gate failed or the
# gates the runner has are not the gates expected, 2 bad invocation, 3 a proof
# did not hold or could not be run.

set -euo pipefail

cd "$(dirname "$0")/.."

prog="$(basename "$0")"

# The gates from AGENTS.md whose machinery exists today, in the order they run.
# The rest — the golden fixtures and the benchmark — are absent rather than
# stubbed green: a gate that cannot fail reads exactly like a gate that is
# holding. A gate that has machinery and nothing to check yet is a different
# thing and belongs here: contracts runs its real check over the contracts that
# exist, which today is none of them.
#
# This is the only place the set is written down, and it is checked against the
# gates the file actually defines before any of them runs. Both loops below read
# it — the one that runs the gates and the one that proves them — so a name
# deleted here would take its own proof with it, and the run would end green
# having checked less. The check is what makes removing a gate cost a visible
# line in a file that cannot land without a signature.
expected_gates() {
	sed 's/#.*//' <<-'EOF' | awk 'NF'
	build
	tests
	deps
	purity
	contracts
	EOF
}

gates="$(expected_gates | tr '\n' ' ')"

# Anchor 2: the pipeline runs fetch → normalize → analyze → store, and a later
# stage never calls an earlier one, so every edge points forward — the dependent
# is the earlier stage. This is the whole permitted set, and the only place the
# direction is written down. A forward edge that skips a stage is left out on
# purpose: anchor 3 gives a stage the contract on either side of it and nothing
# else about its neighbours. Adding an edge here changes the shape anchor 2
# fixes, so it belongs to a task that says so, never to a task that wants one.
allowed_edges="
	vfi-fetch>vfi-normalize
	vfi-normalize>vfi-analyze
	vfi-analyze>vfi-store
"

# Anchor 4: analyze takes data in and returns results out — no network, no disk,
# no clock, no randomness, no environment, no logging with effects. A package
# belongs here when linking it puts one of those in analyze's hands; the word
# beside it is which one, and it is what the failure names. This is the only
# place that judgement is written down.
#
# The last group is the floor the others stand on. On a real target every
# capability above bottoms out in one of them, so a wrapper this list has never
# heard of is still caught by what it must link to do the work.
#
# What the list cannot see is std: std::fs, std::net, std::time and std::env
# arrive with every crate and no dependency list can deny them. This gate is the
# mechanism anchor 4's enforcement clause asks for — analyze cannot acquire the
# libraries — and it is not the whole of what anchor 4 says.
denied_packages() {
	sed 's/#.*//' <<-'EOF' | awk 'NF'
	# Clients, async runtimes — sockets and timers both — and the TLS under them.
	reqwest             network
	ureq                network
	curl                network
	isahc               network
	hyper               network
	tonic               network
	tokio               network
	async-std           network
	smol                network
	mio                 network
	socket2             network
	native-tls          network
	rustls              network
	openssl             network

	# Paths, temp files, watchers, mmap, and the embedded databases that are a
	# file on disk.
	tempfile            filesystem
	walkdir             filesystem
	glob                filesystem
	notify              filesystem
	memmap2             filesystem
	dirs                filesystem
	directories         filesystem
	home                filesystem
	rusqlite            filesystem
	sqlx                filesystem
	diesel              filesystem

	# Reading the current time. Calendar arithmetic over dates that arrived as
	# data would be fine; each of these carries a now() as well.
	chrono              clock
	time                clock
	humantime           clock
	instant             clock
	quanta              clock

	# An unseeded value is not reproducible, and uuid and ulid mint identity
	# from one.
	rand                randomness
	rand_core           randomness
	rand_chacha         randomness
	getrandom           randomness
	fastrand            randomness
	uuid                randomness
	ulid                randomness

	# The process environment, argv, and config files. Settings arrive as
	# arguments (anchor 5); they are never read from around the process.
	dotenv              environment
	dotenvy             environment
	envy                environment
	config              environment
	clap                environment

	# A logger writes somewhere. The effect is what anchor 4 bans, not the
	# message.
	log                 logging
	tracing             logging
	tracing-subscriber  logging
	env_logger          logging
	slog                logging
	fern                logging

	# Direct access to the machine.
	libc                syscalls
	nix                 syscalls
	rustix              syscalls
	windows-sys         syscalls
	winapi              syscalls
	wasi                syscalls
	js-sys              syscalls
	web-sys             syscalls
	EOF
}

gate_build() {
	cargo build --workspace
}

gate_tests() {
	cargo test --workspace
}

edge_allowed() {
	local edge
	for edge in $allowed_edges; do
		if [ "$edge" = "$1>$2" ]; then
			return 0
		fi
	done
	return 1
}

# `{p}` prints "name version (path)", so the package name is the first field.
# For a single package the root prints first and its direct dependencies follow.
workspace_members() {
	cargo tree --workspace --depth 0 --prefix none --format '{p}' | awk 'NF { print $1 }'
}

direct_dependencies() {
	cargo tree -p "$1" --depth 1 --edges normal,build,dev --prefix none --format '{p}' |
		awk 'NF { print $1 }' | tail -n +2
}

# The compiler already refuses a backward edge whose forward twin exists, because
# that pair is a cycle. It accepts every other wrong-direction edge, and reading
# the graph is the only way to catch those. Dev edges count: a stage that reaches
# backward from its tests still knows a neighbour anchor 2 says it cannot.
gate_deps() {
	local members member deps dep offenders
	members=" $(workspace_members | tr '\n' ' ')" || return 1

	offenders=""
	for member in $members; do
		deps="$(direct_dependencies "$member")" || return 1
		for dep in $deps; do
			# An edge leaving the workspace is some other gate's business.
			# Anchor 2 is about the order of the stages.
			case "$members" in
			*" $dep "*) ;;
			*) continue ;;
			esac
			if ! edge_allowed "$member" "$dep"; then
				offenders="$offenders  $member depends on $dep
"
			fi
		done
	done

	if [ -n "$offenders" ]; then
		echo "$prog: dependency edges anchor 2 does not allow:" >&2
		printf '%s' "$offenders" >&2
		return 1
	fi
}

# The whole resolved tree, not the direct edges: a denied package three levels
# down is linked just the same. Build and dev edges count too — a build script
# runs at compile time with the machine in reach, and a test that reads a clock
# is no longer proving the pure function analyze is supposed to be. A dependency
# that only some feature turns on is not in this tree and not linked either; what
# the build compiles is what this reads.
analyze_dependencies() {
	cargo tree -p vfi-analyze --edges normal,build,dev --prefix none --format '{p}' |
		awk 'NF { print $1 }'
}

gate_purity() {
	local tree package reason offenders
	tree=" $(analyze_dependencies | tr '\n' ' ')" || return 1

	offenders=""
	while read -r package reason; do
		case "$tree" in
		*" $package "*) offenders="$offenders  $package ($reason)
" ;;
		esac
	done <<-EOF
	$(denied_packages)
	EOF

	if [ -n "$offenders" ]; then
		echo "$prog: vfi-analyze links what anchor 4 bans it from reaching:" >&2
		printf '%s' "$offenders" >&2
		return 1
	fi
}

# Anchor 3: every boundary between stages is a typed, versioned contract, and a
# change that breaks compatibility fails the build. Which changes those are is
# not something this gate can read off a contract — a rename and a new field
# beside a deleted one are the same edit to anything that cannot read the
# meaning — and the format contracts are written in belongs to the task that
# writes the first one. So what is written down here is the rule that holds
# whatever that format turns out to be, and this is the only place it is stated:
#
#   A contract is a directory under contracts/. What it presents at version N
#   is the single file contracts/<name>/v<N>.*, whatever extension the format
#   gives it. contracts/<name>/versions records every version published so far,
#   one "v<N> <sha256>" line apiece, consecutive from v1.
#
#   A published version is frozen, and every change to one counts as breaking.
#   Not because every change is: because the stage on the other side was
#   compiled against the bytes that were there, and nothing mechanical can tell
#   an addition from a removal without reading a format that does not exist yet.
#
#   What a version does about a change is carry it. A change publishes v<N+1>
#   as its own file with its own line in versions. Nothing already published is
#   edited, and nothing versions names is deleted.
#
# That fixes the shape of contracts/ and says nothing about what is inside a
# contract file, which is the ADR-gated decision this gate is waiting for.

# Nothing today. contracts/ is created by the task that writes the first
# contract; until then the glob matches nothing and the gate runs over an empty
# set, which is an absence rather than a gate that cannot fail.
contract_names() {
	local dir
	for dir in contracts/*/; do
		if [ -d "$dir" ]; then
			dir="${dir%/}"
			printf '%s\n' "${dir#contracts/}"
		fi
	done
}

# Whichever of the three the machine has: sha256sum and shasum print the digest
# first, openssl prints it last. A machine with none of them fails the gate
# rather than skipping the check.
contract_digest() {
	local out
	if command -v sha256sum >/dev/null 2>&1; then
		out="$(sha256sum "$1")" || return 1
		printf '%s\n' "${out%% *}"
	elif command -v shasum >/dev/null 2>&1; then
		out="$(shasum -a 256 "$1")" || return 1
		printf '%s\n' "${out%% *}"
	elif command -v openssl >/dev/null 2>&1; then
		out="$(openssl dgst -sha256 "$1")" || return 1
		printf '%s\n' "${out##* }"
	else
		echo "$prog: no sha256 program to digest a contract with" >&2
		return 1
	fi
}

is_sha256() {
	case "$1" in
	"" | *[!0-9a-f]*) return 1 ;;
	esac
	[ "${#1}" -eq 64 ]
}

# One contract against its own record. Anything wrong prints a line on stdout
# for the gate to collect; a return of 1 means the check could not be made at
# all, which is not the same answer as a contract that failed it. A malformed
# record line stops this contract there — what the rest of the file claims is
# not worth reading once the shape is wrong.
check_contract() {
	local name dir record version digest extra number previous surface count
	local expected published match base

	name="$1"
	dir="contracts/$name"
	record="$dir/versions"

	if [ ! -f "$record" ]; then
		printf '  %s: has no versions file, so nothing records what it published\n' "$name"
		return 0
	fi

	previous=0
	published=" "
	while read -r version digest extra; do
		[ -n "$version" ] || continue

		case "$version" in
		v*) number="${version#v}" ;;
		*) number="" ;;
		esac
		case "$number" in
		"" | *[!0-9]* | 0*)
			printf '  %s: versions names "%s", and a published version is a v<N> from v1\n' \
				"$name" "$version"
			return 0
			;;
		esac
		if [ -n "$extra" ]; then
			printf '  %s: the %s line in versions carries more than a version and a digest\n' \
				"$name" "$version"
			return 0
		fi
		if ! is_sha256 "$digest"; then
			printf '  %s: the %s line in versions records no sha256 for it\n' "$name" "$version"
			return 0
		fi
		if [ "$number" -ne "$((previous + 1))" ]; then
			printf '  %s: versions publishes %s where v%s comes next; a bump is the next number\n' \
				"$name" "$version" "$((previous + 1))"
			return 0
		fi
		previous="$number"
		published="$published$version "

		count=0
		surface=""
		for match in "$dir/$version".*; do
			if [ -f "$match" ]; then
				count=$((count + 1))
				surface="$match"
			fi
		done
		if [ "$count" -eq 0 ]; then
			printf '  %s: versions publishes %s and there is no %s.* file\n' \
				"$name" "$version" "$version"
			continue
		fi
		if [ "$count" -gt 1 ]; then
			printf '  %s: %s has more than one file claiming to be its surface\n' "$name" "$version"
			continue
		fi

		expected="$(contract_digest "$surface")" || return 1
		if [ "$expected" != "$digest" ]; then
			printf '  %s: %s changed since it was published; a change publishes v%s instead\n' \
				"$name" "${surface#"$dir/"}" "$((number + 1))"
		fi
	done <"$record"

	if [ "$published" = " " ]; then
		printf '  %s: versions records nothing, so no version of it is published\n' "$name"
		return 0
	fi

	# A version file the record does not name is a surface published by nobody:
	# either a version that skipped the record, or one deleted from it and left
	# on disk. Other files in the directory are not this gate's business.
	for match in "$dir"/v*.*; do
		if [ -f "$match" ]; then
			base="${match##*/}"
			case "$published" in
			*" ${base%%.*} "*) ;;
			*) printf '  %s: %s is a version file versions does not publish\n' "$name" "$base" ;;
			esac
		fi
	done
}

gate_contracts() {
	local names name found problems count

	# The checker before what it checks: its cases sit with the proofs below.
	check_contract_cases || return 1
	names="$(contract_names)" || return 1

	count=0
	problems=""
	for name in $names; do
		count=$((count + 1))
		found="$(check_contract "$name")" || return 1
		if [ -n "$found" ]; then
			problems="$problems$found
"
		fi
	done

	if [ -n "$problems" ]; then
		echo "$prog: contracts that do not match the versions they publish:" >&2
		printf '%s' "$problems" >&2
		return 1
	fi

	if [ "$count" -eq 0 ]; then
		echo "contracts: none to check — contracts/ holds no contract yet"
	else
		echo "contracts: $count checked against the versions they publish"
	fi
}

# Any crate source will do: the violation only has to reach the compiler.
crate_source() {
	local candidate
	for candidate in "$1"/crates/*/src/lib.rs; do
		if [ -e "$candidate" ]; then
			echo "$candidate"
			return 0
		fi
	done
	echo "$prog: no crate source to inject a violation into" >&2
	return 1
}

# The violation each gate exists to catch, injected into a scratch copy of the
# tree. $1 is that copy's root, because what carries a violation differs by
# gate — a crate source for some, a manifest for others.
violate_build() {
	local file
	file="$(crate_source "$1")" || return 1
	printf '\n%s\n' 'fn this_does_not_compile(' >>"$file"
}

# cargo build compiles no #[cfg(test)] item, so the copy still builds and the
# gate under proof is the only one that goes red.
violate_tests() {
	local file
	file="$(crate_source "$1")" || return 1
	cat >>"$file" <<'EOF'

#[cfg(test)]
mod gate_proof {
    #[test]
    fn fails_on_purpose() {
        panic!("injected to prove the tests gate catches a failing test");
    }
}
EOF
}

# The last stage reaching back to an earlier one. The forward edge this reverses
# is absent from the tree, so the two do not form a cycle, the copy still builds
# and tests clean, and the deps gate is the only thing left that can object —
# which is the case this gate exists for.
violate_deps() {
	cat >>"$1/crates/store/Cargo.toml" <<'EOF'

[dependencies.vfi-analyze]
path = "../analyze"
EOF
}

# A local package carrying a denied name, not the real one from the registry: a
# run's sandbox cannot write cargo's registry cache, so a proof that had to
# download would fail for a reason of its own and read as the gate catching
# nothing. The gate matches the names in the resolved tree, and this resolves to
# the same name the real package would.
#
# It sits beside the copy rather than inside it because a path dependency inside
# the workspace directory becomes a workspace member, and then the deps gate,
# which runs first, objects to the edge and the proof goes red in the wrong
# place.
violate_purity() {
	local denied="$1-denied"
	mkdir -p "$denied/src"
	cat >"$denied/Cargo.toml" <<'EOF'
[package]
name = "rand"
version = "0.0.0"
edition = "2024"
publish = false
EOF
	: >"$denied/src/lib.rs"
	cat >>"$1/crates/analyze/Cargo.toml" <<EOF

[dependencies.rand]
path = "$denied"
EOF
}

# The fixture contracts the contracts proof runs over. They live here, with the
# proofs, and not in contracts/: nothing in this file is a contract between
# stages, and a fixture sitting in contracts/ would be read as one.
#
# This is the shape the rule allows — one contract that has been through a bump.
# v2 deletes a field v1 had, which is as breaking as a change gets, and it is
# fine: it is published as its own version, and v1 still says what it always
# said. The copy carrying this must stay green, because a gate that refused
# every contract it saw would catch the violation below just as well.
accept_contracts() {
	local dir version digest

	dir="$1/contracts/fixture"
	mkdir -p "$dir"

	cat >"$dir/v1.toml" <<'EOF'
boundary = "fixture"

[[field]]
name = "cik"
type = "u32"

[[field]]
name = "fiscal_period"
type = "string"
EOF

	cat >"$dir/v2.toml" <<'EOF'
boundary = "fixture"

[[field]]
name = "cik"
type = "u32"

[[field]]
name = "fiscal_year"
type = "u16"

[[field]]
name = "fiscal_quarter"
type = "u8?"
EOF

	: >"$dir/versions"
	for version in v1 v2; do
		digest="$(contract_digest "$dir/$version.toml")" || return 1
		printf '%s %s\n' "$version" "$digest" >>"$dir/versions"
	done
}

# The same change, made where the rule forbids it: v2's surface written over v1,
# which is the one the stage on the other side already compiled against, with no
# version published to carry it. Nothing else differs from the fixture the gate
# has just accepted, so the digest that no longer matches is the whole of what
# the gate has to go on.
violate_contracts() {
	accept_contracts "$1" || return 1
	cat "$1/contracts/fixture/v2.toml" >"$1/contracts/fixture/v1.toml"
}

# The proof above runs the gate end to end over the case anchor 3 is about, and
# proves it both ways round. The rest of the rule is these: one case apiece, each
# the same fixture differing in one way, and what the checker must say about it.
contract_cases() {
	sed 's/#.*//' <<-'EOF' | awk 'NF'
	first-version         clean
	bumped                clean
	edited-in-place       caught
	surface-deleted       caught
	unpublished-file      caught
	record-missing        caught
	record-empty          caught
	record-starts-late    caught
	record-skips          caught
	record-not-sha256     caught
	record-not-a-version  caught
	record-line-has-more  caught
	two-surfaces          caught
	EOF
}

plant_case() {
	local dir digest

	accept_contracts "$1" || return 1
	dir="$1/contracts/fixture"

	case "$2" in
	bumped) ;;
	first-version)
		rm "$dir/v2.toml"
		digest="$(contract_digest "$dir/v1.toml")" || return 1
		printf 'v1 %s\n' "$digest" >"$dir/versions"
		;;
	edited-in-place) cat "$dir/v2.toml" >"$dir/v1.toml" ;;
	surface-deleted) rm "$dir/v1.toml" ;;
	unpublished-file) cp "$dir/v2.toml" "$dir/v3.toml" ;;
	record-missing) rm "$dir/versions" ;;
	record-empty) : >"$dir/versions" ;;
	record-starts-late)
		digest="$(contract_digest "$dir/v2.toml")" || return 1
		printf 'v2 %s\n' "$digest" >"$dir/versions"
		;;
	record-skips)
		digest="$(contract_digest "$dir/v2.toml")" || return 1
		cp "$dir/v2.toml" "$dir/v4.toml"
		printf 'v4 %s\n' "$digest" >>"$dir/versions"
		;;
	record-not-sha256) printf 'v1 not-a-digest\n' >"$dir/versions" ;;
	record-not-a-version) printf 'one 0\n' >"$dir/versions" ;;
	record-line-has-more)
		digest="$(contract_digest "$dir/v1.toml")" || return 1
		printf 'v1 %s and-more\n' "$digest" >"$dir/versions"
		;;
	two-surfaces) cp "$dir/v1.toml" "$dir/v1.json" ;;
	*)
		echo "$prog: no contract case named $2" >&2
		return 1
		;;
	esac
}

# These run in-band with the gate, for the reason the gate set is asserted
# in-band: a branch nobody has watched refuse anything reads exactly like one
# that returns nothing, and a checker that has stopped checking must not be able
# to report a clean contracts/.
check_contract_cases() {
	local lab name expectation found verdict failures

	lab="$(mktemp -d "${TMPDIR:-/tmp}/vfi-contract-cases.XXXXXX")" || return 1

	failures=""
	while read -r name expectation; do
		if ! plant_case "$lab/$name" "$name"; then
			failures="$failures  $name: the case could not be built
"
			continue
		fi
		if ! found="$(cd "$lab/$name" && check_contract fixture)"; then
			failures="$failures  $name: the checker could not run over it
"
			continue
		fi
		if [ -n "$found" ]; then
			verdict=caught
		else
			verdict=clean
		fi
		if [ "$verdict" != "$expectation" ]; then
			failures="$failures  $name: the checker leaves it $verdict, and the rule says $expectation
"
		fi
	done <<-EOF
	$(contract_cases)
	EOF

	rm -rf "$lab"

	if [ -n "$failures" ]; then
		echo "$prog: the contract checker does not hold to its own rule:" >&2
		printf '%s' "$failures" >&2
		return 1
	fi
}

# The one violation that is against the runner rather than the tree: a gate
# deleted from the list, which is what the gate loop and the proof loop both
# read. Before the expectation this was the blind spot — the copy would build,
# test, and pass every gate that was left, and report every gate green.
violate_gate_set() {
	local runner="$1/scripts/gates.sh" trimmed="$1.trimmed"

	awk 'NF != 1 || $1 != "purity"' "$runner" >"$trimmed"
	if cmp -s "$runner" "$trimmed"; then
		echo "$prog: the expectation has no purity line to delete" >&2
		rm -f "$trimmed"
		return 1
	fi
	cat "$trimmed" >"$runner"
	rm -f "$trimmed"
}

usage() {
	echo "usage: $prog [--gates-only]" >&2
	exit 2
}

# What the runner can actually do, read off the functions themselves rather than
# restated as a second list: two hand-written lists drift together as easily as
# one is edited alone, and only the functions are the thing that runs.
defined_gates() {
	declare -F | awk '$3 ~ /^gate_/ { print substr($3, 6) }'
}

# The expectation against what the file defines, both ways round. A gate the
# expectation names and the runner does not have would be a gate silently
# dropped; one the runner has and the expectation does not name would be a gate
# that never runs. Neither is visible in a green run, which is why this comes
# before the gates rather than after them.
assert_gate_set() {
	local expected defined name differences

	expected=" $(expected_gates | tr '\n' ' ')"
	defined=" $(defined_gates | tr '\n' ' ')"

	differences=""
	for name in $expected; do
		case "$defined" in
		*" $name "*) ;;
		*) differences="$differences  $name: expected, and the runner has no gate for it
" ;;
		esac
	done
	for name in $defined; do
		case "$expected" in
		*" $name "*) ;;
		*) differences="$differences  $name: the runner has this gate, and the expectation does not name it
" ;;
		esac
	done

	if [ -n "$differences" ]; then
		echo "$prog: the gates the runner has are not the gates expected:" >&2
		printf '%s' "$differences" >&2
		return 1
	fi
}

run_gates() {
	if ! assert_gate_set; then
		echo "$prog: gate_set failed" >&2
		return 1
	fi
	echo "gate_set: the runner has the gates expected"

	for gate in $gates; do
		if ! "gate_$gate"; then
			# The first red ends the run. A gate after the build gate would
			# mostly re-report it, since nothing tests a workspace that does
			# not compile.
			echo "$prog: $gate failed" >&2
			return 1
		fi
		echo "$gate: passed"
	done
}

copy_tree() {
	mkdir -p "$1"
	tar -cf - --exclude=./.git --exclude=./target . | (cd "$1" && tar -xf -)
}

# A copy that fails for a reason of its own — a file the copy missed, a
# toolchain the scratch directory cannot reach — would read as every gate
# catching every violation. So the untouched copy runs first and must be green.
control() {
	local output
	copy_tree "$scratch/control"
	if ! output="$("$scratch/control/scripts/gates.sh" --gates-only 2>&1)"; then
		echo "$prog: the untouched scratch copy does not pass its own gates" >&2
		printf '%s\n' "$output" >&2
		return 1
	fi
	echo "control: the untouched copy passes"
}

# Going red is half of what a gate has to do; a gate that refuses everything
# catches its own violation and is worth nothing. Where a gate has a shape to
# accept as well as one to refuse, it carries an accept_<name> — the same
# fixture without the violation — and the copy carrying that must stay green.
# Where the tree itself is the shape a gate accepts, the control already proves
# this side and the gate carries no accept_.
accepts() {
	local name copy output
	name="$1"
	copy="$scratch/$name-accepted"
	copy_tree "$copy"
	"accept_$name" "$copy" || return 1

	if ! output="$("$copy/scripts/gates.sh" --gates-only 2>&1)"; then
		echo "$prog: $name refused what its rule allows" >&2
		printf '%s\n' "$output" >&2
		return 1
	fi
	echo "$name: accepts what the rule allows"
}

# A gate counts as a gate only if it goes red on what it exists to catch. Each
# proof copies the tree, injects that thing, and runs this same script in the
# copy, so a gate weakened later fails here instead of passing unnoticed. The
# copy is run with --gates-only because the plain form would recurse into these
# proofs; it runs the same gates, so what is red there is red for it too.
prove() {
	local name copy output
	name="$1"
	copy="$scratch/$name"
	if declare -F "accept_$name" >/dev/null; then
		# The accepted copy runs first, so a red below is the violation and not
		# something about the fixture the violation is made from.
		accepts "$name" || return 1
	fi
	copy_tree "$copy"
	"violate_$name" "$copy" || return 1

	if output="$("$copy/scripts/gates.sh" --gates-only 2>&1)"; then
		echo "$prog: $name passed a tree carrying the violation it exists to catch" >&2
		printf '%s\n' "$output" >&2
		return 1
	fi
	if ! printf '%s\n' "$output" | grep -Fqx "$prog: $name failed"; then
		echo "$prog: the injected $name violation went red somewhere else" >&2
		printf '%s\n' "$output" >&2
		return 1
	fi
	echo "$name: proof caught it"
}

scratch=""

cleanup() {
	if [ -n "$scratch" ]; then
		rm -rf "$scratch"
	fi
}

case "${1:-}" in
--gates-only)
	[ "$#" -eq 1 ] || usage
	run_gates || exit 1
	;;
"")
	run_gates || exit 1
	scratch="$(mktemp -d "${TMPDIR:-/tmp}/vfi-gates.XXXXXX")"
	trap cleanup EXIT
	control || exit 3
	# The gate-set check is proved like a gate, because it is what stands
	# between a deleted gate and a green run.
	for name in gate_set $gates; do
		prove "$name" || exit 3
	done
	echo "$prog: every gate passed and every proof caught"
	;;
*)
	usage
	;;
esac
