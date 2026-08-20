# Fetch retrieves a filer's XBRL company facts and hands normalize every fact in it, each carrying the tag it was reported under, the filing it was reported in, and the request it came from

- **Status:** Proposed
- **Authority:** Structural
- **Proposed:** 2026-08-19, by `M4-01`
- **Decided:** —
- **Touches:** the fetch → normalize contract (anchor 3), and the retrieval that
  produces it. One decision rather than two, because a contract cannot carry a
  field no retrieval ever asked for. A contract between stages is above a
  worker, which is why this is a proposal and not a commit.

## Context

M4 turns filings into comparable numbers, and normalize cannot begin until it
knows what it is given. What fetch produces today is the history index — what a
filer filed, when, and where its documents are — and `crates/fetch/src/filing.rs`
says so outright: the columns beyond that "arrive when a stage has a use for one,
and what the next stage is owed is the fetch → normalize contract, which is not
settled here." Normalize is a stub that copies its input, and its one fixture
pins the doing of nothing. This is that contract, proposed.

Two questions are settled together because they are one question. Which document
the reported facts are read out of decides what fetch retrieves; what fetch
retrieves bounds what can cross. Answering the second alone produces a contract
with fields nothing stands behind.

Four things are fixed before the decision starts.

- **Fetch makes no reading.** `filing.rs` states the doctrine and gives the
  reason: each field is carried as EDGAR published it, because "a reading made in
  the fetch stage is one the normalize stage cannot check or correct." Every
  parse, filter and alignment this record declines is declined for that reason.
- **Half of M4's provenance exists only here.** Every resolved value must record
  its source tag, its filing, and the rule that set it. The rule is normalize's
  own. The tag and the filing exist only in what fetch hands over, and normalize
  can record only what it was handed — so a contract that drops either makes the
  criterion unmeetable downstream, and it fails late, when the fixtures are being
  written and there is nothing to write down.
- **The funnel's arithmetic.** M3's whole shape is the requests it does not send:
  a seed set, then a metadata gate, then history for survivors only, against the
  ten requests a second the source publishes. A retrieval per filing rather than
  per filer is a different order of magnitude, and the funnel's premise is what
  decides which is affordable.
- **The shape of a contract.** `contracts/` does not exist yet. The contracts
  gate fixes its shape whatever the format turns out to be: a directory per
  contract, a `versions` record of `v<N> <sha256>` lines consecutive from v1, one
  `v<N>.<ext>` file per published version, and nothing already published ever
  edited.

Structural rather than Constitutional: this adds a contract and its fields, which
is the tier's own example. No anchor is edited, no gate weakened, no
protected-path entry changed, and no scope in GOALS.md moved. It applies anchor 2,
anchor 3, anchor 5's ban on the unsourced value, and the invariant that every
resolved fact records where it came from. The host list needs no change either:
`data.sec.gov` is already on it.

## Decision

### What is retrieved

The filer's XBRL company facts document, one request per filer:

```
https://data.sec.gov/api/xbrl/companyfacts/CIK##########.json
```

The key is padded to the ten digits `Cik`'s own `Display` already writes, so the
URL is built the way `submissions_url` is built and by nothing new.

This becomes the funnel's fourth step. It runs after the history step, for the
filers that step admitted and for nobody else, out through the same chokepoint
and behind the same pace as the three before it, and it leaves a verdict per
filer on the ledger exactly as they do. A filer EDGAR publishes no facts for
stops there with that reason on record and never reaches normalize; an empty fact
set does not cross the boundary, because normalize could not tell it from a
retrieval that failed.

### Where the facts are reported, and where they are read

They are **reported** in the filer's own periodic filings — the XBRL exhibit of
each 10-K and 10-Q. They are **read** out of the company facts document, which is
EDGAR's per-filer collation of those exhibits: the facts a filer tagged across its
whole filing history, each one stamped with the filing it was reported in.

So the document retrieved is not the document a fact was reported in, and the
contract carries the second inside every fact rather than the first. That is what
makes one request per filer sufficient: what would otherwise be recovered by
fetching eighty filings and reading eighty exhibits is already assembled, and
assembled with the accession numbers intact, so nothing about where a value came
from is lost in the assembling.

Four reasons for this document rather than the filings themselves:

1. **A filer's whole history in one request.** The corpus is thousands of filers
   and a filer's history is decades. Per-filing retrieval multiplies the funnel's
   surviving population by the length of its history; this multiplies it by one.
2. **Restatements arrive already collated.** One period reported twice — an
   original and its amendment — appears as two facts under two accessions with
   two filed dates. That is precisely the input M4's restatement ruleset needs,
   and it is what a per-filing retrieval would have to reassemble by hand.
