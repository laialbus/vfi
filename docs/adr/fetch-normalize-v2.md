# A filer's key crosses in one spelling, ten digits left-padded, and v2 carries it beside the period a filing reports

- **Status:** Accepted
- **Authority:** Structural
- **Proposed:** 2026-09-10, by `M4-24`
- **Decided:** 2026-09-10, by the decider
- **Touches:** `contracts/fetch-normalize/`, at v2 — one field respelled and one
  field added, proposed here and written nowhere. Structural because a contract
  field is the tier's own example, and within reach because that is all it is:
  no anchor is edited, no gate weakened, no protected-path entry moved, no
  milestone scope changed, and nothing inside either stage is decided. A
  contract version is above a worker and `contracts/` is a protected path, which
  is why this is a proposal and not a commit.

## Context

Three surfaces are fixed. This record reads them and reopens none.

**`contracts/fetch-normalize/v1.toml`**, frozen, digested by the contracts gate.
Its retrieval line says the request is "one per filer, keyed by the filer's CIK
left-padded with zeros to ten digits". Its first field says something else:

```toml
[[filer.field]]
name = "cik"
meaning = "the filer the document is about, as the document states it"
as_published = true
```

**`escalations/2026-09-08-companyfacts-cik-as-a-number.md`**, open, carried in by
M4-20. EDGAR spells a company facts document's `cik` two ways — the ten-digit
string `"0002003750"` and the bare number `1753391` — and `CompanyFacts.cik` in
`crates/fetch/src/edgar/documents.rs` is a `Cow<'a, str>`, so a document of the
second kind is refused before any field is read: *invalid type: integer
`1753391`, expected a string*. Of the 217 filers M4-20 sampled, 207 were the
second kind, Apple and Elanco among them. That count is the escalation's, taken
against EDGAR from outside the workspace, and is not re-derived here.

**`docs/adr/period-alignment.md`**, accepted. It proposes
`contracts/fetch-normalize/v2.toml` adding one field per fact,
`report_period_end`, and hands "the task that publishes v2" a claim to pin with a
fixture. The 2026-09-06 plan asked what such a fixture could pin, since the facts
document publishes nothing about the period a filing is about. This record
answers that here, where both fields are argued together, so the publishing task
inherits a check rather than a question.

### What re-derives, read this run

- **Both facts fixtures spell the key padded.** `"cik":"0002003750"` and
  `"cik":"0001859199"`. So the gate is green over a retrieval that works for the
  minority, exactly as the escalation says.
- **Every submissions document spells it padded.** Five across four filers —
  `0002003750` twice, `0002070534`, `0002141099`, `0001739104` — each a
  ten-digit string. The one overflow page in the repository carries no `cik` at
  all.
- **The registry binds the padded key and compares it as bytes.**
  `registry/filers/0002003750.toml` states `cik = "0002003750"`;
  `read::is_cik` admits a string only when it is ten ASCII digits, both for the
  filename and for the key the file states back; `Registry::overrides` finds a
  filer's file by `binary_search_by` over that string. Nothing anywhere converts
  a spelling.
- **`reportDate` on the submissions fixture for CIK 0002003750.** 41 filings, 20
  with the field empty — the registration statements, the correspondence, the
  ownership filings. Nine are periodic filings the facts fixture also covers, and
  for each of the nine `reportDate` is both the end of a duration that filing
  reports and an instant it states: 2024-03-31, 2024-06-30, 2024-09-30,
  2024-12-31, 2025-03-31, 2025-06-30, 2025-09-30, 2025-12-31, 2026-03-31.
- **The facts fixture holds a tenth filing the submissions fixture does not
  name.** `0001213900-26-088707`, a 10-Q filed 2026-08-13 carrying 193 facts.
  The facts document was recorded after the history document was.
- **The latest date a filing reports is not its period end.** On all ten, the
  latest is the cover instant, which is at `filed`; on the 10-Q filed 2025-05-14
  it is 2025-11-30, an `OperatingLeasePayments` duration running six months past
  the filing. The seam's refusal to pick a period out of a filing by its shape is
  not hypothetical on this filer.

