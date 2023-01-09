use std::{
    convert::{TryFrom, TryInto},
    ops::{Neg, Rem},
    str::FromStr,
};

use cosmwasm_std::{Decimal256, Uint256};
use num_traits::{Num, One, Zero};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::CommonError;

/// Uint256 with a sign
#[derive(Serialize, Deserialize, Clone, Copy, Debug, JsonSchema)]
pub struct SignedInt {
    pub value: Uint256,
    pub is_positive: bool,
}

impl SignedInt {
    pub const fn nan() -> Self {
        Self {
            value: Uint256::zero(),
            is_positive: false,
        }
    }

    pub const fn is_nan(&self) -> bool {
        self.value.is_zero() && !self.is_positive
    }

    pub fn value(&self) -> Uint256 {
        assert!(self.is_positive, "SignedInt is negative!");
        self.value
    }
}

impl Neg for SignedInt {
    type Output = Self;

    fn neg(self) -> Self::Output {
        if self.is_zero() {
            return self;
        }
        Self {
            value: self.value,
            is_positive: !self.is_positive,
        }
    }
}

impl Rem for SignedInt {
    type Output = Self;

    fn rem(self, _rhs: Self) -> Self::Output {
        todo!()
    }
}

impl One for SignedInt {
    fn one() -> Self {
        Self {
            value: Uint256::from_u128(1u128),
            is_positive: true,
        }
    }
}

impl Zero for SignedInt {
    fn zero() -> Self {
        Self {
            value: Uint256::zero(),
            is_positive: true,
        }
    }

    fn is_zero(&self) -> bool {
        self.value.is_zero()
    }
}

impl Num for SignedInt {
    type FromStrRadixErr = Self;

    fn from_str_radix(_str: &str, _radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        panic!("unimplemented")
    }
}

impl num_traits::sign::Signed for SignedInt {
    fn abs(&self) -> Self {
        Self {
            value: self.value,
            is_positive: true,
        }
    }

    fn abs_sub(&self, other: &Self) -> Self {
        let new = *self - *other;
        new.abs()
    }

    fn signum(&self) -> Self {
        todo!()
    }

    fn is_positive(&self) -> bool {
        todo!()
    }

    fn is_negative(&self) -> bool {
        todo!()
    }
}

impl ToString for SignedInt {
    fn to_string(&self) -> String {
        if self.is_nan() {
            String::from("NaN")
        } else {
            let sign_str = if self.is_positive { "" } else { "-" }.to_owned();
            sign_str + self.value.to_string().as_str()
        }
    }
}

impl std::ops::Add<Self> for SignedInt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let value;
        let is_positive;
        if self.is_positive == rhs.is_positive {
            value = self.value + rhs.value;
            is_positive = self.is_positive;
        } else if self.value > rhs.value {
            value = self.value - rhs.value;
            is_positive = self.is_positive;
        } else if self.value < rhs.value {
            value = rhs.value - self.value;
            is_positive = rhs.is_positive
        } else {
            value = Uint256::zero();
            is_positive = true;
        }
        Self { is_positive, value }
    }
}

impl std::ops::Add<SignedInt> for Uint256 {
    type Output = SignedInt;

    fn add(self, rhs: SignedInt) -> SignedInt {
        let signed_int: SignedInt = self.into();
        signed_int + rhs
    }
}

impl std::ops::Sub<Self> for SignedInt {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self + Self {
            value: rhs.value,
            is_positive: !rhs.is_positive,
        }
    }
}

impl std::ops::Mul<Self> for SignedInt {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let value = self.value * rhs.value;
        Self {
            value,
            is_positive: self.is_positive == rhs.is_positive || value.is_zero(),
        }
    }
}

impl std::ops::Mul<Decimal256> for SignedInt {
    type Output = Self;

    fn mul(self, rhs: Decimal256) -> Self {
        let value = self.value * rhs;
        Self {
            value,
            is_positive: self.is_positive || value.is_zero(),
        }
    }
}

impl std::ops::Div<Self> for SignedInt {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        let value = if rhs.value.is_zero() {
            rhs.value
        } else {
            self.value / rhs.value
        };
        Self {
            value,
            is_positive: self.is_positive == rhs.is_positive || value.is_zero(),
        }
    }
}

impl std::cmp::PartialEq for SignedInt {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.is_positive == other.is_positive
    }
}

impl std::cmp::PartialOrd for SignedInt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.is_positive == other.is_positive {
            if self.is_positive {
                self.value.partial_cmp(&other.value)
            } else {
                other.value.partial_cmp(&self.value)
            }
        } else if self.is_positive {
            Some(std::cmp::Ordering::Greater)
        } else {
            Some(std::cmp::Ordering::Less)
        }
    }
}

impl From<Uint256> for SignedInt {
    fn from(value: Uint256) -> Self {
        Self {
            value,
            is_positive: true,
        }
    }
}

impl FromStr for SignedInt {
    type Err = CommonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sign;
        let val_str;
        let mut chars = s.chars();
        if chars.next().unwrap() == '-' {
            sign = false;
            val_str = chars.as_str();
        } else {
            sign = true;
            val_str = s;
        }
        Ok(Self {
            value: Uint256::from_str(val_str)?,
            is_positive: sign,
        })
    }
}

