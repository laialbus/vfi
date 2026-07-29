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

read_frontmatter() {
	awk '
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
			gsub(/[[:space:]"]/, "", value)
			id = value
			in_depends = 0
			next
		}
		/^depends_on:/ {
			value = $0
			sub(/^depends_on:[[:space:]]*/, "", value)
			gsub(/\[/, " ", value)
			gsub(/\]/, " ", value)
			gsub(/,/, " ", value)
			gsub(/"/, "", value)
			depends = depends " " value
			in_depends = 1
			next
		}
		in_depends && /^[[:space:]]*-[[:space:]]*/ {
			value = $0
			sub(/^[[:space:]]*-[[:space:]]*/, "", value)
			gsub(/"/, "", value)
			depends = depends " " value
			next
		}
		/^[^[:space:]-]/ { in_depends = 0 }
		END { if (id != "") print id "\t" depends }
	' "$1"
}

if ! branches="$(git ls-remote --heads origin 2>/dev/null | awk '{print $2}')"; then
	echo "$(basename "$0"): cannot reach origin to check claimed branches" >&2
	exit 1
fi

for task_file in tasks/*.md; do
	[ -e "$task_file" ] || continue

	frontmatter="$(read_frontmatter "$task_file")"
	[ -n "$frontmatter" ] || continue

	id="$(printf '%s\n' "$frontmatter" | cut -f1)"
	depends="$(printf '%s\n' "$frontmatter" | cut -f2)"

	if printf '%s\n' "$branches" | grep -Fxq "refs/heads/$id"; then
		continue
	fi

	if dependencies_merged "$depends"; then
		echo "$id"
	fi
done | sort