Nothing today stores a `Filer`. `company_facts` is called from the fetch crate's
own tests and from nowhere else; the funnel has no facts step yet; normalize
consumes `Fact` and `Period` and never `Filer`. There is no stored key to
migrate, which is what makes the spelling free to fix now and not later.

## Decision

### The key crosses in one spelling

At v2, `cik` is the filer's key in the one spelling this boundary carries: ten
digits, left-padded with zeros, whichever of the two spellings the document
published. It is `as_published = false` — not because the document does not
publish it, but because what crosses is not always the characters it published,
and a flag that is true for some documents and false for others is not a property
of a surface.

**It is read off the document, not taken from the request.** Fetch reads the
document's own `cik` in either spelling as the number it is and writes it back at
ten digits, which is what `Cik`'s `Display` already does everywhere else in the
crate. Taking it from the `Cik` the request was built with would be simpler and
would give the same string every time — but only because `about()` says so, and a
field sourced from the request cannot be checked by the check that makes it true.
If that check were ever loosened or reordered, another filer's facts would cross
labelled with the key that was asked for: well-formed, wrong, and unfalsifiable
from downstream. Read off the document, the field means what it says on its own
terms and `about()` stays a check.

**Are v1's words and the registry's key one string?** For a document that spells
the key padded, yes — `"0002003750"` crosses as `0002003750` and
`registry/filers/0002003750.toml` is bound to exactly that. For a document that
spells it as a number, no, and nothing reconciles them: `1753391` would reach a
lookup that compares bytes against a key `is_cik` guarantees is ten digits, find
no file, and be indistinguishable from a filer that has no overrides — a silent
wrong answer of precisely the kind normalization is watched for. Today that
mismatch is latent rather than live, because the refusal at the parse means no
such document crosses at all. Teaching the reader both spellings without deciding
this field would convert a loud refusal into a quiet mis-key, which is worse than
the bug being fixed. Under this decision the question does not arise: one
spelling on the boundary, the one `registry/filers/` already binds, so there is
nothing left to reconcile and nowhere a reconciliation could hide.

### A document that is not about the filer that was asked for

Nothing crosses. No `Filer` is built and no fact is taken, and the retrieval
answers `Unretrieved::Unreadable` naming the key the document was filed under and
the key that was asked for — which is what `about()` does today, unchanged. What
moves is only where the key is read as a number: at v1 that read lives inside
`about()` and is called "the one place the key is read as a number rather than
carried as characters"; at v2 it is the field's own read and `about()` compares
two keys. The document about another filer is refused in either spelling, because
the comparison was already on the number and never on the characters.

Two shape refusals come with the second spelling, and both are the same answer a
quoted amount already gets — this is not the document the endpoint publishes, and
nothing is taken from it. A `cik` that is neither a string nor a number. And a
`cik` that is a number no CIK could be, one that will not write as ten digits;
the implementing run does not resolve that by truncating or by padding wider,
because a key that is not ten digits is not a key `registry/filers/` can bind.

No verdict is on record for a facts retrieval today, because no funnel step makes
one. When the step is wired, a document refused this way is *not judged* rather
than rejected — `Verdict::Unjudged`, the verdict the history step already records
when a document does not read, since nothing was decided about the filer. Which
reason it carries and where the step sits is the wiring task's. What this record
fixes is that nothing crosses.

### The surface at v2, whole

`contracts/fetch-normalize/v2.toml` is v1's bytes with the two changes below and
no others: a complete file, not a diff. `contracts/fetch-normalize/versions`
gains a second line, `v2 <sha256 of v2.toml>`, beneath v1's. v1.toml is not
edited and its line is not removed, which is the rule the contracts gate states.

Respelled — the filer's key:

```toml
[[filer.field]]
name = "cik"
meaning = "the filer the document is about, in the one spelling this boundary carries: ten digits left-padded with zeros, whichever of the two spellings the document published"
as_published = false
```

Added — one field per fact, after `filed`:

```toml
[[fact.field]]
name = "report_period_end"
meaning = "the date the filing's period of report ends, as the submissions document publishes it in reportDate, unparsed. Empty where that document publishes no report date for the filing, and where the retrieved history does not name the filing at all"
as_published = true
```

