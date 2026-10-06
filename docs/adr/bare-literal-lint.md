# A new gate, `literals`, reads analyze's non-test source as tokens and goes red on any numeric literal but zero and one outside the three places, and on the preset constructor's name outside `settings/`

- **Status:** Accepted
- **Authority:** Structural
- **Proposed:** 2026-10-05, by `M5-09`
- **Decided:** 2026-10-05, by the decider
- **Touches:** anchor 5's enforcement clause, which this applies and does not
  edit. A new gate name, `literals`, in the expected gate set, which a later
  task that owns `scripts/gates.sh` builds. The name of the preset
  constructor, `Settings::preset`, which `what-analyze-takes-and-returns.md`
  requires and leaves unnamed. Naming it supersedes nothing in that record.
  The three places that record fixes stand as they are, and no fourth is
  added. No contract, schema or edge moves. No entry leaves
  `denied_packages`, `allowed_edges` or the expected gate set, and no name
  leaves what M5-04's check reads for. Nothing here is implemented.

## Context

Anchor 5 says nothing arbitrary enters the derivation, and that what holds
this is "a lint on the analyze crate" that "bans bare numeric literals
outside a small allowlist", running in CI. GOALS.md's sixth and seventh M5
criteria rest on it: every methodology constant is named, defined once and
cited, and every free parameter arrives as a setting.
`docs/adr/what-analyze-takes-and-returns.md` (M5-06, Accepted) gives the lint
its three places. It leaves the allowlist, and whether tests are read, to
this record. Under Enforcement it names the lint as not built. It also names
a gap the lint can close: nothing stops the derivation calling the preset
constructor.

Some things are already fixed:

- **The three places.** A bare numeric literal may stand in
  `crates/analyze/src/constants/`, in `crates/analyze/src/settings/`, and on
  the one `METHOD_VERSION` line of `crates/analyze/src/method.rs`. Every
  other source file in `crates/analyze/src/` is the derivation.
- **Analyze today** is `src/lib.rs`, one doc comment and no literal. Main
  passes whatever is decided here.
- **How gates are held.** Every gate is in `scripts/gates.sh`, which is
  protected. Each is proved by a violation planted in a scratch copy and run
  alone with `--gates-only <name>`. A gate with an `accept_<name>` has that
  copy run first, and it must stay green.
