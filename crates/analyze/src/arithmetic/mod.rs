//! The one number the derivation computes in, every operation on it, and the
//! one reader and one writer of its characters.
//!
//! A number is exact: a sign and a numerator over a denominator, both
//! integers of any size. Only a root rounds inside the derivation; every
//! other result is exact until it is written, and writing rounds once. What
//! is read, what each operation returns and how a figure is written is
//! `docs/adr/how-analyze-computes-and-writes-a-figure.md`'s, under the method
//! version.

#![expect(dead_code, reason = "no metric reads the arithmetic yet")]

use std::cmp::Ordering;

use natural::Natural;

use crate::constants::figure::{RADIX, SIGNIFICANT_DIGITS};

mod natural;

/// The `declined` condition of a metric that reads an amount or a close whose
/// characters state no number.
pub(crate) const INPUT_NOT_A_DECIMAL: &str = "input_not_a_decimal";

/// An exact rational.
///
/// It is held in lowest terms, with a denominator that is not zero and a zero
/// that is not negative, so each value has one representation and equality by
/// value is equality of the parts.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Number {
    negative: bool,
    numerator: Natural,
    denominator: Natural,
}

impl Number {
    /// The number the characters state, or none where they are outside the
    /// grammar: an optional `-`, one or more ASCII digits, then optionally a
    /// `.` and one or more ASCII digits. Reading never rounds.
    pub(crate) fn read(characters: &str) -> Option<Number> {
        let (negative, unsigned) = match characters.strip_prefix('-') {
            Some(unsigned) => (true, unsigned),
            None => (false, characters),
        };
        let (whole, fraction) = match unsigned.split_once('.') {
            Some((_, "")) => return None,
            Some((whole, fraction)) => (whole, fraction),
            None => (unsigned, ""),
        };
        if whole.is_empty() {
            return None;
        }
        let mut digits = whole
            .chars()
            .chain(fraction.chars())
            .map(|character| character.to_digit(RADIX))
            .collect::<Option<Vec<u32>>>()?;
        digits.reverse();

        Some(Number::of(
            negative,
            Natural::of_digits(digits),
            Natural::one().shifted(fraction.len()),
        ))
    }

    /// The characters a figure or a numeric setting's value is written as.
    ///
    /// The number is rounded once to `SIGNIFICANT_DIGITS` significant digits,
    /// to the nearest with a tie away from zero: `roundTiesToAway`, IEEE Std
    /// 754-2008 §4.3.1. A negative number rounds to the negation of its
    /// magnitude's rounding. It is written positionally in `RADIX`, with `-`
    /// before a negative number and nothing before any other, no exponent and
    /// no separator, one `0` before the point only when the magnitude is below
    /// one, and no zero trailing the point.
    pub(crate) fn written(&self) -> Box<str> {
        let Some((digits, scale)) = self.significand() else {
            return "0".into();
        };

        let places = digits.places();
        let point = usize::try_from(scale).unwrap_or(0);
        let mut written = String::with_capacity(places + point + 1);
        if self.negative {
            written.push('-');
        }
        if point >= places {
            written.push('0');
            written.push('.');
            written.extend(std::iter::repeat_n('0', point - places));
        }
        for (place, digit) in digits.digits().iter().enumerate().rev() {
            if place + 1 == point && point < places {
                written.push('.');
            }
            written.push(char::from_digit(*digit, RADIX).expect("a digit is below the radix"));
        }
        if scale < 0 {
            let zeros = usize::try_from(scale.unsigned_abs()).expect("a figure fits in memory");
            written.extend(std::iter::repeat_n('0', zeros));
        }
        if point > 0 {
            let kept = written.trim_end_matches('0').trim_end_matches('.').len();
            written.truncate(kept);
        }
        written.into()
    }

    /// Whether the number has no more than `SIGNIFICANT_DIGITS` significant
    /// digits, so that the characters written for it read back as itself.
    /// A numeric setting's constructor refuses any value that does not.
    pub(crate) fn within_significant_digits(&self) -> bool {
        match self.significand() {
            None => true,
            Some((digits, scale)) => Number::scaled(self.negative, digits, scale) == *self,
        }
    }

    pub(crate) fn zero() -> Number {
        Number {
            negative: false,
            numerator: Natural::zero(),
            denominator: Natural::one(),
        }
    }

    pub(crate) fn one() -> Number {
        Number {
            negative: false,
            numerator: Natural::one(),
            denominator: Natural::one(),
        }
    }