3. **No XBRL processor in fetch.** Contexts, units, dimensions, scale and sign on
   an inline document are a reading, and a reading in fetch is one normalize
   cannot check. This document is JSON of the same kind the stage already reads,
   and each fact arrives as a value, a unit and a period.
4. **The vocabulary it must serve is amounts.** The twenty-five concepts
   `canonical-concepts.md` proposes are monetary amounts, share counts and
   per-share amounts, every one of them. A fact product keyed by unit of measure
   is the shape that vocabulary asks for, and nothing in the vocabulary is asked
   of a field this cannot carry.

What it costs is presentation, and that is the whole of what it costs. This
document publishes facts, not statements. It says a filer reported an amount under
a tag for a period; it does not say the line appeared on the face of a statement,
in what order, or under what heading. The cost lands on somebody else's decision
and is settled below rather than waved away.

### What crosses, field by field

One value per filer. Two fields on it, and eight on each fact.

Per filer:

| field | what it is | why it crosses |
| --- | --- | --- |
| `cik` | the filer the document is about, as the document states it | the key every downstream row is keyed by; a fact set naming no filer belongs to nobody |
| `retrieved_from` | the request the document came back from, as the existing `Source` | the third leg of provenance. One per document and shared, which is what `Source` is already for |

Per fact:

| field | what it is | why it crosses |
| --- | --- | --- |
| `taxonomy` | `us-gaap`, `dei`, and whatever else the document publishes, as published | half of the tag: an element name means nothing without the taxonomy that defines it |
| `tag` | the element name, as published | the other half — the source tag M4 requires every resolved value to record |
| `unit` | the unit key as published: `USD`, `shares`, `USD/shares` | the vocabulary fixes every amount in the filing's reporting currency at a scale of one, and without the unit a figure in another currency is indistinguishable from one in dollars |
| `period` | the fact's own period: an instant, which is one date, or a duration, which is two | the vocabulary marks each concept a flow or a balance, and a duration fact answering a balance concept is a resolution error normalize has to be able to see |
| `value` | the decimal literal exactly as published, unparsed | the amount. Unparsed because a conversion made in fetch is a reading normalize cannot check or undo, and a binary float is a lossy one |
| `accession` | the accession number of the filing the fact was reported in | the filing M4 requires every resolved value to record |
| `form` | that filing's form type, as published — `10-K`, `10-K/A` | the amendment ruleset has to tell an original from an amendment, and the form is the only field that says which it is |
| `filed` | the date that filing was received, as published | the restatement ruleset orders two reports of one period by when they were filed |

The three legs of provenance are `taxonomy` with `tag`, `accession` with `form`
and `filed`, and `retrieved_from`. Each sits on the value it belongs to rather
than beside it: `accession`, `form` and `filed` repeat across every fact one
filing reported, and they repeat on purpose — see the last alternative.

### What does not cross

- **`label` and `description`** — the taxonomy's prose for a tag. On every fact,
  read by no rule. Carried because it is there, and so left out.
- **`frame`** — EDGAR's key for the calendar period a fact best fits. The most
  useful-looking field in the document and the most clearly out. GOALS.md makes
  period alignment a ruleset of ours, stated and kept separate from the mapping,
  and this field is another party's alignment of a fiscal period onto a calendar
  one. Taking it would import the judgement the ruleset exists to make.
- **`fy` and `fp`** — the fiscal year and period of the *report* a fact appeared
  in, not of the fact. A field that reads as the fact's own period and is not is
  the exact shape of mistake this project treats as its worst, and the fact's own
  period already crosses. Left out for what it would invite rather than for what
  it is.
- **`entityName`** — the filer's name. No normalization rule reads it, and the
  names a filer was seen under are already on the ledger, recorded as observed at
  the moment of the verdict.
- **The filing history.** The funnel already retrieves the submissions document,
  which lists every filing with its form, its dates and its primary document. It
  does not cross. Every fact already names the filing it came from, and a filing
  the facts never name reported nothing normalize can resolve — so carrying the
  history too would be a second copy of the same fact with nothing checking that
  the two agree, which is the unchecked duplication the one-source-of-truth
  invariant bans.
- **The filer's published classification** — its SIC code, and the rest of what
  the submissions document says about the filer rather than its filings.
  `canonical-concepts.md` needs a filer kind established before any concept is
  looked up, and leaves the assignment to the registry, which is per-company data
  with per-company overrides. No rule that reads a SIC code exists. Carried now it
  is a field with no consumer; wanted later it is a version bump with one. This is
  the field most likely to become v2, and it is left out anyway, because what
  decides is whether a rule reads it and not whether someone can imagine one that
  would.
