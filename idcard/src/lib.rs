//! [《中华人民共和国居民身份证法》] 第三条：
//! > 居民身份证登记的项目包括：姓名、性别、民族、出生日期、常住户口所在地住址、公民身份号码、本人相片、指纹信息、证件的有效期和签发机关。
//!
//! 本仓库根据以上要求提供相关信息的读取接口。
//!
//! [《中华人民共和国居民身份证法》]: http://www.gov.cn/zhengce/2011-10/29/content_2602263.htm

mod id;
mod date;
mod utils;

pub use crate::id::{IdentityNumber, InvalidId};
pub use crate::date::Date;
pub use crate::utils::Sex;

pub trait IdCard {
    fn id(&self) -> IdentityNumber;
    // fn name(&self) -> Name;
    fn sex(&self) -> Sex;
    // fn ethnic(&self) -> EthnicGroup;
    fn birthday(&self) -> Date;
    // fn addr(&self) -> Addr;
    // fn signer(&self) -> Addr;
    // fn valid_time(&self) -> (String, String);
    // fn fringerprint(&self) -> FingerPrint;
    // fn picture(&self) -> Picture;
}
