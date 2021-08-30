use std::convert::TryInto;

use crate::*;

pub const XUSD_SCALE: u8 = 6;
pub const SNY_SCALE: u8 = 6;
pub const PRICE_SCALE: u8 = 8;
pub const UNIFIED_PERCENT_SCALE: u8 = 5;
pub const INTEREST_RATE_SCALE: u8 = 18;

impl Decimal {
    pub fn new(value: u128, scale: u8) -> Self {
        Self { val: value, scale }
    }
    pub fn denominator(self) -> u128 {
        10u128.pow(self.scale.into())
    }
    pub fn from_unified_percent(percent: u16) -> Self {
        Decimal {
            val: percent.into(),
            scale: UNIFIED_PERCENT_SCALE,
        }
    }
    pub fn from_percent(percent: u16) -> Self {
        Decimal::new(percent.into(), 2).to_percent()
    }
    pub fn from_integer(integer: u64) -> Self {
        Decimal {
            val: integer.into(),
            scale: 0,
        }
    }
    pub fn from_price(price: u128) -> Self {
        Decimal {
            val: price,
            scale: PRICE_SCALE,
        }
    }
    pub fn from_usd(value: u128) -> Self {
        Decimal {
            val: value.into(),
            scale: XUSD_SCALE,
        }
    }
    pub fn from_sny(value: u128) -> Self {
        Decimal {
            val: value,
            scale: SNY_SCALE,
        }
    }
    pub fn from_interest_rate(value: u128) -> Self {
        Decimal {
            val: value,
            scale: INTEREST_RATE_SCALE,
        }
    }
    pub fn to_usd(self) -> Decimal {
        self.to_scale(XUSD_SCALE)
    }
    pub fn to_usd_up(self) -> Decimal {
        self.to_scale_up(XUSD_SCALE)
    }
    pub fn to_sny(self) -> Decimal {
        self.to_scale(XUSD_SCALE)
    }
    pub fn to_price(self) -> Decimal {
        self.to_scale(PRICE_SCALE)
    }
    pub fn to_u64(self) -> u64 {
        self.val.try_into().unwrap()
    }
    pub fn to_interest_rate(self) -> Self {
        self.to_scale(INTEREST_RATE_SCALE)
    }
    pub fn to_percent(self) -> Self {
        self.to_scale(UNIFIED_PERCENT_SCALE)
    }
    pub fn to_scale(self, scale: u8) -> Self {
        Self {
            val: if self.scale > scale {
                self.val
                    .checked_div(10u128.pow((self.scale - scale).into()))
                    .unwrap()
            } else {
                self.val
                    .checked_mul(10u128.pow((scale - self.scale).into()))
                    .unwrap()
            },
            scale,
        }
    }
    pub fn to_scale_up(self, scale: u8) -> Self {
        let decimal = Self::new(self.val, scale);
        if self.scale >= scale {
            decimal.div_up(Self::new(
                10u128.pow((self.scale - scale).try_into().unwrap()),
                0,
            ))
        } else {
            decimal.mul_up(Self::new(
                10u128.pow((scale - self.scale).try_into().unwrap()),
                0,
            ))
        }
    }
}

