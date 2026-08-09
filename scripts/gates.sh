#!/usr/bin/env bash
# Usage: scripts/gates.sh              — run every gate, then prove each one catches.
#        scripts/gates.sh --gates-only — run the gates and stop.
#        scripts/gates.sh --gates-only <gate>...
#                                      — run the gates named and stop. This is the
#                                        form a proof runs inside its scratch copy,
#                                        naming the one gate it is a proof about.
#
# Every gate says what it cost, on every form, and the sweep, each proof, and the
# run as a whole do the same.
#
# Exit codes: 0 every gate passed and every proof caught, 1 a gate failed or the
# gates the runner has are not the gates expected, 2 bad invocation, 3 a proof
# did not hold or could not be run.

set -euo pipefail

cd "$(dirname "$0")/.."

prog="$(basename "$0")"
tab="$(printf '\t')"

# Every gate, proof, and sweep says what it cost, on every run, because the last
# time this suite outgrew a worker's turn the number had to be inferred after the
# fact. A mark is a reading of SECONDS, which a subshell carries over from the
# shell that started it, so a step timed inside one measures the same clock.
#
# Whole seconds, from the shell itself: bash 3.2 is what this runs under and it
# has no finer clock that does not cost a process to read, and what this exists to
# expose is measured in minutes.
since() {
	local elapsed
	elapsed=$((SECONDS - $1))
	if [ "$elapsed" -ge 60 ]; then
		printf '%dm%02ds\n' "$((elapsed / 60))" "$((elapsed % 60))"
	else
		printf '%ds\n' "$elapsed"
	fi
}

# How much of the machine this run takes at once, read off the machine rather
# than written down, because the fleet machine and a laptop are not the same
# size. Capped, because several workers share that machine: a run that took all
# of it would cost the others more than it saved itself. A machine that will not
# say how many it has gets a number any machine can carry.
machine_lanes() {
	local count
	count="$(getconf _NPROCESSORS_ONLN 2>/dev/null || true)"
	case "$count" in
	"" | *[!0-9]*) count=4 ;;
	esac
	if [ "$count" -lt 1 ]; then
		count=1
	elif [ "$count" -gt 8 ]; then
		count=8
	fi
	printf '%s\n' "$count"
}

at_once="$(machine_lanes)"

# The gates, in the order they run. Every gate AGENTS.md names now has machinery
# behind it. A gate that has machinery and nothing to check yet still belongs
# here: contracts runs its real check over the contracts that exist, which today
# is none of them. What would not belong is a name stubbed green, because a gate
# that cannot fail reads exactly like a gate that is holding.
#
# Three names here are not on AGENTS.md's list. scripts is over the operational
# scripts themselves, which that list passes over, and it runs first because it
# is the cheapest and because every other gate is reached through a script.
# queue is the one WORKPLAN.md asks for — "the queue gate refuses structural
# collisions" — and it runs second, over the queue this repository holds, once
# the gate above has pinned what reading a queue does. egress is the one GOALS.md
# asks for at M3 — "the fetcher cannot reach a host outside the allowed list.
# This is checked, not intended" — and it sits beside deps and purity, the other
# two that read the tree for a shape an anchor fixes rather than running it.
#
# This is the only place the set is written down, and it is checked against the
# gates the file actually defines before any of them runs. Both loops below read
# it — the one that runs the gates and the one that proves them — so a name
# deleted here would take its own proof with it, and the run would end green
# having checked less. The check is what makes removing a gate cost a visible
# line in a file that cannot land without a signature.
expected_gates() {
	sed 's/#.*//' <<-'EOF' | awk 'NF'
	scripts
	queue
	build
	tests
	fixtures
	deps
	purity
	egress
	contracts
	benchmark
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

# AGENTS.md names seven gates and none of them is over scripts/, which is how
# that directory came to hold the claim protocol with nothing checking it: every
# behaviour verified in review was verified in that review and nowhere after.
# This gate is the one this file adds to AGENTS.md's list, and it has two halves.
#
# Every script under scripts/ parses under the shell it declares. A file there is
# a script when its first line is a shebang, and an executable that declares none
# is refused rather than skipped — otherwise dropping the shebang would drop the
# check with it.
#
# Every corpus under scripts/tests/ runs. A corpus is a directory named for the
# script it pins, holding one <case>.case file apiece, and a case is that script
# run once in a scratch repository with a real bare remote. Which corpora run is
# read off the directory rather than listed here, for the reason fixtures gives:
# a list would be a second place saying which scripts are pinned, and the copy
# that drifts is the one nobody runs.
script_corpora() {
	local dir
	for dir in scripts/tests/*/; do
		if [ -d "$dir" ]; then
			dir="${dir%/}"
			printf '%s\n' "${dir#scripts/tests/}"
		fi
	done
}

# Every file under scripts/, script or not. What makes one a script is read per
# file rather than guessed from its name: a corpus case is a .case and a script
# is a .sh today, and neither of those is what the kernel reads.
script_files() {
	find scripts -type f | sort
}

# The shell a script declares, from its shebang: the interpreter is the word
# after env when the line runs env, and the program itself otherwise. Anything
# else — a flag, a wrapper, a stray carriage return — comes out as a name no
# syntax check matches, which is refused rather than skipped.
declared_shell() {
	local line rest first
	IFS= read -r line <"$1" || return 1
	case "$line" in
	'#!'*) ;;
	*) return 1 ;;
	esac

	rest="${line#\#!}"
	first="${rest%% *}"
	if [ "${first##*/}" = env ]; then
		rest="${rest#* }"
		first="${rest%% *}"
	fi
	printf '%s\n' "${first##*/}"
}

