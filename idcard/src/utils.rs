use chrono::{Local, NaiveDate, TimeZone};
use std::convert::TryFrom;

#[derive(Debug)]
pub struct Seq {
    inner: u16,
}

#[derive(Debug)]
pub enum InvalidSeq {
    StrParseError,
    Overflow(u16),
}

impl TryFrom<&str> for Seq {
    type Error = InvalidSeq;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let seq = s.parse::<u16>().map_err(|_| InvalidSeq::StrParseError)?;
        if seq > 999 {
            return Err(InvalidSeq::Overflow(seq));
        }

        Ok(Seq { inner: seq })
    }
}

pub enum Sex {
    Male,
    Female,
}

/// 代表公民身份号码公民出生日期的结构体，只有根据常识有效日期才能被转换。
#[derive(Debug)]
pub struct Date {
    inner: NaiveDate,
}

#[derive(Debug)]
pub enum InvalidDate {
    StrParseError,
    TooOldDate,
    UncomeDate,
}

impl TryFrom<&str> for Date {
    type Error = InvalidDate;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let inner =
            NaiveDate::parse_from_str(s, "%Y%m%d").map_err(|_| InvalidDate::StrParseError)?;

        if inner < Local.ymd(1990, 1, 1).naive_local() {
            return Err(InvalidDate::TooOldDate);
        }
        if inner > Local::today().naive_local() {
            return Err(InvalidDate::UncomeDate);
        }

        Ok(Date { inner })
    }
}
