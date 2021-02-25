use std::string::ToString;
/// 中华人民共和国中华民族
///
/// 经过民族识别，现今的中华民族包括[汉族]、[满族]、[蒙古族]、[回族]、[藏族]、[维吾尔族]、
/// [苗族]、[彝族]、[壮族]、[布依族]、[侗族]、[瑶族]、[白族]、[土家族]、[哈尼族]、
/// [哈萨克族]、[傣族]、[黎族]、[傈僳族]、[佤族]、[畲族]、[高山族]、[拉祜族]、[水族]、
/// [东乡族]、[纳西族]、[景颇族]、[柯尔克孜族]、[土族]、[达斡尔族]、[仫佬族]、[羌族]、
/// [布朗族]、[撒拉族]、[毛南族]、[仡佬族]、[锡伯族]、[阿昌族]、[普米族]、[朝鲜族]、
/// [塔吉克族]、[怒族]、[乌孜别克族]、[俄罗斯族]、[鄂温克族]、[德昂族]、[保安族]、
/// [裕固族]、[京族]、[塔塔尔族]、[独龙族]、[鄂伦春族]、[赫哲族]、[门巴族]、[珞巴族]、
/// [基诺族]共56个民族。
///
/// [汉族]: https://baike.baidu.com/item/汉族
/// [满族]: https://baike.baidu.com/item/满族
/// [蒙古族]: https://baike.baidu.com/item/蒙古族
/// [回族]: https://baike.baidu.com/item/回族
/// [藏族]: https://baike.baidu.com/item/藏族
/// [维吾尔族]: https://baike.baidu.com/item/维吾尔族
/// [苗族]: https://baike.baidu.com/item/苗族
/// [彝族]: https://baike.baidu.com/item/彝族
/// [壮族]: https://baike.baidu.com/item/壮族
/// [布依族]: https://baike.baidu.com/item/布依族
/// [侗族]: https://baike.baidu.com/item/侗族
/// [瑶族]: https://baike.baidu.com/item/瑶族
/// [白族]: https://baike.baidu.com/item/白族
/// [土家族]: https://baike.baidu.com/item/土家族
/// [哈尼族]: https://baike.baidu.com/item/哈尼族
/// [哈萨克族]: https://baike.baidu.com/item/哈萨克族
/// [傣族]: https://baike.baidu.com/item/傣族
/// [黎族]: https://baike.baidu.com/item/黎族
/// [傈僳族]: https://baike.baidu.com/item/傈僳族
/// [佤族]: https://baike.baidu.com/item/佤族
/// [畲族]: https://baike.baidu.com/item/畲族
/// [高山族]: https://baike.baidu.com/item/高山族
/// [拉祜族]: https://baike.baidu.com/item/拉祜族
/// [水族]: https://baike.baidu.com/item/水族
/// [东乡族]: https://baike.baidu.com/item/东乡族
/// [纳西族]: https://baike.baidu.com/item/纳西族
/// [景颇族]: https://baike.baidu.com/item/景颇族
/// [柯尔克孜族]: https://baike.baidu.com/item/柯尔克孜族
/// [土族]: https://baike.baidu.com/item/土族
/// [达斡尔族]: https://baike.baidu.com/item/达斡尔族
/// [仫佬族]: https://baike.baidu.com/item/仫佬族
/// [羌族]: https://baike.baidu.com/item/羌族
/// [布朗族]: https://baike.baidu.com/item/布朗族
/// [撒拉族]: https://baike.baidu.com/item/撒拉族
/// [毛南族]: https://baike.baidu.com/item/毛南族
/// [仡佬族]: https://baike.baidu.com/item/仡佬族
/// [锡伯族]: https://baike.baidu.com/item/锡伯族
/// [阿昌族]: https://baike.baidu.com/item/阿昌族
/// [普米族]: https://baike.baidu.com/item/普米族
/// [朝鲜族]: https://baike.baidu.com/item/朝鲜族
/// [塔吉克族]: https://baike.baidu.com/item/塔吉克族
/// [怒族]: https://baike.baidu.com/item/怒族
/// [乌孜别克族]: https://baike.baidu.com/item/乌孜别克族
/// [俄罗斯族]: https://baike.baidu.com/item/俄罗斯族
/// [鄂温克族]: https://baike.baidu.com/item/鄂温克族
/// [德昂族]: https://baike.baidu.com/item/德昂族
/// [保安族]: https://baike.baidu.com/item/保安族
/// [裕固族]: https://baike.baidu.com/item/裕固族
/// [京族]: https://baike.baidu.com/item/京族
/// [塔塔尔族]: https://baike.baidu.com/item/塔塔尔族
/// [独龙族]: https://baike.baidu.com/item/独龙族
/// [鄂伦春族]: https://baike.baidu.com/item/鄂伦春族
/// [赫哲族]: https://baike.baidu.com/item/赫哲族
/// [门巴族]: https://baike.baidu.com/item/门巴族
/// [珞巴族]: https://baike.baidu.com/item/珞巴族
/// [基诺族]: https://baike.baidu.com/item/基诺族
#[derive(Copy, Clone)]
pub enum EthnicGroup {
    Han,
    Man,
    MengGu,
    Hui,
    Zang,
    WeiWuEr,
    Miao,
    Yi,
    Zhuang,
    BuYi,
    Dong,
    Yao,
    Bai,
    TuJia,
    HaNi,
    HaSaKe,
    Dai,
    Li,
    LiSu,
    Wa,
    She,
    GaoShan,
    LaHu,
    Shui,
    DongXiang,
    NaXi,
    JingPo,
    KeErKeZi,
    Tu,
    DaWoEr,
    MuLao,
    Qiang,
    BuLang,
    SaLa,
    MaoNan,
    GeLao,
    XiBo,
    AChang,
    PuMi,
    ChaoXian,
    TaJiKe,
    Nu,
    WuZiBieKe,
    ELuoSi,
    EWenKe,
    DeAng,
    BaoAn,
    YuGu,
    Jing,
    TaTaEr,
    DuLong,
    ELunChun,
    HeZhe,
    MenBa,
    LuoBa,
    JiNuo,
}

