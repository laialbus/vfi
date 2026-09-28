# The normalize golden harness checks a fixture's `| ` lines against `expected`: each group consecutive, each listing in order, each group under the period line it follows

- **Status:** Accepted
- **Authority:** Structural. It adds a check to the fixtures gate and a proof to
  the gate runner. It loosens nothing and changes no anchor, contract or schema.
  The runner is a protected path, so the implementing task needs the
  `human-approved` label on its pull request, as
  `protect-paths-owns-grant.md` sets out.
- **Proposed:** 2026-09-27, by M4-51
- **Decided:** 2026-09-27, by the decider
- **Touches:** what the normalize golden harness checks (the fixtures gate) and
  that gate's proof of catch in `scripts/gates.sh`. Also the way a `claims`
  file states placement: a period line put before the claims it places. This is
  the record the accepted `golden-claims-and-baseline-re-recording.md` asks for
  when it says the check "is an exclusive task with its own record since it
  touches a gate". It edits and supersedes nothing. Nothing is implemented here.

## Context

The golden-claims record says the harness checks every claim against
`expected`. Today it checks nothing of `claims`, and the decider re-derives
every claim by hand. M4-44 fixed the form and three fixtures carry it:
1,694 lines, of which 877 begin `| `. Every one of those 877 lines is in its
fixture's `expected`. Most of the rest is prose that no check can read. The
three files do not state their form the same way:

- `every-fact-a-filer-reported`: the header says "lines given together are
  consecutive there". Its 32 period lines are given together and are not
  consecutive in `expected`, as `sessions/2026-09-26-decider-2.md` flagged.
  Its "restated diluted loss" section places two lines under periods in the
  prose after them.
- `a-filer-whose-shareholders-equity-is-negative`: the header states the
  exception, "but for the period lines, which are given in the order
  `expected` writes them". Everywhere it places a line under a period, it
  gives that period's `| ` line first.
- `a-filer-that-changed-its-fiscal-year`: the header says consecutive with no
  exception. The listing's own section adds "They are not consecutive there".
  Its fourteen placements are written in prose, "Under `at 2023-12-31`:", with
  no `| ` period line. Three of the lines it places occur many times in
  `expected`: 15, 2 and 28 times.

## Decision

**What the check reads.** Only lines of `claims` that begin `|`:

- A line beginning `| ` is a claimed line. Its text is everything after those
  two characters, byte for byte, trailing whitespace included.
- A line beginning `|` without the space is red. Otherwise a typo would drop a
  claim into prose without anyone seeing it.
- Every other line is prose to the check, with one exception: a line beginning
  `#` ends a scope (see "Anchor" below).
- A group is a maximal run of claimed lines. Any other line ends it, a blank
  line included.
- A period line is a claimed line whose text begins `at ` or `from `. In
  `expected`, only a period line begins that way.

**What it holds each group to**, against the committed `expected`. The check
does not use the stage's output; the byte-for-byte comparison is what ties that
to `expected`.

- **A listing** is a group of two or more lines, all of them period lines. Each
  line must be in `expected`, and each must come after the one before it.
  Consecutiveness is not asked: each period line in `expected` is followed by
  its twenty-eight concept lines, so no two period lines are ever adjacent. A
  listing could never be consecutive, and order is what all three listings
  claim.
- **An anchor** is any other group whose first line is a period line. It is
  held to consecutiveness like every group, and it also places the groups that
  follow it:
  - Its scope runs from the anchor to the next anchor, the next listing, or
    the next `#` line, whichever comes first.
  - Each group in that scope must lie in `expected` after the anchor and before
    the next period line.
  - The groups must appear in the order written. Each is matched at its first
    occurrence after the end of the group before it.
  - The anchor's period line must occur exactly once in `expected`. Otherwise
    the check cannot tell which period it names, so that case is red.
- **Every other group** must be consecutive in `expected`, in the order
  written: exactly the lines given, adjacent. A group that no anchor places may
  sit anywhere in `expected`.

**`registry` matches as itself, the literal word.** The stage writes the
registry a history is rendered under as the one word `registry`
(`what-normalize-emits.md`; `rendering::REGISTRY`), and writes any other
version as its sixty-four characters. The harness runs under the committed
registry. So a claim's `registry` matches exactly where the stage named the
registry the fixture ran under, and a line naming another version fails the
comparison, which is what the emit record asks for. The check substitutes
nothing and imports no constant from the stage. Matching through the stage's
constant would let the check follow the engine. Writing a digest into claims
would mean editing every `claims` file on every registry edit, which is the
churn the one-word rendering exists to avoid.

**What it reads and what it cannot read.** It reads four kinds of claim:

- a line is in `expected`;
- a group is consecutive there;
- a listing is in `expected`'s order;
- a group sits under a named period, in the order written.

It cannot read:

- the counts (periods, `silence`, `Unknown`, `read`, 28 lines per period);
- the "these are the only lines that begin `at ` or `from `" claims, and
  "each is written once" beyond what an anchor asks;
- claims of absence, such as "no `from 2023-05-01 to 2024-04-30`" and "no line
  beginning `    tie`";
- the vocabulary's order;
- the "ending before the next" of a period given in full;
- every `from:` derivation, which is about facts and records that `expected`
  does not hold.

Everything it cannot read stays the decider's re-derivation by hand, as it is
today. This check narrows that work; it does not end it.

**Red conditions.** The check goes red on any of these:

- a fixture directory with no `claims`;
- a `claims` with no claimed line, so no fixture passes vacuously;
- a line beginning `|` without the space;
- a claimed line absent from `expected`;
- a group that is not consecutive there;
- a listing out of order;
- an anchor whose line is not in `expected` exactly once;
- a placed group that is outside its period or out of order.

Each failure names the fixture, the `claims` line number and the claimed text.
The check collects every failure across fixtures before failing, as the byte
comparison does, and it runs whether or not that comparison holds.

**The gate, and why it can go red apart from the test gate.** Failures are
reported by the gate AGENTS.md calls "the golden fixtures still produce their
expected results" (`fixtures` in `scripts/gates.sh`).

- The check lives in the `golden` target. That target is `test = false`, so
  `cargo test --workspace` does not select it, and `gates.sh` runs it by name.
- The check sits in a module that only `golden.rs` declares. It does not go in
  `tests/fixture/`, because every test target includes that module. Its cases
  would then also run under `cargo test`, and a false claim would turn the
  tests gate red as well, which is the thing `golden.rs`'s header rules out.
- Each red condition above gets a planted case in the `golden` target. The
  cases run over planted text, not over the committed fixtures.

**Proof of catch.** The existing `expected` perturbation stays. A second proof
goes under `fixtures`, and it works on a copy of the tree:

1. It finds the first committed `claims` in glob order, without naming it,
   for the reason `violate_fixtures` gives.
2. It makes that file's first claimed line false by appending characters, so
   the line is in no line of `expected`. `expected` is untouched.
3. The copy builds, passes the tests gate and passes the byte comparison, so
   the claims check is the only thing that can object.
4. The proof holds only if `fixtures` goes red and the output names the claims
   failure.

`prove()` runs one violation per gate, so the implementing task extends it to
run both under one name. Each violation runs in its own copy.

**The engine writes no claim, and the check derives none.**

- The check reads `claims` and `expected` and writes neither.
- It has no mode that records, blesses or regenerates either file.
- It does not run the stage for claims, and it does not build a claim from
  `expected`'s lines.
- Nothing under `crates/normalize/src/` changes.

**Bringing the `claims` files on main into this form.** When the implementing
task runs, it takes whatever `claims` files are on main, M4-46's and M4-48's
included if they have landed, and does the following. It changes no `| ` line
and no prose claim.

1. **Headers.** In each header, it replaces the sentences that say how a `| `
   line is read with one sentence naming this record. The form is then written
   in one place, here. This is how each departure listed in Context is handled:
   - `every-fact`'s header says its listing is consecutive, and cannot be.
     Replacing that sentence settles it. The check reads the listing as
     written, as a listing.
   - The negative-equity header's exception becomes this record's listing
     rule. The check reads that file's claims exactly as written.
   - The fiscal-year header is replaced the same way. Its body sentence "They
     are not consecutive there" is a true claim, and it stays.
2. **Anchors.** Wherever prose places a group under a period and no anchor
   precedes it, the task inserts that period's line as a group of its own,
   with a blank line on each side, directly before the first group the prose
   places:
   - in the fiscal-year file, fourteen anchors over twenty groups. The
     sections are "The fiscal-year change, by its dates alone" (two), "The
     transition period and its six values" (one, over five groups), "The tag
     moves" (six), "The move is not one-way" (two), "A value that changes and
     changes back", "A read zero beside a silence zero" and "Preferred
     dividends over 2025" (one each);
   - in `every-fact`'s "restated diluted loss per share", two anchors.

   Each inserted line is one the file's listing already claims, and the
   placement is what the prose already asserts. Nothing new is claimed; the
   existing placement becomes readable by the check.
3. **A red claim stops the task.** If any claim goes red after steps 1 and 2,
   the task stops and escalates. A false claim or a wrong `expected` is a
   decision above that task, and it edits neither one to get to green.

Checked on main at `2eeead1` with a throwaway reader outside the tree:

- As written, `every-fact` and the negative-equity file hold. The fiscal-year
  file goes red at lines 150 and 154. Those lines fall under the
  `from 2024-01-01 to 2024-12-31` period line given just above them in the same
  section.
