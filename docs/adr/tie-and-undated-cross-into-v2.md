# A Rule 3 tie crosses into v2 as the tied filings' attempts beside the tie, and an unreadable `filed` is a stop, not a state

- **Status:** Accepted
- **Authority:** Structural
- **Proposed:** 2026-09-25, by the lead (supervised session), on the owner's
  delegation
- **Decided:** 2026-09-25, by Albus Lai (human), by merging the pull request
  that carries this record
- **Touches:** how the published `canonical-concepts` v2 `Unknown` line is read
  at a tie, and what the normalize crate does with a filing whose `filed` does
  not read as a date. No contract byte moves and the contracts crate's types are
  untouched; this is a reading of the published line, and the decider's sweep
  said none was on record. No schema, anchor or gate moves.

## Context

Three runs stopped M4-38 on one wall. v2 builds an `Unknown` from
`Attempts::in_filings(first, rest)`, at least one attempt, and hangs a `Tie` on
it. `standing::latest` fills `attempted` only from filings whose attempt came to
`Unknown`. Where Rule 3 ties and every answering filing produced a value,
`attempted` is empty and there is no `first` to hang the tie on. The same holds
for `Undecided::Undated`, the accessions whose `filed` does not read as a date.
The runs read the one available crossing — a tied filing as an attempt with
nothing declined — as collapsing two cases v2 keeps apart, and stopped, as the
task told them to.

What the published line says: an `Unknown` carries "what was attempted in each
filing that answered the period, under that filing's accession: the candidates
considered and the rule that declined each. And where a tie between answering
filings left which of them sets the value undecided, the tied filings'
accessions and the tie." The emit record adds that "Rule 3's surviving tie
carries the tied attempts under their accessions, and what left them undecided
is the tie on `filed` that `form` did not break — Rule 3's clause, not a seventh
reason of candidate choice's."

## Decision

**A tie.** Each tied filing's attempt crosses as an `Attempted` under its
accession, carrying the candidates candidate choice declined inside that filing
— an empty list where it declined none — and the `Tie` names every tied
accession. The candidate that settled inside a tied filing is not a declined
candidate and is not listed as one: what left it undecided is the tie, and the
tie is carried once, as the published line has it. So an `Unknown` at a tie
holds the failed attempts and the tied attempts together, and its reading is
unambiguous: an accession the `Tie` names is a filing whose attempt produced a
value; one it does not name is a filing whose attempt came to nothing, with
what it declined or, where no rule was eligible, nothing. A tie has two filings
at least, so `first` always exists. A filing whose attempt produced a value and
lost on `filed` alone is not carried: Rule 3 reads nothing else about the
earlier filings, and it is neither failed nor undecided.

The crate keeps what a tied filing declined so the wiring can carry it. That is
inside `crates/normalize/`, M4-38's own boundary, and the landed test that
asserts an empty `attempted` at a tie is corrected to the ruled shape.

**An unreadable `filed`.** The fetch boundary publishes `filed` as EDGAR
publishes it, and EDGAR publishes a date. A filing whose `filed` does not read
as one is a broken boundary reached at run time, the same class as facts
colliding on the contract's key, which the candidate-choice record already
makes a stop rather than an `Unknown`. It does not cross into v2 as a state.
The crate's surface for one filer's history returns a failure naming the
accessions, the run that meets one escalates with the document, and
`Undecided::Undated` stays what it is inside the crate: the direction `stands`
takes so that nothing is invented for it.

## Alternatives

**Give `Attempts` room for a tie with no attempt.** A contracts-crate change
behind an exclusive task and a human-approved path, for a case the published
line already covers once the tied attempts are read as attempts.

**Cross a tied filing as an attempt that considered no candidate.** The
crossing the runs refused. Under this record it is not what happens: a tied
filing's attempt carries what it declined, and the `Tie` marks it as one that
produced a value, so the two cases do not read alike.

**Cross an unreadable `filed` as a tie or as an attempt.** It is neither. It is
a document the boundary should not have published, and a state that pretends
otherwise is the number that looks right.

## Consequences

**Easier.** M4-38 proceeds with no contract change. Every `Unknown` at a tie
says which filings tied and what each filing tried.

**Harder.** The crate's surface for a filer's history is fallible, and the run
that first meets an unreadable `filed` stops. No merged fixture reaches one.

**Expensive to reverse.** Nothing is stored yet.

## Enforcement

Not an anchor. M4-38's hand-written tie case pins the shape: two same-day
filings reading a value, neither amending the other, cross as two `Attempted`
under their accessions with what each declined, and a `Tie` of both.

## Decision review

By the owner, on delegation, in the session that proposed it; the review is
the owner's signature on merging, not an independent check.

- **Authority:** Structural, and within the owner's reach: it reads a published
  line and changes no byte of it, and it classifies a broken input as a stop,
  which the candidate-choice record already does for the other broken input.
- **Checked:** v2's `Unknown` `carries` line and its "no empty case"
  property; the emit record's tie sentence; `Attempts`, `Attempted`, `Attempt`
  and `Tie` in the contracts crate; `standing::latest` and the tie test at
  9574b51; the fetch boundary's `filed` field, published as received.
- **Verdict and why:** accepted. The published line asks for each answering
  filing's attempt and the tie; a tied filing has an attempt, and the tie is
  what the line reserves for saying it produced a value.
- **What would have changed it:** a published line that carried the tie
  without the tied attempts. It carries both.