# One script against the shell it declares. Anything wrong prints a line on
# stdout for the gate to collect; a return of 1 means the check could not be made
# at all, which is not the same answer as a script that failed it.
check_script() {
	local file shell output
	file="$1"

	if ! shell="$(declared_shell "$file")"; then
		if [ -x "$file" ]; then
			printf '  %s: is executable and declares no shell to parse it under\n' "$file"
		fi
		return 0
	fi

	case "$shell" in
	bash | sh) ;;
	*)
		printf '  %s: declares %s, and this gate has no syntax check for it\n' "$file" "$shell"
		return 0
		;;
	esac

	if ! command -v "$shell" >/dev/null 2>&1; then
		echo "$prog: no $shell on this machine to parse $file with" >&2
		return 1
	fi

	if ! output="$("$shell" -n "$file" 2>&1)"; then
		printf '  %s: does not parse under %s: %s\n' "$file" "$shell" \
			"$(printf '%s\n' "$output" | head -1)"
	fi
}

# A case is a flat file of directives, one per line, whose body is the
# tab-indented lines under it. The tab is stripped and what is left is content
# byte for byte, which is what lets a case carry a value with a tab inside it or
# whitespace on the end of it — both of which this script has been wrong about
# before. Any other line ends the body above it, so an empty line inside one is
# written as a lone tab.
#
#   run <words>   the arguments the script is run with; none means none
#   exit <code>   the exit code the case pins
#   file <path>   a file the scratch repository holds, its body the content
#   claims <ids>  the branches on origin before the run, one per claim; an id
#                 written <id>@elsewhere is a claim pushed from another commit
#   after <ids>   the branches on origin after it; absent means claims, unmoved
#   stdout        the output pinned, exactly
#   stderr        the script's own stderr, pinned exactly
#   unreachable   origin cannot be reached for the length of this case
#
# stdout belongs to the script alone, so it is compared whole. stderr does not: a
# push the lease refuses prints git's own line before the script says anything,
# and that line carries a path and a git version, neither of which is behaviour
# to pin. So the pinned lines are matched against the tail, and nothing above the
# tail may be the script's own voice — a second message from it is a change even
# when the last line still matches.

# The template scratch repository, built once per corpus and copied per case.
# What a case needs from HEAD is a commit to push, not a tree, so the commit is
# empty.
#
# The same commit is made again inside the remote, so a claim can be planted
# there holding the very commit the run would push — the case git calls a success
# and the script has to tell apart from a claim of its own. The other commit is
# what someone else's claim looks like. Both are made where they are needed,
# because an object no ref names is still an object the repository has, and the
# two repositories agreeing on the first one is checked rather than assumed:
# were they to disagree, the two claim cases would silently become one.
lab_build() {
	local lab script empty here
	lab="$1"
	script="$2"

	git init -q --initial-branch=claimant "$lab/work" || return 1
	git init -q --initial-branch=claimant --bare "$lab/remote.git" || return 1
	git -C "$lab/work" remote add origin "$lab/remote.git" || return 1

	mkdir -p "$lab/work/scripts" || return 1
	cp "scripts/$script" "$lab/work/scripts/$script" || return 1
	chmod +x "$lab/work/scripts/$script" || return 1

	empty="$(git -C "$lab/work" hash-object -t tree /dev/null)" || return 1
	here="$(git -C "$lab/work" commit-tree "$empty" -m base)" || return 1
	git -C "$lab/work" update-ref refs/heads/claimant "$here" || return 1

	git -C "$lab/remote.git" commit-tree "$empty" -m base >"$lab/here" || return 1
	git -C "$lab/remote.git" commit-tree "$empty" -m elsewhere >"$lab/elsewhere" || return 1

	if [ "$here" != "$(cat "$lab/here")" ]; then
		echo "$prog: the scratch repository and its remote do not agree on a commit" >&2
		return 1
	fi
}

# A case's own copy of the template, which is what a case gets instead of one
# repository undone between cases. It carries over nothing by construction rather
# than by remembering to delete, and copies can be made at the same time where
# resets have to take turns.
#
# The remote is repointed because what the template recorded is a path outside
# the copy: left alone, every case would push at the template's remote and see
# every other case's claims.
lab_clone() {
	local template lab
	template="$1"
	lab="$2"

	cp -R "$template" "$lab" || return 1
	git -C "$lab/work" remote set-url origin "$lab/remote.git" || return 1
}

# What a failed comparison shows, so a red case says what happened rather than
# only that something did.
show_lines() {
	if [ -s "$2" ]; then
		sed "s/^/      $1: /" "$2"
	else
		printf '      %s: nothing\n' "$1"
	fi
}

