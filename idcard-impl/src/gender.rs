use idcard_cn::Sex;

type GenderCode = u8;
type GenderDesc = &'static str;

const GENDER_LIST: [(GenderCode, GenderDesc); 4] = [
    (0, "未知的性别"),
    (1, "男性"),
    (2, "女性"),
    (9, "未说明的性别"),
];

/// 公民身份持有人的性别，实现依据标准 [GB/T 2261.1-2003] 。
///
/// 计算过程中只需要保留性别代码。
///
/// [GB/T 2261.1-2003]: http://openstd.samr.gov.cn/bzgk/gb/newGbInfo?hcno=0FC942D542BC6EE3C707B2647EF81CD8
struct Gender(GenderCode);

impl Gender {
    pub fn from_code(c: GenderCode) -> Result<Self, &'static str> {
        match c {
            0 | 1 | 2 | 9 => Ok(Self(c)),
            _ => Err("Code is not in range!"),
        }
    }
}

impl Sex for Gender {
    type Code = GenderCode;
    type Desc = GenderDesc;

    fn code(&self) -> Self::Code {
        self.0
    }

    fn desc(&self) -> Self::Desc {
        GENDER_LIST[self.0 as usize].1
    }
}
