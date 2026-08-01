# The hook honors planner-granted ownership of protected paths

- **Status:** Proposed
- **Authority:** Constitutional
- **Proposed:** 2026-08-01, by the lead (supervised session)
- **Decided:** —
- **Touches:** protected-path enforcement — the pre-edit hook and what the
  protected list means

## Context

The protected list is enforced by two layers of different strictness. The
server allows a protected-path change that carries the human-approved label:
signed changes land. The client hook refuses every agent write to those paths,
signed or not — it runs before a pull request exists, so it has no notion of a
signature. M2-01 merged only because the hook was silently dead (the Python
3.9 load failure, repaired in PR #28); the first task to hit the live hook,
M2-02, stopped and parked its finished gate as a draft
(escalations/2026-08-01-M2-02.md). Every remaining M2 task owns the same
protected file, and every future contract task — at least one per stage
boundary through M6 — owns a protected path too. Under the current hook, each
of those ends in a parked draft and a human hand-install. The policy the
server enforces is "no unsigned change lands"; the hook enforces the stricter
"no agent may deliver at all," and nothing in the anchors asks for the
stricter one.

## Decision

The hook allows a write to a protected path when the run's claimed task grants
it, under all of these conditions:

- The current branch names a task, and `tasks/<branch>.md` **as committed on
  origin/main** lists the path under `owns`. The working-tree copy is never
  consulted — a worker editing a task file locally grants nothing.
- The path is on a fixed grantable sublist inside the hook: `scripts/gates.sh`,
  `.github/`, `contracts/`. The constitution files, `docs/adr/TEMPLATE.md`,
  `.claude/settings.json`, and everything under `.claude/hooks/` — including
  the hook and the protected list themselves — are never grantable, whatever a
  task file says.
- Any failure — no task file on origin/main, unreadable frontmatter, git
  unavailable — is no grant. The hook fails closed to today's behavior.

Everything else is unchanged: ungranted protected paths still refuse, no agent
may apply the human-approved label, merges still require VFI_ROLE=decider, and
the server-side check still blocks unsigned protected changes from landing.
The grant moves *delivery* to workers; *sign-off* stays human, expressed where
it always was — the label. The binding property is untouched: this change
cannot cause an unsigned protected change to land; it only lets one exist on a
branch, where it dies without a signature.

## Alternatives

- **Drop `scripts/gates.sh` (and later `contracts/`) from the protected
  list.** Unblocks the queue and removes the server-side label requirement
  with it — "do not weaken a gate" would rest on review attention instead of
  structure. Rejected: it trades the floor for throughput.
- **Keep hand-installs (route 2, the status quo).** Safe and already working,
  but it makes the human the delivery mechanism for six M2 tasks and then
  every contract task for the life of the project. Acceptable cadence for a
  week; wrong as a steady state. Kept as the fallback if this is rejected.

## Consequences

Easier: M2-03 through M2-07 and every future contract task deliver as ordinary
worker PRs — review by the decider, label and merge by the human. Harder: the
hook grows a git read and a frontmatter parse, which is more code in the one
guard that must not be wrong; the corpus must grow with it. Reversal is cheap:
removing the grant restores exactly today's behavior, because the grant never
changed what could merge.

## Enforcement

The corpus (`protect_paths_tests.py`) gains cases proving: a granted write is
allowed only on the claiming branch; the same write on another branch refuses;
a task file edited in the working tree grants nothing; never-grantable paths
refuse even when a task file lists them; a missing or unparsable task file
refuses. The server-side protected-paths check is unchanged and remains the
binding layer.

Implementation lands the way hook changes always do: as
`docs/proposals/protect_paths.py` (restoring the corpus pairing the proposals
README describes), verified by the corpus under both interpreters, installed
by a human with this ADR's acceptance as the sign-off.

## Decision review

*(for the human — Constitutional, so the decider defers by rule)*