# One case, planted and run. Anything wrong prints a line on stdout for the
# corpus to collect; a return of 1 means the case could not be run at all.
run_corpus_case() {
	local lab script file name line key rest target
	local args pinned_exit claims after has_after unreachable
	local code id ref commit pinned_lines said_lines above

	lab="$1"
	script="$2"
	file="$3"
	name="${file##*/}"
	name="${name%.case}"

	: >"$lab/pinned-stdout"
	: >"$lab/pinned-stderr"

	args=""
	pinned_exit=""
	claims=""
	after=""
	has_after=no
	unreachable=no
	target=""

	while IFS= read -r line || [ -n "$line" ]; do
		case "$line" in
		"$tab"*)
			if [ -z "$target" ]; then
				printf '  %s: a body with no directive above it\n' "$name"
				return 0
			fi
			printf '%s\n' "${line#"$tab"}" >>"$target"
			continue
			;;
		esac

		target=""
		case "$line" in
		"" | "#"*) continue ;;
		esac

		key="${line%% *}"
		case "$line" in
		*" "*) rest="${line#* }" ;;
		*) rest="" ;;
		esac

		case "$key" in
		run) args="$rest" ;;
		exit) pinned_exit="$rest" ;;
		claims) claims="$rest" ;;
		after)
			after="$rest"
			has_after=yes
			;;
		unreachable) unreachable=yes ;;
		stdout) target="$lab/pinned-stdout" ;;
		stderr) target="$lab/pinned-stderr" ;;
		file)
			target="$lab/work/$rest"
			mkdir -p "${target%/*}"
			: >"$target"
			;;
		*)
			printf '  %s: no directive named %s\n' "$name" "$key"
			return 0
			;;
		esac
	done <"$file"

	if [ -z "$pinned_exit" ]; then
		printf '  %s: pins no exit code\n' "$name"
		return 0
	fi

	for id in $claims; do
		case "$id" in
		*@elsewhere)
			ref="${id%@elsewhere}"
			commit="$(cat "$lab/elsewhere")"
			;;
		*)
			ref="$id"
			commit="$(cat "$lab/here")"
			;;
		esac
		git -C "$lab/remote.git" update-ref "refs/heads/$ref" "$commit" || return 1
	done

	if [ "$unreachable" = yes ]; then
		git -C "$lab/work" remote set-url origin "$lab/no-remote-of-this-name.git" || return 1
	fi

	code=0
	"$lab/work/scripts/$script" $args >"$lab/stdout" 2>"$lab/stderr" || code=$?

	if [ "$code" != "$pinned_exit" ]; then
		printf '  %s: exits %s where the case pins %s\n' "$name" "$code" "$pinned_exit"
	fi

	if ! cmp -s "$lab/stdout" "$lab/pinned-stdout"; then
		printf '  %s: says something other than what it pins on stdout\n' "$name"
		show_lines pinned "$lab/pinned-stdout"
		show_lines said "$lab/stdout"
	fi

	pinned_lines="$(awk 'END { print NR }' <"$lab/pinned-stderr")"
	said_lines="$(awk 'END { print NR }' <"$lab/stderr")"
	if [ "$pinned_lines" -gt "$said_lines" ] ||
		! tail -n "$pinned_lines" "$lab/stderr" | cmp -s - "$lab/pinned-stderr"; then
		printf '  %s: says something other than what it pins on stderr\n' "$name"
		show_lines pinned "$lab/pinned-stderr"
		show_lines said "$lab/stderr"
	fi

	above=$((said_lines - pinned_lines))
	if [ "$above" -gt 0 ]; then
		head -n "$above" "$lab/stderr" >"$lab/stderr-above"
		while IFS= read -r line; do
			case "$line" in
			"$script:"* | usage:*)
				printf '  %s: says more on stderr than the case pins: %s\n' "$name" "$line"
				;;
			esac
		done <"$lab/stderr-above"
	fi

	if [ "$has_after" = no ]; then
		after="$claims"
	fi
	: >"$lab/pinned-claims"
	for id in $after; do
		printf '%s\n' "${id%@elsewhere}" >>"$lab/pinned-claims"
	done
	sort -o "$lab/pinned-claims" "$lab/pinned-claims"
	git -C "$lab/remote.git" for-each-ref --format='%(refname:short)' refs/heads |
		sort >"$lab/claims" || return 1

	if ! cmp -s "$lab/claims" "$lab/pinned-claims"; then
		printf '  %s: leaves origin holding claims the case does not pin\n' "$name"
		show_lines pinned "$lab/pinned-claims"
		show_lines held "$lab/claims"
	fi
}

