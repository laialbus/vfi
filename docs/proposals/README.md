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
