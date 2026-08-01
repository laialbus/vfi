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
# The rest — golden fixtures, dependency direction, the analyze deny-list,
# contract compatibility, the benchmark — are absent rather than stubbed green:
# a gate that cannot fail reads exactly like a gate that is holding.
gates="build tests"

gate_build() {
	cargo build --workspace
}

gate_tests() {
	cargo test --workspace
}

# The violation each gate exists to catch, appended to a crate source inside a
# scratch copy. $1 is that file.
violate_build() {
	printf '\n%s\n' 'fn this_does_not_compile(' >>"$1"
}

# cargo build compiles no #[cfg(test)] item, so the copy still builds and the
# gate under proof is the only one that goes red.
violate_tests() {
	cat >>"$1" <<'EOF'

#[cfg(test)]
mod gate_proof {
    #[test]
    fn fails_on_purpose() {
        panic!("injected to prove the tests gate catches a failing test");
    }
}
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

# Any crate source will do: the violation only has to reach the compiler.
injection_target() {
	for candidate in crates/*/src/lib.rs; do
		if [ -e "$candidate" ]; then
			echo "$candidate"
			return 0
		fi
	done
	echo "$prog: no crate source to inject a violation into" >&2
	return 1
}

copy_tree() {
	mkdir -p "$1"
	tar -cf - --exclude=./.git --exclude=./target . | (cd "$1" && tar -xf -)
}

# A copy that fails for a reason of its own — a file the copy missed, a
# toolchain the scratch directory cannot reach — would read as every gate
# catching every violation. So the untouched copy runs first and must be green.
control() {
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
	gate="$1"
	copy="$scratch/$gate"
	copy_tree "$copy"
	"violate_$gate" "$copy/$target"

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
	target="$(injection_target)" || exit 3
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
