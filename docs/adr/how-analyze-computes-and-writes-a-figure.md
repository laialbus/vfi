# Analyze reads each amount and close as the exact number its characters state, computes in exact rationals, and writes a figure once, rounded to thirty-four significant digits, as a plain decimal

- **Status:** Proposed
- **Authority:** Structural
- **Proposed:** 2026-10-07, by `M5-16`
- **Decided:** —
- **Touches:** the two things `what-analyze-takes-and-returns.md` hands "to
  the first metric task, under the method version": the arithmetic analyze
  computes in, and how a figure is written. `analyze-store` v1 leaves the
  second to "analyze, under the method version", so this record fixes the
  characters of its `figure` and of a numeric setting's `value`. No byte of
  that contract changes. It answers the two open questions of
  `bare-literal-lint.md` by name. It puts two named constants under
  `crates/analyze/src/constants/`, which is one of that record's three
  places, and adds no fourth place and no allowlist value. It applies the
  accepted `declined` reason to an input whose characters state no number,
  and adds no reason. No contract's bytes, no schema, no edge, no gate and no
  dependency move. Nothing here is implemented.

## Context

GOALS.md's fourth, fifth, sixth, seventh and ninth M5 criteria each need a
figure computed and written. Three steps stand between the history and the
hand-over: a reading, an arithmetic and a writing. No record has decided them.

Some things are already fixed:

- **What arrives is characters.** A `Value`'s amount crosses
  `canonical-concepts` v2 as the characters it was published as
  (`Resolution::Value { amount: Box<str> }`), and `fetch-normalize` v2 calls
  those characters "the decimal literal the document publishes, unparsed".
  `fetch-analyze` v1 carries a close as "the decimal literal the source
  published, unparsed", and a split factor "as the characters the source
  published, unparsed". Each crosses as characters because a binary float is
  a lossy reading of a published decimal.
- **What leaves is characters.** `analyze-store` v1 carries a `value`
  outcome's `figure`, and each setting's `value`, "as the characters analyze
  writes". Its property "nothing is parsed" keeps store from reading them on
  the way in.
- **Nothing about the data is an error.** The surface admits no error, and
  its six reasons are closed. The accepted record gives the entry point one
  error, which is a crossing whose CIK is not the history's. A zero divisor
  is `declined`.
- **The literal lint.** `bare-literal-lint.md` (accepted) allows zero and
  one, and any literal in its three places. It leaves two questions to this
  record: a scale between units, and the precision a figure is written to.
  Its Decision review calls a constant that cites nothing the exact defect
  anchor 5 names.
- **The method version** changes with "the arithmetic, or how a figure is
  written". `tasks/M5-15.md` starts it at 1 and writes no figure.
- **Normalize already reads the same grammar.** It reads an optional minus,
  digits, and at most one point with digits on both sides, in
  `crates/normalize/src/settling/figure.rs`. It adds and subtracts exactly,
  and treats anything else as no figure. Every amount normalize sets passes
  through one of three routes:
  - a read value is read and rendered through that reader
  - an asserted value is refused by the registry outside the grammar
  - a silence value is the literal zero

  Analyze cannot reach that code, because `vfi-analyze>vfi-normalize` points
  backward.
- **What M5's metrics need.** Sums, differences, products and quotients.
  Integer powers, which discounting uses. Comparison, which the screens use.
  An n-th root, which a compound growth rate over n years needs, and a square
  root where a method takes one.

## Decision

### The number

Analyze computes in one number type, an exact rational. It is a sign and a
numerator and a denominator, both integers of any size, held as digits in
`RADIX`, one digit per place. No binary float (`f32`, `f64`) appears in the
derivation: not as a value, an intermediate or a conversion. There is no
bound on how many digits a number holds. A cap would be a number with no
source, as normalize's reader says of its own.

The type and its operations are one module of the derivation. Every metric
reads, computes and writes through it. No metric reads characters itself,
does arithmetic outside the type, or formats a number.

### The reading

