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
