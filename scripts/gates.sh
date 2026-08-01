# DRAFT — NOT INSTALLED. Delete this block down to the shebang on install.
#
# Destination: `scripts/gates.sh`, which is a protected path. This is M2-02's
# whole deliverable; the pre-edit hook refuses to write the destination, so it
# is parked here. See escalations/2026-08-01-M2-02.md.
#
# Install: replace `scripts/gates.sh` with everything below this block, keeping
# the executable bit — CI runs `scripts/gates.sh` directly and a 644 file fails
# the workflow before any gate runs.
#
# Verified before parking, from a scratch copy with this file in place:
#   - `scripts/gates.sh` exits 0: build, tests, and deps pass, the control copy
#     passes, and all three proofs catch.
#   - vfi-fetch depending on vfi-normalize — an allowed forward edge — stays
#     green, so the gate admits the architecture anchor 2 describes.
#   - vfi-fetch depending on vfi-analyze — forward but skipping a stage — goes
#     red and names the edge.
#   - vfi-normalize dev-depending on vfi-fetch — backward, reached only from
#     tests, and not a cycle — goes red and names the edge.
#!/usr/bin/env bash
# Usage: scripts/gates.sh              — run every gate, then prove each one catches.
#        scripts/gates.sh --gates-only — run the gates and stop. This is the form a
#                                        proof runs inside its scratch copy.
#
# Exit codes: 0 every gate passed and every proof caught, 1 a gate failed,
# 2 bad invocation, 3 a proof did not catch or could not be run.

set -euo pipefail

cd "$(dirname "$0")/.."

prog="$(basename "$0")"

# The gates from AGENTS.md whose machinery exists today, in the order they run.
# The rest — golden fixtures, the analyze deny-list, contract compatibility, the
# benchmark — are absent rather than stubbed green: a gate that cannot fail
# reads exactly like a gate that is holding.
gates="build tests deps"

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

usage() {
	echo "usage: $prog [--gates-only]" >&2
	exit 2
}

run_gates() {
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
	local gate copy output
	gate="$1"
	copy="$scratch/$gate"
	copy_tree "$copy"
	"violate_$gate" "$copy" || return 1

	if output="$("$copy/scripts/gates.sh" --gates-only 2>&1)"; then
		echo "$prog: the $gate gate passed a tree carrying the violation it exists to catch" >&2
		printf '%s\n' "$output" >&2
		return 1
	fi
	if ! printf '%s\n' "$output" | grep -Fqx "$prog: $gate failed"; then
		echo "$prog: the injected $gate violation went red somewhere else" >&2
		printf '%s\n' "$output" >&2
		return 1
	fi
	echo "$gate: proof caught it"
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
	for gate in $gates; do
		prove "$gate" || exit 3
	done
	echo "$prog: every gate passed and every proof caught"
	;;
*)
	usage
	;;
esac
