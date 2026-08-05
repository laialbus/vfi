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
| `worker_terminal_state.py` | `.claude/hooks/worker_terminal_state.py` | header comment in the file — the file, the `Stop` wiring in `.claude/settings.json`, and the AGENTS.md sentence land together, or the hook does nothing. Run `worker_terminal_state_tests.py` first |
| `worker_terminal_state_tests.py` | `.claude/hooks/worker_terminal_state_tests.py` | with `worker_terminal_state.py` above — the corpus and the hook it judges move together |

The pairing the corpus expects is restored: `protect_paths.py` beside it is
the candidate, judged case by case against the installed copy. If the
candidate is absent the corpus says so and exits, rather than reading the
absence as a verdict.
