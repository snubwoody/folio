use rust_decimal::{prelude::FromPrimitive, Decimal};
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


    /// Create a money type from an already scaled value.
    pub const fn from_scaled(value: i64) -> Self{
        Self(value)
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

impl ToString for Money{
    fn to_string(&self) -> String {
        self.to_decimal().to_string()
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
    fn to_decimal(){
        let money = Money::from_unscaled(20);
        let decimal = money.to_decimal();
        assert_eq!(decimal,dec!(20.000_000))
    }   

    #[test]
    fn to_string(){
        let money = Money::from_unscaled(20);
        assert_eq!(money.to_string(),"20.000000");
    }   
}