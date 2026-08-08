#!/usr/bin/env bash
# Usage: scripts/tasks.sh available     — print the id of every claimable task, one per line.
#        scripts/tasks.sh claim <id>    — claim <id> by pushing its branch, then verify it.
#        scripts/tasks.sh sweep         — release the claims of runs that are gone.
#        scripts/tasks.sh release <id> <commit> <seen>
#                                       — release one claim the way a sweep would, given the
#                                         commit origin holds it at and when origin last saw
#                                         it. Both are in the line a sweep prints.
#        scripts/tasks.sh check         — read the queue and say how much of it there is.
#                                         Reaches nothing outside the checked-out tree, so a
#                                         gate can ask it of a copy with no remote.
#
# Exit codes: 0 done, 1 origin unreachable or a claim could not be released,
# 2 bad invocation, 3 unreadable queue, 4 claim lost.

set -euo pipefail

cd "$(dirname "$0")/.."

usage() {
	echo "usage: $(basename "$0") available" >&2
	echo "       $(basename "$0") claim <task-id>" >&2
	echo "       $(basename "$0") sweep" >&2
	echo "       $(basename "$0") release <task-id> <commit> <last-seen>" >&2
	echo "       $(basename "$0") check" >&2
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
# record is id<TAB>depends<TAB>exclusive<TAB>owns with "-" standing for an empty
# field, so consecutive tabs never collapse and the fields cannot shift. Tabs
# inside values become spaces before anything reads them, so the value that is
# validated is the value that is compared. Every exclusive line is checked as
# it is seen — a malformed value on a duplicated key cannot hide behind a later
# valid one. A file that opens frontmatter must carry an id; one the parser
# cannot key would otherwise be invisible, and an invisible guard is no guard.
# A line the parser can see as one of the four keys but would not read as one
# — different case, a space before the colon, an indent — is refused rather
# than ignored: a field that looks set and is not is the fail-open case. Only
# case and surrounding whitespace are forgiven, and the key must be the whole
# text before the colon, so exclusive_reason is untouched, and so is prose that
# quotes or bullets a key inside a value.
#
# owns and depends_on are both lists, written inline or as bullets, and only one
# of them is being read at a time — which is what the list name tracks. A key
# the parser knows, or any line that starts at column zero without a bullet,
# closes the list above it, so an acceptance bullet is never read as a path.
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
			list = ""
			next
		}
		/^depends_on:/ {
			value = $0
			sub(/^depends_on:[[:space:]]*/, "", value)
			gsub(/[\[\],\t]/, " ", value)
			gsub(/["'\'']/, "", value)
			depends = depends " " value
			list = "depends"
			next
		}
		/^owns:/ {
			value = $0
			sub(/^owns:[[:space:]]*/, "", value)
			gsub(/[\[\],\t]/, " ", value)
			gsub(/["'\'']/, "", value)
			owns = owns " " value
			list = "owns"
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
			list = ""
			next
		}
		/^[[:space:]]*[^[:space:]][^:]*:/ {
			key = $0
			sub(/:.*$/, "", key)
			sub(/^[[:space:]]+/, "", key)
			sub(/[[:space:]]+$/, "", key)
			spelling = tolower(key)
			if (spelling == "id" || spelling == "depends_on" ||
				spelling == "exclusive" || spelling == "owns")
				fail("frontmatter key must be written exactly \"" spelling ":\", found: " $0)
		}
		list != "" && /^[[:space:]]*-[[:space:]]*/ {
			value = $0
			sub(/^[[:space:]]*-[[:space:]]*/, "", value)
			gsub(/["'\'']/, "", value)
			gsub(/\t/, " ", value)
			if (list == "owns")
				owns = owns " " value
			else
				depends = depends " " value
			next
		}
		/^[^[:space:]-]/ { list = "" }
		END {
			if (failed) exit 3
			if (!in_frontmatter) exit 0
			if (id == "")
				fail("frontmatter has no id")
			if (failed) exit 3
			sub(/^[[:space:]]+/, "", depends)
			sub(/[[:space:]]+$/, "", depends)
			sub(/^[[:space:]]+/, "", owns)
			sub(/[[:space:]]+$/, "", owns)
			if (depends == "") depends = "-"
			if (exclusive == "") exclusive = "-"
			if (owns == "") owns = "-"
			print id "\t" depends "\t" exclusive "\t" owns
		}
	' "$1"
}

# An open escalation parks the task it names: the run that wrote one stopped on
# a wall that retrying reproduces, so the task leaves the pool until the file is
# deleted. What a file names is its name — <date>-<task-id>.md per
# escalations/README.md — and the id is matched whole, against the queue's own
# ids, so a name carrying a subject slug instead parks nothing and nothing here
# has to work out what an id looks like.
parked() {
	for escalation in escalations/*.md; do
		[ -e "$escalation" ] || continue

		case "${escalation##*/}" in
		[0-9][0-9][0-9][0-9]-[0-9][0-9]-[0-9][0-9]-"$1".md) return 0 ;;
		esac
	done
	return 1
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

# What the queue must be for two agents to work in it at once. Each task is
# well-formed on its own and these are about the set: WORKPLAN.md's rules for
# writing good tasks, which a human eye kept while humans wrote task files and
# which nothing kept once the planner did.
#
#   A task file is named for the id it carries, so the branch that claims it,
#   the file that describes it, and the dependency that names it are one name.
#   This is also what catches a copied id: two files cannot both be tasks/X.md.
#
#   depends_on is an order, so it cannot close a loop. A cycle is a set of tasks
#   none of which can ever be claimed, which reads from outside as a queue that
#   has quietly stopped offering work.
#
#   Two tasks whose owns overlap may never be claimable at once. Ordering them
#   is one remedy and marking one exclusive is the other, so both are read here;
#   the order counts through other queued tasks, because a path of dependencies
#   separates two tasks exactly as well as one edge does. Overlap is a question
#   about paths, so it is asked of path segments: crates/ contains
#   crates/analyze, and crates/analyze does not contain crates/analyzer.
#
# What is deliberately not checked is a depends_on naming a task that is not
# here. Absence is the merge record — dependencies_merged reads it that way —
# so a dependency on an absent task is a dependency that has been met, and a
# typo in one is indistinguishable from that without history the checked-out
# tree does not carry. That belongs to review of the queue, not to this.
#
# The first problem refuses the whole queue, as a malformed task already does:
# the collision may be between the two tasks a reader was about to claim, and a
# partial answer is the one shape worse than none.
check_queue() {
	printf '%s' "$queue" | awk -v prog="$(basename "$0")" -F'\t' '
		function fail(where, msg) {
			printf "%s: %s: %s\n", prog, where, msg > "/dev/stderr"
			exit 3
		}
		function bare(path) {
			sub(/\/+$/, "", path)
			return path
		}
		function overlaps(one, other,   a, b) {
			a = bare(one)
			b = bare(other)
			if (a == "" || b == "") return 0
			if (a == b) return 1
			if (substr(b, 1, length(a) + 1) == a "/") return 1
			if (substr(a, 1, length(b) + 1) == b "/") return 1
			return 0
		}
		function edges(task,   value) {
			value = depends[task]
			if (value == "-") value = ""
			return value
		}
		function paths(task,   value) {
			value = owns[task]
			if (value == "-") value = ""
			return value
		}
		# The dependencies on the stack from the one closed back onto, round to
		# itself: the cycle as a reader would have to trace it by hand.
		function loop(target,   step, tracing, drawn) {
			for (step = 1; step <= depth; step++) {
				if (!tracing && stack[step] != target) continue
				tracing = 1
				drawn = drawn id[stack[step]] " -> "
			}
			return drawn id[target]
		}
		# Where a task has been is read as a value and never as a key: asking
		# whether awk holds a subscript is what creates it, so a walk that
		# tested membership would mark every task it merely looked at as
		# visited and never follow an edge twice removed.
		function walk(task,   part, count, dep, next_task) {
			state[task] = "open"
			stack[++depth] = task
			count = split(edges(task), dep, " ")
			for (part = 1; part <= count; part++) {
				if (!(dep[part] in at)) continue
				next_task = at[dep[part]]
				if (state[next_task] == "open")
					fail(file[task], "depends_on " id[next_task] \
						" closes a cycle: " loop(next_task))
				if (state[next_task] == "") walk(next_task)
			}
			state[task] = "done"
			depth--
		}
		function reaches(task, root,   part, count, dep, next_task) {
			count = split(edges(task), dep, " ")
			for (part = 1; part <= count; part++) {
				if (!(dep[part] in at)) continue
				next_task = at[dep[part]]
				if (!((root, next_task) in reach)) {
					reach[root, next_task] = 1
					reaches(next_task, root)
				}
			}
		}
		function collide(earlier, later,   mine, theirs, m, t, p, q) {
			m = split(paths(earlier), mine, " ")
			t = split(paths(later), theirs, " ")
			for (p = 1; p <= m; p++)
				for (q = 1; q <= t; q++)
					if (overlaps(mine[p], theirs[q]))
						fail(file[later], "owns " theirs[q] \
							", which overlaps the " mine[p] " that " \
							id[earlier] " owns, and neither depends on the other")
		}
		NF {
			file[++n] = $1
			id[n] = $2
			depends[n] = $3
			exclusive[n] = $4
			owns[n] = $5
			at[$2] = n
		}
		END {
			for (i = 1; i <= n; i++)
				if (file[i] != "tasks/" id[i] ".md")
					fail(file[i], "frontmatter says id " id[i] \
						", and a task file is named for the id it carries")
			for (i = 1; i <= n; i++)
				if (state[i] == "") walk(i)
			for (i = 1; i <= n; i++)
				reaches(i, i)
			for (i = 1; i <= n; i++)
				for (j = i + 1; j <= n; j++) {
					if (exclusive[i] == "yes" || exclusive[j] == "yes") continue
					if ((i, j) in reach || (j, i) in reach) continue
					collide(i, j)
				}
		}
	'
}

# exclusive is a guard, so anything the parser cannot vouch for is refused
# rather than read as no: a guard that fails open stops guarding and says
# nothing. The refusal covers the whole queue, since the malformed task may be
# the exclusive one. Exit 3 keeps a broken queue distinct from a bad
# invocation (2) and an unreachable origin (1). A file without frontmatter —
# the README — is not a task and is skipped.
#
# Each file is vouched for as it is read and the set is vouched for once they
# all are, because the structure is a fact about the set: a task that cannot be
# parsed has no owns list to compare and no id to key, so there is nothing to
# ask about it until every file has answered.
read_queue() {
	if [ ! -d tasks ]; then
		echo "$(basename "$0"): tasks/ directory missing" >&2
		exit 3
	fi

	tasks=""
	queue=""
	for task_file in tasks/*.md; do
		[ -e "$task_file" ] || continue

		if ! frontmatter="$(read_frontmatter "$task_file")"; then
			exit 3
		fi
		[ -n "$frontmatter" ] || continue

		tasks="$tasks$frontmatter
"
		queue="$queue$task_file$tab$frontmatter
"
	done

	check_queue || exit 3
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
	while IFS="$tab" read -r id depends exclusive owns; do
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

	printf '%s\n' "$tasks" | while IFS="$tab" read -r id depends exclusive owns; do
		[ -n "$id" ] || continue

		if claimed "$id"; then
			continue
		fi

		if [ "$exclusive" = yes ] && [ "$claims_in_flight" -ne 0 ]; then
			continue
		fi

		if parked "$id"; then
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
delete_claim() {
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

	while IFS="$tab" read -r id depends exclusive owns; do
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
		delete_claim "$1" "$head" || true
		exit 1
	fi

	if ! claimed "$1"; then
		echo "$(basename "$0"): claim on $1 is gone from origin" >&2
		exit 4
	fi

	if claim_conflict "$1"; then
		echo "$(basename "$0"): claim on $1 released: $conflict" >&2
		delete_claim "$1" "$head" || exit 1
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

# The shape origin reports a time in, which is also the shape the comparison
# above and the archive name below both assume. Anything else is refused where
# it arrives rather than compared as if it were a time.
is_timestamp() {
	case "$1" in
	[0-9][0-9][0-9][0-9]-[0-9][0-9]-[0-9][0-9]T[0-9][0-9]:[0-9][0-9]:[0-9][0-9]Z) return 0 ;;
	esac
	return 1
}

# That time, in what a branch name can carry.
stamp_suffix() {
	printf '%s-%s\n' "$(digits "${1%%T*}")" "$(digits "${1#*T}")"
}

# A name this may release is a whole task id and one path segment, which is what
# keeps every branch under escalated/ — including the archives written below —
# out of reach of everything that removes a ref. An id with a slash in it could
# name one, so it is refused rather than resolved.
own_branch_only() {
	case "$1" in
	*/*) return 1 ;;
	esac
	return 0
}

have_commit() {
	git cat-file -e "$1^{commit}" 2>/dev/null
}

# The remedy, which is a release and not a delete. A claim standing at a commit
# main already has is a marker and nothing else, and removing the ref loses
# nothing. A claim standing on commits main does not have is the only reachable
# copy of what a run did, and the run that would have opened a pull request for
# it is gone — so those commits move into escalated/, which no sweep looks at,
# and the claim is released either way. Which of the two a claim is, is a
# question about its commits, so it is asked of the commits.
#
# The archive is named for when origin last saw the claim alive — the fact that
# condemned it — and not for the moment a sweep noticed. Two sweeps of one dead
# claim then reach for the same name, and the second is refused by a lease that
# expects nothing rather than making a second copy of the same work.
#
# The archive and the release are one atomic push, so a record that has not
# landed can never outlive the claim that was the only other copy, and origin is
# read back afterwards: the point of the archive is that it holds what the claim
# held, and that is worth one question.
#
# Returns 0 released, 1 kept for the reason it printed, 2 could not release.
release_claim() {
	if ! own_branch_only "$1"; then
		echo "$1: kept, an id with a / could name a branch this must not touch"
		return 1
	fi

	main_sha="$(claim_head main)"
	if [ -z "$main_sha" ]; then
		echo "$1: kept, origin has no main to measure this claim against"
		return 1
	fi

	if ! have_commit "$2" || ! have_commit "$main_sha"; then
		if ! git fetch --quiet --no-tags origin \
			"refs/heads/$1" refs/heads/main; then
			echo "$1: kept, cannot read the commits this claim holds"
			return 1
		fi
	fi
	if ! have_commit "$2" || ! have_commit "$main_sha"; then
		echo "$1: kept, origin no longer holds the commits this claim was read at"
		return 1
	fi

	on_main=0
	git merge-base --is-ancestor "$2" "$main_sha" || on_main=$?
	case "$on_main" in
	0)
		delete_claim "$1" "$2" || return 2
		echo "$1: released, origin last saw it at $3, at $2"
		return 0
		;;
	1) ;;
	*)
		echo "$1: kept, cannot tell whether main has $2"
		return 1
		;;
	esac

	archive="escalated/$1-swept-$(stamp_suffix "$3")"
	if ! git push --quiet --atomic origin \
		--force-with-lease="refs/heads/$archive:" "$2:refs/heads/$archive" \
		--force-with-lease="refs/heads/$1:$2" ":refs/heads/$1"; then
		echo "$(basename "$0"): could not release $1; $2 is not on main and $archive did not land" >&2
		return 2
	fi

	if ! archived="$(git ls-remote origin "refs/heads/$archive")"; then
		echo "$(basename "$0"): released $1 but cannot read back $archive" >&2
		return 2
	fi
	archived="$(printf '%s\n' "$archived" | awk '{ print $1; exit }')"
	if [ "$archived" != "$2" ]; then
		echo "$(basename "$0"): $archive holds $archived, not the $2 the claim on $1 held" >&2
		return 2
	fi

	echo "$1: released, origin last saw it at $3, kept as $archive at $archived"
}