    pub(crate) fn negated(&self) -> Number {
        Number {
            negative: !self.negative && !self.numerator.is_zero(),
            ..self.clone()
        }
    }

    pub(crate) fn plus(&self, other: &Number) -> Number {
        let one = self.numerator.times(&other.denominator);
        let another = other.numerator.times(&self.denominator);
        let denominator = self.denominator.times(&other.denominator);
        if self.negative == other.negative {
            return Number::of(self.negative, one.plus(&another), denominator);
        }
        match one.cmp(&another) {
            Ordering::Less => Number::of(other.negative, another.minus(&one), denominator),
            _ => Number::of(self.negative, one.minus(&another), denominator),
        }
    }

    pub(crate) fn minus(&self, other: &Number) -> Number {
        self.plus(&other.negated())
    }

    pub(crate) fn times(&self, other: &Number) -> Number {
        Number::of(
            self.negative != other.negative,
            self.numerator.times(&other.numerator),
            self.denominator.times(&other.denominator),
        )
    }

    /// `self` over `divisor`, or none where the divisor is zero.
    pub(crate) fn divided(&self, divisor: &Number) -> Option<Number> {
        if divisor.numerator.is_zero() {
            return None;
        }
        Some(Number::of(
            self.negative != divisor.negative,
            self.numerator.times(&divisor.denominator),
            self.denominator.times(&divisor.numerator),
        ))
    }

    /// `self` to the power `exponent`. The power zero is one, of zero too, and
    /// a negative power is one over the positive power, so zero to a negative
    /// power is a zero divisor and none.
    pub(crate) fn power(&self, exponent: i64) -> Option<Number> {
        let product =
            (0..exponent.unsigned_abs()).fold(Number::one(), |product, _| product.times(self));
        if exponent < 0 {
            Number::one().divided(&product)
        } else {
            Some(product)
        }
    }

    /// The `index`-th root that is not negative, correctly rounded to
    /// `SIGNIFICANT_DIGITS` significant digits with a tie away from zero, so
    /// exact wherever the root has no more digits than that. None for a
    /// negative radicand or an index of zero.
    pub(crate) fn root(&self, index: u32) -> Option<Number> {
        if index == 0 || self.negative {
            return None;
        }
        if self.numerator.is_zero() {
            return Some(Number::zero());
        }
        let exponent = self.exponent().div_euclid(i64::from(index));
        let scale = i64::from(SIGNIFICANT_DIGITS) - 1 - exponent;
        let (radicand, _, _) = self.floor_scaled((scale + 1) * i64::from(index));
        let radix = Natural::of(u64::from(RADIX));
        let (kept, last) = radicand.root_floor(u64::from(index)).divided(&radix);

        Some(Number::scaled(false, rounded(kept, &last, &radix), scale))
    }

    /// Whether the number is below, at or above zero.
    pub(crate) fn sign(&self) -> Ordering {
        match (self.negative, self.numerator.is_zero()) {
            (true, _) => Ordering::Less,
            (false, true) => Ordering::Equal,
            (false, false) => Ordering::Greater,
        }
    }

    pub(crate) fn absolute(&self) -> Number {
        Number {
            negative: false,
            ..self.clone()
        }
    }

    /// The number a sign, a numerator and a denominator that is not zero
    /// state, in lowest terms.
    fn of(negative: bool, numerator: Natural, denominator: Natural) -> Number {
        if numerator.is_zero() {
            return Number::zero();
        }
        let common = numerator.common_divisor(&denominator);
        let (numerator, _) = numerator.divided(&common);
        let (denominator, _) = denominator.divided(&common);
        Number {
            negative,
            numerator,
            denominator,
        }
    }

    /// `digits` over `RADIX` to the power `scale`, with the sign given.
    fn scaled(negative: bool, digits: Natural, scale: i64) -> Number {
        let places = usize::try_from(scale.unsigned_abs()).expect("a figure fits in memory");
        if scale < 0 {
            Number::of(negative, digits.shifted(places), Natural::one())
        } else {
            Number::of(negative, digits, Natural::one().shifted(places))
        }
    }

    /// The `E` for which `RADIX^E` is not above the magnitude and `RADIX^(E+1)`
    /// is above it, for a number that is not zero.
    fn exponent(&self) -> i64 {
        let places =
            |natural: &Natural| i64::try_from(natural.places()).expect("a figure fits in memory");
        let estimate = places(&self.numerator) - places(&self.denominator);
        let places_apart =
            usize::try_from(estimate.unsigned_abs()).expect("a figure fits in memory");
        let reached = if estimate < 0 {
            self.numerator.shifted(places_apart) >= self.denominator
        } else {
            self.numerator >= self.denominator.shifted(places_apart)
        };
        if reached { estimate } else { estimate - 1 }
    }