- With step 2's sixteen anchors inserted, all three hold.
- With a silence line moved under a period where the concept is `Unknown`, two
  groups swapped under an anchor, or two listing lines swapped, the reader goes
  red.

**The implementing task is exclusive.** The golden-claims record says a gate
change is. This one also edits `scripts/gates.sh` and the `claims` files of
fixture directories that other tasks own.

- It owns `crates/normalize/tests/golden.rs`, the module only that file
  declares, `scripts/gates.sh` (granted through its `owns`), and
  `fixtures/normalize/*/claims`.
- It depends on every task in flight that owns a `fixtures/normalize/`
  directory, M4-46 and M4-48 today.

## Alternatives

**A structured claims form that the check parses**, with directives for
placement, counts, "only these lines" and absences. It would read more. It is
not taken, for three reasons:

- To count `silence` or `Unknown` values, the check would need its own grammar
  of the rendering: which line is a concept line, and which word is the way.
  That is a second reading of what normalize emits, kept in step with the
  first by hand. It is a small version of the oracle the golden-claims record
  turned down. A wrong pattern would look just like a claim that holds, or
  just like one that fails.
- The `from:` derivations are about the facts and the records, which no syntax
  puts in `expected`. The decider's re-derivation would not end.
- Moving 1,694 lines, plus M4-46's and M4-48's files, into a new syntax would
  rewrite every claim. That rewrite would happen exactly where a plausible
  wrong answer passes a glance, and the conversion would itself need
  re-deriving.

**The `| ` lines alone**, with each group consecutive somewhere in `expected`
and nothing more. This is M4-44's header taken literally. It reads less than
the files claim:

- A line that `expected` writes many times proves nothing about where it
  appears. The fiscal-year file places
  `  dividends_paid Value 0 silence conditional under registry` under
  `from 2025-01-01 to 2025-12-31`. `expected` writes that line 28 times, so the
  claim would pass even if moved under `at 2024-12-31`, an instant where the
  concept is `Unknown`.
- Taken literally, it turns all three listings red on the first day, or it
  needs the listing exception anyway.

Anchoring adds no syntax. A `| ` period line before a group is already how two
of the three files place a claim.

**Reading placement from the prose**, the "Under `P`:" lines. This would mean
the check parsing English, where rewording a sentence would silently change
what is checked.

## Consequences

**Easier.**

- Every claimed line, listing and placement is checked on every run. A claim a
  fixture task gets wrong goes red at its own line number.
- The decider re-derives only the prose claims.

**Harder.**

- Authors of `claims` must put a `| ` period line before any group they place,
  and must not leave a group inside an anchor's scope by accident.
- The harness gains a reader and its cases.
- The runner gains a second proof under one gate name.
- The implementing task lands under the `human-approved` label.

**Expensive to reverse.** The anchor convention, once fixtures carry it.

## Enforcement

This touches no anchor. The check is how the golden-claims record's
Enforcement section gets carried out. Its own proof is the claims proof in
`scripts/gates.sh`, plus the planted cases in the `golden` target. Nothing
checks the prose claims the check cannot read. That remains the decider's
re-derivation, and this record says so rather than implying it is covered.

## Decision review

By the decider, not the proposer.

- **Authority:** Structural, and within reach. It adds a check and a proof to
  an existing gate, weakens none, and changes no anchor, contract, schema or
  protected path. The runner edit waits for the implementing task and its
  human label. Flagged for later human review, as the tier requires.
- **Checked:** `golden-claims-and-baseline-re-recording.md`, which this carries
  out without editing; `golden.rs`'s rule that the fixtures gate goes red apart
  from the tests gate; `violate_fixtures`, `prove()` and `rendering::REGISTRY`
  as described. On main at `2eeead1` I used a reader of my own, written from
  this Decision, with nothing shared with the proposer's. The three `claims`
  hold 1,694 lines, 877 of them `| `, and every one is in `expected`. As
  written, fiscal-year is red at exactly lines 150 and 154, and the other two
  files hold. With the sixteen anchors of step 2, each a line the file's listing
  claims and once in `expected`, all three hold. A group moved to the wrong
  anchor goes red. The alternatives as argued: a structured form rebuilds the
  rendering's grammar in the check, and the bare `| ` lines pass a line
  written 28 times wherever it is moved.
- **Verdict and why:** accepted. The check reads only what a line of
  `expected` can prove, and it lists the rest as the decider's work. That keeps
  the check from claiming to verify more than it does. Anchoring adds no syntax.
  It gives the check the one position two of the three files already write,
  and it converts a placement that was already asserted rather than adding one.
- **What would have changed it:** a claim that the check reads differently
  after anchoring than the prose asserts, or an anchor period written more than
  once in `expected`. Either would have forced a red that does not follow
  from the claims. Neither occurs. Context's three repeated lines leave out a
  fourth, the transition period's `pretax_income` written twice, which does not
  bear on the decision.
