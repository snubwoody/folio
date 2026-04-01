use chrono::{Datelike, Local, NaiveDate};
use regex::Regex;
use std::str::FromStr;
use std::sync::LazyLock;

/// Parses and returns a [NaiveDate] from a string.
///
/// The goal is this function is to always return a date, no matter what. If no
/// valid date can be parsed, then it returns the current date. It is intended to
/// be used in user facing forms and inputs.
pub fn parse_date(value: &str) -> NaiveDate {
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

    // Parse ambiguous dates like Jan 5, 5 Jan, 1 May 2025
    if let Some(date) = parse_date_like(value) {
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
    static RE_DASH: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\d+-\d+$").unwrap());
    static RE_SLASH: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\d+/\d+$").unwrap());
    static RE_COMMA: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\d+,\d+$").unwrap());

    if RE_DASH.is_match(value) {
        let parts: Vec<u32> = value
            .split("-")
            .map(|s| s.parse::<u32>().unwrap_or_default())
            .collect();
        return NaiveDate::from_ymd_opt(now.year(), parts[1], parts[0]);
    }

    if RE_SLASH.is_match(value) {
        let parts: Vec<u32> = value
            .split("/")
            .map(|s| s.parse::<u32>().unwrap_or_default())
            .collect();
        return NaiveDate::from_ymd_opt(now.year(), parts[1], parts[0]);
    }

    if RE_COMMA.is_match(value) {
        let parts: Vec<u32> = value
            .split(",")
            .map(|s| s.parse::<u32>().unwrap_or_default())
            .collect();
        return NaiveDate::from_ymd_opt(now.year(), parts[1], parts[0]);
    }
    None
}

fn parse_date_like(value: &str) -> Option<NaiveDate> {
    let value = value.replace(",", " ");
    let parts = value
        .split(" ")
        .filter(|s| !s.trim().is_empty())
        .collect::<Vec<_>>();
    if let Some(date) = parse_date_like_dmy(&parts) {
        return Some(date);
    }

    if let Some(date) = parse_date_like_mdy(&parts) {
        return Some(date);
    }

    None
}

fn parse_date_like_dmy(parts: &[&str]) -> Option<NaiveDate> {
    let today = Local::now().date_naive();
    let day = parts.first()?.parse::<u32>().ok()?;
    let month = month_from_str(parts.get(1)?)?;
    let year = match parts.len() {
        3 => parts.get(2)?.trim().parse::<i32>().ok()?,
        _ => today.year(),
    };
    if let Some(date) = NaiveDate::from_ymd_opt(year, month, day) {
        return Some(date);
    }
    None
}

fn parse_date_like_mdy(parts: &[&str]) -> Option<NaiveDate> {
    let today = Local::now().date_naive();
    let month = month_from_str(parts.first()?)?;
    let day = parts.get(1)?.parse::<u32>().ok()?;
    let year = match parts.len() {
        3 => parts.get(2)?.trim().parse::<i32>().ok()?,
        _ => today.year(),
    };
    if let Some(date) = NaiveDate::from_ymd_opt(year, month, day) {
        return Some(date);
    }
    None
}

/// Returns the month index, e.g. "Jan" and "January" both return 1.
fn month_from_str(value: &str) -> Option<u32> {
    match value.trim().to_lowercase().as_str() {
        "jan" | "january" => Some(1),
        "feb" | "february" => Some(2),
        "mar" | "march" => Some(3),
        "apr" | "april" => Some(4),
        "may" => Some(5),
        "jun" | "june" => Some(6),
        "jul" | "july" => Some(7),
        "aug" | "august" => Some(8),
        "sep" | "september" => Some(9),
        "oct" | "october" => Some(10),
        "nov" | "november" => Some(11),
        "dec" | "december" => Some(12),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_date_like_day_month() {
        let now = Local::now().date_naive();
        assert_eq!(
            parse_date("5 Jan"),
            NaiveDate::from_ymd_opt(now.year(), 1, 5).unwrap()
        );
        assert_eq!(
            parse_date("31 December"),
            NaiveDate::from_ymd_opt(now.year(), 12, 31).unwrap()
        );
        assert_eq!(
            parse_date("31 December 2025"),
            NaiveDate::from_ymd_opt(2025, 12, 31).unwrap()
        );
        assert_eq!(
            parse_date("31 December,2025"),
            NaiveDate::from_ymd_opt(2025, 12, 31).unwrap()
        );
        assert_eq!(
            parse_date("31 December, 2025"),
            NaiveDate::from_ymd_opt(2025, 12, 31).unwrap()
        );
    }

    #[test]
    fn parse_date_like_month_day() {
        let now = Local::now().date_naive();
        assert_eq!(
            parse_date("Jan 5"),
            NaiveDate::from_ymd_opt(now.year(), 1, 5).unwrap()
        );
        assert_eq!(
            parse_date("December 31"),
            NaiveDate::from_ymd_opt(now.year(), 12, 31).unwrap()
        );
        assert_eq!(
            parse_date("December 31 2020"),
            NaiveDate::from_ymd_opt(2020, 12, 31).unwrap()
        );
        assert_eq!(
            parse_date("December 31,2020"),
            NaiveDate::from_ymd_opt(2020, 12, 31).unwrap()
        );
        assert_eq!(
            parse_date("December 31, 2020"),
            NaiveDate::from_ymd_opt(2020, 12, 31).unwrap()
        );
    }

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

    #[test]
    fn month_from_str_short() {
        assert_eq!(month_from_str("Jan"), Some(1));
        assert_eq!(month_from_str("Feb"), Some(2));
        assert_eq!(month_from_str("Mar"), Some(3));
        assert_eq!(month_from_str("Apr"), Some(4));
        assert_eq!(month_from_str("May"), Some(5));
        assert_eq!(month_from_str("Jun"), Some(6));
        assert_eq!(month_from_str("Jul"), Some(7));
        assert_eq!(month_from_str("Aug"), Some(8));
        assert_eq!(month_from_str("Sep"), Some(9));
        assert_eq!(month_from_str("Oct"), Some(10));
        assert_eq!(month_from_str("Nov"), Some(11));
    }

    #[test]
    fn month_from_str_long() {
        assert_eq!(month_from_str("January"), Some(1));
        assert_eq!(month_from_str("February"), Some(2));
        assert_eq!(month_from_str("March"), Some(3));
        assert_eq!(month_from_str("April"), Some(4));
        assert_eq!(month_from_str("May"), Some(5));
        assert_eq!(month_from_str("June"), Some(6));
        assert_eq!(month_from_str("July"), Some(7));
        assert_eq!(month_from_str("August"), Some(8));
        assert_eq!(month_from_str("September"), Some(9));
        assert_eq!(month_from_str("October"), Some(10));
        assert_eq!(month_from_str("November"), Some(11));
        assert_eq!(month_from_str("December"), Some(12));
    }
}
