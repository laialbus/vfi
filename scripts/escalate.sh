#!/usr/bin/env bash
# Usage: scripts/escalate.sh <task-id> <reason> — write one escalation file into escalations/.

set -euo pipefail

cd "$(dirname "$0")/.."

task_id="${1:-}"
shift || true
reason="$*"

if [ -z "$task_id" ] || [ -z "$reason" ]; then
	echo "usage: $(basename "$0") <task-id> <reason>" >&2
	exit 2
fi

mkdir -p escalations

today="$(date -u +%Y-%m-%d)"
file="escalations/$today-$task_id.md"
suffix=2
while [ -e "$file" ]; do
	file="escalations/$today-$task_id-$suffix.md"
	suffix=$((suffix + 1))
done

branch="$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo unknown)"
commit="$(git rev-parse --short HEAD 2>/dev/null || echo unknown)"

cat >"$file" <<EOF
# Escalation: $task_id

- Date: $today
- Branch: $branch
- Commit: $commit

$reason
EOF

echo "$file"