- **The filing's own document, its presented statements, their order and
  headings, its footnotes, and any dimensional breakdown.** Not declined — not
  published by this document at all. Named because the absence has a consequence,
  and a reader deserves to find it stated here rather than discover it later.
- **Prices.** Not in filings and not on this boundary. `canonical-concepts.md`
  already puts them behind the price provider interface.

### Three properties, checked rather than assumed

- **Fetch filters nothing.** Every fact the document publishes crosses: not
  filtered by tag, not by taxonomy, not by form. Filtering by tag would put the
  registry's tag list inside fetch, which is normalize's data reaching backward
  into an earlier stage — the edge anchor 2 turns into a compile error. It would
  also destroy the one reading of silence left, because normalize can say "this
  filer reported facts from this filing and this element is not among them" only
  if what it holds is everything the filer reported.
- **Nothing is parsed.** Values and dates cross as the characters EDGAR
  published. The parse belongs where the rules that depend on it live.
- **A fact is identified by `taxonomy`, `tag`, `unit`, `period` and `accession`,
  and no two facts crossing share those five with different values.** This is what
  turns "consolidated over segment" into a question about the retrieval instead of
  a rule normalize must apply blind. If the document publishes only undimensioned
  values, the key is unique and the segment question never arises at this
  boundary. If it publishes a dimensioned fact, two values collide under one key,
  and the collision is visible rather than silent. The task that builds this
  contract records a fixture and checks the property against it. A collision is an
  escalation: the contract would then need the axis and the member, and that is a
  decision, not something a run picks a winner for.

### The contract

`contracts/fetch-normalize/`, in the shape `scripts/gates.sh` already fixes:

- `contracts/fetch-normalize/versions` — one `v<N> <sha256>` line per published
  version, consecutive from v1, holding exactly one line at first publication.
- `contracts/fetch-normalize/v1.toml` — the single file that is the surface at v1.
  Frozen once its line is in `versions`; every later change publishes `v2.toml`
  with its own line, edits nothing already published, and deletes nothing
  `versions` names.

Named for the edge because the repository already names it for the edge: the task
and `filing.rs` before it both call this the fetch → normalize contract. Unlike
the concept vocabulary, this surface serves exactly one edge and nothing else
reads it. The sibling record took the name the constitution already used for its
surface; this one takes the name the code already uses for this one. Same rule,
different answer, because the existing names differ.

Format: TOML. The format is one decision for every contract in the repository
rather than this contract's alone, and `canonical-concepts.md` proposes TOML and
defers to this record if this one is decided first. Both propose the same thing,
so whichever the decider takes first settles it without contradicting the other.
The reasons are that a contract file is data and not code, so nothing about it
should need compiling to be read; that it takes comments, and a surface whose
fields carry meanings needs them; and that it diffs and digests byte for byte,
which is what the gate freezes.

v1 states: the contract's identity and the boundary it sits on; the retrieval that
produces it, by URL; the per-filer shape and the per-fact shape, each field with
its name, its meaning, and whether it is carried as published; the two shapes a
period takes; and the three properties above.

v1 does not state which tag resolves to which concept — that is the registry,
versioned separately — nor which facts matter, which is the vocabulary and its own
contract, nor any threshold or default, which anchor 5 puts in the settings layer.

### What this closes for `canonical-concepts.md`

That record's eighth open question — "what silence is silence in" — turns on this
one and closes with it. Silence is silence in a fact set, not in a presented
statement. What normalize is handed is everything the filer tagged, not everything
the filer presented, so "the filer presented no such line" is not a claim this
boundary can support, and the five zero readings — `inventory`,
`short_term_debt`, `long_term_debt`, `dividends_declared_per_share` and
`dividends_paid` — cannot rest on it as written.

What remains available is weaker and stated over the fact set: the filer reported
facts from this filing, and no fact among them is under a tag the registry
recognises for this concept. Restating the five over that, or falling back to
`Unknown`, is the sibling record's to do at its v1 and is not edited here. Two
things for whoever does it. The safe direction is `Unknown`: a zero that is not
there is a fabricated number, which is the failure this project treats as worst,
while a correct absence with a reason is what M5 and M7 are already built to show.
And two of the five cannot take that fallback — GOALS.md names a dividend
suspension among the golden fixtures, and a suspension that resolved to `Unknown`
would break the streak it exists to measure.

### What this does not decide

Which tags resolve to which concept, and how a filer is assigned a kind, are the
registry's. Choosing among several candidates in one filing, and reconciling
amendments, restatements and period alignment, are their own rulesets. What the
concepts are is `canonical-concepts.md`. How resolved facts are stored, and what
normalize hands analyze, are later boundaries. Turning a value into a number and a
date into a date is normalize's, and where that lives inside normalize is
normalize's to decide.

## Alternatives

