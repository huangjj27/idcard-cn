/// 中华人民共和国中华民族接口，设计依据标准 [GB 3304-1991] 。
///
/// 经过民族识别，现今的中华民族包括[汉族]、[满族]、[蒙古族]、[回族]、[藏族]、[维吾尔族]、
/// [苗族]、[彝族]、[壮族]、[布依族]、[侗族]、[瑶族]、[白族]、[土家族]、[哈尼族]、
/// [哈萨克族]、[傣族]、[黎族]、[傈僳族]、[佤族]、[畲族]、[高山族]、[拉祜族]、[水族]、
/// [东乡族]、[纳西族]、[景颇族]、[柯尔克孜族]、[土族]、[达斡尔族]、[仫佬族]、[羌族]、
/// [布朗族]、[撒拉族]、[毛南族]、[仡佬族]、[锡伯族]、[阿昌族]、[普米族]、[朝鲜族]、
/// [塔吉克族]、[怒族]、[乌孜别克族]、[俄罗斯族]、[鄂温克族]、[德昂族]、[保安族]、
/// [裕固族]、[京族]、[塔塔尔族]、[独龙族]、[鄂伦春族]、[赫哲族]、[门巴族]、[珞巴族]、
/// [基诺族]共56个民族。[^1]
///
/// 另，GB 3304-1991 文档中对民族的翻译 “nationality” 在对外场景中更多表示为国籍含义，
/// 为避免歧义，本仓库使用 “ethnicity” 与 “ethnic" 来翻译。
///
/// [^1]: 参考自 [中华民族-百度百科](https://baike.baidu.com/item/%E4%B8%AD%E5%8D%8E%E6%B0%91%E6%97%8F/1186#6)
///
/// [GB 3304-1991]: http://openstd.samr.gov.cn/bzgk/gb/newGbInfo?hcno=E5C3271B62636C5DA6853A0DA23EBBA9
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
pub trait Ethnic {
    type Code;
    type Name;
    type RomanCode;
    type ShortCode;

    /// 数字代码
    fn code(&self) -> Self::Code;

    /// 民族名称
    fn name(&self) -> Self::Name;

    /// 罗马字母拼写法
    fn roman(&self) -> Self::RomanCode;

    /// 字母代码
    fn short(&self) -> Self::ShortCode;
}