**The grammar** is exactly normalize's: an optional `-`, then one or more
ASCII digits, then optionally a `.` followed by one or more ASCII digits.
There is no `+`, no exponent, no separator, no space, and no leading or
trailing point.

**What the characters are read as.** The exact number they state. A leading
or trailing zero states nothing more: `007` is seven, `6.10` is `6.1` and
`-0` is zero. Reading never rounds.

**What is read.** A history `Value`'s amount, and a close. A split factor is
read by nothing this record decides. Under the share-basis rule any split
dated inside the span declines the metric, so a factor's value never enters
a derivation. It crosses into `split_within` as it was published. A rule
that computes across a split brings the record that hands analyze the basis
date, and that record reads the factor. Dates are not numbers here. The
share-basis rule and each metric read them.

**Characters outside the grammar** state no number. The metric that reads
them is `declined` under one condition, `input_not_a_decimal`, which the
arithmetic defines once beside the reader. There is one `declined` per such
input, in the metric's own input order. The reading itself returns either a
number or nothing, and never fails.

Why `declined`:

- Its witness is "every input was present, and the metric's definition
  states it is undefined at those inputs". The input is present, as a
  `Value` or as a price.
- A metric's definition is a formula over numbers, so it is undefined where
  an input states none.
- Every metric's catalogue entry names the condition (below), so its
  definition states it.

Why not the other reasons, or an error:

- `input_unknown` and `input_not_applicable` are built only from their own
  states, and a `Value` is neither.
- `no_price` is built from no price, and here a price is present.
- An error would be a second error, which the accepted record does not
  admit.

So this needs no seventh reason and changes no frozen byte. From normalize
as main holds it, this cannot be reached. From a close it can, because the
provider is not written yet. Either way the outcome is an absence, and never
a reading invented for the characters.

### The operations

| Operation | Result |
| :--- | :--- |
| negation, addition, subtraction, multiplication | Always exact. |
| division | Exact whenever the divisor is not zero. A zero divisor is `declined`, under the condition the metric's catalogue entry names, as the accepted record says. |
| integer power `x^k` | Exact. `x^0` is one, zero included, as the empty product. A negative `k` is one divided by `x^|k|`, so zero to a negative power is a zero divisor and `declined`. |
| comparison, equality, sign, absolute value, minimum, maximum | Exact and total. Equality is by value, so one half equals two quarters. |
| `k`-th root, `k` a positive integer, of a radicand not below zero | The real root that is not negative, correctly rounded to `SIGNIFICANT_DIGITS` significant digits, ties away from zero, where the root is taken. From then on it is an exact rational like any other. A root that has no more than `SIGNIFICANT_DIGITS` significant digits comes back exact: the root of `6.25` is `2.5`. |

The root has two exclusions, and each is the metric's own `declined`
condition:

- **A negative radicand.** No root of a negative is taken, odd roots
  included, so parity is never needed.
- **An index of zero.**

**Nothing else is an operation.** That rules out a logarithm, an
exponential, a power with an exponent that is not an integer, and
trigonometry. A metric that needs one escalates. Adding an operation changes
the arithmetic, which is a superseding record and a method version bump.

A power's exponent and a root's index come from a constant in `constants/`,
from a setting, or from a count the metric reads off its inputs, such as a
number of years. They never come from a literal in the derivation.

**Rounding happens once.** Only a root rounds inside the derivation. A
quotient that does not terminate is held exactly. Every figure is the exact
result of its formula, rounded once, when it is written. A metric that
writes a figure and reads it back to continue rounds twice, and that is a
defect.

### How a figure is written

One function writes every figure and every numeric setting's value. The rule
is:

1. **Precision.** The number is rounded to `SIGNIFICANT_DIGITS` significant
   digits, to the nearest, with a tie going away from zero. This is
   `roundTiesToAway` of IEEE 754-2008 §4.3.1. A number with no more
   significant digits than that is written exactly.
2. **Sign.** `-` stands before a negative number and nothing stands before
   any other. There is never a `+` and never a `-0`. Rounding is symmetric:
   a negative number rounds to the negation of its magnitude's rounding.
