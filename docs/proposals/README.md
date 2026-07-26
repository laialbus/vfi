# Proposals

Drafts that agents cannot install themselves, because the destination is
protected (`.claude/hooks/`) or outside the workspace (`../run.sh`,
`../prompts/`). An agent writes the draft here; a human reviews, installs, and
the installation commit records the sign-off. Delete a draft once installed.

## Current drafts and where they go

| Draft | Destination | Install |
| :---- | :---------- | :------ |
| `run.sh` | `../run.sh` (workspace root, outside the repo) | `cp docs/proposals/run.sh ../run.sh && chmod +x ../run.sh` |
| `lead.md` | `../prompts/lead.md` | `cp docs/proposals/lead.md ../prompts/` |
| `decider.md` | `../prompts/decider.md` | `cp docs/proposals/decider.md ../prompts/` |
| `protect_paths.py` | `.claude/hooks/protect_paths.py` | review the diff, then `cp docs/proposals/protect_paths.py .claude/hooks/` — this PR needs the `human-approved` label to merge, which is the recorded sign-off |

## After installing the hook revision

Two follow-ups, both edits only a human can make:

1. Add these lines to `.claude/hooks/protected-paths.txt`, so agents cannot
   edit the CI checks or the workflows that enforce them:

       .github/

2. In the GitHub ruleset for main, mark two required status checks:
   `gates` and `protected-paths`. From then on GitHub itself refuses to merge
   a PR that fails the gates or touches the constitution without the
   `human-approved` label.
