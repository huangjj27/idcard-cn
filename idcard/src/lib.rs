//! [《中华人民共和国居民身份证法》] 第三条：
//! > 居民身份证登记的项目包括：姓名、性别、民族、出生日期、常住户口所在地住址、公民身份号码、本人相片、指纹信息、证件的有效期和签发机关。
//!
//! 本仓库根据以上要求，以及根据中华人民共和国公共行业标准 GA/T 490-2019, 设计接口。
//!
//! [《中华人民共和国居民身份证法》]: http://www.gov.cn/zhengce/2011-10/29/content_2602263.htm

mod birth;
mod ethnic;
mod id;
mod sex;

pub use id::Identity;
pub use crate::birth::Birth;
pub use crate::ethnic::Ethnic;
pub use crate::sex::Sex;

/// 中华人民共和国身份证应当提供的机读信息接口，依据标准 GA/T 490-2019。
pub trait IdCard {
    type IdentificationNumber: Identity;
    type Name;
    type Gender: Sex;
    type Ethnicity: Ethnic;
    type Date;
    type Addr;
    type FingerPrint;
    type Picture;

    /// 公民身份号码
    fn id(&self) -> Self::IdentificationNumber;

    /// 公民姓名
    fn name(&self) -> Self::Name;

    /// 公民性别
    fn gender(&self) -> Self::Gender;

    /// 公民民族，包含 [中华民族 56 个民族](enum.Ethnicity.html)
    fn ethnicity(&self) -> Self::Ethnicity;

    /// 公民出生日期
    fn birthday(&self) -> Self::Date;

    /// 公民常住户口所在地住址
    fn addr(&self) -> Self::Addr;

    /// 身份证证件签发机关
    fn authority(&self) -> Self::Addr;

    /// 身份证证件有效期
    fn valid_time(&self) -> (Self::Date, Self::Date);

    // 公民指纹信息
    // fn fringerprint(&self) -> Self::FingerPrint;

    // 公民照片
    // fn picture(&self) -> Self::Picture;
}