The meaning is the accepted record's, with one clause this record adds: *and
where the retrieved history does not name the filing at all*. The accepted record
allowed for the field being absent — "a filing that carried facts and no report
date would simply fail the condition" — for filings whose `reportDate` is empty.
The fixtures show a second way to be absent, and it is not a recording artefact:
the fetch record places the facts step after the history step, so a filing filed
between the two requests carries facts with no row to join to. The repository's
own pair already exhibits it. It is the same state and needs no separate rule,
but it needed saying, because "empty" is now reachable without the submissions
document publishing an empty field.

**Everything else crosses to v2 unchanged, and this record says so.** The other
filer field, `retrieved_from`, with its meaning and its flag. All eight fact
fields — `taxonomy`, `tag`, `unit`, `period`, `value`, `accession`, `form`,
`filed` — with their meanings and their flags. Both period shapes and their date
counts. All three properties: that fetch filters nothing, that nothing is parsed,
and that five fields identify a fact. And `[boundary]`, `[retrieval]`, `[filer]`,
`[fact]` and `[elsewhere]` as they stand. No field is removed and no other
meaning is edited.

The identity property survives the new field without an argument being needed
for it: `report_period_end` is a function of `accession`, so two facts agreeing
on the five identifying fields cannot differ in it. The "nothing is parsed"
property survives the respelling because it is about values and dates, and the
key is neither — which is itself worth noticing, and is the whole of why
alternative two below is refused.

### What a fixture pins, and what no fixture can

**`cik`, from a number-spelled filer.** A facts fixture's `expected` opens
`filer <key>` — today `filer 0002003750` and `filer 0001859199`. Neither moves
under this decision, because both documents already spell the key padded. A
recording from a number-spelled filer pins `filer` followed by ten digits where
the document published a bare number, and that one line pins two things nothing
pins now: that the document is read at all, and that what crosses is the string
`registry/filers/` binds. It fails if the reader refuses the document, and it
fails if the digits cross unpadded.

**`report_period_end`.** The facts document publishes nothing about which period
a filing reports, so a fixture pins the carriage and not the claim. Four checks,
all mechanical, all available from the fixture pair the repository already holds:

- the field crosses per fact, unparsed, joined to the fact's own `accession` from
  the submissions document;
- empty where `reportDate` is empty — 20 of this filer's 41 rows, none of which
  reaches the facts document;
- empty where the history does not name the filing — `0001213900-26-088707`,
  193 facts, in the facts fixture and not in the submissions fixture;
- and, as corroboration rather than proof, that on the recorded filer the date
  carried is a date that filing itself reports: for each of the nine filings both
  fixtures share it is the end of a duration the filing reports and an instant it
  states. This is what catches a join off by one filing, or `filed` carried where
  the report date belongs.

**What cannot be checked, plainly: that the date is the end of the period the
filing is about.** Nothing in either document states that period — that
withholding is why the seam sits where it does — so a fixture asserting it would
only be re-reading `reportDate` under a second name. The claim was checked by eye
on nine filings of one filer; it re-derives, above; and it stays an empirical
claim about EDGAR that no gate in this repository holds. The task that publishes
v2 records the four checks and does not owe a fifth. If the claim is false for
some filer, what finds it is that filer's data.

## Alternatives

- **Cross each spelling as published, and let normalize take either.** The
  smallest change, and the one v1's own words point at: teach the reader both
  spellings and carry the characters. Rejected because the key is what a filer's
  row is keyed by. `Registry::overrides` compares bytes against a key `is_cik`
  holds to ten digits, so `1753391` finds no override file and `0001753391`
  does — one filer, two keys, and the failure is silent, because a filer whose
  overrides did not load looks exactly like a filer that has none. Making
  normalize accept both means normalize knowing an earlier stage's document
  format, read forward across the boundary whose job is to hide it. And it leaves
  the boundary's key as two strings for one value, which "one source of truth per
  value" would then need a reconciliation to satisfy, with nothing checking that
  the two agree.
- **Pad inside fetch under v1, with no version bump.** Tempting: the bytes never
  move, the gate stays green, the escalation closes in one commit. Rejected, and
  it is the one alternative worse than leaving the bug in place. v1 says the
  field is carried "as the document states it" with `as_published = true`.
  Padding makes that sentence false while the file stating it stays frozen, and
  nothing catches the lie: the contracts gate digests the bytes and never reads
  them, and the crate's `states_what_is_published` test compares field *names*
  against the published names, not meanings. The result is a surface that
  misdescribes what crosses — the exact failure the freeze exists to prevent,
  since "the stage on the other side was compiled against the bytes that were
  there". What crosses changing is a version, whether or not a type changed.
