# 2026-08-06 — supervised

Branch `add-planner-role`. The planner role, drafted: prompts and wrapper
support into `docs/proposals/` (planner.md, decider.md, run.sh — the last
two are full replacements for the installed files), and M2-14 queued so the
queue gains structural refusals before the first agent-written refill. The
decider requests a refill by ref when the queue drains; the wrapper consumes
the ref and runs the planner as its own role; a later decider run reviews
the plan. Installs are human acts, and so are the amendments: three
WORKPLAN.md sentences (the refill rule, "Only the planner writes here",
"Who fills this in") and the AGENTS.md carve-out placing the planner
outside the pool, under the same PR flow as the parking clause. Verified by
an independent two-pass review; all findings applied.
