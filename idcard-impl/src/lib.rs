use std::todo;

use idcard_cn::IdCard;
use idcard_cn::{Birth, EthnicGroup, IdentityNumber, Sex, Addr, Name};

/// 第二代中华人民共和国身份证
pub struct IdCardV2 {
    id: IdentityNumber,
    name: Name,
    ethnic: EthnicGroup,
    addr: Addr,
    signed_by: Addr,
}

impl IdCard for IdCardV2 {
    fn id(&self) -> IdentityNumber {
        self.id.clone()
    }

    fn name(&self) -> Name {
        self.name.clone()
    }

    fn sex(&self) -> Sex {
        match self.id.seq() % 2 {
            1 => Sex::Male,
            0 => Sex::Female,
            _ => unreachable!("身份证上不应该存在男/女以外的第三种性别!"),
        }
    }

    fn ethnic(&self) -> EthnicGroup {
        self.ethnic
    }

    fn birth(&self) -> Birth {
        self.id.birth()
    }

    fn addr(&self) -> Addr {
        self.addr.clone()
    }

    fn signer(&self) -> Addr {
        self.signed_by.clone()
    }

    fn valid_time(&self) -> (String, String) {
        todo!("需要进行简单的程序校验")
    }
}
