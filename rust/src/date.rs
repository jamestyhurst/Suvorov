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
