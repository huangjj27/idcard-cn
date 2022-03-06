use gb2260::Division;
use idcard_cn::Identity;

use std::str::FromStr;

use crate::birth::Birth;

const IDNUMBER_LENGTH: usize = 18;
const WEIGHTS: [u8; 17] = [7, 9, 10, 5, 8, 4, 2, 1, 6, 3, 7, 9, 10, 5, 8, 4, 2];
const CHECK_CODE: [char; 11] = ['1', '0', 'X', '9', '8', '7', '6', '5', '4', '3', '2'];
const DIV_CODE_LENGTH: usize = 6;
const BIRTHDAY_LENGTH: usize = 8;
const SEQ_LENGTH: usize = 3;
const ID_MODULE: u8 = 11;

#[derive(Clone, Debug, PartialEq)]
pub struct IdentificationNumber {
    /// 中华人民共和国国家标准 GB/T 2260 行政区划代码
    div: Division,

    /// 八位出生日期，格式YYYYMMDD
    birth: Birth,

    /// 出生顺序号。顺序号可以表示身份人 [性别](enum.Sex)，奇数为男性，偶数为女性
    seq: Seq,

    // 验证码索引，存储该索引以快速校验
    chk_idx: usize,
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
    InvalidBirth(String),

    /// 序列号格式不正确
    InvalidSeq(String),

    /// 校验码为非法字符
    InvalidCheckCode(char),

    /// 字符串格式正确，但是校验码与输入不匹配
    WrongCheckCode(char),
}

impl FromStr for IdentificationNumber {
    type Err = InvalidId;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s_len = s.chars().count();
        if s_len != IDNUMBER_LENGTH {
            return Err(InvalidId::LengthNotMatch(s_len));
        }

        let (div_code, rest) = s.split_at(DIV_CODE_LENGTH);

        let (birth, rest) = rest.split_at(BIRTHDAY_LENGTH);
        let birth = birth
            .parse::<Birth>()
            .map_err(|_| InvalidId::InvalidBirth(birth.to_owned()))?;

        // 区域行政编码为身份持有人出生当年的行政编码，因此需要解析出合法的出生日期后才
        // 能开始解析区域行政编码。
        let birth_year = birth.year();
        let div_opt = if birth_year >= 1980 {
            // 由于 gb2260 只有1980年以后的数据，因此只有1980年以后出生的身份持有人
            // 才能根据出生年份查询准确的当年行政区的编码
            let revision = birth_year.to_string() + "12";
            Division::get_by_revision(div_code, &revision)
        } else {
            // 1980年以前出生的身份持有人只能碰运气在所有的行政编码版本中遍历查找
            Division::search(div_code)
        };

        let div = match div_opt {
            Some(d) => d,
            None => return Err(InvalidId::DivisionNotFound(div_code.to_owned())),
        };

        let (seq, chk_code) = rest.split_at(SEQ_LENGTH);
        let seq = seq
            .parse::<Seq>()
            .map_err(|_| InvalidId::InvalidSeq(seq.to_owned()))?;

        // 在前面的逻辑中，优先对身份号码长度进行了判断，因此一定会找到唯一的校验码字符。
        let chk_code = match chk_code.chars().next() {
            Some(chr) if CHECK_CODE.contains(&chr) => chr,
            Some(chr) if !CHECK_CODE.contains(&chr) => {
                return Err(InvalidId::InvalidCheckCode(chr))
            }
            Some(_) => unreachable!("chk_code should be always found. This is a bug"),
            None => unreachable!("chk_code should be always found. This is a bug"),
        };

        let chk_idx: usize =
            s.chars()
                .take(IDNUMBER_LENGTH - 1)
                .map(|chr| chr.to_digit(10).unwrap() as u8)
                .zip(WEIGHTS.iter())
                .fold(0u8, |acc, (d, w)| (acc + d * w) % ID_MODULE) as usize;
        if chk_code != CHECK_CODE[chk_idx] {
            return Err(InvalidId::WrongCheckCode(chk_code));
        }

        Ok(IdentificationNumber { div, birth, seq, chk_idx })
    }
}

impl ToString for IdentificationNumber {
    fn to_string(&self) -> String {
        let mut s = format!(
            "{:>06}{}{:>03}",
            self.div.code,
            self.birth.code(),
            self.seq.code()
        );

        let chk_idx: usize =
            s.chars()
                .take(IDNUMBER_LENGTH - 1)
                .map(|chr| chr.to_digit(10).unwrap() as u8)
                .zip(WEIGHTS.iter())
                .fold(0u8, |acc, (d, w)| (acc + d * w) % ID_MODULE) as usize;
        s.push(CHECK_CODE[chk_idx]);

        s
    }
}

impl Identity for IdentificationNumber {
    type Division = gb2260::Division;
    type Birthday = Birth;
    type Sequence = Seq;
    type Checksum = char;

    fn division(&self) -> Self::Division {
        self.div.clone()
    }

    fn birthday(&self) -> Self::Birthday {
        self.birth
    }

    fn sequence(&self) -> Self::Sequence {
        self.seq
    }

