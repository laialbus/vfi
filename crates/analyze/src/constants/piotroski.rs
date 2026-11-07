//! The count Piotroski's score cannot do without: how many signals it sums.
//! Why the signals and the score are formed as they are is
//! `docs/adr/piotroski-signals-and-score.md`'s.
//!
//! Origin, named as `docs/adr/which-other-period-a-metric-reads.md` names it
//! and not read from the workspace: Piotroski, J. D., "Value Investing: The
//! Use of Historical Financial Statement Information to Separate Winners from
//! Losers", *Journal of Accounting Research* 38, Supplement, 2000, pp. 1–41.
//!
//! The open copy beside it: *Selected Paper 84*, The University of Chicago
//! Graduate School of Business, 2002, read from
//! <https://www.chicagobooth.edu/~/media/FE874EE65F624AAEBD0166B1974FD74D.pdf>.
//! The section numbers below are this copy's, and its printed page numbers.

/// How many signals Piotroski's score sums, so that it runs from zero to this
/// many and the signals are held as `[bool; PIOTROSKI_SIGNALS]`.
///
/// Source: the copy, §2.3, p. 6, "I chose nine fundamental signals to measure
/// three areas of the firm's financial condition: profitability, financial
/// leverage/liquidity, and operating efficiency.", and §2.3.4, p. 9, "Given
/// the nine underlying signals, F_SCORE can range from a low of 0 to a high
/// of 9".
pub(crate) const PIOTROSKI_SIGNALS: usize = 9;
