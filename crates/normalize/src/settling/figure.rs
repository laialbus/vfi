//! The decimal literals the boundary publishes, added and subtracted exactly.
//!
//! A `sum` and a `difference` compose a number the filing does not state, so
//! something has to do arithmetic over what it does state. What crosses the
//! fetch boundary is characters — "the amount, as the decimal literal the
//! document publishes", unparsed, because "a binary float is a lossy reading of
//! a published decimal" — and reading those characters into a float here would
//! put the loss back one line further on. So the digits are added as digits.
//!
//! There is no bound on how many of them there may be. A cap would be a number
//! with no source, and the shape of failure it buys is a composition that is
//! silently absent for a filer whose figures ran one digit past it. The only
//! literal this refuses is one that is not a decimal at all, which the boundary
//! does not publish and could not, so refusing it costs nothing and inventing a
//! reading for it would cost the thing this milestone exists to protect.

use std::cmp::Ordering;

/// One published decimal literal, read as what it is: its digits, and how many
/// of them fall after the point.
///
/// The digits are held most significant first, one to a byte, and the sign is
/// held apart from them, so that a negative zero cannot be written down.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Figure {
    negative: bool,
    digits: Vec<u8>,
    scale: usize,
}

impl Figure {
    /// The literal as a figure, or nothing where it is not a decimal literal.
    ///
    /// The shape is the one the registry already holds an asserted value to:
    /// an optional minus, digits, and at most one point with digits on both
    /// sides of it. No exponent, no separator, no space.
    pub fn read(literal: &str) -> Option<Figure> {
        let (negative, rest) = match literal.strip_prefix('-') {
            Some(rest) => (true, rest),
            None => (false, literal),
        };

        let (whole, fraction) = match rest.split_once('.') {
            Some((whole, fraction)) if !fraction.is_empty() => (whole, fraction),
            Some(_) => return None,
            None => (rest, ""),
        };
        if whole.is_empty() {
            return None;
        }

        let mut digits = Vec::with_capacity(whole.len() + fraction.len());
        for held in whole.bytes().chain(fraction.bytes()) {
            if !held.is_ascii_digit() {
                return None;
            }
            digits.push(held - b'0');
        }

        Some(Figure {
            negative,
            digits,
            scale: fraction.len(),
        })
    }

    /// Zero, at no scale: what a composition of nothing comes to, and the one
    /// figure this module states rather than reads.
    pub fn zero() -> Figure {
        Figure {
            negative: false,
            digits: vec![0],
            scale: 0,
        }
    }

    pub fn is_zero(&self) -> bool {
        self.digits.iter().all(|digit| *digit == 0)
    }

    pub fn negated(&self) -> Figure {
        Figure {
            negative: !self.negative,
            digits: self.digits.clone(),
            scale: self.scale,
        }
    }

    /// The two added, at the scale of whichever states more decimal places, so
    /// that neither loses a digit it published.
    pub fn plus(&self, other: &Figure) -> Figure {
        let scale = self.scale.max(other.scale);
        let one = self.at(scale);
        let other_digits = other.at(scale);

        if self.negative == other.negative {
            return Figure {
                negative: self.negative,
                digits: added(&one, &other_digits),
                scale,
            };
        }

        match compared(&one, &other_digits) {
            Ordering::Less => Figure {
                negative: other.negative,
                digits: subtracted(&other_digits, &one),
                scale,
            },
            _ => Figure {
                negative: self.negative,
                digits: subtracted(&one, &other_digits),
                scale,
            },
        }
    }

    /// The figure back as a decimal literal, at the scale it holds and with no
    /// leading zero the whole part does not need.
    ///
    /// A zero is written without a sign. Every other value keeps the one it
    /// has, including the trailing zeros a scale carries: a figure published to
    /// four decimal places says something by publishing them.
    pub fn rendered(&self) -> String {
        let mut from = 0;
        while self.digits.len() - from > self.scale + 1 && self.digits[from] == 0 {
            from += 1;
        }
        let digits = &self.digits[from..];

        let mut out = String::with_capacity(digits.len() + 2);
        if self.negative && !self.is_zero() {
            out.push('-');
        }
        for (at, digit) in digits.iter().enumerate() {
            if self.scale > 0 && at + self.scale == digits.len() {
                out.push('.');
            }
            out.push(char::from(b'0' + digit));
        }
        out
    }

    /// The digits this would have at `scale`, which is this scale or more.
    fn at(&self, scale: usize) -> Vec<u8> {
        let mut digits = Vec::with_capacity(self.digits.len() + scale - self.scale);
        digits.extend_from_slice(&self.digits);
        digits.resize(self.digits.len() + (scale - self.scale), 0);
        digits
    }
}

