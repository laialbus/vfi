#!/usr/bin/env bash
# Usage: scripts/tasks.sh available     — print the id of every claimable task, one per line.
#        scripts/tasks.sh claim <id>    — claim <id> by pushing its branch, then verify it.
#        scripts/tasks.sh sweep         — release the claims of runs that are gone.
#
# Exit codes: 0 done, 1 origin unreachable or a claim could not be released,
# 2 bad invocation, 3 unreadable queue, 4 claim lost.

set -euo pipefail

cd "$(dirname "$0")/.."

usage() {
	echo "usage: $(basename "$0") available" >&2
	echo "       $(basename "$0") claim <task-id>" >&2
	echo "       $(basename "$0") sweep" >&2
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

# One read of origin, kept whole as well as by name: the sweep releases a ref at
# the commit it was seen at, and a second read would be a second moment.
read_branches() {
	if ! branch_refs="$(git ls-remote --heads origin 2>/dev/null)"; then
		echo "$(basename "$0"): cannot reach origin to check claimed branches" >&2
		return 1
	fi
	branches="$(printf '%s\n' "$branch_refs" | awk '{print $2}')"
}

claim_head() {
	printf '%s\n' "$branch_refs" | awk -v ref="refs/heads/$1" '$2 == ref { print $1; exit }'
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

# Only ever removes the ref as it was read: the lease names the commit the ref
# was seen at, so one that has moved since is left alone rather than deleted out
# from under whoever moved it. That is what makes releasing another run's claim
# safe — anything its owner pushes between the read and the delete refuses it,
# whatever the sweep concluded in between.
release() {
	if ! git push --quiet origin \
		--force-with-lease="refs/heads/$1:$2" --delete "refs/heads/$1"; then
		echo "$(basename "$0"): could not release $1; a later sweep takes it if it is dead" >&2
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

# A claim still on origin belongs to one of three runs: one still working, one
# that handed off and is waiting on review, or one that was killed outright. The
# wrapper releases the claim on every exit it survives, so only the third leaves
# a claim behind by accident, and only the third is swept. The other two have to
# be ruled out from origin alone — this runs in a different process from the runs
# it judges and sees nothing of them but what they left here.
#
# The one waiting on review is ruled out by its open pull request, which is left
# alone: deleting the branch would close it.
#
# The one still working is ruled out by age, and the age that counts is when the
# claim landed on origin — not the date on the commit it points at. A claim is
# pushed at whatever main's tip was, so a claim made this minute can point at a
# commit from last week, and sweeping by commit date would take the live claims
# first. Origin's activity record is the one place that says when the ref itself
# appeared, so a claim whose age it cannot answer for is kept and reported
# instead: a claim held one run too long costs a run, and a claim taken from a
# live run puts two agents on one task.
#
# What bounds a live claim's age is the wrapper's wall-clock limit — an hour by
# default, and the wrapper kills what does not stop by then. Six times that
# leaves room for a limit raised on the day and a clock that disagrees, and it is
# the one assumption here that a change outside the repository could break.
claim_lifetime_hours=6

# The moment a claim must have landed before to be too old for a live run to
# hold, in the shape origin reports. BSD date first, GNU second: the fleet runs
# on macOS and CI on Linux.
claim_cutoff() {
	date -u -v-"$claim_lifetime_hours"H +%Y-%m-%dT%H:%M:%SZ 2>/dev/null ||
		date -u -d "$claim_lifetime_hours hours ago" +%Y-%m-%dT%H:%M:%SZ 2>/dev/null
}

# When origin last saw anything happen to this ref. Any activity is evidence of
# life at that moment, so the kind does not matter and the most recent one
# answers. Empty when origin's record does not reach back to this claim.
last_activity() {
	gh api "repos/{owner}/{repo}/activity?ref=refs/heads/$1&per_page=1" \
		--jq '.[].timestamp'
}

open_pull_requests() {
	gh pr list --head "$1" --state open --json number \
		--jq '[.[].number | "#\(.)"] | join(" and ")'
}

# Both stamps carry the same fixed shape, so their digits order them and nothing
# here has to parse a date.
digits() {
	printf '%s' "$1" | tr -cd '0-9'
}

# Only a branch whose whole name is a task id still in the queue is a candidate.
# That is what keeps main, session/… and escalated/… out of reach: an
# escalated/M1-08-… branch is the record of a run that failed, not a claim, and a
# record is not something to tidy away. It holds as long as a task id is one path
# segment, so an id with a slash in it — which could name a branch inside
# escalated/ — is refused rather than swept.
sweep() {
	read_branches || exit 1
	read_queue

	if ! cutoff="$(claim_cutoff)"; then
		echo "$(basename "$0"): cannot work out how old a claim would have to be" >&2
		exit 1
	fi

	claims=""
	while IFS="$tab" read -r id depends exclusive; do
		[ -n "$id" ] || continue

		if claimed "$id"; then
			claims="$claims $id"
		fi
	done <<<"$tasks"

	if [ -z "$claims" ]; then
		echo "no claims on origin"
		return 0
	fi

	echo "a claim origin last saw before $cutoff is nobody's live run"

	stuck=0
	for id in $claims; do
		case "$id" in
		*/*)
			echo "$id: kept, an id with a / could name a branch this must not touch"
			continue
			;;
		esac

		if ! pulls="$(open_pull_requests "$id")"; then
			echo "$id: kept, cannot ask origin whether a pull request is open"
			continue
		fi
		if [ -n "$pulls" ]; then
			echo "$id: kept, pull request $pulls is open"
			continue
		fi

		if ! stamp="$(last_activity "$id")"; then
			echo "$id: kept, cannot ask origin when this claim landed"
			continue
		fi
		if [ -z "$stamp" ]; then
			echo "$id: kept, origin has no record of when this claim landed"
			continue
		fi
		case "$stamp" in
		[0-9][0-9][0-9][0-9]-[0-9][0-9]-[0-9][0-9]T[0-9][0-9]:[0-9][0-9]:[0-9][0-9]Z) ;;
		*)
			echo "$id: kept, origin dates this claim in a shape this cannot read: $stamp"
			continue
			;;
		esac
		if [ "$(digits "$stamp")" -ge "$(digits "$cutoff")" ]; then
			echo "$id: kept, origin last saw this claim at $stamp"
			continue
		fi

		# The commit goes in the line because the branch does not survive it.
		# A run killed mid-task may have pushed work nobody has seen, and the
		# wrapper discards exactly that on every exit it survives; naming the
		# commit is what keeps it findable afterward rather than only gone.
		claim_sha="$(claim_head "$id")"
		if release "$id" "$claim_sha"; then
			echo "$id: released, origin last saw it at $stamp, at $claim_sha"
		else
			stuck=$((stuck + 1))
		fi
	done

	if [ "$stuck" -ne 0 ]; then
		exit 1
	fi
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
sweep)
	[ "$#" -eq 1 ] || usage
	sweep
	;;
*)
	usage
	;;
esac
