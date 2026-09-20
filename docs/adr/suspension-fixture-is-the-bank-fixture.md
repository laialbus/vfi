# The dividend-suspension fixture is also the first bank fixture, and the emit record's claims about it are re-derived under that kind

- **Status:** Accepted
- **Authority:** Structural
- **Proposed:** 2026-09-19, by the lead (supervised session), on the owner's
  delegation
- **Decided:** 2026-09-19, by Albus Lai (human), by merging the pull request
  that carries this record
- **Touches:** the claims the accepted `what-normalize-emits.md` makes about
  CIK 0001778784, which are superseded here; the record itself is not edited
  and its decision stands whole. No contract, schema, anchor or gate moves.
  This record is also the human review the Structural tier required of the
  emit record: it stands, with these claims re-derived.

## Context

The emit record derived its fixture claims on 71365cc, when CIK 0001778784 had
no kind. M4-33 then read the filer's own facts and made it `bank`, and
`short_term_investments` applies to `operating` only. So the record's claims
that this filer renders `short_term_investments` as a read figure at 24 instants
and as a silence zero over the year 2023, "the measure seam in one fixture", are
no longer what the stage emits: the concept is `NotApplicable` there at every
period, as M4-38 already pins. The record's sentence that "no merged fixture
reaches a `NotApplicable`" is no longer true either. The 2026-09-17 decider
sweep asked for a human decision before the fixture task is queued.

## Decision

Keep the filer. `fixtures/normalize/a-filer-that-suspended-its-dividend` serves
two of the milestone's criteria at once: the dividend suspension, and the first
`NotApplicable` rows — every concept whose `applies_to` omits `bank`, as the
vocabulary publishes them, at every admitted period, each carrying the kind and
the clause. The emit record's `short_term_investments` claims on this filer are
superseded by that. The measure seam it wanted shown in one fixture is already
in the restatement fixture, whose Rule 3 tables carry silence zeros at
durations. The dividend claims hold as the registry stands and change as
`declared-cash-dividends-stand-in.md` states once M4-42 lands; the fixture task
is queued after M4-42 and takes its dividend claims from that record's
Enforcement section. Any other claim the emit record makes about this filer —
the year 2023's five attempts and no candidate for `revenue` among them — is
re-derived under the bank kind by the fixture task, which stops where a claim
does not re-derive rather than restating it.

## Alternatives

**Pick another suspension filer.** Discards a recorded fetch fixture and a
derived kind file, and a bank is where suspensions are common. It would also
postpone the first `NotApplicable` fixture to a filer not yet recorded.

**Write the fixture against the stale claims.** The harness goes red or the
claims are false; either way the fixture proves nothing.

## Consequences

**Easier.** The fixture task's acceptance can be written; the planner's queue
is unblocked after M4-38.

**Harder.** The Q4 2020 and Q4 2021 dividend zeros render `silence` in this
fixture, beside year-to-date payments the filer made. That is the priced cost
the emit record already names, and it stands until the vocabulary's next
version takes the witness condition the declared-cash record sets out.

**Expensive to reverse.** Nothing is stored yet.

## Enforcement

Not an anchor. The fixture task re-derives every claim it pins, as the emit
record already requires of it.

## Decision review

By the owner, on delegation, in the session that proposed it; the review is
the owner's signature on merging, not an independent check.

- **Authority:** Structural, and within the owner's reach: it supersedes
  fixture claims of a Structural record and no part of its decision.
- **Checked:** the vocabulary's `applies_to` for `short_term_investments`;
  `registry/filers/0001778784.toml` as merged in #203's sweep; the emit
  record's fixture section and its review's re-derivation list; M4-38's
  acceptance, which already pins `NotApplicable` there.
- **Verdict and why:** accepted. A fixture that reaches a state no other
  fixture reaches is worth more, not less, and the filer's kind was read from
  its own facts.
- **What would have changed it:** a filer already recorded whose kind is not
  `operating` and which does not also carry the suspension. None is.
