use std::ops::Mul;

use crate::{Digit, DoubleDigit, Number, BITS};

use super::addition::adc;

impl Mul<&Number> for &Number {
    type Output = Number;

    fn mul(self, rhs: &Number) -> Number {
        let limbs = self.data.len() + rhs.data.len();
        let mut result_data = vec![0; limbs];

        for (i, &digit1) in self.data.iter().enumerate() {
            let mut mul_acc = 0;
            let mut sum_carry = 0;
            let mut index = i;

            for &digit2 in rhs.data.iter() {
                let (mul_result, new_mul_acc) = muladd(digit1, digit2, mul_acc);
                mul_acc = new_mul_acc;

                let (sum_result, new_sum_carry) = adc(mul_result, result_data[index], sum_carry);
                result_data[index] = sum_result;
                sum_carry = new_sum_carry;

                index += 1;
            }
            result_data[index] = sum_carry + mul_acc;
        }
        Number { data: result_data }
    }
}

impl Mul<&Number> for Number {
    type Output = Number;

    fn mul(self, rhs: &Number) -> Number {
        &self * rhs
    }
}

impl Mul<Number> for &Number {
    type Output = Number;

    fn mul(self, rhs: Number) -> Number {
        self * &rhs
    }
}

impl Mul for Number {
    type Output = Number;

    fn mul(self, rhs: Number) -> Number {
        &self * &rhs
    }
}

#[inline]
pub(crate) fn muladd(digit1: Digit, digit2: Digit, acc: Digit) -> (Digit, Digit) {
    let result: DoubleDigit =
        DoubleDigit::from(digit1) * DoubleDigit::from(digit2) + DoubleDigit::from(acc);

    (result as Digit, (result >> BITS) as Digit)
}

#[test]
fn test_mul() {
    let a = Number::new("FF234567987654234567BC345679876AA");
    let b = Number::new("23456789234567890987654323456789ABCD");
    assert_eq!(
        format!("{:X}", &a * &b),
        "2326FE2EBCB8165C563C61A18818655E7A4B57398825828C0FDB73E2CBCEEB3549422"
    );
}