- **M5-04** builds a source-name check over the same crate. It reads every
  Rust file under `crates/analyze/`, tests and any build script included, for
  names that reach the filesystem, the environment, the clock, a process or
  a randomized hasher. Its first run (#273) was rejected and a re-run is
  pending, so it is not on main.
- **The arithmetic** analyze computes in, and how a figure is written, are
  the first metric task's.

## Decision

### The check

A new gate, `literals`. It reads the files below as Rust tokens and goes red
on each numeric literal that stands outside the three places and is not on
the allowlist. Each failure is one line, `<file>:<line>: <literal as
written>`, for example `crates/analyze/src/dcf.rs:14: 0.15`. Every offending
literal is listed, not only the first.

It also goes red, naming the file and line, on these:

- the preset constructor's name outside `settings/` (below)
- a `settings/` directory that no longer defines the constructor
- a file it cannot read: an unterminated string, comment or character
  literal, or a token outside its grammar. A file it cannot read is refused,
  not skipped.
- reading no file at all. An empty sweep reads exactly like a gate that is
  holding.

It reads text and runs without cargo, as `egress` does. The lexer lives in
`scripts/gates.sh` beside the check, in the same narrow style as the registry
reader there.

### What it reads

The rule is every file ending in `.rs` under `crates/analyze/`, found by
walking the directory on each run, minus two exclusions.

**The first exclusion** is anything under `crates/analyze/tests/`,
`crates/analyze/benches/` and `crates/analyze/examples/`. These are Cargo's
own directories for targets that link analyze as an outside crate and are
built only by `cargo test`, `cargo bench` or `cargo run --example`. Nothing
in them is compiled into the library a caller links, so no literal there can
reach a result. A test's expected values are numbers by nature. Reading them
would push them into `constants/`, where they would pass as methodology
with nothing to cite.

**The second exclusion** is an item that stands under exactly
`#[cfg(test)]`, from the attribute through the end of that item, which is its
closing brace or its semicolon. That code is compiled only for the crate's
own unit tests. Any other `cfg` is read, including `cfg(not(test))`,
`cfg(any(test, …))` and `cfg_attr(test, …)`. Only `cfg(test)` is certain
never to enter the library.

**What is read.** Every other file under `src/` is read, whichever module
declares it, and so is `crates/analyze/build.rs`. A build script runs on every
build of the library, and what it emits shapes the library. Analyze has none
today, and a build script that needs a number has no better reason for it
than the derivation would.

Analyze has no benchmark harness. The project's `benchmarks/` is outside
`crates/analyze/` and outside the rule.

Leaving tests out is cheap to reverse. Reading more later tightens the gate.

### Where a literal may stand

The three places, exactly as the accepted record fixes them:

- any file under `crates/analyze/src/constants/`
- any file under `crates/analyze/src/settings/`
- in `crates/analyze/src/method.rs`, the literal that initialises
  `METHOD_VERSION`, in the item `const METHOD_VERSION: <type> = <literal>;`
  (`pub` or not) written on one line. Every other literal in that file is
  read, including any other literal on that line.

### The allowlist

The allowlist is written once, in `scripts/gates.sh` beside the check, each
entry with its reason, as `denied_packages` is. It holds two values:

- **zero**, spelled `0` or `0.0`
- **one**, spelled `1` or `1.0`

Either may carry a type suffix, with or without one underscore before it.
So `1u32`, `1_usize` and `0.0_f64` pass.

**Why these two.** They belong to the arithmetic and to no method. Zero is
the empty sum, the start of a count, and the zero divisor that the accepted
record's `declined` reason names. One is the empty product, a count's step,
the whole that a fraction is part of, and the index of the second term. No
source fixes them. A file for them in `constants/` would carry a citation
that cites nothing, which is the false citation anchor 5 exists to keep out.

**Why it stops there.** Every other number is a choice that a method or a
user makes. Some examples:

- The `2` in an average of opening and closing balances is the method's
  definition. It is a constant in `constants/` under that method's citation.
- So is a `365` in days outstanding.
- So is Piotroski's count of nine signals.

A number that is neither a method's nor the arithmetic's is an open question
(below), and not an allowlist entry.

**Other spellings are red.** This covers `0x1`, `0b0`, `1e0`, `1.`, `1.00`,
`00` and `0_0`. Nothing needs them, and one spelling per value keeps the
match exact.

**The list starts small on purpose.** Once the gate is on main, adding an
entry loosens it. The ADR tiers make that a weakening, which is
Constitutional. So the list starts at what the derivation cannot be written
without, and a task that needs a third value escalates.

### What counts as a bare numeric literal

The check reads tokens the way rustc's lexer splits them. It does not read
raw text. Each form is decided below.

- **Integer and float literals: in.** This covers every base, an exponent, a
  type suffix (`15u32`, `0.15f64`) and underscores (`1_000`). A suffix types
  the value and an underscore groups its digits, and neither sources it. A
  literal is compared with the allowlist after its suffix, and the
  underscore before it, are removed.
- **A literal behind a unary minus** is read without the minus. Rust has no
  negative literal: `-0.15` is negation applied to `0.15`, so every negative
  holds a positive the check already sees. The only negatives that pass are
  minus zero and minus one, which carry nothing zero and one do not. The
  check therefore never has to tell a unary minus from a binary one.
- **A tuple index (`.0`, `t.0.1`): out.** It names a field of a type, not a
  quantity, and the grammar holds it apart from a literal. It is a digit run
  straight after a single field-access dot. A `..` or `..=` is a range, and
  the number after it is read.
- **An array length or repeat count (`[T; 4]`, `[x; 4]`): in.** A fixed
  length is a count the derivation chose. Piotroski's signals are
  `[bool; PIOTROSKI_SIGNALS]`, with the constant in `constants/`.
- **A slice index or range (`x[2]`, `&x[3..]`, `2..n`): in.** An index
  chooses which period or term is read, and the accepted record puts that
  under the method version. `[0]` and `[1..]` pass as zero and one.
- **A literal inside a `const` or `static` item outside the three places:
  in.** A name is not a source. A `const` in the derivation is an uncited
  value with a name on it. The accepted record puts named constants in
  `constants/` so that the citation sits beside them.
- **A literal inside an attribute (`#[repr(align(8))]`): in.** An attribute
  macro can turn a number into a value in the item it stands on. No
  attribute analyze needs carries a number, so reading them costs nothing.
- **A literal inside any string, character or byte literal: out.** It is
  text, not a number the arithmetic reads. What that hides is listed under
  what the check cannot see.
- **A literal inside a comment, doc comments included: out.** It is prose. A
  citation's year and page are numbers and must stay writable. Doc tests are
  built only under `cargo test`.
- **A literal inside a macro's arguments, or inside a `macro_rules!` body:
  in.** These are tokens that become code. `vec![0.15; n]` puts `0.15` in
  the derivation as surely as a bare `0.15` does.
- **Literals in match patterns, enum discriminants and const generic
  arguments (`Foo<3>`): in.** Each is a literal in code like any other.
- **Digits inside an identifier (`f64`, `u32`, `cagr_5y`): not a literal.**
  These are names.

### The preset constructor

This closes the accepted record's gap. That record says preset values are
reached through one named constructor in `settings/`, which the caller calls
and the derivation never does, and it leaves the name open. This record names
it: an associated function, `Settings::preset`. It is an associated function
rather than a free one because `use` cannot rename an associated function, so
the derivation can reach it only by writing `preset`.

The check reads the identifier `preset`, as a whole token outside strings and
comments, in every file it reads outside `crates/analyze/src/settings/`. On a
hit it goes red with `<file>:<line>: preset`. It reads the identifier and not
the call. So a local variable called `preset` in the derivation is red too,
and the derivation names that thing something else.

**How it knows the name.** The name is written once, in `scripts/gates.sh`
beside the check. A rename must not leave the ban reading a dead name, so the
check also goes red when `crates/analyze/src/settings/` holds a `.rs` file
and none of them holds `fn preset`. Before `settings/` exists, there is no
constructor to call.

**Tests are not read**, so a test may call `Settings::preset`. A test stands
where the caller stands.

This is a name, but it is not one of M5-04's. It reaches nothing outside
analyze, and M5-04 reads for nothing like it. The check takes no name from
M5-04's lists and gives it none.

### Beside M5-04's check

Each check walks `crates/analyze/` on its own. Neither reads a list the other
keeps. The reasons:

- **Different files.** M5-04 reads tests and everything else. This check
  reads the library and the build script only.
- **Different readings.** M5-04 matches names in text. This check lexes
  tokens and passes over strings, comments and `cfg(test)` items. A shared
  walk would hand each check the other's reading, and one bug in it would
  blind both at once.
- **Proofs.** The runner proves a gate by running it alone and checking that
  the gate it aimed at is the one that went red. Two gates fed by one walk
  could each pass their proofs while the walk was wrong.
- **Cost.** Analyze is one crate. A second walk is one `find` over it, which
  does not register beside the build.

### The gate

`literals` is a new name. It enters `expected_gates` after `purity` and after
M5-04's check, with the gates that read the tree for a shape an anchor fixes.

It is not an extension of `purity`. Purity reads the resolved dependency
tree, which holds no literal, and its proof plants a dependency. Nor is it an
extension of M5-04's check, which is not on main and reads a different set of
files.

Every existing gate and proof runs unchanged. No entry leaves
`denied_packages`, `allowed_edges` or the expected gate set.

**The proof-of-catch the implementing task brings**, run by the runner's
existing `prove`:

- **`accept_literals`**, a copy that stays green. It carries:
  - a literal in each of the three places: a file under `constants/` holding
    `pub const PLANTED: f64 = 0.15;`; a file under `settings/` holding a
    literal and `fn preset`; and `method.rs` holding
    `pub const METHOD_VERSION: u32 = 2;`. The `2` shows that the place lets
    the version through, not the allowlist.
  - a derivation file holding allowlisted literals, `1` and `0.0`.
  - one of each form decided out: a tuple index, a number in a string, a
    number in a comment, a `#[cfg(test)]` module holding a literal, and a
    file under `tests/` holding a literal and calling `Settings::preset`.
- **`violate_literals`**: `0.15` planted in a derivation file outside the
  three places. It goes red, and `must_name` pins the planted file, line and
  literal.
- **`violate_literals_preset`**: a call to `Settings::preset()` planted in a
  derivation file. It goes red naming `preset`.
- **`violate_literals_preset_gone`**: a `settings/` file with no
  `fn preset`. It goes red.
- **`violate_literals_unreadable`**: an unterminated block comment in a
  derivation file. It goes red naming the file.

### Open questions

A literal the derivation might need that fits none of the three places and
has no allowlist reason. Each is named here and not admitted. The task that
first needs one escalates.

- **A scale between units**, such as `100` for a percentage. It is no
  method's and not the arithmetic's. Stating a ratio as a fraction avoids
  it, because the accepted record states the unit once, in the metric's
  catalogue entry. So none may ever be needed.
- **The precision a figure is written to**, such as a count of decimal
  places or a rounding step. How a figure is written is the first metric
  task's, under the method version. If writing it needs a literal, that task
  says where the literal belongs.

### What the check cannot see

As the egress gate does, this says what passes the check.

- **A number spelled as a string and parsed**, such as `"0.15".parse()`. The
  same goes for the width or precision in a format string, which changes how
  a figure is written.
- **A constant in an allowed place whose citation is false or missing.** The
  check reads no citation. Whether an entry in `constants/` cites anything,
  and whether the citation is true, is for review. The same holds for a
  stated reason in `settings/`.
- **A value built by arithmetic over allowed names**, such as `1 + 1` or
  `1.0 / (1.0 + 1.0)`. So also a constant from one method used in another,
  and the numbers std names for itself, such as `f64::EPSILON`, `u8::MAX`,
  `std::f64::consts::E` or a string's `len()`.
- **Code in an allowed place that is not what the place is for.** A helper in
  `constants/` that does arithmetic with literals passes, because the place
  is allowed whole.
- **Source reaching analyze from outside what it reads.** This covers a
  `#[path]` to a file elsewhere, a manifest `[lib] path` pointing into
  `tests/`, and a macro or constant from another crate, such as
  `vfi-contracts` once that edge lands.
- **A second route to the presets.** This could be a `Default` impl on
  `Settings`, or a second function in `settings/` that returns the preset
  values under another name. The check bans one name.
- **A form the lexer misreads.** What it cannot read goes red. A form it
  reads wrongly, such as a raw string whose delimiter it miscounts, would
  hide what is inside. The proofs pin each decided form, not every spelling.

## Alternatives

- **Read text, as `egress` does with `grep`.** Rejected: text cannot tell
  `0.15` from the `64` in `f64`, from a year in a citation, or from a tuple
  index.
- **A checker crate built on `syn`, which is already in the lockfile.** This
  would parse exactly. Rejected for two reasons:
  - It adds a workspace member, which is a layout change and a manifest and
    lockfile edit.
  - It puts the lint's logic in an unprotected crate. Any task could then
    narrow it without the `human-approved` label that `scripts/gates.sh`
    requires. Keeping the logic in the protected runner is what makes
    weakening it cost a signature.
- **A Clippy or dylint lint.** Clippy has no lint that bans numeric literals
  in general. A dylint needs a pinned nightly toolchain and a driver the
  project does not have, and its logic would also sit outside the protected
  runner.
- **Read tests too.** Rejected: every expected value would need a home.
  `constants/` would fill with uncited test numbers, or the allowlist would
  grow past small, and a test's numbers never reach a result.
- **A wider allowlist of the derivation's common numbers**, such as `2`,
  `100` and `365`. Rejected: each one either belongs to a method or is open.
  Admitting them as arithmetic would let a method's number in uncited.
- **Leave the preset gap open by name.** Rejected: the ban costs one name.
  Naming the constructor before any metric calls anything is cheaper than
  naming it after.
- **One walk shared with M5-04.** Rejected, for the reasons under "Beside
  M5-04's check".

## Consequences

**Easier:**

- Anchor 5's enforcement clause has a mechanism, not only a reviewer.
- Every metric task knows where each number it writes must go.
- Tests write numbers freely.
- Each decided form is pinned by a proof, so a later change to the lexer that
  reads one differently goes red.

**Harder:**

- A method's small numbers, such as the `2` in an average, each need a named
  and cited constant.
- The derivation keeps no `const` of its own. Even `const HALF: f64 = 0.5`
  belongs in `constants/` under a citation.
- The runner gains a second reading of Rust, and it is only as right as its
  proofs.
- `preset` is unusable as a name anywhere in the derivation.

**Expensive to reverse:**

- Once on main, every widening is a weakening of a gate, which is
  Constitutional. That includes an allowlist entry, a form moved out, and a
  directory left unread. Tightening, such as reading tests later, is
  Structural.
- The name `Settings::preset`, once callers outside analyze use it.

## Enforcement

This applies anchor 5 and the accepted record's three places. Each item is
held by the check below, or left visibly open.

- **"Bans bare numeric literals outside a small allowlist", in CI.**
  `literals`, once built, with the proofs above. Until then the accepted
  record's gap stands.
- **The derivation never calls the preset constructor.** `literals`, through
  the name `Settings::preset` and the check that `settings/` still defines
  it.
  - **Gap:** a second route to the preset values.
- **Every constant cites its source.**
  - **Gap:** nothing checks that a citation is present, or that it is true.
- **Nothing arbitrary enters the derivation.**
  - **Gap:** arithmetic over allowed names, a number parsed from a string,
    and the constants std names for itself.
  - **Gap:** source reaching analyze from outside what the check reads,
    through `#[path]`, the manifest, or another crate.

## Decision review

By the decider, not the proposer.

- **Authority:** Structural, and within reach: a new gate name entering the
  expected gate set is the tier's own example. It edits no anchor, no
  protected path and no gate; the runner change lands in a later task under
  the `human-approved` label, where a human sees it. Flagged for later human
  review, as the tier requires.
- **Checked:**
  - Anchor 5's enforcement clause, which this applies and does not edit: a
    lint on the analyze crate, a small allowlist, run in CI.
  - `what-analyze-takes-and-returns.md`, accepted. The three places are
    copied as that record writes them. The allowlist and whether tests are
    read are the two things it left to this record by name. The constructor
    it requires and leaves unnamed is named here without an edit to it.
  - M5-04's task against "Beside M5-04's check". M5-04 reads every Rust file
    under the crate, tests and build script included, for names; this reads
    the library and the build script, as tokens. Different files, different
    readings, no list passed either way.
  - The runner as main holds it. `expected_gates`, `prove`, the `accept_`
    copies, `must_name` and `--gates-only` exist as the proof set assumes,
    and `egress` reads text without cargo as the record says of itself.
  - The template's tiers. A later allowlist entry, a form moved out or a
    directory left unread is a weakening and so Constitutional; reading
    tests later is a tightening. The record says both.
  - The seven alternatives as argued.
- **Verdict and why:** accepted. What carries it is the allowlist's reason:
  zero and one are the arithmetic's and no method's, and a file for them in
  `constants/` would carry a citation that cites nothing, which is the exact
  defect anchor 5 names. Every other form is decided in with a reason, the
  two numbers that fit nowhere are left open by name rather than admitted,
  the blind spots are listed rather than assumed, and the proof set pins each
  decided form so a lexer that drifts goes red. The cost it owns, a second
  reading of Rust inside the runner, is the price of keeping the lint's logic
  where weakening it costs a signature.
- **What would have changed it:** a value the derivation cannot be written
  without that is neither zero nor one, which would make the allowlist a
  method's list under another name. So would a fourth place, or a reading of
  the three that differs from the accepted record's. A checker crate the
  protected-path list covered would have reopened the `syn` alternative, but
  widening that list is Constitutional and not this record's to decide.
