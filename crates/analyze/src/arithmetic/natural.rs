//! The integers a number is built from: not negative, of any size, held as
//! digits in `RADIX`, one digit per place.

use std::cmp::Ordering;

use crate::constants::figure::RADIX;

/// A natural number. Its digits are held least significant first, each below
/// `RADIX`, and the most significant is never zero, so zero holds no digit and
/// each number has one spelling.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct Natural {
    digits: Vec<u32>,
}

impl Natural {
    pub(super) fn zero() -> Natural {
        Natural { digits: Vec::new() }
    }

    pub(super) fn one() -> Natural {
        Natural { digits: vec![1] }
    }

    /// The number whose digits these are, least significant first, each
    /// below `RADIX`.
    pub(super) fn of_digits(mut digits: Vec<u32>) -> Natural {
        while digits.last() == Some(&0) {
            digits.pop();
        }
        Natural { digits }
    }

    /// A count, as its digits in `RADIX`.
    pub(super) fn of(count: u64) -> Natural {
        let radix = u64::from(RADIX);
        let digits = std::iter::successors(Some(count), |rest| Some(rest / radix))
            .take_while(|rest| *rest > 0)
            .map(|rest| u32::try_from(rest % radix).expect("a digit is below the radix"))
            .collect();
        Natural::of_digits(digits)
    }

    /// The digits, least significant first.
    pub(super) fn digits(&self) -> &[u32] {
        &self.digits
    }

    pub(super) fn is_zero(&self) -> bool {
        self.digits.is_empty()
    }

    /// How many places it takes to write. Zero takes none.
    pub(super) fn places(&self) -> usize {
        self.digits.len()
    }

    fn at(&self, place: usize) -> u32 {
        self.digits.get(place).copied().unwrap_or(0)
    }

    pub(super) fn plus(&self, other: &Natural) -> Natural {
        let places = self.places().max(other.places());
        let (mut digits, carry) = (0..places).fold(
            (Vec::with_capacity(places + 1), 0),
            |(mut digits, carry), place| {
                let sum = self.at(place) + other.at(place) + carry;
                digits.push(sum % RADIX);
                (digits, sum / RADIX)
            },
        );
        digits.push(carry);
        Natural::of_digits(digits)
    }

    /// `self` less `other`, which is not greater than `self`.
    pub(super) fn minus(&self, other: &Natural) -> Natural {
        debug_assert!(
            *other <= *self,
            "a natural less a greater one is not natural"
        );
        let (digits, _) = (0..self.places()).fold(
            (Vec::with_capacity(self.places()), 0),
            |(mut digits, borrow), place| {
                let taken = other.at(place) + borrow;
                let held = self.at(place);
                if held >= taken {
                    digits.push(held - taken);
                    (digits, 0)
                } else {
                    digits.push(held + RADIX - taken);
                    (digits, 1)
                }
            },
        );
        Natural::of_digits(digits)
    }

    pub(super) fn times(&self, other: &Natural) -> Natural {
        if self.is_zero() || other.is_zero() {
            return Natural::zero();
        }
        let mut digits = vec![0; self.places() + other.places()];
        for (from, held) in self.digits.iter().enumerate() {
            let carry = other
                .digits
                .iter()
                .enumerate()
                .fold(0, |carry, (by, factor)| {
                    let sum = digits[from + by] + held * factor + carry;
                    digits[from + by] = sum % RADIX;
                    sum / RADIX
                });
            digits[from + other.places()] = carry;
        }
        Natural::of_digits(digits)
    }

    /// `self` times `RADIX` to the power `places`.
    pub(super) fn shifted(&self, places: usize) -> Natural {
        if self.is_zero() {
            return Natural::zero();
        }
        let digits = std::iter::repeat_n(0, places)
            .chain(self.digits.iter().copied())
            .collect();
        Natural { digits }
    }

    /// `self` to the power `exponent`, as the product of that many copies.
    pub(super) fn power(&self, exponent: u64) -> Natural {
        (0..exponent).fold(Natural::one(), |product, _| product.times(self))
    }

    /// The quotient and the remainder of `self` by a divisor that is not
    /// zero, by long division, one place at a time.
    pub(super) fn divided(&self, divisor: &Natural) -> (Natural, Natural) {
        debug_assert!(!divisor.is_zero(), "no number is divided by zero");
        let (mut quotient, remainder) = self.digits.iter().rev().fold(
            (Vec::with_capacity(self.places()), Natural::zero()),
            |(mut quotient, remainder), digit| {
                let mut remainder = remainder.shifted(1).plus(&Natural::of(u64::from(*digit)));
                let mut place = 0;
                while remainder >= *divisor {
                    remainder = remainder.minus(divisor);
                    place += 1;
                }
                quotient.push(place);
                (quotient, remainder)
            },
        );
        quotient.reverse();
        (Natural::of_digits(quotient), remainder)
    }

    /// The greatest common divisor, by Euclid's algorithm. That of zero and a
    /// number is the number.
    pub(super) fn common_divisor(&self, other: &Natural) -> Natural {
        let (mut larger, mut smaller) = (self.clone(), other.clone());
        while !smaller.is_zero() {
            let (_, remainder) = larger.divided(&smaller);
            larger = smaller;
            smaller = remainder;
        }
        larger
    }

    /// The greatest natural whose `index`-th power is not above `self`, for an
    /// index that is not zero.
    ///
    /// Newton's iteration from above: each step from a guess above the root
    /// lands lower but never below it, so the first step that does not go
    /// lower stands on it.
    pub(super) fn root_floor(&self, index: u64) -> Natural {
        debug_assert!(index > 0, "no root has index zero");
        if self.is_zero() || index == 1 {
            return self.clone();
        }
        let below_index = Natural::of(index - 1);
        let index_number = Natural::of(index);
        let places = usize::try_from(index)
            .map(|index| self.places().div_ceil(index))
            .unwrap_or(1);
        let mut guess = Natural::one().shifted(places);
        loop {
            let (share, _) = self.divided(&guess.power(index - 1));
            let (next, _) = below_index
                .times(&guess)
                .plus(&share)
                .divided(&index_number);
            if next >= guess {
                return guess;
            }
            guess = next;
        }
    }
}

impl Ord for Natural {
    fn cmp(&self, other: &Natural) -> Ordering {
        self.places()
            .cmp(&other.places())
            .then_with(|| self.digits.iter().rev().cmp(other.digits.iter().rev()))
    }
}

impl PartialOrd for Natural {
    fn partial_cmp(&self, other: &Natural) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