impl Mul<Decimal> for Decimal {
    fn mul(self, value: Decimal) -> Self {
        Self {
            val: self
                .val
                .checked_mul(value.val)
                .unwrap()
                .checked_div(value.denominator())
                .unwrap(),
            scale: self.scale,
        }
    }
}
impl Mul<u128> for Decimal {
    fn mul(self, value: u128) -> Self {
        Self {
            val: self.val.checked_mul(value).unwrap(),
            scale: self.scale,
        }
    }
}
impl MulUp<Decimal> for Decimal {
    fn mul_up(self, other: Decimal) -> Self {
        let denominator = other.denominator();

        Self {
            val: self
                .val
                .checked_mul(other.val)
                .unwrap()
                .checked_add(denominator.checked_sub(1).unwrap())
                .unwrap()
                .checked_div(denominator)
                .unwrap(),
            scale: self.scale,
        }
    }
}
impl Add<Decimal> for Decimal {
    fn add(self, value: Decimal) -> Result<Self> {
        require!(self.scale == value.scale, DifferentScale);

        Ok(Self {
            val: self.val.checked_add(value.val).unwrap(),
            scale: self.scale,
        })
    }
}
impl Sub<Decimal> for Decimal {
    fn sub(self, value: Decimal) -> Result<Self> {
        require!(self.scale == value.scale, DifferentScale);
        Ok(Self {
            val: self.val.checked_sub(value.val).unwrap(),
            scale: self.scale,
        })
    }
}
impl Div<Decimal> for Decimal {
    fn div(self, other: Decimal) -> Self {
        Self {
            val: self
                .val
                .checked_mul(other.denominator())
                .unwrap()
                .checked_div(other.val)
                .unwrap(),
            scale: self.scale,
        }
    }
}
impl DivUp<Decimal> for Decimal {
    fn div_up(self, other: Decimal) -> Self {
        Self {
            val: self
                .val
                .checked_mul(other.denominator())
                .unwrap()
                .checked_add(other.val.checked_sub(1).unwrap())
                .unwrap()
                .checked_div(other.val)
                .unwrap(),
            scale: self.scale,
        }
    }
}
impl DivScale<Decimal> for Decimal {
    fn div_to_scale(self, other: Decimal, to_scale: u8) -> Self {
        let decimal_difference = self.scale as i32 - to_scale as i32 - other.scale as i32;

        let val = if decimal_difference > 0 {
            self.val
                .checked_div(other.val)
                .unwrap()
                .checked_div(10u128.pow(decimal_difference.try_into().unwrap()))
                .unwrap()
        } else {
            self.val
                .checked_mul(10u128.pow((-decimal_difference).try_into().unwrap()))
                .unwrap()
                .checked_div(other.val)
                .unwrap()
        };
        Self {
            val,
            scale: to_scale,
        }
    }
}
impl PowAccuracy<u128> for Decimal {
    fn pow_with_accuracy(self, exp: u128) -> Self {
        let one = Decimal {
            val: 1 * self.denominator(),
            scale: self.scale,
        };
        if exp == 0 {
            return one;
        }
        let mut current_exp = exp;
        let mut base = self;
        let mut result = one;

        while current_exp > 0 {
            if current_exp % 2 != 0 {
                result = result.mul(base);
            }
            current_exp /= 2;
            base = base.mul(base);
        }
        return result;
    }
}
impl Into<u64> for Decimal {
    fn into(self) -> u64 {
        self.val.try_into().unwrap()
    }
}
impl Into<u128> for Decimal {
    fn into(self) -> u128 {
        self.val.try_into().unwrap()
    }
}
impl Compare<Decimal> for Decimal {
    fn lte(self, other: Decimal) -> Result<bool> {
        require!(self.scale == other.scale, DifferentScale);
        Ok(self.val <= other.val)
    }
    fn lt(self, other: Decimal) -> Result<bool> {
        require!(self.scale == other.scale, DifferentScale);
        Ok(self.val < other.val)
    }
    fn gt(self, other: Decimal) -> Result<bool> {
        require!(self.scale == other.scale, DifferentScale);
        Ok(self.val > other.val)
    }
    fn gte(self, other: Decimal) -> Result<bool> {
        require!(self.scale == other.scale, DifferentScale);
        Ok(self.val >= other.val)
    }
    fn eq(self, other: Decimal) -> Result<bool> {
        require!(self.scale == other.scale, DifferentScale);
        Ok(self.val == other.val)
    }
}
pub trait Sub<T>: Sized {
    fn sub(self, rhs: T) -> Result<Self>;
}
pub trait Add<T>: Sized {
    fn add(self, rhs: T) -> Result<Self>;
}
pub trait Div<T>: Sized {
    fn div(self, rhs: T) -> Self;
}
pub trait DivScale<T> {
    fn div_to_scale(self, rhs: T, to_scale: u8) -> Self;
}
pub trait DivUp<T>: Sized {
    fn div_up(self, rhs: T) -> Self;
}
pub trait Mul<T>: Sized {
    fn mul(self, rhs: T) -> Self;
}
pub trait MulUp<T>: Sized {
    fn mul_up(self, rhs: T) -> Self;
}
pub trait PowAccuracy<T>: Sized {
    fn pow_with_accuracy(self, rhs: T) -> Self;
}
pub trait Compare<T>: Sized {
    fn eq(self, rhs: T) -> Result<bool>;
    fn lt(self, rhs: T) -> Result<bool>;
    fn gt(self, rhs: T) -> Result<bool>;
    fn gte(self, rhs: T) -> Result<bool>;
    fn lte(self, rhs: T) -> Result<bool>;
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_to_scale() {
        // Increasing precision
        {
            let decimal = Decimal::new(42, 2);
            let result = decimal.to_scale(3);

            assert_eq!(result.scale, 3);
            assert_eq!({ result.val }, 420);
        }
        // Decreasing precision
        {
            let decimal = Decimal::new(42, 2);
            let result = decimal.to_scale(1);

            assert_eq!(result.scale, 1);
            assert_eq!({ result.val }, 4);
        }
        // Decreasing precision over value
        {
            let decimal = Decimal::new(123, 4);
            let result = decimal.to_scale(0);

            assert_eq!(result.scale, 0);
            assert_eq!({ result.val }, 0);
        }
    }