- **One version per field — v2 for the key and v3 for the report period end, in
  either order.** Each version then carries one decision and reads cleanly on its
  own. Rejected because a published version is frozen and every change to one
  counts as breaking, so two bumps buy two frozen files, two `versions` lines and
  a consumer recompiled twice inside one milestone, with an intermediate version
  nothing ever consumes. The decider read the escalation and recommended the
  opposite: one record, one version, the contract moves once. This is that
  record.
- **Carry both: the padded key, and the document's own spelling beside it.**
  Keeps one field honest as published and gives the key a field of its own.
  Rejected because nothing downstream reads the spelling — normalize keys by the
  string, the registry binds ten digits — so it is a second representation of one
  value with nothing checking the two agree, which is the unchecked duplication
  the one-source-of-truth invariant bans. EDGAR's choice of spelling is that
  document's formatting, not a fact about the filer, and the provenance that does
  matter is already carried: `retrieved_from` names the request the document came
  back from.
- **Cross the key as a number, or as the `Cik` type itself.** The number is the
  thing; the spelling is presentation. Rejected because the boundary carries
  characters, by the property sitting beside the field, and a number on the
  boundary pushes the padding into normalize, where the registry's key format
  would be re-derived instead of read — the first alternative's objection, moved
  one stage later. `Cik` lives in `vfi-fetch`; moving it into `vfi-contracts` to
  put it on the boundary would widen that crate from the shapes it publishes to
  the types a stage works in.
- **Take the key from the request rather than from the document.** Simpler than
  the decision: fetch holds the `Cik` it asked with, `Display` already writes ten
  digits, nothing needs reading, and the malformed-key refusal disappears
  entirely. Rejected because the field would then say what was asked for rather
  than what the document is about, and those coincide only because `about()` says
  so. Sourcing a field from the request makes it unfalsifiable by the check that
  justifies it. Reading it off the document costs one parse that `about()` was
  already doing, and the two strings are identical whenever a `Filer` exists at
  all.

## Consequences

**Easier.** One key: a filer's request URL, the string on the boundary, its
registry filename and the key that file states back are the same ten digits, so
a lookup is a comparison and never a conversion. The retrieval starts working for
the nineteen filers in twenty it refuses today, which makes the majority case
recordable as a fixture — including from CIK 0001739104, a filer whose history is
already `a-history-longer-than-its-first-page` and whose facts do not retrieve.
And `shares_outstanding` gets the entry the accepted alignment record needs, on
the same version, so the boundary moves once for both.

**Harder.** Publishing v2 moves `crates/contracts/src/fetch_normalize.rs` and its
`states_what_is_published` test from `v1.toml` to `v2.toml`; `v1.toml` keeps its
bytes and its line in `versions`, and nothing states it in Rust any more. That
follows from the gate's own rule rather than from a decision, and it is named
here so the publishing task does not treat it as one. That task's diff touches
`contracts/`, a protected path: its task file must list the path under `owns` for
the hook to allow the write, and its pull request still needs the human-approved
label on the server. `report_period_end` also makes fetch join two documents it
retrieves separately, and where the join finds nothing the field is empty — so a
fact's report period end can be absent for a reason that has nothing to do with
the filing it came from, and normalize reads that absence as the condition not
being met, which is the accepted record's own reading of it.

**Expensive to reverse.** Once v2 is published its bytes are frozen; returning to
the document's spelling is a v3, not an edit. Nothing else is: v1 keeps its bytes
and its line, both existing facts fixtures keep theirs, and no stored value
changes, because nothing stores a `Filer` yet. This is the last moment the
spelling is free to fix.

**Not decided here.** No registry entry, no rule inside normalize, nothing about
what a `Value` carries, and no field of the submissions history beyond the one
the accepted record already proposed. The escalation
`escalations/2026-09-08-companyfacts-cik-as-a-number.md` stays open: the run that
teaches the retrieval both spellings deletes it, in the same diff as the change
that closes it.

