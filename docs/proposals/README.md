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
| `com.vfi.agent.decider.plist` | `~/Library/LaunchAgents/` | same condition as the lead plist |
| `protect_paths.py` | `.claude/hooks/protect_paths.py` | **not before `docs/adr/protect-paths-hook-matching.md` is approved.** Run `protect_paths_tests.py` first |
| `protect_paths_tests.py` | `.claude/hooks/protect_paths_tests.py` | with the hook it tests |
| `lead-prompt.md` | `../prompts/lead.md` | header comment in the file — two prerequisites go in with it, one a change to `run.sh` |
