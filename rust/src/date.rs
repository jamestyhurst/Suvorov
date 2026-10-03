use crate::error::{Error, Result};

/// Gregorian calendar date used as a value type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
    pub year: i32,
    pub month: i32,
    pub day: i32,
}

impl Date {
    pub fn new(year: i32, month: i32, day: i32) -> Self {
        Self { year, month, day }
    }

    /// Parse `YYYY-MM-DD`. Leading zeros on the year are allowed (`0980-03-01`).
    pub fn from_iso(text: &str) -> Result<Self> {
        let mut parts = text.split('-');
        let year = parts.next().and_then(|s| s.parse().ok());
        let month = parts.next().and_then(|s| s.parse().ok());
        let day = parts.next().and_then(|s| s.parse().ok());
        if parts.next().is_some() {
            return Err(Error::InvalidArgument("ISO date is invalid"));
        }
        match (year, month, day) {
            (Some(year), Some(month), Some(day)) => {
                let date = Self::new(year, month, day);
                if date.valid() {
                    Ok(date)
                } else {
                    Err(Error::InvalidArgument("ISO date is invalid"))
                }
            }
            _ => Err(Error::InvalidArgument("ISO date is invalid")),
        }
    }

    pub fn valid(self) -> bool {
        if self.month < 1 || self.month > 12 || self.day < 1 {
            return false;
        }
        self.day <= days_in_month(self.year, self.month)
    }

    pub fn advance_one_day(&mut self) {
        self.day += 1;
        if self.day <= days_in_month(self.year, self.month) {
            return;
        }
        self.day = 1;
        self.month += 1;
        if self.month <= 12 {
            return;
        }
        self.month = 1;
        self.year += 1;
    }
}

pub fn biological_age(birth: Date, on: Date) -> Result<i32> {
    if !birth.valid() || !on.valid() {
        return Err(Error::InvalidArgument("Age date is invalid"));
    }
    if on < birth {
        return Err(Error::InvalidArgument("Birth date is after the age date"));
    }
    let mut years = on.year - birth.year;
    if on.month < birth.month || (on.month == birth.month && on.day < birth.day) {
        years -= 1;
    }
    Ok(years)
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn days_in_month(year: i32, month: i32) -> i32 {
    match month {
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_iso_accepts_leading_zeros_and_rejects_junk() {
        assert_eq!(Date::from_iso("0980-03-01").unwrap(), Date::new(980, 3, 1));
        assert!(Date::from_iso("1000-02-30").is_err());
        assert!(Date::from_iso("not-a-date").is_err());
        assert!(Date::from_iso("1000-01").is_err());
    }

    #[test]
    fn leap_day_age_boundary() {
        assert_eq!(
            biological_age(Date::new(2000, 2, 29), Date::new(2023, 2, 28)).unwrap(),
            22
        );
        assert_eq!(
            biological_age(Date::new(2000, 2, 29), Date::new(2023, 3, 1)).unwrap(),
            23
        );
    }
}