    #[test]
    fn test_to_scale_up() {
        // Increasing precision
        {
            let decimal = Decimal::new(42, 2);
            let result = decimal.to_scale_up(3);

            assert_eq!(result.scale, 3);
            assert_eq!({ result.val }, 420);
        }
        // Decreasing precision
        {
            let decimal = Decimal::new(42, 2);
            let result = decimal.to_scale_up(1);

            assert_eq!(result.scale, 1);
            assert_eq!({ result.val }, 5);
        }
        // Decreasing precision over value
        {
            let decimal = Decimal::new(123, 4);
            let result = decimal.to_scale_up(0);

            assert_eq!(result.scale, 0);
            assert_eq!({ result.val }, 1);
        }
    }

    #[test]
    fn test_pow_with_accuracy() {
        // Zero base
        {
            let decimal: u8 = PRICE_SCALE;
            let base = Decimal::new(0, decimal);
            let exp: u128 = 100;
            let result = base.pow_with_accuracy(exp);
            let expected = Decimal::new(0, decimal);
            assert_eq!(result, expected);
        }
        // Zero exponent

        let decimal: u8 = PRICE_SCALE;
        let base = Decimal::from_integer(10).to_scale(decimal);
        let exp: u128 = 0;
        let result = base.pow_with_accuracy(exp);
        let expected = Decimal::from_integer(1).to_scale(decimal);
        assert_eq!(result, expected);
        // 2^17, with price decimal
        {
            let decimal: u8 = PRICE_SCALE;
            let base = Decimal::from_integer(2).to_scale(decimal);
            let exp: u128 = 17;
            let result = base.pow_with_accuracy(exp);
            // should be 131072
            let expected = Decimal::from_integer(131072).to_scale(decimal);
            assert_eq!(result, expected);
        }
        // 1.00000002^525600, with interest decimal
        {
            let base = Decimal::new(1_000_000_02, 8).to_interest_rate();
            let exp: u128 = 525600;
            let result = base.pow_with_accuracy(exp);
            // expected 1.010567445075371...
            // real     1.010567445075377...
            let expected = Decimal::from_interest_rate(1010567445075371366);
            assert_eq!(result, expected);
        }
        // 1.000000015^2, with interest decimal
        {
            let base = Decimal::new(1_000_000_015, 9).to_interest_rate();
            let exp: u128 = 2;
            let result = base.pow_with_accuracy(exp);
            // expected 1.000000030000000225
            // real     1.000000030000000225.
            let expected = Decimal::from_interest_rate(1000000030000000225);
            assert_eq!(result, expected);
        }
        // 1^525600, with interest decimal
        {
            let base = Decimal::from_integer(1).to_interest_rate();
            let exp: u128 = 525600;
            let result = base.pow_with_accuracy(exp);
            // expected not change value
            let expected = Decimal::from_integer(1).to_interest_rate();
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_mul_up() {
        // mul of little
        {
            let a = Decimal::new(1, 10);
            let b = Decimal::new(1, 10);
            assert_eq!(a.mul_up(b), Decimal::new(1, 10));
        }
        // mul calculable without precision loss
        {
            let a = Decimal::new(1000, 3);
            let b = Decimal::new(300, 3);
            assert_eq!(a.mul_up(b), Decimal::new(300, 3));
        }
        // mul by zero
        {
            let a = Decimal::new(1000, 3);
            let b = Decimal::new(0, 0);
            assert_eq!(a.mul_up(b), Decimal::new(0, 3));
        }
        // mul with different decimals
        {
            let a = Decimal::new(1_000_000_000, 9);
            let b = Decimal::new(3, 8);
            assert_eq!(a.mul_up(b), Decimal::new(30, 9));
        }
    }

    #[test]
    fn test_div_up() {
        // div of zero
        {
            let a = Decimal::new(0, 0);
            let b = Decimal::new(1, 0);
            assert_eq!(a.div_up(b), Decimal::new(0, 0));
        }
        // div check rounding up
        {
            let a = Decimal::new(1, 0);
            let b = Decimal::new(2, 0);
            assert_eq!(a.div_up(b), Decimal::new(1, 0));
        }
        // div big number
        {
            let a = Decimal::new(200_000_000_001, 6);
            let b = Decimal::new(2_000, 3);
            assert!(!a.div_up(b).lt(Decimal::new(100_000_000_001, 6)).unwrap());
        }
        {
            let a = Decimal::new(42, 2);
            let b = Decimal::new(10, 0);
            assert_eq!(a.div_up(b), Decimal::new(5, 2));
        }
    }

    #[test]
    fn test_div_to_scale() {
        // nominator scale == denominator scale
        {
            let nominator = Decimal::new(20_000, 8);
            let denominator = Decimal::new(4, 8);

            // to_scale == scale
            let to_scale = 8;
            let result = nominator.div_to_scale(denominator, to_scale);
            let expected = Decimal::from_integer(5_000).to_scale(to_scale);
            assert_eq!(result, expected);

            // // to_scale > scale
            let to_scale = 11;
            let result = nominator.div_to_scale(denominator, to_scale);
            let expected = Decimal::from_integer(5_000).to_scale(to_scale);
            assert_eq!(result, expected);

            // // to_scale < scale
            let to_scale = 5;
            let result = nominator.div_to_scale(denominator, to_scale);
            let expected = Decimal::from_integer(5_000).to_scale(to_scale);
            assert_eq!(result, expected);
        }
        // nominator scale != denominator scale
        {
            let nominator = Decimal::new(35, 5);
            let denominator = Decimal::new(5, 1);

            // to_scale == nominator scale
            let to_scale = 7;
            let result = nominator.div_to_scale(denominator, to_scale);
            let expected = Decimal::new(7000, to_scale);
            assert_eq!(result, expected);

            // to_scale > nominator scale
            let to_scale = 9;
            let result = nominator.div_to_scale(denominator, to_scale);
            let expected = Decimal::new(700_000, to_scale);
            assert_eq!(result, expected);

            // to_scale < nominator scale
            let to_scale = 5;
            let result = nominator.div_to_scale(denominator, to_scale);
            let expected = Decimal::new(70, to_scale);
            assert_eq!(result, expected);
        }
    }
}
