use std::ops::Add;

use crate::{Digit, Number, BITS};

impl Add<&Number> for &Number {
    type Output = Number;

    fn add(self, rhs: &Number) -> Number {
        let max_length = std::cmp::max(self.data.len(), rhs.data.len());
        let mut result_data = Vec::with_capacity(max_length);
        let mut carry = 0;

        for index in 0..max_length {
            let digit1 = *self.data.get(index).unwrap_or(&0);
            let digit2 = *rhs.data.get(index).unwrap_or(&0);
            let (sum, new_carry) = adc(digit1, digit2, carry);

            result_data.push(sum);
            carry = new_carry;
        }

        if carry != 0 {
            result_data.push(carry);
        }

        Number { data: result_data }
    }
}

impl Add<&Number> for Number {
    type Output = Number;

    fn add(self, rhs: &Number) -> Number {
        &self + rhs
    }
}

impl Add<Number> for &Number {
    type Output = Number;

    fn add(self, rhs: Number) -> Number {
        self + &rhs
    }
}

impl Add for Number {
    type Output = Number;

    fn add(self, rhs: Number) -> Number {
        &self + &rhs
    }
}

#[inline]
pub(crate) fn adc(digit1: Digit, digit2: Digit, carry: Digit) -> (Digit, Digit) {
    let sum: Digit = digit1.wrapping_add(digit2).wrapping_add(carry);
    let carry: Digit = ((digit1 & digit2) | ((digit1 | digit2) & !sum)) >> (BITS - 1);

    (sum, carry)
}

#[test]
fn test_add() {
    let a = Number::new("FF234567987654234567BC345679876AA");
    let b = Number::new("23456789234567890987654323456789ABCD");
    let c = Number::new("6E4B15718F0F3971ADF43CF99AFC4A527");
    assert_eq!(
        format!("{:X}", a + b + c),
        "235C3E6ED0D7DFE1E2D69B02B6247EE6C79E"
    );
}
