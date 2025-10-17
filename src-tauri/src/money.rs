use std::{fmt::Display, ops::{Add, AddAssign, Sub, SubAssign}, str::FromStr};

use rust_decimal::{prelude::*, Decimal};
use serde::Serialize;


/// The scale used for all decimals stored in
/// the database.
pub const DECIMAL_SCALE:u32 = 6;

/// A type that stores money with 6 digits after the decimal point.
#[derive(Debug,Clone,Copy,PartialEq, Eq, PartialOrd, Ord)]
pub struct Money(i64);

impl Money{
    /// The number of digits after the decimal.
    pub const SCALE: u32 = 6;
    pub const ZERO: Money = Money::from_unscaled(0);
    pub const ONE: Money = Money::from_unscaled(1);

    pub const fn new(value: i64) -> Self{
        Self::from_scaled(value)
    }

    pub fn inner(&self) -> i64{
        self.0
    }

    /// Create a money type from an already scaled value.
    pub const fn from_scaled(value: i64) -> Self{
        Self(value)
    }


    pub fn from_f64(value: f64) -> Self{
        // TODO: probable precision loss
        let scaled = (value * 10f64.powi(Self::SCALE as i32)).round() as i64;
        Self(scaled)
    }

    /// Parse and scale an unscaled value.
    pub const fn from_unscaled(value: i64) -> Self{
        // TODO: test max
        let scaled = value * 10i64.pow(Money::SCALE);
        Self(scaled)
    }

    pub fn to_decimal(&self) -> Decimal{
        Decimal::new(self.0,Self::SCALE)
    }
}

impl Add for Money{
    type Output = Money;
    
    fn add(self, rhs: Self) -> Self::Output {
        Money(self.0 + rhs.0)
    }
}

impl Sub for Money{
    type Output = Money;
    
    fn sub(self, rhs: Self) -> Self::Output {
        Money(self.0 - rhs.0)
    }
}

impl AddAssign for Money{
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl SubAssign for Money{
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

impl Display for Money{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.to_string())
    }
}

impl FromStr for Money{
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(value) = s.parse::<i64>(){
            return Ok(Self::from_unscaled(value))
        }

        let value:f64 = s.parse()?;
        Ok(Self::from_f64(value))
    }
}

impl Serialize for Money{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        serializer
            .serialize_str(&self.to_string())
    }
}

#[cfg(test)]
mod test{
    use std::str::FromStr;
    use rust_decimal::dec;
    use super::*;

    #[test]
    fn from_unscaled(){
        let money = Money::from_unscaled(20);
        assert_eq!(money.0,20_000000);
    }   

    #[test]
    fn from_scaled(){
        let money = Money::from_scaled(20);
        assert_eq!(money.0,20);
    }   
    
    #[test]
    fn from_f64(){
        let money = Money::from_f64(999.999_999);
        assert_eq!(money.0,999_999_999);
    }   

    #[test]
    fn to_decimal(){
        let money = Money::from_unscaled(20);
        let decimal = money.to_decimal();
        assert_eq!(decimal,dec!(20.000_000))
    }   

    #[test]
    fn from_str_int() -> crate::Result<()>{
        let money = Money::from_str("150")?;
        assert_eq!(money.0,150_000_000);
        Ok(())
    }   

    #[test]
    fn from_str_float() -> crate::Result<()>{
        let money = Money::from_str("150.24706")?;
        assert_eq!(money.0,150_247_060);
        Ok(())
    }   

    #[test]
    fn to_string(){
        let money = Money::from_unscaled(20);
        assert_eq!(money.to_string(),"20.000000");
    }   
}