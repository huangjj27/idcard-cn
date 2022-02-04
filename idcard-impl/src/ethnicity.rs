use idcard_cn::Ethnic;

pub type EthnicityCode = u8;
pub type EthnicityName = &'static str;
pub type EthnicityRomanCode = &'static str;
pub type EthnicityShortCode = &'static str;

const ETHNICITY_LIST: [(EthnicityCode, EthnicityName, EthnicityRomanCode, EthnicityShortCode); 56] = [
    (1, "汉族", "Han", "HA"),
    (2, "蒙古族", "Mongol", "MG"),
    (3, "回族", "Hui", "HU"),
    (4, "藏族", "Zang", "AZ"),
    (5, "维吾尔族", "Uygur", "UG"),
    (6, "苗族", "Miao", "MH"),
    (7, "彝族", "Yi", "YI"),
    (8, "壮族", "Zhuang", "ZH"),
    (9, "布依族", "Buyei", "BY"),
    (10, "朝鲜族", "Chosen", "CS"),
    (11, "满族", "Man", "MA"),
    (12, "侗族", "Dong", "DO"),
    (13, "瑶族", "Yao", "YA"),
    (14, "白族", "Bai", "BA"),
    (15, "土家族", "Tujia", "TJ"),
    (16, "哈尼族", "Hani", "HN"),
    (17, "哈萨克族", "Kazak", "KZ"),
    (18, "傣族", "Dai", "DA"),
    (19, "黎族", "Li", "LI"),
    (20, "傈僳族", "Lisu", "LS"),
    (21, "佤族", "Va", "VA"),
    (22, "畲族", "She", "SH"),
    (23, "高山族", "Gaoshan", "GS"),
    (24, "拉祜族", "LaHu", "LH"),
    (25, "水族", "Shui", "Su"),
    (26, "东乡族", "DongXiang", "DX"),
    (27, "纳西族", "Naxi", "NX"),
    (28, "景颇族", "Jingpo", "JP"),
    (29, "柯尔克孜族", "Kirgiz", "KG"),
    (30, "土族", "Tu", "TU"),
    (31, "达斡尔族", "Daur", "DU"),
    (32, "仫佬族", "Mulao", "ML"),
    (33, "羌族", "Qiang", "QI"),
    (34, "布朗族", "Blang", "BL"),
    (35, "撒拉族", "Salar", "SL"),
    (36, "毛南族", "Maonan", "MN"),
    (37, "仡佬族", "Gelao", "GL"),
    (38, "锡伯族", "Xibe", "XB"),
    (39, "阿昌族", "Achang", "AC"),
    (40, "普米族", "Pumi", "PM"),
    (41, "塔吉克族", "Tajik", "TA"),
    (42, "怒族", "Nu", "NU"),
    (43, "乌兹别克族", "Uzbek", "UZ"),
    (44, "俄罗斯族", "Russ", "RS"),
    (45, "鄂温克族", "Ewenki", "EW"),
    (46, "德昂族", "Deang", "DE"),
    (47, "保安族", "Bonan", "BN"),
    (48, "裕固族", "Yugur", "YG"),
    (49, "京族", "Gin", "GI"),
    (50, "塔塔儿族", "Tatar", "TT"),
    (51, "独龙族", "Derung", "DR"),
    (52, "鄂伦春族", "Oroqen", "OR"),
    (53, "赫哲族", "Hezhen", "HZ"),
    (54, "门巴族", "Monba", "MB"),
    (55, "珞巴族", "Lhoba", "LB"),
    (56, "基诺族", "Jino", "JN"),
];

/// 公民身份持有人的民族，实现依据标准 [GB 3304-1991] 。
///
/// 计算过程中只需要保留民族的数字编码。
///
/// [GB 3304-1991]: http://openstd.samr.gov.cn/bzgk/gb/newGbInfo?hcno=E5C3271B62636C5DA6853A0DA23EBBA9
pub struct Ethnicity(EthnicityCode);

impl Ethnicity {
    pub fn from_code(c: EthnicityCode) -> Result<Self, &'static str> {
        if c == 0 || c > 56 {
            return Err("Provided code out of range!");
        }

        Ok(Self(c))
    }
}

impl Ethnic for Ethnicity {
    type Code = EthnicityCode;
    type Name = EthnicityName;
    type RomanCode = EthnicityRomanCode;
    type ShortCode = EthnicityShortCode;

    fn code(&self) -> Self::Code {
        self.0
    }

    fn name(&self) -> Self::Name {
        ETHNICITY_LIST[self.0 as usize].1
    }

    fn roman(&self) -> Self::RomanCode {
        ETHNICITY_LIST[self.0 as usize].2
    }

    fn short(&self) -> Self::ShortCode {
        ETHNICITY_LIST[self.0 as usize].3
    }
}
