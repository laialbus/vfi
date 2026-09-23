# ADRs

Decision records. One file per decision, named `<slug>.md` for the decision
itself — `gui-toolkit.md`. Never numbered: parallel proposers collide on numbers,
not on names. Start one by copying `TEMPLATE.md`, which carries the sections, the
authority tiers, and the rules.

What belongs: a decision above an agent's own authority — an anchor, a contract,
a schema, a new consumer of a component with no explicit interface. What does
not: anything an agent may settle inside its own boundary, and a re-argument of
something already recorded here.

An accepted record is never edited. Supersede it with a new one; its value is
that it shows what was believed at the time.

## Partial supersessions and rulings

A record that supersedes part of another names the part in its **Touches**
line, and the pair is listed here so a reader of the older record can find the
newer one. The older record is not annotated. A ruling made outside a record —
in a task file or a pull request — is listed here too, since the file it lived in
is deleted when the task retires.

| Superseded or ruled on | By | What |
| :--- | :--- | :--- |
| `period-alignment.md`, Rule 3 | `which-filing-sets-the-value.md` | The latest filing sets every value, agreeing ones included. |
| `fetch-normalize-v2.md`, "no others" | PR #181, the owner's ruling on the 2026-09-13 M4-25 stop | `v2.toml` rewords three descriptive lines the record said cross unchanged; the published bytes are the record. |
| `which-filing-sets-the-value.md`, a silence `Value` "in the contest" | `silence-zero-supplied-once-per-period.md` | A silence zero is supplied once for the period and never enters Rule 3. |
| `what-normalize-emits.md`, its claims on CIK 0001778784 | `suspension-fixture-is-the-bank-fixture.md` | Re-derived under the `bank` kind; `short_term_investments` is `NotApplicable` there. |
| `period-alignment.md`, Rule 1 as read by M4-36 | PR #209, the owner's ruling on the 2026-09-18 M4-36 stop | A `Value` a silence reading supplied makes no period exist. |
| `declared-cash-dividends-stand-in.md`, "the years 2024 and 2025 remain the silence zero" | the owner's ruling on the 2026-09-21 M4-42 stop, in `tasks/M4-42.md` | The year 2024 and every 2025 period the fixture carries; no filing answers a 2025 year. |
| `which-filing-sets-the-value.md`, the `short_term_investments` rows of its two tables; `silence-beside-a-read-figure.md`, "three silence rows stand"; `what-normalize-emits.md`, its counts for CIK 0002003750 | `silence-zero-only-at-the-concepts-own-shape.md` | A silence zero is supplied only at a period of the concept's own shape; those rows are `Unknown`, and the counts are 51 and 576. |
