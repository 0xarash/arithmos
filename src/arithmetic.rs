use crate::{Digit, DoubleDigit, BITS};

#[inline]
pub(crate) fn adc(digit1: Digit, digit2: Digit, carry: Digit) -> (Digit, Digit) {
    let sum: Digit = digit1.wrapping_add(digit2).wrapping_add(carry);
    let carry: Digit = ((digit1 & digit2) | ((digit1 | digit2) & !sum)) >> (BITS - 1);

    (sum, carry)
}

#[inline]
pub(crate) fn muladd(digit1: Digit, digit2: Digit, acc: Digit) -> (Digit, Digit) {
    let result: DoubleDigit =
        DoubleDigit::from(digit1) * DoubleDigit::from(digit2) + DoubleDigit::from(acc);

    (result as Digit, (result >> BITS) as Digit)
}