impl ToString for EthnicGroup {
    fn to_string(&self) -> String {
        match *self {
            EthnicGroup::Han => "汉".to_owned(),
            EthnicGroup::Man => "满".to_owned(),
            EthnicGroup::MengGu => "蒙古".to_owned(),
            EthnicGroup::Hui => "回".to_owned(),
            EthnicGroup::Zang => "藏".to_owned(),
            EthnicGroup::WeiWuEr => "维吾尔".to_owned(),
            EthnicGroup::Miao => "苗".to_owned(),
            EthnicGroup::Yi => "彝".to_owned(),
            EthnicGroup::Zhuang => "壮".to_owned(),
            EthnicGroup::BuYi => "布依".to_owned(),
            EthnicGroup::Dong => "侗".to_owned(),
            EthnicGroup::Yao => "瑶".to_owned(),
            EthnicGroup::Bai => "白".to_owned(),
            EthnicGroup::TuJia => "土家".to_owned(),
            EthnicGroup::HaNi => "哈尼".to_owned(),
            EthnicGroup::HaSaKe => "哈萨克".to_owned(),
            EthnicGroup::Dai => "傣".to_owned(),
            EthnicGroup::Li => "黎".to_owned(),
            EthnicGroup::LiSu => "傈僳".to_owned(),
            EthnicGroup::Wa => "佤".to_owned(),
            EthnicGroup::She => "畲".to_owned(),
            EthnicGroup::GaoShan => "高山".to_owned(),
            EthnicGroup::LaHu => "拉祜".to_owned(),
            EthnicGroup::Shui => "水".to_owned(),
            EthnicGroup::DongXiang => "东乡".to_owned(),
            EthnicGroup::NaXi => "纳西".to_owned(),
            EthnicGroup::JingPo => "景颇".to_owned(),
            EthnicGroup::KeErKeZi => "柯尔克孜".to_owned(),
            EthnicGroup::Tu => "土".to_owned(),
            EthnicGroup::DaWoEr => "达斡尔".to_owned(),
            EthnicGroup::MuLao => "仫佬".to_owned(),
            EthnicGroup::Qiang => "羌".to_owned(),
            EthnicGroup::BuLang => "布朗".to_owned(),
            EthnicGroup::SaLa => "撒拉".to_owned(),
            EthnicGroup::MaoNan => "毛南".to_owned(),
            EthnicGroup::GeLao => "仡佬".to_owned(),
            EthnicGroup::XiBo => "锡伯".to_owned(),
            EthnicGroup::AChang => "阿昌".to_owned(),
            EthnicGroup::PuMi => "普米".to_owned(),
            EthnicGroup::ChaoXian => "朝鲜".to_owned(),
            EthnicGroup::TaJiKe => "塔吉克".to_owned(),
            EthnicGroup::Nu => "怒".to_owned(),
            EthnicGroup::WuZiBieKe => "乌兹别克".to_owned(),
            EthnicGroup::ELuoSi => "俄罗斯".to_owned(),
            EthnicGroup::EWenKe => "鄂温克".to_owned(),
            EthnicGroup::DeAng => "德昂".to_owned(),
            EthnicGroup::BaoAn => "保安族".to_owned(),
            EthnicGroup::YuGu => "裕固".to_owned(),
            EthnicGroup::Jing => "京".to_owned(),
            EthnicGroup::TaTaEr => "塔塔尔".to_owned(),
            EthnicGroup::DuLong => "独龙".to_owned(),
            EthnicGroup::ELunChun => "鄂伦春".to_owned(),
            EthnicGroup::HeZhe => "赫哲".to_owned(),
            EthnicGroup::MenBa => "门巴".to_owned(),
            EthnicGroup::LuoBa => "珞巴".to_owned(),
            EthnicGroup::JiNuo => "基诺".to_owned(),
        }
    }
}