    /// The magnitude times `RADIX^scale`, as its floor, the remainder, and the
    /// divisor that remainder is against.
    fn floor_scaled(&self, scale: i64) -> (Natural, Natural, Natural) {
        let places = usize::try_from(scale.unsigned_abs()).expect("a figure fits in memory");
        let (dividend, divisor) = if scale < 0 {
            (self.numerator.clone(), self.denominator.shifted(places))
        } else {
            (self.numerator.shifted(places), self.denominator.clone())
        };
        let (floor, remainder) = dividend.divided(&divisor);
        (floor, remainder, divisor)
    }

    fn magnitude_cmp(&self, other: &Number) -> Ordering {
        self.numerator
            .times(&other.denominator)
            .cmp(&other.numerator.times(&self.denominator))
    }

    /// The magnitude rounded to `SIGNIFICANT_DIGITS` significant digits, as
    /// digits over `RADIX^scale`, or none for zero.
    fn significand(&self) -> Option<(Natural, i64)> {
        if self.numerator.is_zero() {
            return None;
        }
        let scale = i64::from(SIGNIFICANT_DIGITS) - 1 - self.exponent();
        let (floor, remainder, divisor) = self.floor_scaled(scale);
        Some((rounded(floor, &remainder, &divisor), scale))
    }
}

/// `kept`, or the natural after it where the remainder `left` against
/// `divisor` is a tie or more, which is where `left` is at least
/// `divisor - left`. So a tie goes away from zero, with no half to compare to.
fn rounded(kept: Natural, left: &Natural, divisor: &Natural) -> Natural {
    if *left >= divisor.minus(left) {
        kept.plus(&Natural::one())
    } else {
        kept
    }
}

