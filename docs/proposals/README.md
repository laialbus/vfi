# Proposals

Drafts that agents cannot install themselves, because the destination is
protected (`.claude/hooks/`) or outside the workspace (`../run.sh`,
`../prompts/`, `~/Library/LaunchAgents/`). An agent writes the draft here; a
human reviews, installs, and the installation commit records the sign-off.
Delete a draft once installed.

## Current drafts and where they go

| Draft | Destination | Install |
| :---- | :---------- | :------ |
| `com.vfi.agent.lead.plist` | `~/Library/LaunchAgents/` | header comment in the file — **not before M2's gates exist and a supervised dry run has passed** |
| `run.sh` | `../run.sh` (workspace root) | header comment in the file — diff against the installed copy first; only the handoff recovery block is meant to differ |

The pairing the corpus expects is restored: `protect_paths.py` beside it is
the candidate, judged case by case against the installed copy. If the
candidate is absent the corpus says so and exits, rather than reading the
absence as a verdict.