3. **Digits.** ASCII `0` to `9`, in `RADIX`.
4. **Point.** A `.` appears only when a digit after the point remains once
   trailing zeros are removed.
5. **No exponent.** A figure is always positional, however small or large.
6. **No separator.** No thousands separator and no space.
7. **Leading zeros.** A number whose magnitude is below one has exactly one
   `0` before the point. Any other number has no leading zero. Zero is
   written `0`.
8. **Trailing zeros.** None after the point. A trailing zero in the whole
   part is place value and stays.

Every written figure is in the reading grammar, and each rounded number has
one spelling under the rule. So:

- **equal numbers give equal characters**: one half, two quarters and the
  characters `0.50` are all written `0.5`
- **the characters read back give the number written**: reading a figure
  gives the rounded number exactly
- **writing what was read is a no-op**: reading a figure and writing it
  again gives the same characters

This differs on purpose from normalize, which keeps a published amount's
scale because the filing stated it. A figure analyze writes is canonical.

**A numeric setting's value** is written by the same rule. A numeric setting
reaches the `Settings` constructor as characters in the reading grammar or
as the number type, and never as a binary float. The written value must be
the value used, so every numeric setting's domain admits only values of at
most `SIGNIFICANT_DIGITS` significant digits. The constructor refuses any
other value as the caller's error, raised before analyze runs, as the
accepted record says of every refused value. A setting whose domain is not
numbers writes no figure. Its spelling is its own task's, stated beside it
in `settings/`.

### The two open questions of `bare-literal-lint.md`

**A scale between units: no literal is needed.** Analyze writes each figure
in the unit its catalogue entry states, and never writes a figure scaled to
another unit:

- a ratio is a fraction of one, `0.25` and never `25`
- a currency figure is at a scale of one, as v2 states for amounts
- a per-share figure is currency per one share
- a count is a count
- a setting that is a rate is a fraction of one

A days count or an annualising factor, such as `365` or four quarters,
belongs to a method. It stands in `constants/` under that method's citation,
as the lint record already says of `365`. Whether the shell shows a fraction
as a percentage belongs to the shell boundary, where the accepted record
leaves the catalogues' reach. Anchor 1 keeps that arithmetic out of the
shell. If a percentage is wanted, that record decides it, and analyze writes
no second, scaled figure in the meantime.

**The precision a figure is written to: a literal is needed, and it stands
in `constants/`.** No construction avoids it. A quotient whose decimal does
not terminate must be cut somewhere. Written exactly as a repeating decimal,
its period can run to nearly as many digits as its denominator's value, and
a share count is an eleven-digit denominator.

So `SIGNIFICANT_DIGITS`, 34, is defined in
`crates/analyze/src/constants/figure.rs`. Its source is the precision `p` of
the decimal128 format: IEEE Computer Society, *IEEE Standard for
Floating-Point Arithmetic*, IEEE Std 754-2008, 2008, §3.6, decimal
interchange format parameters. The constant's documentation gives the table
and page as the implementing task reads them off the standard.

The reason stated beside it is why this format and not a narrower one. A
figure is what store keeps, and what ranking composes late, over stored
results. It is never recomputed from the exact value. So the figure should
carry more digits than any input states. decimal128 is the widest basic
decimal format the standard defines, so choosing it needs no argument about
which narrower width is enough. At 34 digits, no amount a filing states at a
scale of one, and no sum or difference of such amounts, is rounded when it
is written. Only a quotient that does not terminate, and a root, are
rounded.

### Every number the arithmetic itself needs