fn added(one: &[u8], other: &[u8]) -> Vec<u8> {
    let width = one.len().max(other.len()) + 1;
    let mut sum = vec![0; width];
    let mut carry = 0;

    for at in 0..width {
        let held = digit(one, at) + digit(other, at) + carry;
        sum[width - 1 - at] = held % 10;
        carry = held / 10;
    }
    sum
}

/// `one` less `other`, where `one` is the larger of the two.
fn subtracted(one: &[u8], other: &[u8]) -> Vec<u8> {
    let width = one.len();
    let mut left = vec![0; width];
    let mut borrow = 0;

    for at in 0..width {
        let held = digit(one, at);
        let taken = digit(other, at) + borrow;
        let (held, next) = match held < taken {
            true => (held + 10 - taken, 1),
            false => (held - taken, 0),
        };
        left[width - 1 - at] = held;
        borrow = next;
    }
    left
}

fn compared(one: &[u8], other: &[u8]) -> Ordering {
    for at in (0..one.len().max(other.len())).rev() {
        let held = digit(one, at).cmp(&digit(other, at));
        if held != Ordering::Equal {
            return held;
        }
    }
    Ordering::Equal
}

/// The digit `at` places from the right, and zero past the left-hand end.
fn digit(digits: &[u8], at: usize) -> u8 {
    match digits.len().checked_sub(at + 1) {
        Some(index) => digits[index],
        None => 0,
    }
}

/// The digits, against what a filing publishes and what a composition of two of
/// them has to come to.
///
/// Every literal below is one this filer's facts state or one a composition over
/// them lands on. What the cases hold is the property the arithmetic exists for:
/// two figures published at different scales compose without either losing a
/// digit, and a figure composed is rendered as a literal that reads back as
/// itself.
#[cfg(test)]
mod the_digits_are_added_as_digits {
    use super::Figure;

    fn read(literal: &str) -> Figure {
        Figure::read(literal).unwrap_or_else(|| panic!("{literal} is a decimal literal"))
    }

    fn plus(one: &str, other: &str) -> String {
        read(one).plus(&read(other)).rendered()
    }

    fn less(one: &str, other: &str) -> String {
        read(one).plus(&read(other).negated()).rendered()
    }

    /// The two compositions the cases beside this settle: a balance sheet that
    /// adds, and a gross profit that subtracts.
    #[test]
    fn a_composition_comes_to_the_figure_the_filing_states() {
        assert_eq!(plus("76485", "5922"), "82407");
        assert_eq!(less("804887", "439260"), "365627");
    }

    /// A carry and a borrow across a run of nines and zeros, which is where a
    /// digit-by-digit sum is wrong if it is wrong at all.
    #[test]
    fn a_carry_and_a_borrow_run_the_length_of_the_figure() {
        assert_eq!(plus("999999", "1"), "1000000");
        assert_eq!(less("1000000", "1"), "999999");
        assert_eq!(less("-999999", "1"), "-1000000");
    }

    /// Neither figure loses a digit it published: the composition is at the
    /// scale of whichever states the most decimal places.
    #[test]
    fn two_scales_compose_at_the_longer_of_them() {
        assert_eq!(plus("1", "0.0004"), "1.0004");
        assert_eq!(less("0.25", "1"), "-0.75");
        assert_eq!(plus("0.10", "0.20"), "0.30");
        assert_eq!(plus("-0.0053", "0.0053"), "0.0000");
    }

    /// A figure crossing zero keeps the sign of the larger, and a zero is
    /// written without one. It keeps its scale, as every other figure does: the
    /// zero that came of two figures stated to four places is stated to four
    /// places too, and nothing here rounds a published figure back to a shorter
    /// one.
    #[test]
    fn a_zero_is_written_without_a_sign() {
        assert_eq!(less("30810", "30810"), "0");
        assert_eq!(plus("-30810", "30810"), "0");
        assert_eq!(read("-0").rendered(), "0");
        assert!(read("-0").is_zero());
        assert!(!read("-0.0001").is_zero());
    }

    /// What the boundary publishes and nothing else. A literal outside the shape
    /// states no figure, so a rule that would have to read one has nothing to
    /// read.
    #[test]
    fn a_literal_that_is_not_a_decimal_is_no_figure() {
        for held in [
            "", "-", ".", "1.", ".5", "1.2.3", "1e9", "1 000", "0x10", "＋1",
        ] {
            assert!(
                Figure::read(held).is_none(),
                "`{held}` was read as a decimal literal"
            );
        }
    }
}
