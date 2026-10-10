//! The two numbers the arithmetic cannot do without: the radix a figure is
//! read and written in, and the precision it is written to. Why each is this
//! number is `docs/adr/how-analyze-computes-and-writes-a-figure.md`'s.

/// The radix every amount and close is read in and every figure is written in.
///
/// Source: Bray, T., ed., *The JavaScript Object Notation (JSON) Data
/// Interchange Format*, RFC 8259, IETF, 2017, §6: "A number is represented in
/// base 10 using decimal digits." That is the grammar EDGAR's amounts and the
/// source's closes are published in, and a figure is written back in it.
pub(crate) const RADIX: u32 = 10;

/// How many significant digits a figure is written to.
///
/// Source: the precision `p` of the decimal128 format. IEEE Computer Society,
/// *IEEE Standard for Floating-Point Arithmetic*, IEEE Std 754-2008, 2008,
/// §3.6, Table 3.6 "Decimal interchange format parameters", p. 13: `p`,
/// precision in digits, is 34 for decimal128.
///
/// Why this format and not a narrower one: a figure is what store keeps and
/// what ranking composes late, over stored results, and it is never
/// recomputed from the exact value, so it should carry more digits than any
/// input states. decimal128 is the widest basic decimal format the standard
/// defines, so choosing it needs no argument about which narrower width is
/// enough. At this width no amount a filing states at a scale of one, and no
/// sum or difference of such amounts, is rounded when written; only a
/// quotient that does not terminate, and a root, are.
pub(crate) const SIGNIFICANT_DIGITS: u32 = 34;
