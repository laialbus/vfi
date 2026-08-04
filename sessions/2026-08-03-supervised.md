# 2026-08-03 — supervised

Branch `harden-unattended-runs`. Direct human commission, from the M2-07
escalation analysis.

Opened PR #50 by hand for the stranded M2-07 branch — finished and gates
green, but its run ended without a pull request. Queued the follow-ups:
M2-10 (sweep archives work instead of deleting it), M2-11 (the gate suite
back under a worker's clock), M2-12 (draft the terminal-state stop gate).
Proposed `docs/adr/worker-terminal-state.md` and drafted the wrapper's
missing-PR recovery into `docs/proposals/run.sh` — both install by human
hand.