impl Ord for Number {
    fn cmp(&self, other: &Number) -> Ordering {
        match (self.sign(), other.sign()) {
            (Ordering::Less, Ordering::Less) => other.magnitude_cmp(self),
            (one, another) if one != another => one.cmp(&another),
            _ => self.magnitude_cmp(other),
        }
    }
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Number) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::Number;

    fn number(characters: &str) -> Number {
        Number::read(characters).expect("the characters are in the grammar")
    }

    fn quotient(dividend: &str, divisor: &str) -> Number {
        number(dividend)
            .divided(&number(divisor))
            .expect("the divisor is not zero")
    }

    fn written(number: &Number) -> String {
        number.written().into()
    }

    fn repeated(digit: char, count: usize) -> String {
        std::iter::repeat_n(digit, count).collect()
    }

    #[test]
    fn what_was_written_reads_back_as_the_number_written() {
        let figures = [
            Number::zero(),
            number("-30810"),
            number("0.0004"),
            number("394328000000"),
            quotient("1", "3"),
            quotient("-2", "3"),
            number("1234567890123456789012345678901234567890.123"),
            number("-0.000000000012345678901234567890123456789"),
        ];
        for figure in figures {
            let characters = written(&figure);
            let read = number(&characters);
            assert_eq!(written(&read), characters);
            assert!(read.within_significant_digits(), "{characters}");
            if figure.within_significant_digits() {
                assert_eq!(read, figure, "{characters}");
            }
        }
    }

    #[test]
    fn writing_what_was_read_gives_every_canonical_spelling_back() {
        let spellings = [
            "0",
            "1",
            "-1",
            "7",
            "1000",
            "-30810",
            "0.0004",
            "-0.0053",
            "0.5",
            "6.1",
            "394328000000",
            "9007199254740993",
            "1234567890123456789012345678901234",
            "0.1234567890123456789012345678901234",
            "-123456789012345678901234567890.1234",
        ];
        for spelling in spellings {
            assert_eq!(written(&number(spelling)), spelling);
        }
    }

    #[test]
    fn a_quotient_that_does_not_terminate_is_written_to_the_precision() {
        let third = format!("0.{}", repeated('3', 34));
        let two_thirds = format!("0.{}7", repeated('6', 33));
        assert_eq!(written(&quotient("1", "3")), third);
        assert_eq!(written(&quotient("2", "3")), two_thirds);
        assert_eq!(written(&quotient("2", "6")), third);
        assert!(!quotient("1", "3").within_significant_digits());
    }

    #[test]
    fn a_negative_figure_rounds_as_its_magnitude_does() {
        assert_eq!(
            written(&quotient("-2", "3")),
            format!("-0.{}7", repeated('6', 33))
        );
        assert_eq!(written(&number("-0")), "0");
        assert_eq!(number("-0"), Number::zero());
        assert_eq!(written(&number("-0.0053")), "-0.0053");
        assert_eq!(written(&number("-0.000")), "0");
    }

    #[test]
    fn currency_too_large_for_a_binary_float_stays_exact() {
        let past_binary64 = number("9007199254740993");
        assert_eq!(written(&past_binary64), "9007199254740993");
        assert_eq!(
            written(&past_binary64.plus(&Number::one())),
            "9007199254740994"
        );
    }

    #[test]
    fn a_history_amount_is_read_as_the_characters_published() {
        for (published, figure) in [
            ("394328000000", "394328000000"),
            ("-30810", "-30810"),
            ("0.0004", "0.0004"),
            ("6.10", "6.1"),
            ("007", "7"),
            ("-007.500", "-7.5"),
        ] {
            assert_eq!(written(&number(published)), figure);
        }
    }

    #[test]
    fn characters_outside_the_grammar_are_read_as_no_number() {
        for characters in [
            "1e9", "1,000", "+1", ".5", "1.", "", "-", "--1", "-.5", "1.2.3", " 1", "1 ", "0x1",
            "1_000", "\u{ff11}", "\u{0663}", "1.-5", "Infinity", "NaN",
        ] {
            assert_eq!(Number::read(characters), None, "{characters:?}");
        }
    }

    #[test]
    fn a_tie_rounds_away_from_zero_in_both_signs() {
        let tie = "1234567890123456789012345678901234.5";
        assert_eq!(written(&number(tie)), "1234567890123456789012345678901235");
        assert_eq!(
            written(&number(&format!("-{tie}"))),
            "-1234567890123456789012345678901235"
        );
        let below = "1234567890123456789012345678901234.4999";
        assert_eq!(
            written(&number(below)),
            "1234567890123456789012345678901234"
        );
        let small_tie = format!("0.000{}5", repeated('1', 34));
        assert_eq!(
            written(&number(&small_tie)),
            format!("0.000{}2", repeated('1', 33))
        );
    }

    #[test]
    fn rounding_up_can_carry_into_a_new_place() {
        assert_eq!(
            written(&number(&repeated('9', 40))),
            format!("1{}", repeated('0', 40))
        );
        assert_eq!(
            written(&number(&format!("-9.{}", repeated('9', 40)))),
            "-10"
        );
        assert_eq!(written(&number(&format!("0.{}", repeated('9', 40)))), "1");
    }

    #[test]
    fn a_root_is_correctly_rounded_and_exact_where_it_can_be() {
        let root = |characters: &str, index| {
            written(&number(characters).root(index).expect("the root is taken"))
        };
        assert_eq!(root("2", 2), "1.414213562373095048801688724209698");
        assert_eq!(root("6.25", 2), "2.5");
        assert_eq!(root("2", 3), "1.259921049894873164767210607278228");
        assert_eq!(root("10", 5), "1.584893192461113485202101373391507");
        assert_eq!(root("27", 3), "3");
        assert_eq!(root("0.000008", 3), "0.02");
        assert_eq!(root("0", 7), "0");
        assert_eq!(root("15241578750190521", 2), "123456789");
        assert_eq!(
            root(&format!("0.{}1", repeated('0', 39)), 2),
            format!("0.{}1", repeated('0', 19))
        );
        assert_eq!(root(&format!("0.{}", repeated('9', 40)), 2), "1");
        assert_eq!(root("2", 1), "2");
        assert_eq!(
            number("6.25").root(2),
            Some(quotient("5", "2")),
            "an exact root is the exact number"
        );
        assert!(
            number("2")
                .root(2)
                .expect("the root is taken")
                .within_significant_digits()
        );
    }

    #[test]
    fn a_root_declines_a_negative_radicand_and_index_zero() {
        assert_eq!(number("-8").root(3), None);
        assert_eq!(number("-4").root(2), None);
        assert_eq!(number("4").root(0), None);
        assert_eq!(number("0").root(0), None);
    }

    #[test]
    fn a_figure_is_positional_however_large_or_small() {
        let ten = number("10");
        let large = ten.power(40).expect("a positive power");
        let small = ten.power(-40).expect("ten is not zero");
        assert_eq!(written(&large), format!("1{}", repeated('0', 40)));
        assert_eq!(written(&small), format!("0.{}1", repeated('0', 39)));
        assert_eq!(small, quotient("1", &written(&large)));
    }

    #[test]
    fn a_setting_holds_no_more_digits_than_a_figure_is_written_to() {
        let thirty_five = "12345678901234567890123456789012345";
        let thirty_four = "0.1234567890123456789012345678901234";
        assert!(!number(thirty_five).within_significant_digits());
        assert!(!number("1.0000000000000000000000000000000001").within_significant_digits());
        assert!(number(thirty_four).within_significant_digits());
        assert!(number("100000000000000000000000000000000000000").within_significant_digits());
        assert_eq!(written(&number(thirty_four)), thirty_four);
        assert_eq!(number(&written(&number(thirty_four))), number(thirty_four));
        assert!(Number::zero().within_significant_digits());
    }

    #[test]
    fn negation_addition_subtraction_and_multiplication_are_exact() {
        let third = quotient("1", "3");
        assert_eq!(third.plus(&third).plus(&third), Number::one());
        assert_eq!(third.minus(&third), Number::zero());
        assert_eq!(third.times(&number("3")), Number::one());
        assert_eq!(number("0.1").plus(&number("0.2")), number("0.3"));
        assert_eq!(number("5").minus(&number("7.25")), number("-2.25"));
        assert_eq!(number("-5").minus(&number("-7.25")), number("2.25"));
        assert_eq!(number("-1.5").times(&number("-4")), number("6"));
        assert_eq!(number("-1.5").times(&number("4")), number("-6"));
        assert_eq!(number("0").times(&number("-4")), Number::zero());
        assert_eq!(number("2.5").negated(), number("-2.5"));
        assert_eq!(Number::zero().negated(), Number::zero());
        assert_eq!(
            number("99999999999999999999").times(&number("99999999999999999999")),
            number("9999999999999999999800000000000000000001")
        );
    }

    #[test]
    fn division_is_exact_and_a_zero_divisor_is_no_number() {
        assert_eq!(quotient("1", "4"), number("0.25"));
        assert_eq!(quotient("-7", "0.5"), number("-14"));
        assert_eq!(quotient("-7", "-0.5"), number("14"));
        assert_eq!(quotient("0", "-3"), Number::zero());
        assert_eq!(quotient("1", "3").times(&number("3")), Number::one());
        assert_eq!(number("1").divided(&number("0")), None);
        assert_eq!(number("0").divided(&number("-0.000")), None);
    }

    #[test]
    fn an_integer_power_is_exact() {
        let power = |characters: &str, exponent| number(characters).power(exponent);
        assert_eq!(power("0", 0), Some(Number::one()));
        assert_eq!(power("-2.5", 0), Some(Number::one()));
        assert_eq!(power("-2", 3), Some(number("-8")));
        assert_eq!(power("-2", 2), Some(number("4")));
        assert_eq!(power("-2", -2), Some(number("0.25")));
        assert_eq!(power("-2", -3), Some(number("-0.125")));
        assert_eq!(power("1.05", 3), Some(number("1.157625")));
        assert_eq!(power("0", 3), Some(Number::zero()));
        assert_eq!(power("0", -1), None);
    }

    #[test]
    fn comparison_sign_absolute_minimum_and_maximum_are_exact() {
        assert_eq!(quotient("1", "2"), quotient("2", "4"));
        assert_eq!(number("0.50"), quotient("1", "2"));
        assert_eq!(written(&quotient("2", "4")), "0.5");
        let ordered = [
            number("-100"),
            number("-2"),
            quotient("-1", "3"),
            number("-0.3333"),
            Number::zero(),
            number("0.0001"),
            quotient("1", "3"),
            number("0.3334"),
            number("10"),
        ];
        for (at, one) in ordered.iter().enumerate() {
            for (other_at, other) in ordered.iter().enumerate() {
                assert_eq!(
                    one.cmp(other),
                    at.cmp(&other_at),
                    "{one:?} against {other:?}"
                );
            }
        }
        assert_eq!(number("-3").sign(), Ordering::Less);
        assert_eq!(number("-0").sign(), Ordering::Equal);
        assert_eq!(number("0.001").sign(), Ordering::Greater);
        assert_eq!(number("-3.5").absolute(), number("3.5"));
        assert_eq!(number("3.5").absolute(), number("3.5"));
        assert_eq!(number("-3").min(number("2")), number("-3"));
        assert_eq!(number("-3").max(number("2")), number("2"));
        assert_eq!(quotient("1", "3").max(number("0.3333")), quotient("1", "3"));
    }
}
