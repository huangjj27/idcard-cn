use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub(crate) struct Seq {
    inner: u16,
}

#[derive(Debug, PartialEq)]
pub enum InvalidSeq {
    StrParseError,
    Overflow(u16),
}

impl FromStr for Seq {
    type Err = InvalidSeq;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let seq = s.parse::<u16>().map_err(|_| InvalidSeq::StrParseError)?;
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

pub enum Sex {
    Male,
    Female,
}
