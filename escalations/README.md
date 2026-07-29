# Escalations

One file per stop, named `<YYYY-MM-DD>-<task-id>.md` —
`2026-07-28-M1-04.md`, and a short subject slug where there is no task id.

Write one whenever a run stops: ambiguity, a conflict between instructions, a
missing gate, or a decision above its authority. Say what you were doing, what
stopped you, and what you would need to proceed. Proposing the decision itself is
an ADR, not an escalation.

An escalation is an open item, not history. The change that resolves it deletes
the file in the same diff, so this directory holds only what is still waiting and
nothing here is ever annotated as closed. A resolution that changes nothing in
the repository is closed by a human deleting the file, or by the first run that
acts on the answer.
