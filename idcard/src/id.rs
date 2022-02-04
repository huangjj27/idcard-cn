use std::str::FromStr;

/// 第二代中华人民共和国身份证公民身份号码接口，接口设计依据标准 [GB 11643-1999]。
///
/// 公民身份号码定义以下数据：
/// - 本体码，由以下三部分组成：
///     - 地址码（[`Division`](Identity::Division)）
///     - 出生日期码（[`Birthday`](Identity::Birthday)）
///     - 顺序码（[`Sequence`](Identity::Sequence)）
/// - 校验码（[`Checksum`](Identity::Checksum)）
///
/// [GB 11643-1999]: http://openstd.samr.gov.cn/bzgk/gb/newGbInfo?hcno=080D6FBF2BB468F9007657F26D60013E
pub trait Identity: FromStr + ToString {
    type Division;
    type Birthday;
    type Sequence;
    type Checksum;

    /// 地址码，表示编码对象常住户口所在县（市、旗、区）的行政区划代码，按照 GB/T 2260[^1][^2] 的规定执行。
    ///
    /// [^1]: 现行标准为 [GB/T 2260-2007]，替代了旧标准 [GB/T 2260-2002]
    ///
    /// [^2]: 区划代码每年变化，参见[民政部数据]
    ///
    /// [GB/T 2260-2007]: http://openstd.samr.gov.cn/bzgk/gb/newGbInfo?hcno=C9C488FD717AFDCD52157F41C3302C6D
    /// [GB/T 2260-2002]: http://openstd.samr.gov.cn/bzgk/gb/newGbInfo?hcno=94AE2AC001D612E63BCA3CE2B5E63E20
    /// [民政部数据]: http://www.mca.gov.cn/article/sj/xzqh/1980/
    fn division(&self) -> Self::Division;

    /// 出生日期码，表示编码对象的年、月、日，按 GB/T 7408[^1] 的规定执行。
    ///
    /// [^1]: 现行标准为 [GB/T 7408-2005]，采标标准为 ISO 8601:2000，替代了旧标准 [GB/T 7408-1994]
    ///
    /// [GB/T 7408-2005]: http://openstd.samr.gov.cn/bzgk/gb/newGbInfo?hcno=3CFD9AE8FEADB062B3BC53651930DED1
    /// [GB/T 7408-1994]: http://openstd.samr.gov.cn/bzgk/gb/newGbInfo?hcno=52DD936203144EEA358C7492C85D72DD
    fn birthday(&self) -> Self::Birthday;

    /// 顺序码，表示同一地址码所标识的区域范围内，对同年、同月、同日出生的人编定的顺序号，
    /// 顺序码的奇数分配给男性，偶数分配给女性。
    fn sequence(&self) -> Self::Sequence;

    /// 校验码，采用 ISO 7064:1983, MOD 11 2 校验系统[^1]。
    ///
    /// [^1]: 该校验系统详见 [GB 11643-1999] 5.1.4.1 小节
    ///
    /// [GB 11643-1999]: http://openstd.samr.gov.cn/bzgk/gb/newGbInfo?hcno=080D6FBF2BB468F9007657F26D60013E
    fn checksum(&self) -> Self::Checksum;
}