# The corpus for one script. Every case is one run of that script against a
# repository of its own, so no case can see another's, and the order they run in
# was never part of what any of them pins — which is what lets them run at once.
# They did not, once, and the sequential corpus was most of what the whole suite
# cost.
#
# What each case reports is written to a file named for its place in the corpus,
# and the files are read back in that order afterwards. Two cases printing down
# the same pipe would interleave, and a corpus that reported its problems in
# whatever order they finished would say something different every run.
#
# The environment is closed off deliberately, and this runs inside a command
# substitution, which is what keeps the exports below from reaching the rest of
# the run. The machine's git configuration, its identity, and its terminal are
# all outside the repository, and a case whose answer depended on any of them
# would pass or fail by where it ran.
run_corpus() {
	local script root case_file index place marker found

	script="$1"

	export GIT_CONFIG_GLOBAL=/dev/null GIT_CONFIG_SYSTEM=/dev/null
	export GIT_AUTHOR_NAME=vfi GIT_AUTHOR_EMAIL=vfi@invalid
	export GIT_COMMITTER_NAME=vfi GIT_COMMITTER_EMAIL=vfi@invalid
	export GIT_AUTHOR_DATE='@0 +0000' GIT_COMMITTER_DATE='@0 +0000'
	export GIT_TERMINAL_PROMPT=0

	root="$(mktemp -d "${TMPDIR:-/tmp}/vfi-corpus.XXXXXX")" || return 1
	if ! lab_build "$root/template" "$script"; then
		rm -rf "$root"
		return 1
	fi

	index=0
	for case_file in "scripts/tests/$script"/*.case; do
		[ -f "$case_file" ] || continue
		index=$((index + 1))
		place="$(printf '%04d' "$index")"
		(
			if lab_clone "$root/template" "$root/case-$place" &&
				run_corpus_case "$root/case-$place" "$script" "$case_file"; then
				exit 0
			fi
			# A case that could not be run at all is a different answer from one
			# that failed, and it has to survive the subshell it ran in.
			: >"$root/broken-$place"
		) >"$root/found-$place" &
		if [ "$((index % at_once))" -eq 0 ]; then
			wait
		fi
	done
	wait

	for marker in "$root"/broken-*; do
		if [ -e "$marker" ]; then
			rm -rf "$root"
			return 1
		fi
	done

	for found in "$root"/found-*; do
		[ -f "$found" ] || continue
		cat "$found"
	done

	rm -rf "$root"
}

gate_scripts() {
	local file corpus corpora found problems parsed corpus_count case_file cases pinned

	# The checker before what it checks: its cases sit with the proofs below.
	check_script_cases || return 1

	problems=""
	parsed=0
	for file in $(script_files); do
		if declared_shell "$file" >/dev/null; then
			parsed=$((parsed + 1))
		fi
		found="$(check_script "$file")" || return 1
		if [ -n "$found" ]; then
			problems="$problems$found
"
		fi
	done

	corpora="$(script_corpora)" || return 1
	corpus_count=0
	cases=0
	for corpus in $corpora; do
		corpus_count=$((corpus_count + 1))
		if [ ! -f "scripts/$corpus" ]; then
			problems="$problems  $corpus: a corpus for a script scripts/ does not have
"
			continue
		fi

		# Counted here rather than inside the run, so that one place says how
		# much was pinned and an empty corpus cannot report itself clean.
		pinned=0
		for case_file in "scripts/tests/$corpus"/*.case; do
			[ -f "$case_file" ] || continue
			pinned=$((pinned + 1))
		done
		if [ "$pinned" -eq 0 ]; then
			problems="$problems  $corpus: the corpus holds no case, so it pins nothing
"
			continue
		fi
		cases=$((cases + pinned))

		found="$(run_corpus "$corpus")" || return 1
		if [ -n "$found" ]; then
			problems="$problems$found
"
		fi
	done

	if [ -n "$problems" ]; then
		echo "$prog: the operational scripts are not what is pinned of them:" >&2
		printf '%s' "$problems" >&2
		return 1
	fi

	# As with fixtures, an empty set here is not an absence: the corpus is
	# committed, so nothing under scripts/tests/ means the corpus was deleted.
	if [ "$corpus_count" -eq 0 ]; then
		echo "$prog: scripts/tests/ holds no corpus, so this gate checks nothing" >&2
		return 1
	fi

	echo "scripts: $parsed parse under the shell they declare, $cases cases pinned"
}

# WORKPLAN.md's "the queue gate refuses structural collisions". The rule is in
# scripts/tasks.sh, because reading the queue is what refuses and every
# subcommand reads it; the gate above pins what that read does, over queues
# planted for the purpose, and this one aims it at the queue this repository is
# actually holding. Two agents claiming tasks that own the same paths is the
# collision, and the moment it can be caught is while the task file is still a
# diff on a branch.
#
# check is the read with nothing else attached, which is what a scratch copy
# with no .git in it can answer. Every other subcommand reaches origin first and
# would fail here for a reason that is not about the queue.
#
# An empty queue is green, and that is not the absence fixtures and contracts
# guard against. A drained queue is what finishing a milestone looks like: the
# branch that completes a task deletes its file, so the last task merging leaves
# tasks/ holding nothing, and a gate that called that a failure would go red on
# the project's best day. What stands in for the missing-baseline check is the
# count, which says out loud how much was read.
gate_queue() {
	local summary
	summary="$(scripts/tasks.sh check)" || return 1
	echo "queue: $summary"
}

gate_build() {
	cargo build --workspace
}

gate_tests() {
	cargo test --workspace
}

# AGENTS.md's "the golden fixtures still produce their expected results". A
# fixture is a directory under fixtures/<stage>/ holding what goes into that
# stage and what must come out, both committed; the harness that runs them is
# code and lives with the crate it exercises (docs/layout.md), as that crate's
# `golden` test target. Anything about the shape of a fixture is written down
# there, with the code that reads it.
#
# Which harnesses run is read off fixtures/ rather than listed here, because a
# list would be a second place saying which stages have fixtures and the copy
# that drifts is the one nobody runs. A stage directory whose crate has no
# `golden` target turns this red, which is the answer that fits: a committed
# fixture nothing runs is worth less than no fixture at all.
fixture_stages() {
	local dir
	for dir in fixtures/*/; do
		if [ -d "$dir" ]; then
			dir="${dir%/}"
			printf '%s\n' "${dir#fixtures/}"
		fi
	done
}

