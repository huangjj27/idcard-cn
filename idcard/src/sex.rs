/// 身份持有人的性别，接口设计依据标准 [GB/T 2261.1-2003]。
///
/// 这里包含的基本假设是，一个人的性别从出生开始就不会变化，尽管现代提供了变化性别的手段。
/// 因此，身份证上的性别按照常理以身份持有人出生时的生理性别为准，划分为男性和女性。
///
/// [GB/T 2261.1-2003]: http://openstd.samr.gov.cn/bzgk/gb/newGbInfo?hcno=0FC942D542BC6EE3C707B2647EF81CD8
pub trait Sex {
    type Code;
    type Desc;

    fn code(&self) -> Self::Code;
    fn desc(&self) -> Self::Desc;
}

// #[derive(Copy, Clone)]
// pub enum Sex {
//     /// 未知的性别
//     Unknown = 0,
//     /// 男性
//     Male = 1,
//     /// 女性
//     Female = 2,
//     /// 未说明的性别
//     Unspecified = 9,
// }

// impl AsRef<str> for Sex {
//     fn as_ref(&self) -> &str {
//         match *self {
//             Sex::Unknown => "未知的性别",
//             Sex::Male => "男性",
//             Sex::Female => "女性",
//             Sex::Unspecified => "未说明的性别",
//         }
//     }
// }
