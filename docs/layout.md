# LAYOUT

Where everything goes. One kind of artifact, one home, no exceptions.

This is the single source of truth for placement. If you are about to create a
file and cannot find its home below, that is an escalation, not an invitation to
pick somewhere sensible — a home invented mid-run is a home the next agent will
not find.

Some paths here are fixed elsewhere and only recorded here: anything in
`.claude/hooks/protected-paths.txt` has its location set by that list, and
changing it needs human sign-off. This document does not grant itself authority
over those.

## The repository

`vfi/` is the repository. Its root holds the constitution, the entry points a
newcomer needs, and the Rust workspace manifest. Nothing else.

| Path | What lives there |
| :--- | :--- |
| `ANCHORS.md`, `AGENTS.md`, `GOALS.md`, `WORKPLAN.md`, `CLAUDE.md` | The constitution. Protected. |
| `README.md` | The front door: what VFI is, and where to read next. |
| `Cargo.toml`, `Cargo.lock` | The Rust workspace manifest and its lockfile. |
| `crates/` | The engine. One directory per crate — see naming below. |
| `contracts/` | The typed, versioned contracts between stages (anchor 3). A workspace member like any crate, but at the root because the protected-path list puts it there. |
| `shell/` | The Python presentation shell, including its `pyproject.toml`. |
| `scripts/` | Every operational script. See below. |
| `fixtures/` | Golden fixture inputs and their expected results, as data. The harness that runs them is code and lives with the crate it exercises. |
| `tasks/` | The task queue, one file per task. |
| `sessions/` | Session entries, one file per run. |
| `escalations/` | Escalations, one file per stop. |
| `docs/` | Everything written for a reader rather than a machine. |
| `docs/adr/` | Decision records, plus `TEMPLATE.md` (protected). |
| `docs/proposals/` | Drafts an agent cannot install itself, because the destination is protected or outside the repository. |
| `docs/layout.md` | This file. |
| `.claude/` | Agent configuration: `settings.json`, `hooks/`, `rules/`. Protected. |
| `.claude/worktrees/` | Teammate worktrees. Created by the tooling, ignored by git. |
| `.github/workflows/` | CI. Protected. |

Two rules about the root, both load-bearing:

- **No script at the root, and none outside `scripts/`.** One entry point per
  purpose. `scripts/tasks.sh` reads the queue, `scripts/ci-status.sh` checks a
  branch, `scripts/escalate.sh` writes an escalation, `scripts/gates.sh` runs
  every gate (M2, protected). A second script for a job one of these already
  does is a defect, not a convenience.
- **No new top-level directory without amending this file.** The root is small
  on purpose; it is the first thing anyone reads.

Build output — `target/`, `__pycache__/`, virtualenvs — is ignored, never
committed, and belongs to whichever tool made it.

## Crate naming

One crate per pipeline stage, because the dependency graph is what enforces
one-way flow (anchor 2). Three names for each crate, mechanically derived from
the stage:

| | Form | Example |
| :--- | :--- | :--- |
| Directory | `crates/<stage>/` | `crates/normalize/` |
| Package | `vfi-<stage>` | `vfi-normalize` |
| Library target and import path | `vfi_<stage>` | `use vfi_normalize::…` |

The directory is bare because its parent already says these are VFI crates. The
package carries the `vfi-` prefix because package names are global and a crate
called `analyze` says nothing about whose it is. The underscore form is Cargo's
own translation of the package name; do not set it by hand.

The stages are `fetch`, `normalize`, `analyze`, and `store`. `contracts` follows
the same naming — package `vfi-contracts` — at its own root path.

A crate that is not a pipeline stage still follows the pattern. Adding one is a
layout change: amend this file in the same diff.

## The shell

All Python lives under `shell/`. The shell displays what the engine produces and
holds no analysis logic (anchor 1), so nothing under `shell/` reads a filing,
computes a metric, or reaches storage directly. Secrets are the shell's job and
are read here, never in the engine.

`shell/` is also the path the Python style rules scope to, and `crates/` and
`contracts/` are the paths the Rust rules scope to.

## One file per item

`tasks/`, `sessions/`, `escalations/`, and `docs/adr/` each hold **one file per
item**. Never an index, a log, or a shared list.

This is not tidiness. Agents run in parallel and unattended; two runs appending
to one file collide, and the loser's record is lost or the merge is a conflict
nobody is awake to resolve. Separate files cannot collide — git merges two new
files without being asked. The same reasoning is why the queue is a directory
and not a list inside WORKPLAN.md, and why nothing anywhere records status: it
is derived from branches and merge history, so it cannot go stale.

Naming:

| Directory | File name | Example |
| :--- | :--- | :--- |
| `tasks/` | `<task-id>.md` | `tasks/M1-04.md` |
| `sessions/` | `<YYYY-MM-DD>-<task-id>.md` | `sessions/2026-07-28-M1-04.md` |
| `escalations/` | `<YYYY-MM-DD>-<task-id>.md` | `escalations/2026-07-28-M1-04.md` |
| `docs/adr/` | `<slug>.md`, named for the decision | `docs/adr/gui-toolkit.md` |

A run with no task id — the lead and the decider claim none — uses a short slug
for the subject instead: `sessions/2026-07-28-lead.md`. If the name is already
taken, append `-2`, then `-3`. ADRs are never numbered, because parallel
proposers collide on numbers but not on names.

A task's own file is deleted by the branch that finishes it, in the same diff.
The queue drains itself.

## Around the repository

The workspace directory containing `vfi/` holds what an agent must not be able
to edit. The separation is the point: a wrapper an agent can rewrite is not a
wrapper.

| Path | What it is |
| :--- | :--- |
| `run.sh` | The run wrapper. Owns preflight, the lock, the wall-clock limit, and claim release. Outside the repo deliberately. |
| `prompts/` | One file per role: `lead.md`, `decider.md`. Read by `run.sh`. |
| `vfi-work/` | Run scratch, created by `run.sh` and never committed: `worker-<n>/` clones, `logs/`, and the per-worker lock. |
| `com.vfi.agent.<role>.plist` | Reviewed LaunchAgent definitions, staged here before a human installs them to `~/Library/LaunchAgents/`. |

An agent that needs a change out here writes the draft to `docs/proposals/` and
stops. It does not install it, and it does not work around the fact that it
cannot.

## Amending this file

A new kind of artifact gets its home here first and is created second. The
amendment is part of the same diff as the work that needed it, so the map is
never behind the tree.

Placement is a routine decision (see `docs/adr/TEMPLATE.md`) except where it
touches a protected path, a contract, or a schema — then it is an ADR, and the
ADR lands before the move.
