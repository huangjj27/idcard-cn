//! 第二代中华人民共和国身份证公民身份号码

use gb2260::Division;
use std::convert::TryFrom;

use crate::utils::{Date, Seq};

const IDNUMBER_LENGTH: usize = 18;
const WEIGHTS: [u8; 17] = [7, 9, 10, 5, 8, 4, 2, 1, 6, 3, 7, 9, 10, 5, 8, 4, 2];
const CHECK_CODE: [char; 11] = ['1', '0', 'X', '9', '8', '7', '6', '5', '4', '3', '2'];
const DIV_CODE_LENGTH: usize = 6;
const BIRTHDAY_LENGTH: usize = 7;
const SEQ_LENGTH: usize = 3;
const ID_MODULE: u8 = 11;

/// 第二代中华人民共和国身份证公民身份号码。包括身份证持有人出生时行政区划分代码（6位数字）、
/// 出生日期（8位数字）、 当日出生顺序号（3位数字），以及一位的校验码。
///
/// 结构中不需要存校验码，只有合法的身份号码才能被转换成该结构体。
#[derive(Debug)]
pub struct IdentityNumber {
    // 中华人民共和国国家标准 GB/T 2260 行政区划代码
    div: Division,

    // 出生日期
    birth: Date,

    // 出生顺序号
    seq: Seq,
}

/// 通常违反身份号码校验规则的错误
#[derive(Debug, PartialEq)]
pub enum InvalidId {
    /// 第二代身份号码长度为18位，其他位数的字符串均不可能为身份号码
    LengthNotMatch(usize),

    /// 行政地区代码没有在历史的 GB/T 2260 地区编码中找到
    DivisionNotFound(String),

    /// 正常人类寿命一般不超过 120年，因此不会还有 1900 年之前出生的老者
    /// 另外也不可能超过验证时出生。
    InvalidBirthday(String),

    /// 序列号格式不正确
    InvalidSeq(String),

    /// 字符串格式正确，但是校验码与输入不匹配
    WrongCheckCode,
}

impl TryFrom<&str> for IdentityNumber {
    type Error = InvalidId;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let s_len = s.chars().count();
        if s_len != IDNUMBER_LENGTH {
            return Err(InvalidId::LengthNotMatch(s_len));
        }

        let (div_code, rest) = s.split_at(DIV_CODE_LENGTH);
        let div = match Division::get(div_code) {
            Some(d) => d,
            None => return Err(InvalidId::DivisionNotFound(div_code.to_owned())),
        };

        let (birthday, rest) = rest.split_at(BIRTHDAY_LENGTH);
        let birth = Date::try_from(birthday)
            .map_err(|_| InvalidId::InvalidBirthday(birthday.to_owned()))?;

        let (seq, chk_code) = rest.split_at(SEQ_LENGTH);
        let seq = Seq::try_from(seq).map_err(|_| InvalidId::InvalidSeq(seq.to_owned()))?;

        let chk_idx: usize =
            s.chars()
                .take(IDNUMBER_LENGTH - 1)
                .map(|chr| chr.to_digit(10).unwrap() as u8)
                .zip(WEIGHTS.iter())
                .fold(0u8, |acc, (d, w)| (acc + d * w) % ID_MODULE) as usize;
        let chk_code = chk_code.chars().next().unwrap();

        if chk_code != CHECK_CODE[chk_idx] {
            return Err(InvalidId::WrongCheckCode);
        }

        Ok(IdentityNumber { div, birth, seq })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_wrong_length_id() {
        let shorter = "51010819720505213";
        assert_eq!(IdentityNumber::try_from(shorter).unwrap_err(), InvalidId::LengthNotMatch(17));

        let longer = "5101081972050521378";
        assert_eq!(IdentityNumber::try_from(longer).unwrap_err(), InvalidId::LengthNotMatch(19));
    }

    fn test_invalid_division() {

    }
}
