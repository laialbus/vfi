# A silence zero is supplied once for the period and never enters Rule 3's contest

- **Status:** Accepted
- **Authority:** Structural
- **Proposed:** 2026-09-19, by the lead (supervised session), on the owner's
  delegation
- **Decided:** 2026-09-19, by Albus Lai (human), by merging the pull request
  that carries this record
- **Touches:** the edge where the accepted `silence-beside-a-read-figure.md`
  and `which-filing-sets-the-value.md` meet. Neither is edited. No contract
  byte, schema, anchor or gate moves. This record is also the human review the
  Structural tier required of both: both stand, read as below.

## Context

The 2026-09-18 decider sweep flagged, before M4-36 built on `standing::stands`:
two answering filings that share the greatest `filed` and are both silent on a
zero-reading concept come to `Unknown` under Rule 3's tie clause, because the
landed code builds each filing's silence zero as a `Value`, hands both to
`latest`, and the tie survives `form`. The accepted silence record reads the same
case as the zero.

The two records do disagree in their words. The Rule 3 record says a `Value` the
silence reading supplied "is a `Value` and so is in the contest, and where it
wins on `filed` it stands as it was built". The silence record, accepted a day
later, says the reading is made over the period: where any answering filing
reached the concept, every silent attempt is `Unknown` and Rule 3 runs over the
read values; and "every answering filing silent is the zero, as before, naming no
filing". It adds that Rule 3's text is unchanged and "what changes is what the
vocabulary hands Rule 3".

## Decision

The silence reading is made once, for the period, after every answering
filing's attempt has run and none reached the concept — nor, for a conditional
reading, the concept its condition names. The vocabulary then supplies the zero
once. It carries its reading and the registry version, names no filing, and has
no `filed` and no `form`, so there is nothing for Rule 3 to order and Rule 3 does
not run. Where one filing reached the concept, silent attempts are `Unknown` and
Rule 3 runs over the values that were read or asserted, as both records already
say.

So a silence zero never ties and never displaces. Two same-day filings both
silent come to the zero, exactly as one silent filing does. The Rule 3 record's
sentence about a silence-supplied `Value` in the contest describes a case the
silence record removed; read together, the two records leave it vacuous, and
this record says so rather than leaving it to be inferred.

The code follows: `stands` returns the period's zero without passing silence
values through `latest`. That is task M4-41, ordered after M4-36 on the shared
crate and before M4-38, which crosses whatever `stands` answers.

## Alternatives

**`Unknown`, as the code has it.** One filing that says nothing yields a zero
and two filings that say nothing yield less than that. More silence must not
produce less certainty. And it puts a `filed` on a value that names no filing,
which the emit record's carriage for a silence value has no field for.

**A Rule 3 clause resolving ties between equal values.** It reads the number to
decide the tie, which anchor 5 forbids and the Rule 3 record already declined
for that reason.

## Consequences

**Easier.** The two records agree at every edge. M4-38 crosses one shape for a
silence value with no tie beside it.

**Harder.** Nothing. The pair condition, the read-value tie and the undated
absence are untouched.

**Expensive to reverse.** Nothing is stored yet.

## Enforcement

Not an anchor. M4-41's hand-written case: two answering filings sharing the
greatest `filed`, both silent, come to the silence zero and not to
`Undecided::Tie`.

## Decision review

By the owner, on delegation, in the session that proposed it; the two are the
same hand and the review is therefore the owner's signature on merging, not an
independent check. Recorded so the tier's requirement is visibly met rather
than assumed.

- **Authority:** Structural, and within the owner's reach: it reads two
  accepted Structural records together and edits neither.
- **Checked:** anchor 5, which nothing here crosses since nothing new reads a
  value; the silence record's "every answering filing silent" and "what the
  vocabulary hands Rule 3" sentences; the Rule 3 record's contest sentence;
  `standing.rs` at 478d6ae, where `stands` builds `attempted` per filing and
  `latest` runs over every `Settled::Value` including a silence zero.
- **Verdict and why:** accepted. A tie is about which filing a value was read out
  of, and a silence zero is read out of none.
- **What would have changed it:** a silence value that carried a filing. It
  carries none, by the emit record's own carriage.
