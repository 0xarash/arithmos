use std::fmt;

pub mod arithmos {
    pub mod addition;
    pub mod multiplication;
    pub mod substraction;
}

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