gate_fixtures() {
	local stages stage count
	stages="$(fixture_stages)" || return 1

	count=0
	for stage in $stages; do
		count=$((count + 1))
		cargo test -p "vfi-$stage" --test golden || return 1
	done

	# Unlike contracts, an empty set here is not an absence. The baseline is
	# committed, so nothing under fixtures/ means the baseline was deleted, and
	# a sweep that passes over nothing reads exactly like one that is holding.
	if [ "$count" -eq 0 ]; then
		echo "$prog: fixtures/ holds no stage, so this gate checks nothing" >&2
		return 1
	fi
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

# GOALS.md at M3: the fetcher cannot reach a host outside the allowed list, and
# that is checked rather than intended. The list is one file in vfi-fetch, and
# the chokepoint below is the only code that reads it and the only way out of
# the stage. Which hosts are on the list is not this gate's business; that a
# request can leave without meeting it is.
#
# Two things stand between a call site and a host nobody allowed, and only one
# of them is here. A transport is handed a value the chokepoint alone can
# construct, so a call site that tried to send around the list does not compile
# — the build gate is that half, and it holds without anything being written
# here. What the compiler cannot see is a call site that wants no transport:
# opening a connection directly is in every crate's reach, and a connection
# opened that way is a request that never met the list. So the names that open
# one may appear inside the chokepoint and nowhere else in the workspace.
#
# This reads source rather than the resolved tree, because linking a client is
# not yet a request — that is purity's shape of check, and purity is asking a
# different question about a different crate. What it cannot see is a name it
# has never been told about, and that is why the client packages come from
# denied_packages rather than a second list here: that is where this project
# writes down what reaches the wire, so a new one is added once and both gates
# learn it.
#
# Two things stay out of its sight, said here rather than left to be found: a
# downloader run as a process, which is std::process and not one of the names
# below; and a name spelled some other way, since this reads text and an alias
# would carry a socket past it. Neither is the case it exists for — a call site
# that reached out because reaching out was the shorter way — and both are the
# kind of thing a reader sees in a diff.
chokepoint="crates/fetch/src/egress"

wire_names() {
	printf '%s\n' std::net TcpStream TcpListener UdpSocket ToSocketAddrs
	denied_packages | awk '$2 == "network" { gsub(/-/, "_", $1); print $1 "::" }'
}

# Every Rust source in the tree, wherever a crate sits. target/ is whatever the
# last build left behind rather than anything committed, and .git holds no
# source at all.
rust_sources() {
	find . -name '*.rs' -not -path './target/*' -not -path './.git/*' | sort
}

gate_egress() {
	local file found problems inside outside

	if [ ! -d "$chokepoint" ]; then
		echo "$prog: $chokepoint is gone, so there is no chokepoint for this to hold" >&2
		return 1
	fi

	problems=""
	inside=0
	outside=0
	for file in $(rust_sources); do
		case "$file" in
		"./$chokepoint"/*)
			inside=$((inside + 1))
			continue
			;;
		esac
		outside=$((outside + 1))
		if found="$(grep -Fn -f <(wire_names) "$file")"; then
			problems="$problems$(printf '%s\n' "$found" | sed "s|^|  ${file#./}:|")
"
		fi
	done

	if [ -n "$problems" ]; then
		echo "$prog: these open a connection outside $chokepoint, where no list is read:" >&2
		printf '%s' "$problems" >&2
		return 1
	fi

	# Neither count may be nothing. An empty chokepoint is one that was deleted
	# or moved, and a tree with no source outside it is one this gate swept over
	# without checking anything — both read exactly like a gate that is holding.
	if [ "$inside" -eq 0 ]; then
		echo "$prog: $chokepoint holds no source, so nothing goes through it" >&2
		return 1
	fi
	if [ "$outside" -eq 0 ]; then
		echo "$prog: no source outside $chokepoint, so this gate checks nothing" >&2
		return 1
	fi

	echo "egress: $inside in the chokepoint may reach a host, $outside elsewhere cannot"
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

# AGENTS.md's "the benchmark shows no regression past the set threshold". A
# workload is a directory under benchmarks/<stage>/ holding what the stage runs
# over and what it cost, both committed; the thresholds it is allowed to drift by
# are benchmarks/thresholds, one file for every stage. The harness that measures
# is code and lives with the crate it exercises (docs/layout.md), as that crate's
# `bench` test target. What it measures, and why none of it is a wall clock, is
# written down there with the code that does it.
#
# Which harnesses run is read off benchmarks/ rather than listed here, for the
# reason fixtures gives: a list would be a second place saying which stages have a
# workload, and the copy that drifts is the one nobody runs. benchmarks/thresholds
# is a file, so the glob passes over it.
#
# Release is not a preference. The committed baseline describes optimized code,
# and the same numbers taken from a debug build measure something else.
benchmark_stages() {
	local dir
	for dir in benchmarks/*/; do
		if [ -d "$dir" ]; then
			dir="${dir%/}"
			printf '%s\n' "${dir#benchmarks/}"
		fi
	done
}

gate_benchmark() {
	local stages stage count
	stages="$(benchmark_stages)" || return 1

	count=0
	for stage in $stages; do
		count=$((count + 1))
		cargo test -p "vfi-$stage" --test bench --release || return 1
	done

	# As with fixtures, an empty set here is not an absence: the baseline is
	# committed, so nothing under benchmarks/ means the baseline was deleted.
	if [ "$count" -eq 0 ]; then
		echo "$prog: benchmarks/ holds no stage, so this gate checks nothing" >&2
		return 1
	fi
}

# The cases the syntax half must get right, one apiece, each a file differing in
# one way, and what the checker must say about it. They run in-band with the
# gate, for the reason the contract cases do: a checker that has quietly stopped
# checking must not be able to report a clean scripts/.
script_cases() {
	sed 's/#.*//' <<-'EOF' | awk 'NF'
	parses                clean
	does-not-parse        caught
	posix-parses          clean
	posix-does-not-parse  caught
	data                  clean
	executable-data       caught
	unknown-shell         caught
	EOF
}

plant_script_case() {
	local file
	mkdir -p "$1"
	file="$1/subject"

	case "$2" in
	parses) printf '#!/usr/bin/env bash\ntrue\n' >"$file" ;;
	does-not-parse) printf '#!/usr/bin/env bash\nif true; then\n' >"$file" ;;
	posix-parses) printf '#!/bin/sh\ntrue\n' >"$file" ;;
	posix-does-not-parse) printf '#!/bin/sh\nif true; then\n' >"$file" ;;
	data) printf 'a line of data, with no shell to run it under\n' >"$file" ;;
	executable-data)
		printf 'a line of data, in a file the kernel would be asked to run\n' >"$file"
		chmod +x "$file"
		;;
	unknown-shell) printf '#!/usr/bin/env perl\nprint "hello";\n' >"$file" ;;
	*)
		echo "$prog: no script case named $2" >&2
		return 1
		;;
	esac
}

