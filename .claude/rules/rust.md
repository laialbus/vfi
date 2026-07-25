# Rust

Not yet scoped to paths. Once the workspace layout is fixed in M1, this file gets
`paths` frontmatter so it loads only when Rust files are being worked on. Until
then it loads every session.

## Guides

- API design between crates:
  https://rust-lang.github.io/api-guidelines/checklist.html
- Formatting and layout: https://doc.rust-lang.org/style-guide/

Consult a guide when making a decision it covers. Do not read them end to end,
and do not copy them into this repo — a local summary of an upstream guide is a
second source of truth and it will drift.

The API guidelines matter most at crate boundaries, since those boundaries are
the contracts between stages and are the hardest thing to change later.

## Style is guidance, not a gate

No build fails over formatting. Match these unless there is a stated reason not
to, recorded in an ADR.

## Write each part for what it does

Use the language differently depending on the stage:

- **Analyze** is pure by anchor. Write it functionally: data in, results out, no
  mutation reaching outside a function. It should read like the derivation it is.
- **Fetch and normalize** handle large volumes. Write them for the machine:
  explicit control flow, few allocations, reused buffers.

Neither style is correct everywhere. Do not make the hot paths elegant at the
cost of speed, and do not make the analysis clever at the cost of being readable
as a proof.
