# <Decision, as a sentence>

<!--
Copy to docs/adr/<slug>.md. Name it for the decision, not a number —
`gui-toolkit.md`, not `004.md`. Parallel proposers collide on numbers.

Keep it short. This is read later to find out why, not to relive the reasoning.
A section with nothing real in it means the decision is not ready.

The proposer fills in everything down to Consequences. The decider fills in
Decision review. Never the same agent. Delete this comment.
-->

- **Status:** Proposed | Accepted | Rejected | Deferred, needs a human |
  Superseded by `<slug>`
- **Authority:** Routine | Structural | Constitutional
- **Proposed:** YYYY-MM-DD, by `<task id or agent>`
- **Decided:** YYYY-MM-DD, by `<decider or person>`
- **Touches:** `<anchor, contract, schema, or layout — the reason an ADR was
  required>`

## Context

What forced this, and what is fixed. A few sentences.

## Decision

What will be done. One paragraph, as a decision, not a suggestion.

## Alternatives

At least one, with why not. No alternatives means either obvious — so no ADR
needed — or not thought through.

## Consequences

What gets easier. What gets harder. What is now expensive to reverse.

## Enforcement

Only if this touches an anchor. What check makes it hold. If nothing does, say
so — a visible gap beats an assumed one.

## Decision review

By the decider, not the proposer.

- **Authority:** which tier, and why it is within reach. Constitutional means
  stop and mark Deferred.
- **Checked:** anchors it touches, ADRs it contradicts, the alternatives as
  argued.
- **Verdict and why:** in the decider's own words, not a restatement.
- **What would have changed it:** the fact that would have flipped this. If
  none, it was not evaluated.

---

<!--
AUTHORITY

Routine — the decider accepts alone.
  Naming, layout, library choice within a crate, implementation approach, a new
  implementation behind an existing interface.

Structural — the decider accepts and it stands, flagged for later human review
and possible revert.
  Contract fields, new gates, schema additions, new interfaces.

Constitutional — the decider never accepts. Mark Deferred.
  Changes to ANCHORS.md, weakening or removing a gate, the protected-path list,
  scope in GOALS.md, these tiers, or the decider's own authority.

The last matters most: a decider that can widen its own authority is unbounded
one step later.

RULES

Proposer and decider are never the same agent. The decider sits outside the
worker pool: it claims no tasks and writes no code.

Being blocked is not a reason to accept.

Rejections and deferrals are normal. A log with none is evidence the review is
not real.

Accepted ADRs are never edited. Supersede with a new one — the value is that it
shows what was believed at the time.
-->
