# A task cites a source the worker can read from the run, by the locator that source has; a book nobody can open is named as the record names it and never cited by a page or exhibit

- **Status:** Accepted
- **Authority:** Structural
- **Proposed:** 2026-10-15, by the lead (supervised session), on the owner's
  delegation, from the M5-29 stop
- **Decided:** 2026-10-15, by Albus Lai (human), in the session that proposed
  it; merging the pull request that carries this record is the signature
- **Touches:** one sentence of the accepted
  `which-other-period-a-metric-reads.md`, "Each metric's documentation cites
  its source by page or exhibit as its task reads it", which this record
  narrows to sources a worker can read. It applies anchor 5, that every
  constant and method carries its source, and edits no anchor, no contract,
  no protected path and no gate. It also fixes what a task may ask for when
  it names a source, which the planner writes.

## Context

M5-29 stopped before writing code. Its acceptance asked each metric's
documentation to cite Robinson, Henry, Pirie and Broihahn, *International
Financial Statement Analysis*, 3rd ed., "by exhibit as the task reads it".
The book is a paid Wiley title. It is not in the repository, no legal copy is
reachable from a run, and the one attempt returned 403. The period record
says the implementing task's reading is the only check on a source, so an
exhibit number written from memory would be a citation nobody read. The
worker refused to write one. That was right.

The same sentence reaches further than one task. M5-30 asks for the same
book by exhibit, and the thirteenth batch, open as it was proposed, asks for
two more books by exhibit, Pinto, Henry, Robinson and Stowe's *Equity Asset
Valuation* and Penman. Every such task will stop the same way.

The owner cannot close the gap by hand. The owner is not a finance reader,
owns none of these books, and will not be supplying pages or exhibits.
Learning the subject through the tool is one of the project's purposes, so a
source the owner can open is worth more here than one they cannot.

What a run can reach is fixed outside the record. The sandbox allows a short
list of hosts, and the ones that matter for sources are arXiv, SSRN, MIT
OpenCourseWare and Wikipedia; a fetch tool reaches public pages beyond that
list, within what a site serves to an unauthenticated reader. Adding a host
is the owner's, on a protected file.

The M5-29 stop itself is ruled in `tasks/M5-29.md` and `tasks/M5-30.md`:
the chapter is cited as the record cites it, and the locator is the CFA
Institute's *CFA Program Level II Financial Ratio List* by item, quoted as
printed. This record generalises that ruling.

## Decision

A citation has two parts, and they are held to different standards.

**The origin is named as the record names it.** Where an accepted record
fixes a method or a constant to a source, the metric's documentation names
that source as the record does, author, title, edition, year, chapter or
section. This part is never dropped and never replaced: it is what anchor 5
asks for, the paper or method the value comes from.

**The locator is read, or it is absent.** A page, an exhibit, a table, an
item number or a quotation appears in a citation only when the worker
opened that copy during the run and read it there. A locator from memory,
from a study guide or from a second-hand quotation is not written. Where the
origin cannot be opened from a run, the documentation says so in a sentence,
"not read from the workspace", and cites the origin by its chapter alone.

**An open primary source stands beside an unreadable origin.** Where the
body that owns the method publishes the same definition in a form a run can
open, an institute's list, a standard, a working paper, the author's own
paper, that publication is cited beside the origin, by the locator it has,
quoted as printed. It is a second source for the same statement, not a
substitute for the origin. A secondary source, notes or a textbook summary
by someone else, is not cited; it may lead the worker to a primary, and
nothing more.

**A task names what it asks for, and names only what can be read.** A task
that requires a locator names the readable copy, by URL or by the file in
the repository, and the planner has opened it before writing the task. A
task whose source is a book nobody can open asks for the origin by chapter
and for an open primary where one exists, and asks for no page or exhibit.

**An unreadable source is a stop, as now.** A worker that cannot open a
source its task names as readable stops and escalates, naming what it tried.
The resolution is the planner's: re-scope the task to an open primary, or
to the chapter alone. The owner does not supply pages.

The period record's sentence is read under this: "by page or exhibit as its
task reads it" holds for a source the task can read, and for no other. Its
substance, which reading each metric takes and from which source, stands.

## Alternatives

- **The owner supplies the page or exhibit from a copy they own.** Rejected
  by the owner: they own none, and will not be acquiring finance texts to
  type locators from. It would also make a citation the worker never
  verified, which is the thing the period record's enforcement says it
  cannot see.
- **Drop locators altogether and cite chapters only.** Rejected: where a
  source can be read, the locator and the quotation are what let a later
  reader check the derivation against its axiom without re-reading the
  chapter. The cost is a few words; the gain is anchor 5's traceability made
  concrete.
- **Allow secondary sources as locators, study notes and summaries.**
  Rejected: a summary is someone else's reading, and the record's check is
  the worker's reading. Where a summary disagrees with the origin, nothing
  here would see it.
- **Put copies of the books in the repository.** Rejected: they are not the
  project's to distribute.
- **Widen the sandbox allowlist until the books are reachable.** Not
  possible: the books are behind paywalls, not behind the allowlist. The
  allowlist is the owner's to widen for open hosts, and this record does not
  ask for it.

## Consequences

**Easier:**

- A metric task no longer stops on a locator it was never going to get. The
  planner writes the citation it can verify, and the worker reads what the
  task names.
- Every locator in the crate is one a reader with a browser can check, which
  serves the owner's own learning as much as review.
- The M5-29 and M5-30 rulings stop being special cases.

**Harder:**

- The planner opens a source before naming it, which costs a planning run a
  fetch per source.
- Some definitions will carry a chapter and no locator, where the only
  statement is in a closed book. For those the record's own statement of
  the reading is the derivation's axiom, and the citation says it was not
  read here.
- The thirteenth batch's tasks, as proposed, ask for Pinto and Penman by
  exhibit, and must be redelivered under this record before they are
  claimable work.

**Expensive to reverse:** nothing. A locator added later to a citation that
lacked one is an ordinary documentation change.

## Enforcement

This applies anchor 5 and does not change it. Nothing checks it
mechanically, and that is stated rather than assumed:

- **Gap:** nothing reads a citation for a locator and asks whether the
  worker opened the source. Only review sees a page number from memory. The
  phrase "not read from the workspace" at least makes the unread case
  visible to a reader.
- **Gap:** nothing checks that the planner opened a source before naming it
  as readable. A worker's stop is the check, one run late.
- The literals gate still holds that every constant is named and defined
  once; this record is about the words beside the constant, not the constant.

## Decision review

By the owner, in the session that proposed it; the review is the owner's
decision as stated there, and merging is the signature.

- **Authority:** Structural, and within the owner's reach: it narrows one
  sentence of an accepted record about documentation, touches no anchor,
  contract, schema or protected path, and moves no gate.
- **Checked:** anchor 5's wording, which asks that a constant carry its
  source and does not ask for a page; the period record's "What no check can
  see", which already names the worker's reading as the only check; the
  M5-29 escalation and the published ratio list, whose items 14, 20 and 21
  were read in the session; the sandbox allowlist, for what a run can reach.
- **Verdict and why:** accepted. A citation nobody read is none, and the
  owner will not be the reader of last resort. The planner names what can be
  read, the worker reads it, and a closed book is named and left closed.
- **What would have changed it:** an owner with the books and the time to
  read them for every metric. There is neither.
