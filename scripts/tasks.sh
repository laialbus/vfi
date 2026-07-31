#!/usr/bin/env bash
# Usage: scripts/tasks.sh available — print the id of every claimable task, one per line.

set -euo pipefail

cd "$(dirname "$0")/.."

if [ "${1:-}" != "available" ]; then
	echo "usage: $(basename "$0") available" >&2
	exit 2
fi

# A dependency is merged when its task file is gone: the branch that finishes a
# task deletes it in the same diff, so absence is the merge record.
dependencies_merged() {
	for dep in $1; do
		if [ -e "tasks/$dep.md" ]; then
			return 1
		fi
	done
	return 0
}

# One view of the frontmatter, validated where the whole line is visible. The
# record is id<TAB>depends<TAB>exclusive with "-" standing for an empty field,
# so consecutive tabs never collapse and the fields cannot shift. Tabs inside
# values become spaces before anything reads them, so the value that is
# validated is the value that is compared. Every exclusive line is checked as
# it is seen — a malformed value on a duplicated key cannot hide behind a later
# valid one. A file that opens frontmatter must carry an id; one the parser
# cannot key would otherwise be invisible, and an invisible guard is no guard.
read_frontmatter() {
	awk -v prog="$(basename "$0")" '
		function fail(msg) {
			printf "%s: %s: %s\n", prog, FILENAME, msg > "/dev/stderr"
			failed = 1
			exit 3
		}
		NR == 1 {
			if ($0 !~ /^---[[:space:]]*$/) exit
			in_frontmatter = 1
			next
		}
		!in_frontmatter { next }
		/^---[[:space:]]*$/ { exit }
		/^id:/ {
			value = $0
			sub(/^id:[[:space:]]*/, "", value)
			gsub(/[[:space:]"'\''"]/, "", value)
			id = value
			in_depends = 0
			next
		}
		/^depends_on:/ {
			value = $0
			sub(/^depends_on:[[:space:]]*/, "", value)
			gsub(/[\[\],\t]/, " ", value)
			gsub(/["'\'']/, "", value)
			depends = depends " " value
			in_depends = 1
			next
		}
		/^[[:space:]]*exclusive:/ {
			value = $0
			sub(/^[[:space:]]*exclusive:[[:space:]]*/, "", value)
			gsub(/\t/, " ", value)
			gsub(/["'\'']/, "", value)
			sub(/[[:space:]]+$/, "", value)
			if (tolower(value) !~ /^(yes|no)?$/)
				fail("exclusive must be yes or no, found: " value)
			exclusive = tolower(value)
			in_depends = 0
			next
		}
		in_depends && /^[[:space:]]*-[[:space:]]*/ {
			value = $0
			sub(/^[[:space:]]*-[[:space:]]*/, "", value)
			gsub(/["'\'']/, "", value)
			gsub(/\t/, " ", value)
			depends = depends " " value
			next
		}
		/^[^[:space:]-]/ { in_depends = 0 }
		END {
			if (failed) exit 3
			if (!in_frontmatter) exit 0
			if (id == "")
				fail("frontmatter has no id")
			if (failed) exit 3
			sub(/^[[:space:]]+/, "", depends)
			sub(/[[:space:]]+$/, "", depends)
			if (depends == "") depends = "-"
			if (exclusive == "") exclusive = "-"
			print id "\t" depends "\t" exclusive
		}
	' "$1"
}

claimed() {
	printf '%s\n' "$branches" | grep -Fxq "refs/heads/$1"
}

if ! branches="$(git ls-remote --heads origin 2>/dev/null | awk '{print $2}')"; then
	echo "$(basename "$0"): cannot reach origin to check claimed branches" >&2
	exit 1
fi

tab="$(printf '\t')"

# exclusive is a guard, so anything the parser cannot vouch for is refused
# rather than read as no: a guard that fails open stops guarding and says
# nothing. The refusal covers the whole queue, since the malformed task may be
# the exclusive one. Exit 3 keeps a broken queue distinct from a bad
# invocation (2) and an unreachable origin (1). A file without frontmatter —
# the README — is not a task and is skipped.
if [ ! -d tasks ]; then
	echo "$(basename "$0"): tasks/ directory missing" >&2
	exit 3
fi

tasks=""
for task_file in tasks/*.md; do
	[ -e "$task_file" ] || continue

	if ! frontmatter="$(read_frontmatter "$task_file")"; then
		exit 3
	fi
	[ -n "$frontmatter" ] || continue

	tasks="$tasks$frontmatter
"
done

# A claim is in flight while a task still in the queue has its branch on origin.
# An exclusive task runs alone, so it is claimable only when no claim is in
# flight, and once its own claim is in flight nothing is claimable until it
# merges or the claim is released.
claims_in_flight=0
exclusive_in_flight=no
while IFS="$tab" read -r id depends exclusive; do
	[ -n "$id" ] || continue

	if claimed "$id"; then
		claims_in_flight=$((claims_in_flight + 1))
		if [ "$exclusive" = yes ]; then
			exclusive_in_flight=yes
		fi
	fi
done <<<"$tasks"

if [ "$exclusive_in_flight" = yes ]; then
	exit 0
fi

printf '%s\n' "$tasks" | while IFS="$tab" read -r id depends exclusive; do
	[ -n "$id" ] || continue

	if claimed "$id"; then
		continue
	fi

	if [ "$exclusive" = yes ] && [ "$claims_in_flight" -ne 0 ]; then
		continue
	fi

	if [ "$depends" = "-" ]; then
		depends=""
	fi
	if dependencies_merged "$depends"; then
		echo "$id"
	fi
done | sort
