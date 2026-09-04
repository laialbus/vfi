# GOALS

What finished looks like. This file is the destination; ANCHORS.md and AGENTS.md
are the floor. A task is worth doing only if it moves a milestone forward, and it
is valid only if every gate still passes. Both, always.

Milestones are ordered. Each states what it means and one criterion an agent can
check itself against. How to get there is not written here — that is the agent's
work, planned in WORKPLAN.md.

A milestone is a destination, reached once. A gate is a property, held forever.
Nothing in this file is a gate.

The first three milestones are built under human supervision. They are the
increments that cannot check themselves: they build the machinery that checks
everything after them.

## M0 — The rules are enforced, not just written

Smallest and first. An agent can edit any file until something stops it, and
ANCHORS.md saying "human sign-off required" stops nothing on its own.

**Done when:**
- A pre-edit block prevents an agent from changing any protected path: the
  anchors, the agent rules, the goals, the workplan format, the contracts, the
  canonical concept definitions, the storage schemas, the gate scripts, and the
  block's own configuration. The task queue in `tasks/` is not protected — the
  planner writes there; only the format file is fixed.
- The block covers shell commands too, not only the editing tools. A block that
  only guards the editor is walked around with one shell redirect.
- An attempted edit to a protected path is demonstrably refused. A block nobody
  has watched refuse something is indistinguishable from no block.

The requirement is the block. The mechanism, under Claude Code, is a pre-tool-use
hook: a script in `.claude/hooks/`, wired in `.claude/settings.json`, reading a
protected-path list. The list is a human decision. The script and the wiring are
the agent's.

Nothing else is attempted before this. Everything after it is built with the
constitution already protected.

## M1 — The workspace has a shape

Every kind of artifact has exactly one home, and the layout is written down.
Scripts scattered across the repo cost every future agent and every future
reader, and the cost compounds.

**Done when:**
- The directory layout is defined and documented in one place, including
  `.claude/` and everything created in M0. A new agent can tell where something
  belongs without asking.
- The Rust workspace exists, one crate per stage, and compiles — empty is fine.
- All scripts live under one directory. One entry point per purpose, no ad-hoc
  scripts elsewhere and none at the repo root.
- `tasks/`, `sessions/`, `escalations/`, and `docs/adr/` exist, each with a short
  README saying what belongs in it and what does not.
- Each holds one file per item, so runs happening at the same time never collide
  writing their records.
- An ADR template exists, so decision records come out in one shape.
- `scripts/tasks.sh available` reports claimable work by reading the queue and
  git, not a list anyone maintains.
- The style rules in `.claude/rules/` are scoped to the paths they apply to, so
  Rust guidance does not load while working on the shell, and the reverse. This
  is possible only once the layout above is fixed.
- A scheduled job starts an agent run on its own: it pulls, refuses to start on a
  dirty tree or red CI, enforces a wall-clock limit, and releases its claim on
  any failure.

## M2 — The floor holds

The anchors become machinery. Until this is done, unattended work has nothing
catching its mistakes.

**Done when:**
- Every gate in AGENTS.md runs from a single command, and the same command runs
  in CI.
- Every gate has a test that violates it on purpose and proves the gate fails. A
  gate with no proof-of-catch does not count as existing.
- The fixture harness and the benchmark harness exist, with a committed
  baseline. The fixtures themselves accumulate later.
- A deliberately slow job runs in the engine while the interface stays
  responsive.

## M3 — Data arrives, and only from sources we trust

The engine retrieves filings from official government sources. Primary sources
only: SEC EDGAR for filings. Market prices, which filings do not contain, come
from one user-keyed provider and are connected in M5, the first milestone that
consumes one.

**Done when:**
- Given a ticker, the engine retrieves that company's filing history from EDGAR.
- The fetcher cannot reach a host outside the allowed list. This is checked, not
  intended.
- Request rates stay inside the source's published limits.
- The filer funnel runs: seed set, then metadata gate, then history for
  survivors only.
