use std::fmt;
use std::ops::{Add, Mul, Sub};

mod arithmetic;
use arithmetic::{adc, muladd};

pub(crate) type Digit = u64;
pub(crate) type DoubleDigit = u128;

pub(crate) const BITS: u32 = Digit::BITS;

pub(crate) const DIGITS: usize = std::mem::size_of::<Digit>() * 2;

pub struct Number {
    data: Vec<Digit>,
}

impl Number {
    pub fn new(input: &str) -> Number {
        let input_size = input.chars().count();
        let capacity = (input_size + DIGITS - 1) / DIGITS;
        let mut result_data = Vec::with_capacity(capacity);

        for index in (0..=input_size).rev().step_by(DIGITS) {
            if index >= DIGITS {
                result_data.push(Digit::from_str_radix(&input[index - DIGITS..index], 16).unwrap());
            } else if input_size % DIGITS != 0 {
                result_data.push(Digit::from_str_radix(&input[0..index], 16).unwrap());
            }
        }
        Number { data: result_data }
    }
}

impl Add for Number {
    type Output = Number;

    fn add(self, rhs: Number) -> Number {
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

impl Sub for Number {
    type Output = Number;

    fn sub(self, rhs: Number) -> Number {
        let max_length = std::cmp::max(self.data.len(), rhs.data.len());
        let mut result_data = Vec::with_capacity(max_length);
        let mut carry = 1;

        for index in 0..max_length {
            let digit1 = *self.data.get(index).unwrap_or(&0);
            let digit2 = *rhs.data.get(index).unwrap_or(&0);
            let (sum, new_carry) = adc(digit1, !digit2, carry);

            result_data.push(sum);
            carry = new_carry;
        }

        Number { data: result_data }
    }
}

impl Mul for Number {
    type Output = Number;

    fn mul(self, rhs: Number) -> Number {
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

impl fmt::UpperHex for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: Vec<String> = self
            .data
            .iter()
            .rev()
            .map(|&digit| format!("{:016X}", digit))
            .collect();
        let output = s.join("");
        write!(f, "{}", output.trim_start_matches('0'))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_sub() {
        let a = Number::new("23456789234567890987654323456789ABCD");
        let b = Number::new("FF234567987654234567BC345679876AA");
        assert_eq!(
            format!("{:X}", a - b),
            "23357554CCCBE023C7530EC75FFFFFF13523"
        );
    }

    #[test]
    fn test_mul() {
        let a = Number::new("FF234567987654234567BC345679876AA");
        let b = Number::new("23456789234567890987654323456789ABCD");
        assert_eq!(
            format!("{:X}", a * b),
            "2326FE2EBCB8165C563C61A18818655E7A4B57398825828C0FDB73E2CBCEEB3549422"
        );
    }
}
