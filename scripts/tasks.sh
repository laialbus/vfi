#!/usr/bin/env bash
# Usage: scripts/tasks.sh available     — print the id of every claimable task, one per line.
#        scripts/tasks.sh claim <id>    — claim <id> by pushing its branch, then verify it.
#
# Exit codes: 0 done, 1 origin unreachable, 2 bad invocation, 3 unreadable queue,
# 4 claim lost.

set -euo pipefail

cd "$(dirname "$0")/.."

usage() {
	echo "usage: $(basename "$0") available" >&2
	echo "       $(basename "$0") claim <task-id>" >&2
	exit 2
}

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
# A line the parser can see as one of the three keys but would not read as one
# — different case, a space before the colon, an indent — is refused rather
# than ignored: a field that looks set and is not is the fail-open case. Only
# case and surrounding whitespace are forgiven, and the key must be the whole
# text before the colon, so exclusive_reason is untouched, and so is prose that
# quotes or bullets a key inside a value.
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
		/^[[:space:]]*[^[:space:]][^:]*:/ {
			key = $0
			sub(/:.*$/, "", key)
			sub(/^[[:space:]]+/, "", key)
			sub(/[[:space:]]+$/, "", key)
			spelling = tolower(key)
			if (spelling == "id" || spelling == "depends_on" || spelling == "exclusive")
				fail("frontmatter key must be written exactly \"" spelling ":\", found: " $0)
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

read_branches() {
	if ! branches="$(git ls-remote --heads origin 2>/dev/null | awk '{print $2}')"; then
		echo "$(basename "$0"): cannot reach origin to check claimed branches" >&2
		return 1
	fi
}

# exclusive is a guard, so anything the parser cannot vouch for is refused
# rather than read as no: a guard that fails open stops guarding and says
# nothing. The refusal covers the whole queue, since the malformed task may be
# the exclusive one. Exit 3 keeps a broken queue distinct from a bad
# invocation (2) and an unreachable origin (1). A file without frontmatter —
# the README — is not a task and is skipped.
read_queue() {
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
}

available() {
	read_branches || exit 1
	read_queue

	# A claim is in flight while a task still in the queue has its branch on
	# origin. An exclusive task runs alone, so it is claimable only when no
	# claim is in flight, and once its own claim is in flight nothing is
	# claimable until it merges or the claim is released.
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
}

# Only ever removes a ref this run created: the lease names the commit the
# claim was pushed at, so a ref that has since moved is left alone rather than
# deleted out from under whoever moved it.
release() {
	if ! git push --quiet origin \
		--force-with-lease="refs/heads/$1:$2" --delete "refs/heads/$1"; then
		echo "$(basename "$0"): claim branch $1 is still on origin and must be deleted by hand" >&2
		return 1
	fi
}

# Whether the queue permits this claim to stand beside the others on origin.
# An exclusive task runs alone, so its claim conflicts with any other claim,
# and any other claim that is exclusive conflicts with this one — the rule is
# symmetric, and each side applies it to itself without knowing whether the
# other side will.
claim_conflict() {
	own_exclusive=no
	claims_beside_own=0
	exclusive_beside_own=""

	while IFS="$tab" read -r id depends exclusive; do
		[ -n "$id" ] || continue

		if [ "$id" = "$1" ]; then
			if [ "$exclusive" = yes ]; then
				own_exclusive=yes
			fi
			continue
		fi

		if claimed "$id"; then
			claims_beside_own=$((claims_beside_own + 1))
			if [ "$exclusive" = yes ]; then
				exclusive_beside_own="$id"
			fi
		fi
	done <<<"$tasks"

	if [ -n "$exclusive_beside_own" ]; then
		conflict="the exclusive claim on $exclusive_beside_own is in flight"
		return 0
	fi
	if [ "$own_exclusive" = yes ] && [ "$claims_beside_own" -ne 0 ]; then
		conflict="$1 is exclusive and $claims_beside_own other claims are in flight"
		return 0
	fi

	conflict=""
	return 1
}

# Claiming is pushing the task's branch: one ref cannot be created twice, so
# the push settles who holds a given task. What it cannot settle is the
# exclusive rule, which spans tasks — two runs both reading an idle queue both
# push different refs, and both pushes succeed. So the claim is verified after
# the ref lands, against origin read fresh: a run that finds its claim beside
# one it may not stand with deletes its own ref and reports the claim lost.
# Both sides deciding that is fine — the claims are retried; both sides
# proceeding is what may never happen, and cannot, because neither can miss a
# ref that was already on origin when it read.
#
# The id must come from `available`: this checks the exclusive rule and who
# holds the ref, not whether the task's dependencies have merged.
claim() {
	read_queue

	if ! printf '%s' "$tasks" | cut -f1 | grep -Fxq "$1"; then
		echo "$(basename "$0"): $1 is not a task in the queue" >&2
		exit 2
	fi

	# The claim is the ref this push creates, so what matters is that it was
	# created and not merely written: git calls a push onto a ref already at
	# this commit a success, and two runs on the same base commit push the very
	# same commit, so the exit code alone would report someone else's claim as
	# ours. The porcelain marker says which happened — * for a ref that did not
	# exist before — and the lease refuses a ref that landed since, rather than
	# fast-forwarding another run's claim onto our commit.
	head="$(git rev-parse HEAD)"
	push_report="$(git push --porcelain origin "HEAD:refs/heads/$1" \
		--force-with-lease="refs/heads/$1:")" || true
	case "$(printf '%s\n' "$push_report" |
		awk -F'\t' -v spec="HEAD:refs/heads/$1" '$2 == spec { print $1; exit }')" in
	'*')
		;;
	'')
		echo "$(basename "$0"): could not push the claim on $1" >&2
		exit 1
		;;
	*)
		echo "$(basename "$0"): $1 is already claimed" >&2
		exit 4
		;;
	esac

	if ! read_branches; then
		echo "$(basename "$0"): claim on $1 cannot be verified" >&2
		release "$1" "$head" || true
		exit 1
	fi

	if ! claimed "$1"; then
		echo "$(basename "$0"): claim on $1 is gone from origin" >&2
		exit 4
	fi

	if claim_conflict "$1"; then
		echo "$(basename "$0"): claim on $1 released: $conflict" >&2
		release "$1" "$head" || exit 1
		exit 4
	fi

	echo "$1: claimed"
}

tab="$(printf '\t')"

case "${1:-}" in
available)
	[ "$#" -eq 1 ] || usage
	available
	;;
claim)
	[ "$#" -eq 2 ] || usage
	claim "$2"
	;;
*)
	usage
	;;
esac