- Every filer evaluated has a verdict and a reason in the decision ledger,
  including the rejected ones.
- Every fact carries where it came from.

## M4 — Filings become comparable numbers

The hard part. Filings arrive in many shapes, and the job is to turn them into
one internal shape so companies can be compared. Some differences are only style
— the same idea tagged a different way. Others are real: a bank has no gross
profit, and an insurer's revenue is not a manufacturer's sales. Normalizing the
first kind is the work; pretending the second kind away would fabricate numbers.
A wrong answer here looks right and quietly corrupts everything downstream.

**Done when:**
- A canonical set of financial concepts is defined, with the meaning of each one
  written down. Filings are mapped into it.
- That set contains exactly what analysis needs and nothing else. A concept no
  metric consumes is not normalized.
- Each concept resolves to one of three states: a value, not applicable to this
  kind of company, or unknown. The last two are never confused — one is a
  correct absence, the other is a mapping that failed.
- Mapping happens through a versioned registry with per-company overrides —
  data, not branching code.
- Choosing among several candidates in one filing follows a stated rule:
  consolidated over segment, and the right period basis.
- Amendments, restatements, and period alignment follow a stated ruleset,
  separate from the mapping.
- Every resolved value records its source tag, its filing, and the rule that set
  it.
- The golden fixtures produce their expected results, including a restatement, a
  fiscal-year change, a company that changed tags mid-history, a dividend
  suspension, negative equity, and a company whose statements do not fit the
  ordinary shape.
- Nothing is ever guessed.

## M5 — Numbers become judgments

Value and dividend analysis, computed as a proof from the data and the user's
settings.

**Done when:**
- Market prices arrive through the price provider interface from one
  user-keyed source. The key reaches the engine only as a parameter, and
  without one every price-dependent metric is absent with a reason.
- The first provider implementation works within a free-tier key's published
  limits: screening the corpus needs no price call, and valuing a shortlist
  fits inside the tier's daily allowance.
- Value metrics are computed: profitability, returns on capital, valuation
  multiples, liquidity, leverage, financial health and bankruptcy scoring, the
  established screens, and discounted cash flow with margin of safety.
- Dividend metrics are computed: yield and relative yield, payout on earnings and
  on free cash flow, coverage, growth rate and streak, retention and sustainable
  growth, quality heuristics, and dividend-based valuation.
- Every methodology constant is named, defined once, and cites the paper or book
  it comes from.
- Every free parameter arrives as a setting. Every default states its reason.
- Each result records the settings and method version it was computed from.
- A metric with no price data is absent with a reason. It never fails the run and
  never shows a wrong number.

## M6 — Results persist and rank

Analysis output is stored, and the ranked list is built over what is stored.

**Done when:**
- Metrics are written through the storage interface, one row per company and
  period.
- Ranking and screening are queries over stored results.
- The user chooses the criteria and their weights; the composite is not fixed.
- Adding a new screen recomputes nothing from filings.
- A default composite works with no price data, using filing-derived metrics
  only.

## M7 — The interface

All user interaction is graphical. The command line is for development only.

**Done when:**
- The landing view is a ranked list over the full corpus, and scrolling stays
  smooth at that size.
- Selecting a company opens statements, trends, scores, valuation, and dividend
  safety.
- No long operation ever blocks the interface.
- Absent data is shown as absence, with the reason.
- The shell contains no analysis logic.
- The look follows the Chadō aesthetic: restraint, asymmetry, space, quiet
  color, deliberate pace.

## Deferred — AI annotation

Not built now. But the seam is designed from M5 onward, because a port added
after the contracts harden is not one step away.

**Standing requirement, checked at M6:** a throwaway stub that reads analysis
output and attaches a comment can be written without changing anything upstream
of it, and deleted again. If the stub is awkward, the seam is wrong.

## Not in scope

Stated so an agent does not drift toward them: no web or hosted service, no
foreign private issuers filing 20-F, no intraday or real-time data, no portfolio
tracking, no order placement, no advice framed as a recommendation to buy or
sell.