- **Retrieve each filing's own XBRL exhibit.** The primary source, with
  presentation, footnotes and dimensions intact — it would answer the silence
  question outright. Rejected on cost and on reading. On cost: one retrieval per
  filing instead of per filer, against a published ten requests a second, which is
  the funnel's premise inverted. On reading: it needs an XBRL processor inside
  fetch — contexts, units, dimensions, scale and sign — and every one of those is
  a judgement normalize could not check or correct, which is the thing
  `filing.rs` says the stage does not do. What it would buy is real and is
  recorded as a consequence rather than dismissed; a later version can add the
  instance for the filings where presentation turns out to decide something.
- **The quarterly Financial Statement Data Sets.** Bulk archives carrying, among
  other things, which statement and line a tag appeared on — the one alternative
  that would close the silence question without a per-filing retrieval. Rejected
  on freshness and on shape: a filer's history is assembled from decades of
  quarterly archives, a filer's newest filing is absent until its quarter's
  archive publishes, and a bulk product makes the funnel meaningless, since the
  requests it does not send are the whole of what it is. Rejected on those and not
  on quality.
- **The company concept endpoint, one request per tag per filer.** Rejected: it
  needs the registry's tag list inside fetch to know what to ask for, which is the
  backward edge anchor 2 forbids, and it multiplies requests by the number of
  candidate tags while carrying strictly less than the one document that already
  holds them all.
- **Carry only the facts the registry names.** Rejected for the same backward
  edge, and because it removes the last reading of silence: normalize could not
  tell a tag the filer never reported from a tag fetch did not carry.
- **Carry the filings once, and have each fact point at one by accession.** The
  normalized shape, and it would stop `accession`, `form` and `filed` repeating
  across every fact a filing reported. Rejected because it makes provenance a
  join, and a join has a failure mode — a fact pointing at a filing that is not
  there — whose result is a value whose filing cannot be named. Losing provenance
  is the one thing this contract exists to prevent, so it does not get a way to
  fail that repetition does not have.

## Consequences

Easier: normalize acquires an input it can be written against — a filer, a flat
list of facts, and provenance on each one — so the registry task is finding tags
for concepts rather than deciding what a fact is. Restatements and amendments
arrive already collated under their accessions, so the ruleset that reconciles
them reads dates and forms instead of re-fetching. A fiscal-year change is visible
in the facts' own periods, which is what GOALS.md's fixture for it needs. And
fetch grows by one step of the shape it already has: one JSON document per filer,
through the chokepoint, at the published pace, with a ledger verdict, on a host
already allowed.

Harder: everything crosses, and a filer with a long history and a wide taxonomy
publishes far more than the submissions document the funnel handles today. The
implementing task records what that costs against the benchmark baseline M2
committed rather than assuming it is free. Presentation is gone, so the five zero
readings above have to be restated or given up. And the filer kind now has no
evidence crossing this boundary at all: the registry must carry it as per-company
data, which is the shape GOALS.md asks for and is also a table somebody has to
fill.

Expensive to reverse: adding a field is a version bump — a new file and a new
line, cheap mechanically, an ADR each time by the anchor that made this one an
ADR. What is expensive is the retrieval. Once fixtures are recorded against this
document and the store holds facts keyed the way it publishes them, moving to
per-filing exhibits re-fetches the corpus and re-records every fixture.

For whoever queues the follow-on task: `contracts/` is protected and grantable, so
the task that writes v1 must list `contracts/fetch-normalize/` under `owns` in its
task file as committed on `origin/main` or the hook refuses the write, and the
merge still needs the human-approved label. Whichever contract lands first also
hits the collision `canonical-concepts.md` already recorded between
`docs/layout.md` and the contracts gate over what a directory under `contracts/`
means; it is not restated here.

## Enforcement

This changes no anchor; it applies three. The mechanical half is the contracts
gate: once `versions` names v1, that version's bytes are frozen, and an edit to
the published surface goes red with no change to the gate.

What nothing checks, said plainly. None of these is a gap in this diff, which adds
no code — each is one the task that builds the contract must not walk past.

- **That the type the stages compile against says what `v1.toml` says.** The gate
  digests the file and never reads it, so a type that drifted from it stays green.
  The remedy is the one the one-source-of-truth invariant names: generate one from
  the other, or check them for drift in CI.
- **That fetch filters nothing.** The shape of the check is a golden fixture — a
  recorded company facts document, and an expected result holding every fact in
  it — so a filter added later goes red instead of quietly shrinking what
  normalize sees.
- **That no two facts collide on their five identifying fields.** Until the
  fixture above exists, this paragraph is the only guard the property has, and it
  is the property the segment question rests on.

## Decision review

By the decider, not the proposer.
