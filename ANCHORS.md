# ANCHORS

These are the fixed rules of VFI. Everything else in the project is open to the
agent's judgment; these are not. An agent may not violate, reinterpret, or work
around anything here. Changing this file requires explicit human sign-off (see
the last section).

Each anchor states the rule and how it is enforced. Enforcement is mechanical
wherever possible — a build failure or a CI gate, not a reviewer's memory.

## The six anchors

**1. The engine computes; the shell presents.**
All data work and computation live in the compiled Rust engine. The Python shell
only displays what the engine produces. The shell holds no analysis logic and
never touches raw filings — it reads engine output and nothing else.
*Enforced by:* the shell has no dependency path to filings or computation code.
Any numeric or analytic logic found in the shell is a review-blocking defect.

**2. Data flows one way.**
A later stage never calls an earlier one. The pipeline runs
fetch → normalize → analyze → store, and edges point forward only.
*Enforced by:* each stage is its own crate. The dependency graph encodes the
direction. A backward call is a dependency cycle, which the Rust compiler
rejects. This is a build failure, not a guideline.

**3. Stages talk through explicit contracts.**
Every boundary between stages is a typed, versioned contract. A stage knows the
contract on either side of it and nothing else about its neighbors.
*Enforced by:* contracts live in one place, are versioned, and a CI check fails
the build if a change breaks compatibility.

**4. Analysis is pure.**
The analyze stage takes financial data in and returns results out. No network,
no disk, no clock, no randomness, no logging with effects. Same input, same
output, every time.
*Enforced by:* a dependency deny-list on the analyze crate, checked in CI. It
cannot even link the libraries that would let it reach the outside world.

**5. Analysis is a proof. Nothing arbitrary enters it.**
Read the analyze stage as a proof. It has two premises — the credible data (facts
from filings) and the assumptions (user settings) — and the screens, ratios, and
DCF are the derivation from those premises to a result. Every value in the proof
must be traceable to a source. Concretely:
- Every constant is named, defined in one place, and carries its source — the
  paper or method it comes from. Piotroski's thresholds cite Piotroski. A bare
  literal with no citation is a defect.
- Every free parameter arrives as a setting, passed in. Analyze never invents a
  value for one.
- A default is an assumption with a preset value. It lives in the settings layer
  with its own stated reason, never as a literal buried in the derivation. The
  first-time user who has set nothing is still running the proof with explicit
  assumptions; they are simply pre-filled.
- Each result records the premises it used — the settings and the method version
  — so the derivation can be replayed and checked, the way a proof re-runs from
  its axioms.
"Nothing arbitrary" bans the *unsourced* value, not the non-user value. A cited
threshold from the literature is an axiom and belongs here; an uncited one is a
smuggled assumption and does not.
*Enforced by:* a lint on the analyze crate bans bare numeric literals outside a
small allowlist, forcing every methodology constant into a named, cited
definition. This runs in CI, so it is a build failure, not a reviewer's memory.

**6. AI annotates; it never computes.**
Any AI assistance reads finished results and adds commentary. It never produces a
number that enters analysis. Every number the user sees comes from the analyze
stage.
*Enforced by:* the AI layer sits after analyze and receives its output as input.
It has no write path into the metric table. If AI output can change a number, the
wiring is wrong.

## Invariants that follow

These are not new ideas. They are the anchors applied to specific parts of the
system, and they are equally fixed.

**Secrets live in the shell, never the engine.** The engine receives an API key
as a parameter. It never reads a keychain, config file, or environment for one.
Key storage is the shell's job, because it is desktop-specific and a service
would do it differently.

**Storage sits behind an interface.** The engine writes through a storage trait,
not to a concrete database. The local store is one implementation. No stage
depends on how storage works inside.

**Prices sit behind an interface, and a missing price is a state, not an error.**
Market data comes through a provider trait with one implementation per source.
Every price-dependent metric is optional at the type level. With no price data
the tool still works and shows why a metric is absent — it does not fail.

**The engine holds no per-user state.** Every operation takes its inputs
explicitly. Nothing about "the current user" is carried implicitly. This is what
lets the same engine later serve one desktop user or many.

**A component that many stages depend on is reached only through an explicit
interface.** Its internal shape, storage, and implementation are private and may
change freely. Adding a second consumer to a component that has no such interface
requires an ADR first. This is the general rule behind isolating the filer
decision ledger, the storage layer, the price provider, and the tag mapping —
each is widely depended upon, so each is walled off behind a narrow interface.

**One source of truth per value.** Every value — a constant, a contract, a
setting, a schema — is defined in one place and read from there. Where two
representations must exist, one is generated from the other, and CI fails if the
generated copy drifts from its source. This does not ban duplication; it bans
unchecked duplication. It is the general rule behind defining contracts once,
defining constants once, and reaching shared components through one interface.

**Ranking is composed late, over stored results.** Analysis emits primitive
metrics per company and period into a wide table. Ranking and screening are
queries and combinations over that table. A new screen never triggers
recomputation from filings.

**Normalization is data, not code.** The mapping from filing tags to canonical
concepts is a versioned data registry with per-company overrides — not a pile of
branching logic. Reconciling amendments, restatements, and periods is a separate
ruleset. Every resolved fact records where it came from and which rule set it.

## Fixed technical decisions

Settled per the project brief. Not the agent's call, though narrower than the
anchors above.

- Engine in Rust, shell in Python.
- Local desktop application. No web, no separate server to run.
- Local persistence, written only by the engine.
- Market data uses a user-supplied API key (bring your own key).

The Python GUI toolkit is not yet fixed. It is decided in ADR-001, which the
agent proposes and a human approves. Until then, treat it as open.

## Changing this file

This file changes only with explicit human sign-off, recorded in an ADR. An agent
that believes an anchor is wrong stops and escalates. It does not edit around the
anchor, and it does not proceed on the assumption that the anchor will be changed.