## Enforcement

Anchor 3 — stages talk through explicit contracts — and the invariant beside it,
one source of truth per value.

What holds it: the contracts gate, which freezes v2.toml by digest, requires
`versions` consecutive from v1, allows one file per published version, and fails
if anything published is edited or deleted. The crate's `states_what_is_published`
test, comparing the fields the Rust types declare against the `[[filer.field]]`
and `[[fact.field]]` names v2 states, once it is pointed at v2. `read::is_cik`,
which refuses a filer file not named ten digits, so the far end of the key is
already checked. And the golden fixture recorded from a number-spelled filer,
which is what pins that the document is read and that the key crosses padded.

What does not, said plainly. **No gate compares a field's meaning to what fetch
does with it.** The gate digests bytes; the test compares names. A later change
that padded, truncated or re-spelled the key while the bytes stood still would be
caught by that fixture and by nothing else — which is why the fixture belongs to
the same task that publishes v2, and why padding under v1 is refused above.
**And nothing checks that `reportDate` is the period a filing reports**, for the
reason given under what a fixture pins: neither document states that period, so
there is nothing to check it against.

## Decision review

- **Authority:** Structural, and within reach: it respells one field and adds
  one on the fetch → normalize boundary, the tier's own example, proposed here
  and written nowhere. No anchor is edited, no gate weakened, no protected-path
  entry or milestone scope moved; the write to the contract directory is the
  publishing task's, under its own `owns` grant and the human-approved label.
  Flagged for later human review, as the tier requires.
- **Checked:** anchor 3 and the one-source-of-truth invariant, applied: the key
  on the boundary becomes the string the registry binds, so there is one
  representation and nothing to reconcile. Anchor 2, in the first alternative's
  refusal to have normalize read a fetch document's spelling. Against the
  accepted records: the fetch record's "nothing is parsed" is about values and
  dates, and its own URL already pads the key by `Cik`'s `Display`; the
  alignment record's `report_period_end` is carried with its meaning intact and
  one clause added — empty where the retrieved history does not name the
  filing — which is a second way into the state that record already reads as
  the condition not being met; the registry record's ten-digit key is the one
  that crosses. Repository claims re-derived: `CompanyFacts.cik` typed as a
  string; `about()` comparing on the number and naming both keys in
  `Unreadable`; `is_cik` and `binary_search_by` in the registry;
  `states_what_is_published` comparing names and never meanings;
  `company_facts` called from fetch's tests and nowhere else;
  `Verdict::Unjudged` as the history step's verdict for a document that does
  not read. Fixture claims re-derived: both facts fixtures and all five
  submissions documents spell the key padded and the overflow page carries
  none; 41 rows with 20 empty `reportDate`; on the nine shared periodic filings
  `reportDate` is both the end of a duration the filing reports and an instant
  it states; the tenth filing, `0001213900-26-088707` with 193 facts, is absent
  from the history; the 10-Q filed 2025-05-14 is the one whose latest date is a
  lease term. Six alternatives, each rejection naming a mechanism.
- **Verdict and why:** Accepted. One key, read off the document: a lookup is a
  comparison, a mis-keyed filer has nowhere to hide, and `about()` stays a
  check rather than a tautology because the field it guards is not sourced
  from what it compares against. A document about another filer crosses as
  nothing in either spelling. Both fields move on one version, which is what
  the open escalation was read as needing. The record is plain about what no
  fixture can hold — that `reportDate` is the period a filing is about — and
  hands the publishing task four mechanical checks in place of a claim. Two
  wordings are loose and neither is a defect: the two Form 3 rows carry a
  `reportDate`, so among the twenty empty rows "the ownership filings" is the
  Schedule 13D alone, a sentence inherited from the accepted record; and the
  cover instant sits at `filed` on all ten filings but is the latest date on
  nine.
- **What would have changed it:** the key taken from the request, which would
  have made the field unfalsifiable by the check that justifies it; padding
  under v1 with the frozen surface left saying "as the document states it";
  two versions for two fields; a malformed key truncated or padded wider
  instead of refused; or a repository or fixture claim that did not re-derive —
  the padded spelling of the five submissions documents and the tenth filing
  above all.
