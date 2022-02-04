//! [idcard-cn] 仓库接口的开源实现。
//!
//! 本仓库实现依据 [LGPL 3.0] 协议进行开源。
//!
//! [idcard-cn]: https://docs.rs/idcard-cn
//! [LGPL 3.0]: https://www.gnu.org/licenses/lgpl-3.0.en.html

pub mod prelude {
    pub use idcard_cn::*;
    pub use crate::id::{
        IdentificationNumber,
        InvalidId,
    };
    pub use crate::ethnicity::{
        Ethnicity,
        EthnicityCode,
        EthnicityName,
        EthnicityRomanCode,
        EthnicityShortCode,
    };
    pub use crate::gender::{
        Gender,
        GenderCode,
        GenderDesc,
    };
}

pub mod id;
pub mod ethnicity;
pub mod gender;
pub mod birth;

// /// 第二代中华人民共和国身份证
// pub struct IdCardV2 {
//     id: IdentityNumber,
//     name: Name,
//     ethnic: EthnicGroup,
//     addr: Addr,
//     signed_by: Addr,
// }

// impl IdCard for IdCardV2 {
//     fn id(&self) -> IdentityNumber {
//         self.id.clone()
//     }

//     fn name(&self) -> Name {
//         self.name.clone()
//     }

//     fn sex(&self) -> Sex {
//         match self.id.seq() % 2 {
//             1 => Sex::Male,
//             0 => Sex::Female,
//             _ => unreachable!("身份证上不应该存在男/女以外的第三种性别!"),
//         }
//     }

//     fn ethnic(&self) -> EthnicGroup {
//         self.ethnic
//     }

//     fn birth(&self) -> Birth {
//         self.id.birth()
//     }

//     fn addr(&self) -> Addr {
//         self.addr.clone()
//     }

//     fn signer(&self) -> Addr {
//         self.signed_by.clone()
//     }

//     fn valid_time(&self) -> (String, String) {
//         todo!("需要进行简单的程序校验")
//     }
// }