impl TryFrom<&str> for SignedInt {
    type Error = CommonError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryInto<Uint256> for SignedInt {
    type Error = CommonError;

    fn try_into(self) -> Result<Uint256, Self::Error> {
        if !self.is_positive && !self.value.is_zero() {
            return Err(CommonError::Generic(
                "Cannot convert negative SignedInt to Uint256".into(),
            ));
        }
        Ok(self.value)
    }
}

impl Default for SignedInt {
    fn default() -> Self {
        Self {
            value: Uint256::default(),
            is_positive: true,
        }
    }
}

#[test]
fn signed_int_test() {
    let big_pos = SignedInt::from_str("100").unwrap();
    let big_neg = SignedInt::from_str("-100").unwrap();
    let small_pos = SignedInt::from_str("50").unwrap();
    let small_neg = SignedInt::from_str("-50").unwrap();
    let dec_neg = SignedInt::from_str("-50").unwrap();

    let big_pos_f64 = f64::from_str("100").unwrap();
    let big_neg_f64 = f64::from_str("-100").unwrap();
    let small_pos_f64 = f64::from_str("50").unwrap();
    let small_neg_f64 = f64::from_str("-50").unwrap();
    let dec_neg_f64 = f64::from_str("-50").unwrap();

    // Test partial_cmp
    assert!(big_pos > big_neg);
    assert!(big_pos > small_neg);
    assert!(big_pos > small_pos);
    assert!(big_pos > big_neg);
    assert!(small_pos > small_neg);
    assert!(small_pos > big_neg);
    assert!(small_neg > big_neg);

    // Utility function
    fn f64_to_signed_int(val: f64) -> SignedInt {
        let string = val.to_string();
        println!("val is {string}");
        SignedInt::from_str(string.as_str()).unwrap()
    }

    // Test mul
    assert!(big_pos * small_pos == f64_to_signed_int(big_pos_f64 * small_pos_f64));
    assert!(big_pos * small_neg == f64_to_signed_int(big_pos_f64 * small_neg_f64));
    assert!(big_pos * big_neg == f64_to_signed_int(big_pos_f64 * big_neg_f64));
    assert!(small_pos * small_neg == f64_to_signed_int(small_pos_f64 * small_neg_f64));
    assert!(small_pos * big_neg == f64_to_signed_int(small_pos_f64 * big_neg_f64));
    assert!(small_neg * big_neg == f64_to_signed_int(small_neg_f64 * big_neg_f64));

    // Test div
    assert!(big_pos / small_pos == f64_to_signed_int(big_pos_f64 / small_pos_f64));
    assert!(big_pos / small_neg == f64_to_signed_int(big_pos_f64 / small_neg_f64));
    assert!(big_pos / big_neg == f64_to_signed_int(big_pos_f64 / big_neg_f64));

    // Test add
    assert!(big_pos + small_pos == f64_to_signed_int(big_pos_f64 + small_pos_f64));
    assert!(big_pos + small_neg == f64_to_signed_int(big_pos_f64 + small_neg_f64));
    assert!(big_pos + big_neg == f64_to_signed_int(big_pos_f64 + big_neg_f64));
    assert!(small_pos + small_neg == f64_to_signed_int(small_pos_f64 + small_neg_f64));
    assert!(small_pos + big_neg == f64_to_signed_int(small_pos_f64 + big_neg_f64));
    assert!(small_neg + big_neg == f64_to_signed_int(small_neg_f64 + big_neg_f64));

    // Test sub
    assert!(big_pos - small_pos == f64_to_signed_int(big_pos_f64 - small_pos_f64));
    assert!(big_pos - small_neg == f64_to_signed_int(big_pos_f64 - small_neg_f64));
    assert!(big_pos - big_neg == f64_to_signed_int(big_pos_f64 - big_neg_f64));
    assert!(small_pos - small_neg == f64_to_signed_int(small_pos_f64 - small_neg_f64));
    assert!(small_pos - big_neg == f64_to_signed_int(small_pos_f64 - big_neg_f64));
    assert!(small_neg - big_neg == f64_to_signed_int(small_neg_f64 - big_neg_f64));

    // Test conversion
    assert!(big_pos == f64_to_signed_int(big_pos_f64));
    assert!(big_neg == f64_to_signed_int(big_neg_f64));
    assert!(small_pos == f64_to_signed_int(small_pos_f64));
    assert!(small_neg == f64_to_signed_int(small_neg_f64));
    assert!(dec_neg == f64_to_signed_int(dec_neg_f64));
}

#[test]
fn test_zero_is_positive() {
    {
        let mut x = SignedInt::zero();
        let y = SignedInt::one().neg();

        x = x * y;
        assert!(x.is_positive);

        x = y * x;
        assert!(x.is_positive);

        x = x / y;
        assert!(x.is_positive);

        x = x + y;
        x = x - y;
        assert!(x.is_positive);

        x = x - y;
        x = x + y;
        assert!(x.is_positive);
    }
    {
        let x = SignedInt::one() * SignedInt::from_str("5").unwrap();
        let y = SignedInt::one() * SignedInt::from_str("-5").unwrap();

        let z = x + y;
        assert!(z.is_positive);

        let z = -x - y;
        assert!(z.is_positive);
    }
    {
        let x = -SignedInt::zero();
        assert!(x.is_positive);
    }
    {
        let x = SignedInt::zero().neg();
        assert!(x.is_positive);
    }
    {
        let x = SignedInt::zero().neg();
        let y = SignedInt::from_str("5").unwrap();

        let z = x * y;
        assert!(z.is_positive);

        let z = y * x;
        assert!(z.is_positive);
    }
}
