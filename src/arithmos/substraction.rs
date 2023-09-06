use std::ops::Sub;

use crate::Number;

use super::addition::adc;

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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub() {
        let a = Number::new("23456789234567890987654323456789ABCD");
        let b = Number::new("FF234567987654234567BC345679876AA");
        assert_eq!(
            format!("{:X}", a - b),
            "23357554CCCBE023C7530EC75FFFFFF13523"
        );
    }
}