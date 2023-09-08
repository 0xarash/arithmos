use crate::{Digit, Number};

impl Number {
    pub fn pow(self, exp: Digit) -> Number {
        let mut result = Number::new("1");
        let mut base = self.clone();
        let mut e = exp;

        loop {
            if e & 1 == 1 {
                result = &result * &base;
            }

            e >>= 1;

            if e == 0 {
                break;
            }
            base = &base * &base;
        }
        result
    }
}

#[test]
fn test_pow() {
    let a = Number::new("D80");
    assert_eq!(
        format!("{:X}", a.pow(23)),
        "5246D7C54A60153714AEA69ACFE60000000000000000000000000000000000000000"
    );
}