check_script_cases() {
	local lab name expectation found verdict failures

	lab="$(mktemp -d "${TMPDIR:-/tmp}/vfi-script-cases.XXXXXX")" || return 1

	failures=""
	while read -r name expectation; do
		if ! plant_script_case "$lab/$name" "$name"; then
			failures="$failures  $name: the case could not be built
"
			continue
		fi
		if ! found="$(check_script "$lab/$name/subject")"; then
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
	$(script_cases)
	EOF

	rm -rf "$lab"

	if [ -n "$failures" ]; then
		echo "$prog: the script checker does not hold to its own rule:" >&2
		printf '%s' "$failures" >&2
		return 1
	fi
}

# A guard that stops guarding, injected where only the corpus can see it: the
# exclusive value read off a task file is thrown away, so every task reads as
# ordinary and an exclusive one is offered beside a claim it may not stand with.
# Nothing about it is a syntax error and nothing about it touches the workspace —
# a corpus that only caught a script which no longer parses would not be pinning
# behaviour, and a violation the other gates could see would not be proving this
# one.
violate_scripts() {
	local script opened
	script="$1/scripts/tasks.sh"
	opened="$1.opened"

	sed 's/exclusive = tolower(value)/exclusive = "no"/' "$script" >"$opened" || {
		rm -f "$opened"
		return 1
	}
	if cmp -s "$script" "$opened"; then
		rm -f "$opened"
		echo "$prog: tasks.sh has no exclusive value to throw away" >&2
		return 1
	fi
	cat "$opened" >"$script"
	rm -f "$opened"
}

# The queue fixtures. They are planted rather than taken from tasks/, because
# tasks/ drains to nothing as a milestone finishes — a proof that needed two
# real queued tasks would go red on the day the queue emptied, for a reason that
# is nothing to do with this gate.
#
# This is the shape the rule allows: two tasks own the same crate, and one waits
# on the other, so they can never be claimed at the same time and there is
# nothing for them to collide over. The copy carrying this must stay green,
# because a gate that refused every overlap it saw would catch the violation
# below just as well.
accept_queue() {
	mkdir -p "$1/tasks"

	cat >"$1/tasks/M0-90.md" <<'EOF'
---
id: M0-90
title: The task the other one waits for
milestone: M0
owns:
  - crates/fixture
depends_on: []
exclusive: no
acceptance:
  - Nothing. This is a fixture for the queue gate's proof, not work.
---
EOF

	cat >"$1/tasks/M0-91.md" <<'EOF'
---
id: M0-91
title: The task that waits
milestone: M0
owns:
  - crates/fixture/src
depends_on: [M0-90]
exclusive: no
acceptance:
  - Nothing. This is a fixture for the queue gate's proof, not work.
---
EOF
}

