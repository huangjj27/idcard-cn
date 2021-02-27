use chrono::{Datelike, Local, NaiveDate, TimeZone, Weekday};
use std::str::FromStr;
use std::string::String;

/// 代表公民身份号码公民出生日期的结构体，只有根据常识有效日期才能被转换。
///
/// 常识1：[中国最长寿的人阿丽米罕老人] 于 1886 年 6 月 25日 出生, 因此不应该存在更长
/// 寿的人
/// 常识2：没有人能在执行日期的第二天或以后出生
///
/// [中国最长寿的人阿丽米罕老人]: https://new.qq.com/omn/20200531/20200531A08PF800.html
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Birth {
    inner: NaiveDate,
}

#[derive(Debug, PartialEq)]
pub enum InvalidBirth {
    StrParseError,
    TooOldDate,
    UncomeDate,
}

impl FromStr for Birth {
    type Err = InvalidBirth;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner =
            NaiveDate::parse_from_str(s, "%Y%m%d").map_err(|_| InvalidBirth::StrParseError)?;

        // TODO: 尽管根据常识，理应存在 1886-06-25 ~ 18990-2-31 出生并且健在的中国人，
        // 但是由于 chrono::NaiveDate 只会记录 1900-01-01 之后的日期，因此暂时先忽略
        // 1900-01-01 之前出生的人士，待到有需求的时候再改进。
        if inner < Local.ymd(1900, 1, 1).naive_local() {
            return Err(InvalidBirth::TooOldDate);
        }
        if inner > Local::today().naive_local() {
            return Err(InvalidBirth::UncomeDate);
        }

        Ok(Birth { inner })
    }
}

impl ToString for Birth {
    fn to_string(&self) -> String {
        self.code()
    }
}

impl Birth {
    pub fn code(&self) -> String {
        format!("{:>04}{:>02}{:>02}", self.year(), self.month(), self.day())
    }

    pub fn year(&self) -> i32 {
        self.inner.year()
    }

    pub fn month(&self) -> u32 {
        self.inner.month()
    }

    pub fn month0(&self) -> u32 {
        self.inner.month0()
    }

    pub fn day(&self) -> u32 {
        self.inner.day()
    }

    pub fn day0(&self) -> u32 {
        self.inner.day0()
    }

    pub fn ordinal(&self) -> u32 {
        self.inner.ordinal()
    }

    pub fn ordinal0(&self) -> u32 {
        self.inner.ordinal0()
    }

    pub fn weekday(&self) -> Weekday {
        self.inner.weekday()
    }

    pub fn with_year(&self, year: i32) -> Option<NaiveDate> {
        self.inner.with_year(year)
    }

    pub fn with_month(&self, month: u32) -> Option<NaiveDate> {
        self.inner.with_month(month)
    }

    pub fn with_month0(&self, month0: u32) -> Option<NaiveDate> {
        self.inner.with_month0(month0)
    }

    pub fn with_day(&self, day: u32) -> Option<NaiveDate> {
        self.inner.with_day(day)
    }

    pub fn with_day0(&self, day0: u32) -> Option<NaiveDate> {
        self.inner.with_day0(day0)
    }

    pub fn with_ordinal(&self, ordinal: u32) -> Option<NaiveDate> {
        self.inner.with_ordinal(ordinal)
    }

    pub fn with_ordinal0(&self, ordinal0: u32) -> Option<NaiveDate> {
        self.inner.with_ordinal0(ordinal0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_string_parse_error() {
        assert_eq!(
            "1x900101".parse::<Birth>().unwrap_err(),
            InvalidBirth::StrParseError
        );
    }

    #[test]
    fn test_too_old_date() {
        assert_eq!(
            "18991231".parse::<Birth>().unwrap_err(),
            InvalidBirth::TooOldDate
        );
    }

    #[test]
    fn test_uncome_date() {
        assert_eq!(
            "99991231".parse::<Birth>().unwrap_err(),
            InvalidBirth::UncomeDate
        );
    }

    #[test]
    fn test_valid_date() {
        let date = "19000101".parse::<Birth>().unwrap();
        assert_eq!(date.code(), "19000101".to_string());
    }
}
