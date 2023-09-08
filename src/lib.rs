use std::fmt;

pub mod arithmos {
    pub mod addition;
    pub mod multiplication;
    pub mod power;
    pub mod substraction;
}

pub(crate) type Digit = u64;
pub(crate) type DoubleDigit = u128;

pub(crate) const BITS: u32 = Digit::BITS;

#[derive(Clone)]
pub struct Number {
    data: Vec<Digit>,
}

impl Number {
    pub fn new(input: &str) -> Number {
        let input_size = input.chars().count();
        let digit_count = std::mem::size_of::<Digit>() * 2;
        let capacity = (input_size + digit_count - 1) / digit_count;
        let mut result_data = Vec::with_capacity(capacity);

        for index in (0..=input_size).rev().step_by(digit_count) {
            if index >= digit_count {
                result_data
                    .push(Digit::from_str_radix(&input[index - digit_count..index], 16).unwrap());
            } else if input_size % digit_count != 0 {
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