# The same two tasks with the order taken out of them, which is the only
# difference. Both are now claimable at once over one crate, and two agents
# would edit the same files on separate branches with nothing between them.
violate_queue() {
	local waiting="$1/tasks/M0-91.md" freed="$1.freed"
	accept_queue "$1" || return 1

	sed 's/^depends_on: \[M0-90\]$/depends_on: []/' "$waiting" >"$freed" || {
		rm -f "$freed"
		return 1
	}
	if cmp -s "$waiting" "$freed"; then
		rm -f "$freed"
		echo "$prog: the queue fixture has no dependency to take out" >&2
		return 1
	fi
	cat "$freed" >"$waiting"
	rm -f "$freed"
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

# The whole of this gate is the comparison, so the violation is an expected
# result that no longer says what the stage produces. Whichever committed
# fixture comes first will do: the gate has to catch a drifted expectation
# wherever in fixtures/ it sits, and singling one out by name would leave the
# proof passing while the rest of the sweep had quietly stopped running.
#
# The perturbed copy still builds and still passes the test gate, because the
# harness is the one target `cargo test` does not select. That is what lets this
# violation go red under its own name and no other.
violate_fixtures() {
	local expected
	for expected in "$1"/fixtures/*/*/expected; do
		if [ -f "$expected" ]; then
			printf 'a line the stage does not produce\n' >>"$expected"
			return 0
		fi
	done
	echo "$prog: no committed fixture to perturb the expected result of" >&2
	return 1
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

# One call site that opens a connection of its own, appended to $1. Where it
# lands is the whole difference between the two proofs below: the same lines are
# what the chokepoint is for and what it exists to keep out, and a gate that
# could not tell those apart by place would be reading something else.
plant_connection() {
	cat >>"$1" <<'EOF'

/// Planted by the egress proof: a call site that opens a connection itself.
pub fn straight_out(host: &str) -> std::io::Result<std::net::TcpStream> {
    std::net::TcpStream::connect(host)
}
EOF
}

# Whichever source in the chokepoint comes first, for the reason crate_source
# gives: naming one would leave this proof passing over a file that had been
# renamed out from under it.
chokepoint_source() {
	local candidate
	for candidate in "$1/$chokepoint"/*.rs; do
		if [ -e "$candidate" ]; then
			echo "$candidate"
			return 0
		fi
	done
	echo "$prog: no source in $chokepoint to allow a connection inside" >&2
	return 1
}

# The connection where the rule allows it. Reaching a source is what the
# chokepoint is for, and a gate that refused this would be refusing the thing it
# protects — and would catch the violation below just as well.
accept_egress() {
	local file
	file="$(chokepoint_source "$1")" || return 1
	plant_connection "$file"
}

# The same connection opened somewhere else: a call site that skips the list
# entirely, in a crate that has never heard of it. It compiles, so the build
# gate has nothing to say about it, and it changes no output, so nothing that
# compares results does either — which is what leaves it to this gate and no
# other. Whichever crate source comes first, for the reason above.
violate_egress() {
	local file
	file="$(crate_source "$1")" || return 1
	plant_connection "$file"
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

# Work the stage did not do before, on every call: the slowdown a benchmark gate
# exists to catch. It leaves the output exactly as it was, because fixtures runs
# first and a violation that changed what the stage produces would go red there
# and prove nothing about this gate.
#
# Injected into whichever benched stage comes first and into whatever its first
# public function is — singling one out by name would leave this proof passing
# while the rest of the sweep had quietly stopped running. The insertion is
# checked rather than assumed: a signature this cannot find is a proof that could
# not be made, which is a different answer from a gate that failed to catch, and
# it says so instead of leaving a copy that was never slowed down.
violate_benchmark() {
	local dir stage source slowed
	for dir in "$1"/benchmarks/*/; do
		[ -d "$dir" ] || continue
		dir="${dir%/}"
		stage="${dir##*/}"
		source="$1/crates/${stage}/src/lib.rs"
		[ -f "$source" ] || continue

		slowed="$source.slowed"
		awk '
			!slowed && /^pub fn / && /\{$/ {
				print
				print "    std::hint::black_box(String::from(\"a deliberate slowdown\"));"
				slowed = 1
				next
			}
			{ print }
		' "$source" >"$slowed" || {
			rm -f "$slowed"
			return 1
		}

		if cmp -s "$source" "$slowed"; then
			rm -f "$slowed"
			echo "$prog: $stage has no signature on one line to slow down" >&2
			return 1
		fi
		cat "$slowed" >"$source"
		rm -f "$slowed"
		return 0
	done

	echo "$prog: no benched stage to inject a slowdown into" >&2
	return 1
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
	echo "usage: $prog [--gates-only [gate...]]" >&2
	exit 2
}

# A name --gates-only will answer to: a gate the runner defines, or gate_set for
# the assertion over the set itself. Read off the runner rather than the
# expectation, so that the copy a gate_set proof has trimmed can still be asked
# for the check it trimmed.
#
# Not named gate_something: what the runner counts as a gate is every function
# with that prefix, so a helper wearing it is a gate the expectation does not
# name, and the assertion says so.
known_gate() {
	local defined
	case "$1" in
	gate_set) return 0 ;;
	esac
	defined=" $(defined_gates | tr '\n' ' ')"
	case "$defined" in
	*" $1 "*) return 0 ;;
	esac
	return 1
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

# Every gate, or the ones named. The assertion runs either way: it is what stands
# between a deleted gate and a green run, and a scoped run that skipped it could
# report a gate holding that the runner no longer has.
run_gates() {
	local started gate_started ran chosen name

	started=$SECONDS
	ran=0
	if ! assert_gate_set; then
		echo "$prog: gate_set failed" >&2
		return 1
	fi
	echo "gate_set: the runner has the gates expected"

	if [ "$#" -eq 0 ]; then
		chosen="$gates"
	else
		# gate_set names the assertion above rather than a gate, so a run scoped
		# to it is the assertion and nothing else.
		chosen=""
		for name in "$@"; do
			case "$name" in
			gate_set) continue ;;
			esac
			chosen="$chosen $name"
		done
	fi

	for gate in $chosen; do
		gate_started=$SECONDS
		if ! "gate_$gate"; then
			# The first red ends the run. A gate after the build gate would
			# mostly re-report it, since nothing tests a workspace that does
			# not compile.
			#
			# What it cost is said on its own line, because the line below is
			# what a proof matches against and pinning it is the whole of how a
			# proof tells the gate it aimed at from any other going red.
			echo "$gate: failed in $(since "$gate_started")"
			echo "$prog: $gate failed" >&2
			return 1
		fi
		echo "$gate: passed in $(since "$gate_started")"
		ran=$((ran + 1))
	done

	echo "gates: $ran passed in $(since "$started")"
}

