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
		/^exclusive:/ {
			value = $0
			sub(/^exclusive:[[:space:]]*/, "", value)
			sub(/[[:space:]]+$/, "", value)
			gsub(/"/, "", value)
			exclusive = tolower(value)
			in_depends = 0
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
		END { if (id != "") print id "\t" depends "\t" exclusive }
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

# exclusive is a guard, so an unrecognised value is refused rather than read as
# no: a guard that fails open stops guarding and says nothing. The refusal
# covers the whole queue, since the malformed task may be the exclusive one.
tasks=""
for task_file in tasks/*.md; do
	[ -e "$task_file" ] || continue

	frontmatter="$(read_frontmatter "$task_file")"
	[ -n "$frontmatter" ] || continue

	exclusive_value="${frontmatter##*$tab}"
	case "$exclusive_value" in
	"" | yes | no) ;;
	*)
		echo "$(basename "$0"): $task_file: exclusive must be yes or no, found: $exclusive_value" >&2
		exit 3
		;;
	esac

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

	if dependencies_merged "$depends"; then
		echo "$id"
	fi
done | sort
