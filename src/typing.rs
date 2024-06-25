use chrono::{prelude::*, LocalResult};
use std::fmt::Display;
// use super::*;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn kira_date_to_string_fmtd_works_as_expected() {
        let date = KiraDate::new(2024, 2, 1);

        assert_eq!(
            date.to_string_fmtd(Some("Y-M-D")),
            format!(
                "{}-{}-{}",
                date.get_year(),
                date.get_month(),
                date.get_day()
            )
        );
        assert_eq!(
            date.to_string_fmtd(Some("d/m/y")),
            format!(
                "{}/{}/{}",
                date.get_day(),
                date.get_month(),
                date.get_year()
            )
        );
        assert_eq!(
            date.to_string_fmtd(Some("Y.M.D")),
            format!(
                "{}.{}.{}",
                date.get_day(),
                date.get_month(),
                date.get_year()
            )
        );
        assert_eq!(
            date.to_string_fmtd(None),
            format!(
                "{}/{}/{}",
                date.get_day(),
                date.get_month(),
                date.get_year()
            )
        );
    }
}

pub type Year = i32;
pub type Month = u8;
pub type Day = u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct KiraDate {
    year: Year,
    month: Month,
    day: Day,
}

#[allow(dead_code)]
impl KiraDate {
    pub const fn new(year: Year, month: Month, day: Day) -> Self {
        Self { year, month, day }
    }

    /// Return a string formatted in either day-month-year or year-month-day
    /// arrangement, and with any of the three separators `/`, `.` or `-`.
    ///
    /// `format`s:
    ///
    /// - `Some("Y-M-D")`
    /// - `Some("D-M-Y")`
    /// - `Some("Y/M/D")`
    /// - `Some("D/M/Y")`
    /// - `Some("Y.M.D")`
    /// - `Some("D.M.Y")`
    /// - `Some(_)` = `D/M/Y`
    /// - `None` = `D/M/Y`
    ///
    /// # Examples
    ///
    /// ```
    /// let date = KiraDate::new(2024, 2, 1);
    ///
    /// assert_eq!(date.to_string_fmtd(Some("Y-M-D")), format!("{}-{}-{}", date.get_year(), date.get_month(), date.get_day()));
    /// assert_eq!(date.to_string_fmtd(Some("d/m/y")), format!("{}/{}/{}", date.get_day(), date.get_month(), date.get_year()));
    /// assert_eq!(date.to_string_fmtd(Some("Y.M.D")), format!("{}.{}.{}", date.get_day(), date.get_month(), date.get_year()));
    /// assert_eq!(date.to_string_fmtd(None), format!("{}/{}/{}", date.get_day(), date.get_month(), date.get_year()));
    /// ```
    pub fn to_string_fmtd(self, format: Option<&str>) -> String {
        format.map_or_else(
            || format!("{}/{}/{}", self.day, self.month, self.year),
            |fstr| match fstr.to_lowercase().as_str() {
                "y-m-d" => format!("{}-{}-{}", self.year, self.month, self.day),
                "d-m-y" => format!("{}-{}-{}", self.day, self.month, self.year),
                "y.m.d" => format!("{}.{}.{}", self.year, self.month, self.year),
                "d.m.y" => format!("{}.{}.{}", self.day, self.month, self.year),
                "y/m/d" => format!("{}/{}/{}", self.year, self.month, self.day),
                _ => format!("{}/{}/{}", self.day, self.month, self.year),
            },
        )
    }

    /// Returns the day of the month of the date.
    pub const fn get_day(self) -> Day {
        self.day
    }

    /// Returns the month of the year of the date.
    pub const fn get_month(self) -> Month {
        self.month
    }

    /// Returns the number of days in the specified month.
    ///
    /// # Panics
    ///
    /// This will panic if the month is outside the range of 1..12, inclusive.
    pub const fn days_in_month(month: Month) -> Day {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            2 => 28,
            4 | 6 | 9 | 11 => 30,
            _ => unreachable!(),
        }
    }

    /// Returns the number of days in the calling `KiraDate`'s current month.
    ///
    /// # Panics
    ///
    /// This will panic if the month is outside the range of 1..12, inclusive.
    pub const fn get_days_in_month(self) -> Day {
        Self::days_in_month(self.month)
    }

    /// Returns the year of the date.
    pub const fn get_year(self) -> Year {
        self.year
    }

    /// Returns `true` if the year is a leap year, `false` otherwise.
    pub const fn is_leap_year(self) -> bool {
        self.year % 4 == 0 && self.year % 100 != 0 || self.year % 400 == 0
    }

    /// Returns `true` if the date is valid, `false` otherwise.
    ///
    /// To be valid:
    /// - the month must be between 1 and 12, inclusive
    /// - the day must be between 1 and the month's number of days, inclusive
    pub const fn is_valid(self) -> bool {
        1 <= self.month && self.month <= 12 && 1 <= self.day && self.day <= self.get_days_in_month()
    }

    /// Returns a new `KiraDate` with the year, month and day clamped to valid
    /// values.
    pub fn make_valid(mut self) -> Self {
        self.month = self.month.clamp(1, 12);
        self.day = self.day.clamp(1, self.get_days_in_month());
        self
    }

    /// Returns a new [`LocalResult<DateTime<Utc>>`] from the calling
    /// `KiraDate`'s data.
    pub fn to_chrono_datetime(self) -> LocalResult<DateTime<Utc>> {
        self.into()
        ////Utc.with_ymd_and_hms(
        ////    self.year,
        ////    u32::from(self.month),
        ////    u32::from(self.day),
        ////    0,
        ////    0,
        ////    0,
        ////)
    }
}

impl From<LocalResult<DateTime<Utc>>> for KiraDate {
    fn from(date: LocalResult<DateTime<Utc>>) -> Self {
        Self::new(
            Year::try_from(date.unwrap().year()).expect(
                "Failed to downcast year from i32 to u16 in chrono DateTime to KiraDate conversion",
            ),
            Month::try_from(date.unwrap().month()).expect(
                "Failed to downcast month from u32 to u8 in chrono DateTime to KiraDate conversion",
            ),
            Day::try_from(date.unwrap().day()).expect(
                "Failed to downcast day from u32 to u8 in chrono DateTime to KiraDate conversion",
            ),
        )
    }
}

impl From<KiraDate> for LocalResult<DateTime<Utc>> {
    fn from(date: KiraDate) -> Self {
        Utc.with_ymd_and_hms(
            date.year,
            u32::from(date.month),
            u32::from(date.day),
            0,
            0,
            0,
        )
    }
}

impl Display for KiraDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string_fmtd(None))
    }
}