| Number | Where it stands |
| :--- | :--- |
| zero and one | The allowlist: the empty sum, the start of a count, a count's step, and the denominator of a number read with no point. |
| the radix, ten | `RADIX` in `crates/analyze/src/constants/figure.rs`. Its source: Bray, T., ed., *The JavaScript Object Notation (JSON) Data Interchange Format*, RFC 8259, IETF, 2017, §6, "A number is represented in base 10 using decimal digits". That is the grammar EDGAR's amounts and the source's closes are published in, and the figure is written back in the same grammar. Every digit is read with `char::to_digit(RADIX)` and written with `char::from_digit(_, RADIX)`, never by arithmetic on `b'0'`. |
| the precision | `SIGNIFICANT_DIGITS`, in the same file, as above. |
| a half, for ties | None. A remainder `r` against a divisor `d` rounds away from zero when `r` is at least `d - r`, and toward zero otherwise. |
| two, for parity | None. A tie goes away from zero, and no root of a negative is taken. |
| a limb width | None. There is one digit per place in `RADIX`. A wider limb would need a width this record does not admit. A task that wants one for speed brings its own record. |
| a power's exponent, a root's index | The metric's constant, setting or count, as above. |

The characters `-` and `.` are character literals. `literals` does not read
character literals, and the grammar above states both.

The rounding direction is a name and not a number, so it needs no constant.
The writing function's documentation cites IEEE 754-2008 §4.3.1.

A number this table does not hold is one the implementing task escalates
on. Nothing decided here puts a literal that `literals` reads into the
derivation.

### No dependency

`vfi-analyze` takes no dependency for the arithmetic, which is written in
the crate. Nothing enters analyze's resolved tree, so `purity` reads what it
reads today. Nothing enters its source beyond the one module and the one
constants file, so `ambient` and `literals` read them like any other file.

### The method version

The reading, the operations, the rule for a root, the writing, `RADIX`'s use
and `SIGNIFICANT_DIGITS` are all part of the method `METHOD_VERSION` names.
That is where the accepted record places "the arithmetic, or how a figure is
written". A later change to any of them is a version bump and a superseding
record, never an edit to this one.

They land at version 1. M5-15 starts the version at 1 and writes no figure,
so no stored result predates them and nothing bumps.

### What a metric's catalogue entry says about its figure

Beyond the unit the accepted record already puts there:

- **No precision of its own.** Every figure is written to one precision. The
  catalogue states that precision once, generated from `SIGNIFICANT_DIGITS`,
  and no entry repeats it.
- **The unit as a scale.** An entry whose figure is a ratio states its unit
  as a fraction of one, so nothing presenting it can read it as a
  percentage.
- **The unreadable-input condition.** The entry of every metric that reads
  an amount or a close lists `input_not_a_decimal` among its `declined`
  conditions. The list is generated from the one definition beside the
  reader, and not written per metric.

A metric that takes a root lists a negative radicand and a zero index among
its own conditions, as it lists a zero divisor.

### The proof the implementing task brings

These tests pin the written form. They stand in `tests/` or under
`#[cfg(test)]`, which `literals` does not read, so their numbers need no
home. At the least:

- **Read back.** Over figures that include zero, a negative, a magnitude
  below one, a whole number with trailing zeros, and a number of more than
  34 significant digits, reading what was written gives the number written.
  Writing what was read gives back every canonical spelling unchanged.
- **A quotient that does not terminate.**
  - One divided by three is `0.` followed by thirty-four `3`s.
  - Two divided by three is `0.` followed by thirty-three `6`s and a `7`.
  - Two divided by six is written the same as one divided by three.
- **A negative figure.**
  - Minus two divided by three is `-0.` followed by thirty-three `6`s and a
    `7`.
  - `-0` is read and written as `0`.
  - `-0.0053` is read and written back unchanged.
- **Currency too large for a binary float.** `9007199254740993` is two to
  the fifty-third plus one, which no binary64 holds. It is read and written
  back unchanged, and one added to it is written `9007199254740994`.
- **A history amount read as the characters published.**
  - A `Value`'s amount goes through the reading. `394328000000`, `-30810`
    and `0.0004` come back as they are, `6.10` is written `6.1`, and `007`
    is written `7`.
  - `1e9`, `1,000`, `+1`, `.5`, `1.` and the empty string are each read as
    no number.
- **A tie.** A number of thirty-five significant digits whose last is `5`
  rounds away from zero, both positive and negative.
- **A root.** The square root of two is `1.414213562373095048801688724209698`.
  The square root of `6.25` is `2.5`.
- **No exponent.** One divided by ten to the fortieth power, and ten to the
  fortieth power, are both written positionally, with no exponent and no
  separator.
