//! The numbers the test of whether a duration is a year cannot do without:
//! the days of a week, and the two lengths in weeks an annual period may run.
//! Why a year is read from its own dates, and only so, is
//! `docs/adr/which-duration-precedes-a-duration.md`'s.
//!
//! Source for every constant here: Internal Revenue Code, 26 U.S.C. §441, as
//! printed by the Legal Information Institute at
//! <https://www.law.cornell.edu/uscode/text/26/441>, read in the run. The
//! annual periods a filer may keep are these three:
//!
//! - (d), "Calendar year": "For purposes of this subtitle, the term
//!   “calendar year” means a period of 12 months ending on December 31."
//! - (e), "Fiscal year": "For purposes of this subtitle, the term “fiscal
//!   year” means a period of 12 months ending on the last day of any month
//!   other than December. In the case of any taxpayer who has made the
//!   election provided by subsection (f) the term means the annual period
//!   (varying from 52 to 53 weeks) so elected."
//! - (f)(1), "General rule": "A taxpayer who, in keeping his books, regularly
//!   computes his income on the basis of an annual period which varies from
//!   52 to 53 weeks and ends always on the same day of the week and ends
//!   always— (A) on whatever date such same day of the week last occurs in a
//!   calendar month, or (B) on whatever date such same day of the week falls
//!   which is nearest to the last day of a calendar month, may (in accordance
//!   with the regulations prescribed under paragraph (3)) elect to compute his
//!   taxable income for purposes of this subtitle on the basis of such annual
//!   period."
//!
//! The twelve months of (d) and (e) are read as months, from the calendar
//! fields, and need no number here.

/// How many days a week has, so that a period of whole weeks steps back this
/// many days for each week.
///
/// Source: 26 U.S.C. §441(f)(1), which counts the annual period in weeks and
/// ends it "always on the same day of the week", so that it runs whole weeks
/// of seven days.
pub(crate) const DAYS_IN_A_WEEK: usize = 7;

/// The fewest weeks an annual period of §441(f) runs.
///
/// Source: 26 U.S.C. §441(f)(1), "an annual period which varies from 52 to
/// 53 weeks".
pub(crate) const FEWEST_WEEKS: usize = 52;

/// The most weeks an annual period of §441(f) runs.
///
/// Source: 26 U.S.C. §441(f)(1), as `FEWEST_WEEKS` cites it.
pub(crate) const MOST_WEEKS: usize = 53;