# Only a branch whose whole name is a task id still in the queue is a candidate.
# That is what keeps main, session/… and escalated/… out of reach: an
# escalated/M1-08-… branch is the record of a run that failed, not a claim, and a
# record is not something to tidy away.
sweep() {
	read_branches || exit 1
	read_queue

	if ! cutoff="$(claim_cutoff)"; then
		echo "$(basename "$0"): cannot work out how old a claim would have to be" >&2
		exit 1
	fi

	claims=""
	while IFS="$tab" read -r id depends exclusive owns; do
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
		if ! is_timestamp "$stamp"; then
			echo "$id: kept, origin dates this claim in a shape this cannot read: $stamp"
			continue
		fi
		if [ "$(digits "$stamp")" -ge "$(digits "$cutoff")" ]; then
			echo "$id: kept, origin last saw this claim at $stamp"
			continue
		fi

		outcome=0
		release_claim "$id" "$(claim_head "$id")" "$stamp" || outcome=$?
		if [ "$outcome" -eq 2 ]; then
			stuck=$((stuck + 1))
		fi
	done

	if [ "$stuck" -ne 0 ]; then
		exit 1
	fi
}

# The same remedy by hand, for the claim a sweep reported it could not release
# and for the run that knows its own claim is dead before origin's record does.
# It decides nothing a sweep decides: age and the open pull request are what
# make a claim releasable, and the caller has already answered both. What it
# will not do is release anything a sweep could not have — the id must be a task
# in the queue, and origin must hold that claim at the commit named, or this is
# not a release but a delete of whatever ref was passed.
release_one() {
	read_branches || exit 1
	read_queue

	if ! printf '%s' "$tasks" | cut -f1 | grep -Fxq "$1"; then
		echo "$(basename "$0"): $1 is not a task in the queue" >&2
		exit 2
	fi
	if ! is_timestamp "$3"; then
		echo "$(basename "$0"): $3 is not a time in the shape origin reports" >&2
		exit 2
	fi
	if ! claimed "$1"; then
		echo "$(basename "$0"): origin holds no claim on $1" >&2
		exit 1
	fi
	if [ "$(claim_head "$1")" != "$2" ]; then
		echo "$(basename "$0"): origin holds the claim on $1 at $(claim_head "$1"), not $2" >&2
		exit 1
	fi

	release_claim "$1" "$2" "$3" || exit 1
}

# Reading the queue, and nothing else. Every other subcommand reads it on the
# way to something, and each of those also needs origin — so this is the one
# form a gate can run against a checked-out tree that has no remote at all. It
# decides nothing and reports the size of what it read, because a check that
# passed over an empty directory and a check that passed read the same on the
# way past.
check() {
	count=0
	read_queue

	while IFS="$tab" read -r id depends exclusive owns; do
		[ -n "$id" ] || continue
		count=$((count + 1))
	done <<<"$tasks"

	echo "$count tasks, no structural collision"
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
release)
	[ "$#" -eq 4 ] || usage
	release_one "$2" "$3" "$4"
	;;
check)
	[ "$#" -eq 1 ] || usage
	check
	;;
*)
	usage
	;;
esac