- **A setting.** A numeric setting of thirty-five significant digits is
  refused. One of thirty-four is written back as the value used.

The first metric that reads an amount adds two tests. Characters outside the
grammar give `declined` with `input_not_a_decimal`, and a zero divisor gives
its own `declined`.

### What no check can see

- **The precision a format string carries**, such as `format!("{:.2}", x)`.
  `literals` does not read a string's contents. The same holds for a figure
  written by `format!` or `to_string` on any type other than the one writer.
  Only review sees these.
- **A binary float in the derivation.** No gate reads for `f32` or `f64`.
  Checking for them would be a new gate, and a record of its own.
- **A number parsed from a string in the derivation**, such as
  `"0.15".parse()`. This is the lint record's own gap.
- **A second rounding**, by a metric that writes a figure and reads it back
  mid-derivation.
- **An operation outside the list built from operations on it**, such as a
  logarithm by its series. It passes every check.
- **Whether the two citations are true.** These are decimal128's precision
  in IEEE 754-2008, and RFC 8259's radix.
- **Whether analyze's reader and normalize's writer agree on the grammar.**
  Each states the grammar, and nothing compares them. Analyze's tests cannot
  read normalize's fixtures: `ambient` refuses a file read, and analyze has
  no edge to normalize. A disagreement lands as `declined` and never as a
  wrong number. But a metric that is absent across the whole corpus passes
  every gate.
- **Speed.** Digit-by-digit arithmetic over growing denominators is slower
  than a float. The benchmark sees only what it measures.

### What stands

Nothing proved gets narrower. No entry leaves `denied_packages`,
`allowed_edges`, `ambient_names` or the expected gate set. No place, form or
allowlist entry of `bare-literal-lint.md` moves.

These stay exactly as accepted:

- the accepted analyze record's outcome, its six closed reasons, its three
  places and its catalogue
- the frozen bytes of `analyze-store` v1, `fetch-analyze` v1 and
  `canonical-concepts` v2, which still carry characters unparsed
- the analyze skeleton as `tasks/M5-15.md` states it

No question here needed one of those to change.

**Why Structural.** The record fixes the characters of two fields that cross
a contract, which store and M6 will read. It also fixes a rule every metric
shares. It needs nothing Constitutional: no fourth place, no allowlist
value, and no gate weakened.

## Alternatives

- **Binary floating point (`f64`).** Rejected:
  - It cannot hold `0.1`, or an amount past two to the fifty-third, exactly.
    It rounds at every operation, so a published figure is quietly a
    different number.
  - A root through `powf` is not correctly rounded, and can differ by
    platform, which puts anchor 4's same-output-every-time at risk.
  - The contracts carry characters precisely so that no float reads them.
- **A fixed-point integer at a chosen scale**, such as an `i128` counting
  units of ten to some negative power. Rejected:
  - The scale is a literal no source fixes.
  - Every division rounds, so the error compounds through a discount chain.
  - A product of two currency amounts at that scale overflows. An overflow
    is either an absence no closed reason holds, or a wrong number.
- **An exact rational written as a fraction**, such as `1/3`. This needs no
  precision and loses nothing. Rejected:
  - The shell would have to divide to show it, and anchor 1 keeps
    arithmetic out of the shell.
  - Ranking over stored figures would compare fractions outside analyze.
  - A root still cannot be written exactly.
- **A precision applied through a format string.** Rejected:
  - `literals` cannot see it.
  - It rounds a binary float, not the exact value, so it rounds twice.
  - It counts decimal places, not significant digits.
- **A fixed count of decimal places.** Rejected. Figures run from a currency
  amount near ten to the thirteenth down to a yield near ten to the minus
  fourth. A count small enough not to pad the first rounds the second to
  nothing.
- **decimal64's sixteen digits.** It would hold every amount a filing
  states. Rejected only because it is narrower. The figure is what ranking
  composes over, so the width should need no argument, and the widest basic
  format needs none.