    fn checksum(&self) -> Self::Checksum {
        CHECK_CODE[self.chk_idx]
    }
}

impl IdentificationNumber {
    /// 根据结构体已经存储的行政区划、出生日期与序列号信息重新检验身份号码。
    pub fn recheck(&self) -> bool {
        let s = format!(
            "{:>06}{}{:>03}",
            self.div.code,
            self.birth.code(),
            self.seq.code()
        );

        let chk_idx: usize =
            s.chars()
                .take(IDNUMBER_LENGTH - 1)
                .map(|chr| chr.to_digit(10).unwrap() as u8)
                .zip(WEIGHTS.iter())
                .fold(0u8, |acc, (d, w)| (acc + d * w) % ID_MODULE) as usize;

        CHECK_CODE[self.chk_idx] == CHECK_CODE[chk_idx]
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Seq {
    inner: u16,
}

#[derive(Debug, PartialEq)]
pub enum InvalidSeq {
    StrParseError,
    ShouldNotBeZero,
    Overflow(u16),
}

impl FromStr for Seq {
    type Err = InvalidSeq;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let seq = s.parse::<u16>().map_err(|_| InvalidSeq::StrParseError)?;

        // 相同出生地区、相同出生日期的公民，其身份号码出生序列号从 1 开始
        if seq == 0 {
            return Err(InvalidSeq::ShouldNotBeZero);
        }

        // 目前身份证设定相同地区、相同日期出生的公民计数不超过 999 个人
        if seq > 999 {
            return Err(InvalidSeq::Overflow(seq));
        }

        Ok(Seq { inner: seq })
    }
}

impl Seq {
    pub fn code(&self) -> String {
        format!("{:>03}", self.inner)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_wrong_length_id() {
        let shorter = "51010819720505213";
        assert_eq!(
            shorter.parse::<IdentificationNumber>().unwrap_err(),
            InvalidId::LengthNotMatch(IDNUMBER_LENGTH - 1)
        );

        let longer = "5101081972050521378";
        assert_eq!(
            longer.parse::<IdentificationNumber>().unwrap_err(),
            InvalidId::LengthNotMatch(IDNUMBER_LENGTH + 1)
        );
    }

    #[test]
    fn test_invalid_division() {
        let wrong_division = "000000197205052137";
        assert_eq!(
            wrong_division.parse::<IdentificationNumber>().unwrap_err(),
            InvalidId::DivisionNotFound("000000".to_string())
        );

        let wrong_format = "51X108197205052137";
        assert_eq!(
            wrong_format.parse::<IdentificationNumber>().unwrap_err(),
            InvalidId::DivisionNotFound("51X108".to_string())
        );
    }

    #[test]
    fn test_invalid_birth() {
        let wrong_format = "5101081972?5052137";
        assert_eq!(
            wrong_format.parse::<IdentificationNumber>().unwrap_err(),
            InvalidId::InvalidBirth("1972?505".to_string())
        );

        let old_date = "510108187205052137";
        assert_eq!(
            old_date.parse::<IdentificationNumber>().unwrap_err(),
            InvalidId::InvalidBirth("18720505".to_string())
        );

        let upcoming = "510108297205052137";
        assert_eq!(
            upcoming.parse::<IdentificationNumber>().unwrap_err(),
            InvalidId::InvalidBirth("29720505".to_string())
        );
    }

    #[test]
    fn test_invalid_seq() {
        let wrong_format = "5101081972050521$7";
        assert_eq!(
            wrong_format.parse::<IdentificationNumber>().unwrap_err(),
            InvalidId::InvalidSeq("21$".to_string())
        );

        let zero_seq =  "510108197205050007";
        assert_eq!(
            zero_seq.parse::<IdentificationNumber>().unwrap_err(),
            InvalidId::InvalidSeq("000".to_string())
        );
    }

    #[test]
    fn test_invalid_checkcode() {
        let wrong_format = "51010819720505213%";
        assert_eq!(
            wrong_format.parse::<IdentificationNumber>().unwrap_err(),
            InvalidId::InvalidCheckCode('%')
        );

        // 小写的 `x` 校验码也被认为是非法校验码
        let wrong_format = "51010819720505213x";
        assert_eq!(
            wrong_format.parse::<IdentificationNumber>().unwrap_err(),
            InvalidId::InvalidCheckCode('x')
        );
    }

    #[test]
    fn test_wrong_checkcode() {
        let wrong_chk = "51010819720505213X";
        assert_eq!(
            wrong_chk.parse::<IdentificationNumber>().unwrap_err(),
            InvalidId::WrongCheckCode('X')
        );
    }

    #[test]
    fn test_valid_id() {
        let id = IdentificationNumber {
            div: Division::get("510108").unwrap(),
            birth: str::parse::<Birth>("19720505").unwrap(),
            seq: str::parse::<Seq>("213").unwrap(),
            chk_idx: 5,
        };

        let valid_str = "510108197205052137";
        assert_eq!(valid_str.parse::<IdentificationNumber>().unwrap(), id);
    }
}
