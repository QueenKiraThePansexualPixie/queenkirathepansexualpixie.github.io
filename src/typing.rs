use chrono::LocalResult;

use super::*;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct KiraDate {
    year: u16,
    month: u8,
    day: u8,
}

#[allow(dead_code)]
impl KiraDate {
    pub(crate) fn new(year: u16, month: u8, day: u8) -> KiraDate {
        KiraDate { year, month, day }
    }

    /// ```
    /// let date = KiraDate::new(2024, 2, 1);
    ///
    /// assert_eq!(date.to_string("Y-M-D"), format!("{}-{}-{}", date.get_year(), date.get_month(), date.get_day()));
    /// assert_eq!(date.to_string("D/M/Y"), format!("{}/{}/{}", date.get_day(), date.get_month(), date.get_year()));
    /// assert_eq!(date.to_string("Y.M.D"), format!("{}.{}.{}", date.get_day(), date.get_month(), date.get_year()));
    /// assert_eq!(date.to_string(""), format!("{}/{}/{}", date.get_day(), date.get_month(), date.get_year()));
    /// ```
    pub(crate) fn to_string(&self, format_string: &str) -> String {
        match format_string.to_lowercase().as_str() {
            "y-m-d" => format!("{}-{}-{}", self.year, self.month, self.day),
            "d-m-y" => format!("{}-{}-{}", self.day, self.month, self.year),
            "y/m/d" => format!("{}/{}/{}", self.year, self.month, self.day),
            "d/m/y" => format!("{}/{}/{}", self.day, self.month, self.year),
            "y.m.d" => format!("{}.{}.{}", self.year, self.month, self.year),
            "d.m.y" => format!("{}.{}.{}", self.day, self.month, self.year),
            _ => format!("{}/{}/{}", self.day, self.month, self.year),
        }
    }

    pub(crate) fn get_day(&self) -> u8 {
        self.day
    }

    pub(crate) fn get_month(&self) -> u8 {
        self.month
    }

    pub(crate) fn get_year(&self) -> u16 {
        self.year
    }

    pub(crate) fn is_leap_year(&self) -> bool {
        self.year % 4 == 0 && self.year % 100 != 0 || self.year % 400 == 0
    }

    pub(crate) fn is_valid(&self) -> bool {
        self.month > 0 && self.month <= 12 && self.day > 0 && self.day <= 31
    }

    pub(crate) fn to_chrono_datetime(&self) -> LocalResult<DateTime<Utc>> {
        Utc.with_ymd_and_hms(
            self.year as i32,
            self.month as u32,
            self.day as u32,
            0,
            0,
            0,
        )
    }
}
