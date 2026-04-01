
// export function parseDate(value: string,reference: CalendarDate): CalendarDate{
// let newDate = reference;
// // TODO:
// // - n
// // - n-n
// // - n,n
// // - n/n
// // - dd/mm/yyyy
// // - wrap in try catch
// const singleDigit = /^\d+$/.test(value.trim());
// if (singleDigit){
// return reference.set({day: parseInt(value.trim())});
// }
// // Test constraints
// console.log(singleDigit);
//
// // return previous date if all else failes
// return reference;
//
// }

use std::str::FromStr;
use chrono::{Datelike, Local, NaiveDate};

/// Parses and returns a [NaiveDate] from a string.
///
/// The goal is this function is to always return a date, no matter what. If no
/// valid date can be parsed, then it returns the current date. It is intended to
/// be used in user facing forms and inputs.
fn parse_date(value: &str) -> NaiveDate{
    if let Ok(date) = NaiveDate::from_str(value){
        return date;
    }

    let now = Local::now().date_naive();


    // Single digits are parsed as the day of the current month
    if let Ok(day) = value.parse::<u32>()
    && let Some(date) = NaiveDate::from_ymd_opt(now.year(),now.month(),day){
        return date;
    }

    now
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn parse_naive_date(){
        assert_eq!(parse_date("2024-12-12"),NaiveDate::from_str("2024-12-12").unwrap())
    }

    #[test]
    fn parse_single_digit(){
        let now = Local::now().date_naive();
        assert_eq!(parse_date("24"),NaiveDate::from_ymd_opt(now.year(),now.month(),24).unwrap())
    }

    #[test]
    fn parse_single_digit_overflow(){
        let now = Local::now().date_naive();
        assert_eq!(parse_date("100"),now)
    }
}