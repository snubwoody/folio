// Copyright (C) 2025 Wakunguma Kalimukwa
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::{Database, Decode, Encode, Sqlite, Type, sqlite::SqliteTypeInfo};
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Sub, SubAssign},
    str::FromStr,
};

// TODO: impl FromRow?
/// A type that stores money with 6 digits after the decimal point.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Money(i64);

impl Money {
    /// The number of digits after the decimal.
    pub const SCALE: u32 = 6;
    /// The multiplication factor for scaling values.
    const FACTOR: u32 = 10u32.pow(Self::SCALE);

    pub const ZERO: Money = Money::new(0);
    pub const MAX: Money = Money::new(i64::MAX);
    pub const MIN: Money = Money::new(i64::MIN);

    pub const fn new(value: i64) -> Self {
        Self::from_scaled(value)
    }

    /// Computes the absolute value.
    pub const fn abs(&self) -> Self {
        Self::from_scaled(self.0.abs())
    }

    pub fn inner(&self) -> i64 {
        self.0
    }

    /// Create a money type from an already scaled value.
    pub const fn from_scaled(value: i64) -> Self {
        Self(value)
    }

    pub fn from_f64(value: f64) -> Self {
        // TODO: probable precision loss
        // TODO: test overflow

        let scaled = (value * 10f64.powi(Self::SCALE as i32)).round() as i64;
        Self(scaled)
    }

    /// Parse and scale an unscaled value.
    pub const fn from_unscaled(value: i64) -> Self {
        let scaled = value.saturating_mul(Self::FACTOR as i64);
        Self(scaled)
    }

    pub fn to_decimal(&self) -> Decimal {
        Decimal::new(self.0, Self::SCALE)
    }
}

impl Add for Money {
    type Output = Money;

    fn add(self, rhs: Self) -> Self::Output {
        Money(self.0 + rhs.0)
    }
}

impl Sub for Money {
    type Output = Money;

    fn sub(self, rhs: Self) -> Self::Output {
        Money(self.0 - rhs.0)
    }
}

impl AddAssign for Money {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl SubAssign for Money {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

impl Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_decimal())
    }
}

impl FromStr for Money {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(value) = s.parse::<i64>() {
            return Ok(Self::from_unscaled(value));
        }

        let value: f64 = s.parse()?;
        Ok(Self::from_f64(value))
    }
}

impl Serialize for Money {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Money {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Money::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl Type<Sqlite> for Money {
    fn type_info() -> SqliteTypeInfo {
        <i64 as Type<Sqlite>>::type_info()
    }
}

impl<'a> Encode<'a,Sqlite> for Money{
    fn encode_by_ref(&self,buf: &mut <Sqlite as Database>::ArgumentBuffer<'a>,) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        <i64 as Encode<Sqlite>>::encode_by_ref(&self.0, buf)
    }
}

impl<'r, DB: Database> Decode<'r, DB> for Money
where
    i64: Decode<'r, DB>,
{
    fn decode(
        value: <DB as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let value = <i64 as Decode<DB>>::decode(value)?;
        Ok(Money(value))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rust_decimal::dec;
    use serde_json::json;
    use std::str::FromStr;

    #[test]
    fn saturate_unscaled_overflow() {
        let max_i64 = i64::MAX;
        let money = Money::from_unscaled(max_i64);
        assert_eq!(money.inner(), i64::MAX);
    }

    #[test]
    fn saturate_f64_overflow() {
        let money = Money::from_f64(f64::MAX);
        assert_eq!(money.inner(), i64::MAX);
    }

    #[test]
    fn non_finite() {
        assert_eq!(Money::from_f64(f64::NAN), Money::ZERO,);
        assert_eq!(Money::from_f64(f64::INFINITY), Money::MAX,);
        assert_eq!(Money::from_f64(f64::NEG_INFINITY), Money::MIN,);
    }

    #[test]
    fn from_unscaled() {
        let money = Money::from_unscaled(20);
        assert_eq!(money.0, 20_000000);
    }

    #[test]
    fn parse_json() -> crate::Result<()> {
        let money: Money = serde_json::from_value(json!("19.24")).unwrap();
        assert_eq!(money.inner(), 19_240_000);
        Ok(())
    }

    #[test]
    fn from_scaled() {
        let money = Money::from_scaled(20);
        assert_eq!(money.0, 20);
    }

    #[test]
    fn from_f64() {
        let money = Money::from_f64(999.999_999);
        assert_eq!(money.0, 999_999_999);
    }

    #[test]
    fn to_decimal() {
        let money = Money::from_unscaled(20);
        let decimal = money.to_decimal();
        assert_eq!(decimal, dec!(20.000_000))
    }

    #[test]
    fn from_str_int() -> crate::Result<()> {
        let money = Money::from_str("150")?;
        assert_eq!(money.0, 150_000_000);
        Ok(())
    }

    #[test]
    fn from_str_float() -> crate::Result<()> {
        let money = Money::from_str("150.24706")?;
        assert_eq!(money.0, 150_247_060);
        Ok(())
    }

    #[test]
    fn clip_float_string() -> crate::Result<()> {
        let money = Money::from_str("150.2470650935093059305930593095")?;
        assert_eq!(money.0, 150_247_065);
        Ok(())
    }

    #[test]
    fn to_string() {
        let money = Money::from_unscaled(20);
        assert_eq!(money.to_string(), "20.000000");
    }
}
