//! [《中华人民共和国居民身份证法》] 第三条：
//! > 居民身份证登记的项目包括：姓名、性别、民族、出生日期、常住户口所在地住址、公民身份号码、本人相片、指纹信息、证件的有效期和签发机关。
//!
//! 本仓库根据以上要求提供相关信息的读取接口。
//!
//! [《中华人民共和国居民身份证法》]: http://www.gov.cn/zhengce/2011-10/29/content_2602263.htm

mod birth;
mod ethnic;
mod id;
mod sex;

use id::Identity;

pub use crate::birth::Birth;
pub use crate::ethnic::Ethnic;
pub use crate::sex::Sex;

//TODO: 暂时没有想好设计，姑且用字符串表示先
pub type Name = String;
pub type Addr = String;

/// 中华人民共和国身份证应当提供的机读信息接口
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
