// export function parseDate(value: string,reference: CalendarDate): CalendarDate{
// let newDate = reference;
// // TODO:
// // - n
// // - dd-mm
// // - dd,mm
// // - dd/mm
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

use chrono::{Datelike, Local, NaiveDate};
use regex::Regex;
use std::str::FromStr;
use std::sync::LazyLock;

/// Parses and returns a [NaiveDate] from a string.
///
/// The goal is this function is to always return a date, no matter what. If no
/// valid date can be parsed, then it returns the current date. It is intended to
/// be used in user facing forms and inputs.
fn parse_date(value: &str) -> NaiveDate {
    if let Ok(date) = NaiveDate::from_str(value) {
        return date;
    }

    let now = Local::now().date_naive();

    // Single digits are parsed as the day of the current month
    if let Ok(day) = value.parse::<u32>()
        && let Some(date) = NaiveDate::from_ymd_opt(now.year(), now.month(), day)
    {
        return date;
    }

    if let Some(date) = parse_dd_mm(value) {
        return date;
    }

    now
}

/// Parses dates of the format:
/// - dd/mm
/// - dd-mm
/// - dd,mm
fn parse_dd_mm(value: &str) -> Option<NaiveDate> {
    let now = Local::now().date_naive();
    static RE_DASH: LazyLock<Regex> = LazyLock::new(||Regex::new(r"^\d+-\d+$").unwrap());
    static RE_SLASH: LazyLock<Regex> = LazyLock::new(||Regex::new(r"^\d+/\d+$").unwrap());
    static RE_COMMA: LazyLock<Regex> = LazyLock::new(||Regex::new(r"^\d+,\d+$").unwrap());

    if RE_DASH.is_match(value) {
        let parts: Vec<u32> = value.split("-")
            .map(|s|s.parse::<u32>().unwrap_or_default())
            .collect();
        return NaiveDate::from_ymd_opt(now.year(),parts[1],parts[0]);
    }

    if RE_SLASH.is_match(value) {
        let parts: Vec<u32> = value.split("/")
            .map(|s|s.parse::<u32>().unwrap_or_default())
            .collect();
        return NaiveDate::from_ymd_opt(now.year(),parts[1],parts[0]);
    }

    if RE_COMMA.is_match(value) {
        let parts: Vec<u32> = value.split(",")
            .map(|s|s.parse::<u32>().unwrap_or_default())
            .collect();
        return NaiveDate::from_ymd_opt(now.year(),parts[1],parts[0]);
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_naive_date() {
        assert_eq!(
            parse_date("2024-12-12"),
            NaiveDate::from_str("2024-12-12").unwrap()
        )
    }

    #[test]
    fn parse_single_digit() {
        let now = Local::now().date_naive();
        assert_eq!(
            parse_date("24"),
            NaiveDate::from_ymd_opt(now.year(), now.month(), 24).unwrap()
        )
    }

    #[test]
    fn parse_dd_mm() {
        let now = Local::now().date_naive();
        assert_eq!(
            parse_date("24-12"),
            NaiveDate::from_ymd_opt(now.year(), 12, 24).unwrap()
        );
        assert_eq!(
            parse_date("24/12"),
            NaiveDate::from_ymd_opt(now.year(), 12, 24).unwrap()
        );
        assert_eq!(
            parse_date("24,12"),
            NaiveDate::from_ymd_opt(now.year(), 12, 24).unwrap()
        )
    }

    #[test]
    fn parse_single_digit_overflow() {
        let now = Local::now().date_naive();
        assert_eq!(parse_date("100"), now)
    }
}
