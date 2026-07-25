# VFI

A local desktop tool for value and dividend investing analysis, built on official
SEC filings. Compiled engine in Rust, presentation shell in Python.

## Read these, in this order

@ANCHORS.md
@AGENTS.md

ANCHORS.md is what may never change. AGENTS.md is how you work. If they conflict
with each other, ANCHORS.md wins. If either conflicts with a task instruction,
the document wins.

This file is context, not enforcement. Nothing here is binding on its own — what
is actually enforced is enforced by hooks, lint, tests, and CI. If you find
yourself relying on this file to stop you from doing something, the protection is
missing and that is an escalation.

## The map

- **ANCHORS.md** — the fixed rules and how each is enforced. Changing it needs
  human sign-off.
- **AGENTS.md** — how a run works: one task, claim, branch, gates, escalate.
- **GOALS.md** — the milestones. What finished looks like, in order.
- **WORKPLAN.md** — the task format and the claim rules. The queue itself lives
  in `tasks/`.
- **docs/adr/** — decisions and why they were made. Read before proposing a
  change to something already decided.

Read GOALS.md and WORKPLAN.md when you are claiming or planning work, not every
session.

## Style

Language style guidance lives in `.claude/rules/`, one file per language. It is
not repeated here.

Style is guidance, not a gate. No build fails over formatting.

## Before you write anything

- One task per run. Claim it by pushing its branch. Never commit to main.
- Stay inside the paths your task owns.
- All gates must pass. Do not weaken a gate to make it pass.
- Never merge your own work.
- When in doubt, stop and escalate. Do not guess. This matters most in
  normalization, where a wrong answer looks right.
