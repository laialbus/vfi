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
| `planner.md` | `../prompts/planner.md` | with the two rows below and the human amendments to WORKPLAN.md — three sentences: the refill sentence, "Only the planner writes here" (an agent planner must not read as granted a protected file), and "Who fills this in" — and to AGENTS.md (the planner stands outside the pool like the decider). The role does nothing until `run.sh` knows it |
| `decider.md` | `../prompts/decider.md` | full replacement; adds queue-PR review and the refill request. Install together with `planner.md` and `run.sh` — a request ref nothing consumes, or a consumer nothing requests, is half a protocol |
| `run.sh` | `../run.sh` | full replacement; adds the planner launch. Diff against the installed copy before overwriting — it carries your local dark-wake guard unchanged |

The pairing the corpus expects is restored: `protect_paths.py` beside it is
the candidate, judged case by case against the installed copy. If the
candidate is absent the corpus says so and exits, rather than reading the
absence as a verdict.
