# A worker run ends only in a terminal state

- **Status:** Proposed
- **Authority:** Structural
- **Proposed:** 2026-08-03, by the supervised session (from the M2-07
  escalation)
- **Decided:** —
- **Touches:** AGENTS.md (the finish step) and a new hook under
  `.claude/hooks/` — both protected, which is why an ADR is required

## Context

M2-07's second run finished its task — gates green, session entry written —
and ended its final turn with the pull request deferred to a background
monitor: "I'll push the branch and open the PR the moment it comes back
green." A headless session does not outlive its turn, so the promise could
never execute. The wrapper found no PR at handoff and escalated
(escalations/2026-08-03-M2-07.md in the fleet worktree; PR #50 is the
recovery). Because M2-07 is exclusive, the stranded claim withheld the entire
queue, and the sweep would have deleted the only origin ref to the finished
work within hours.

AGENTS.md already names the terminal states — push and open a PR, or revert
and escalate — but nothing enforces reaching one. CLAUDE.md states the
project's own rule for this situation: if prose is the only thing stopping a
behavior, the protection is missing.

## Decision

A worker session may not end except in a terminal state: **handoff** (branch
pushed, pull request open), **escalation** (escalation committed and pushed to
an `escalated/` ref), or **no claim held**. A Stop hook enforces it for
`VFI_ROLE=worker`: a stop attempted with a claimed branch and no terminal
state is blocked once, with feedback naming the missing step, so the agent
that still has the full context finishes the job in-session. The hook never
blocks twice in one session (it yields when `stop_hook_active` is set), and
when it cannot answer — origin unreachable, `gh` failing — it warns and
allows, so it cannot wedge a session; the wrapper's own postcondition check
remains the backstop behind it.

AGENTS.md's finish step gains one sentence: *"The run ends when your turn
ends. A headless session does not outlive its final message, so work deferred
to a background task or monitor is work that never happens. Reach a terminal
state — PR open, or escalation pushed — before you finish."*

The hook and its corpus are drafted into `docs/proposals/` by task M2-12; the
installation — the hook file, the settings wiring, the AGENTS.md sentence —
lands only on never-grantable paths, so installing is a human act and this
ADR's acceptance is the sign-off it records.

## Alternatives

- **Wrapper-only recovery.** The wrapper opens the missing PR after the fact
  (proposed separately in `docs/proposals/run.sh`). Kept — as the backstop.
  Wrong as the primary: by the time the wrapper acts, the agent that had the
  context is gone, and the review artifact is authored by a shell script from
  a commit subject.
- **Prompt guidance alone.** Add the sentence to AGENTS.md and stop there.
  Rejected on the evidence: the finish steps were already written down, and
  M2-07 happened anyway. This is the exact failure mode CLAUDE.md warns about.
- **The wrapper always opens the PR, agents never do.** Deterministic, and it
  removes the failure class entirely — by moving authorship of every PR body
  from the agent (which has the provenance and the review notes) to the
  wrapper (which has a commit subject). Rejected: it degrades every review to
  fix a rare miss.

## Consequences

Easier: the M2-07 failure class disappears at the source, and a session that
forgets corrects itself one bounce later, while its context is intact.
Harder: one more hook to keep correct, with a corpus that must grow beside it,
in the one layer that must not be wrong. Reversal is cheap: remove the wiring
and behavior returns to today's, because the wrapper's check never left.

## Enforcement

This touches no anchor. The hook is itself the enforcement of AGENTS.md's
finish step; its corpus (drafted with it, per M2-12) proves the block, the
allow cases, the one-bounce limit, and the fail-open path. The wrapper's
handoff check and the server's branch protection are unchanged and remain the
outer layers.

## Decision review

Left for the decider — or for the human, if acceptance is taken together with
the installation this ADR describes.