- **Ties to even.** It removes a bias that accumulates when rounded values
  are summed. Rejected:
  - A figure is rounded once and is never summed after, so no bias
    accumulates.
  - A tie happens only to an exact figure of more than 34 significant
    digits ending in five.
  - Testing for an even digit needs the number two in the derivation, with
    nothing to cite for it.
- **The precision as a setting.** Rejected:
  - The accepted record puts how a figure is written under the method
    version. A setting would move part of it into the premises, so two
    results at one method version could be written to different precisions.
    That is a different reading of an accepted record, and not this record's
    to make.
  - No metric's meaning needs it, which is what makes a value a setting.
- **A dependency.** Two candidates were considered, and neither is taken:
  - `rust_decimal` holds 28 decimal digits in 96 bits, rounds a division at
    its own scale, and fails on overflow.
  - `num-bigint` with `num-rational` would hold the exact rational.

  Each adds source that no source check reads, because `ambient` reads only
  `crates/analyze/`. The rounding and the written form are this record's in
  any case, so a library would bring a second rounding policy to keep out.
  The operations are a few hundred lines, and normalize already writes two
  of them.
- **Reading an exponent form**, such as `1.5E9`. It would be exact.
  Rejected: no other reader in the engine admits one, and one grammar for
  amounts and closes keeps the proof one proof. A close published that way
  makes its metrics `declined`, which is an absence and never a wrong
  number. The provider's task, reading a real response, will see which form
  the source writes.
- **Characters outside the grammar as an error, or as a seventh reason.**
  Rejected:
  - An error is a second error the accepted record does not admit, and it
    makes a data defect fail the run.
  - A seventh reason is an `analyze-store` v2.

  `declined` already holds the case, because the definition the catalogue
  states is undefined there.
- **Sharing normalize's reader through `vfi-contracts`.** Rejected:
  - That crate holds the surfaces, not arithmetic.
  - Moving the reader there gives a component a second consumer with no
    interface, which needs a record of its own.
  - Normalize's reader only adds, while analyze needs division and roots.

## Consequences

**Easier:**

- Every metric computes in one type, under one rounding and one written
  form, and none of them chooses.
- A figure is exact until it is written.
- A replay gives the same characters on any machine, because the arithmetic
  is integers.
- The lint's two open questions are closed, and the derivation needs no
  number beyond zero, one and two cited constants.

**Harder:**

- Arithmetic digit by digit is slower than a float. Denominators grow along
  a compounding chain, so speed rests on how the implementing task reduces
  them, and the benchmark holds it.
- A metric that needs a logarithm, an exponential or a real power waits for
  a superseding record.
- Figures are long. The shell shows a figure as written. A shorter display
  is a rounding, and anchor 1 keeps rounding out of the shell. If one is
  wanted, it is a second written form from analyze, decided by the
  shell-boundary record under a method version.
- Every metric that reads an amount carries one more `declined` condition in
  its catalogue entry, though it is generated.

**Expensive to reverse:**

- The written form, once figures are stored. M6's schema and its ranking
  will read it.

## Enforcement

This applies anchors 1, 4 and 5, and edits none of them.

- **Anchor 5, nothing arbitrary.** `literals` keeps every number but zero
  and one out of the derivation. `RADIX` and `SIGNIFICANT_DIGITS` stand in
  `constants/` under their citations.
  - **Gap:** nothing checks that either citation is true.
  - **Gap:** a precision in a format string, and a number parsed from a
    string.
- **Anchor 4, the same input gives the same output.** This holds by
  construction: the arithmetic is integers, and only one function rounds.
  - **Gap:** nothing checks that the derivation names no `f32` or `f64`.
- **Anchor 1, the shell only presents.** A figure arrives in final
  characters, so the shell needs no arithmetic to show it.
  - **Gap:** whether the shell shows a percentage, or a shorter figure, is
    left to the shell-boundary record.
- **The method version moves when it must.**
  - **Gap:** nothing forces a bump, as the accepted record already says.

## Decision review

By the decider, not the proposer.

- **Authority:**
- **Checked:**
- **Verdict and why:**
- **What would have changed it:**
