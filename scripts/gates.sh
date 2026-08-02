#!/usr/bin/env bash
# Usage: scripts/gates.sh              — run every gate, then prove each one catches.
#        scripts/gates.sh --gates-only — run the gates and stop. This is the form a
#                                        proof runs inside its scratch copy.
#
# Exit codes: 0 every gate passed and every proof caught, 1 a gate failed or the
# gates the runner has are not the gates expected, 2 bad invocation, 3 a proof
# did not catch or could not be run.

set -euo pipefail

cd "$(dirname "$0")/.."

prog="$(basename "$0")"

# The gates from AGENTS.md whose machinery exists today, in the order they run.
# The rest — golden fixtures, contract compatibility, the benchmark — are absent
# rather than stubbed green: a gate that cannot fail reads exactly like a gate
# that is holding.
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

# A gate counts as a gate only if it goes red on what it exists to catch. Each
# proof copies the tree, injects that thing, and runs this same script in the
# copy, so a gate weakened later fails here instead of passing unnoticed. The
# copy is run with --gates-only because the plain form would recurse into these
# proofs; it runs the same gates, so what is red there is red for it too.
prove() {
	local name copy output
	name="$1"
	copy="$scratch/$name"
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
