/// 身份持有人的性别
///
/// 这里包含的基本假设是，一个人的性别从出生开始就不会变化，尽管现代提供了变化性别的手段。
/// 因此，身份证上的性别按照常理以身份持有人出生时的生理性别为准，划分为男性和女性
#[derive(Copy, Clone)]
pub enum Sex {
    Male,
    Female,
}

impl ToString for Sex {
    fn to_string(&self) -> String {
        match *self {
            Sex::Male => "男".to_owned(),
            Sex::Female => "女".to_owned(),
        }
    }
}