copy_tree() {
	mkdir -p "$1"
	tar -cf - --exclude=./.git --exclude=./target . | (cd "$1" && tar -xf -)
}

# A copy that fails for a reason of its own — a file the copy missed, a
# toolchain the scratch directory cannot reach — would read as every gate
# catching every violation. So the untouched copy runs first and must be green.
control() {
	local output started
	started=$SECONDS
	copy_tree "$scratch/control"
	if ! output="$("$scratch/control/scripts/gates.sh" --gates-only 2>&1)"; then
		echo "$prog: the untouched scratch copy does not pass its own gates" >&2
		printf '%s\n' "$output" >&2
		return 1
	fi
	printf '%s\n' "$output" | sed 's/^/  /'
	echo "control: the untouched copy passes, in $(since "$started")"
}

# Going red is half of what a gate has to do; a gate that refuses everything
# catches its own violation and is worth nothing. Where a gate has a shape to
# accept as well as one to refuse, it carries an accept_<name> — the same
# fixture without the violation — and the copy carrying that must stay green.
# Where the tree itself is the shape a gate accepts, the control already proves
# this side and the gate carries no accept_.
accepts() {
	local name copy output started
	name="$1"
	copy="$scratch/$name-accepted"
	started=$SECONDS
	copy_tree "$copy"
	"accept_$name" "$copy" || return 1

	if ! output="$("$copy/scripts/gates.sh" --gates-only "$name" 2>&1)"; then
		echo "$prog: $name refused what its rule allows" >&2
		printf '%s\n' "$output" >&2
		return 1
	fi
	echo "$name: accepts what the rule allows, in $(since "$started")"
}

# A gate counts as a gate only if it goes red on what it exists to catch. Each
# proof copies the tree, injects that thing, and runs this same script in the
# copy, so a gate weakened later fails here instead of passing unnoticed. The
# copy is run with --gates-only because the plain form would recurse into these
# proofs; it runs the same gate, so what is red there is red for it too.
#
# The copy is asked for the one gate the violation belongs to, not the whole
# sweep. What the sweep added was the chance for some other gate to go red first
# and answer for this one, which is why the line below is checked and not just
# the exit code — and a run holding one gate cannot be answered for by another at
# all. Running the rest was the multiplier that made this suite cost what it did:
# every gate, once per gate, for a verdict about one of them.
prove() {
	local name copy output started
	name="$1"
	copy="$scratch/$name"
	if declare -F "accept_$name" >/dev/null; then
		# The accepted copy runs first, so a red below is the violation and not
		# something about the fixture the violation is made from.
		accepts "$name" || return 1
	fi
	started=$SECONDS
	copy_tree "$copy"
	"violate_$name" "$copy" || return 1

	if output="$("$copy/scripts/gates.sh" --gates-only "$name" 2>&1)"; then
		echo "$prog: $name passed a tree carrying the violation it exists to catch" >&2
		printf '%s\n' "$output" >&2
		return 1
	fi
	if ! printf '%s\n' "$output" | grep -Fqx "$prog: $name failed"; then
		echo "$prog: the injected $name violation did not go red under $name" >&2
		printf '%s\n' "$output" >&2
		return 1
	fi
	echo "$name: proof caught it, in $(since "$started")"
}

scratch=""

cleanup() {
	if [ -n "$scratch" ]; then
		rm -rf "$scratch"
	fi
}

suite_started=$SECONDS

case "${1:-}" in
--gates-only)
	shift
	for name in "$@"; do
		if ! known_gate "$name"; then
			echo "$prog: no gate named $name" >&2
			usage
		fi
	done
	run_gates "$@" || exit 1
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
	echo "$prog: every gate passed and every proof caught, in $(since "$suite_started")"
	;;
*)
	usage
	;;
esac
