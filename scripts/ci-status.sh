#!/usr/bin/env bash
# Usage: scripts/ci-status.sh <branch> — exit 0 only if the latest CI run on <branch> is green.

set -euo pipefail

branch="${1:-}"
if [ -z "$branch" ]; then
	echo "usage: $(basename "$0") <branch>" >&2
	exit 2
fi

if ! run="$(gh run list --branch "$branch" --limit 1 \
	--json status,conclusion --jq '.[] | "\(.status) \(.conclusion)"')"; then
	echo "$(basename "$0"): gh could not list runs for $branch" >&2
	exit 1
fi

if [ -z "$run" ]; then
	echo "$(basename "$0"): no CI run on $branch" >&2
	exit 1
fi

status="${run%% *}"
conclusion="${run#* }"

if [ "$status" = "completed" ] && [ "$conclusion" = "success" ]; then
	echo "$branch: CI green"
	exit 0
fi

echo "$(basename "$0"): $branch is not green (status $status, conclusion $conclusion)" >&2
exit 1
