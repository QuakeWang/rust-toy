fn main() {
    // 对于字符串而言，切片就是对 string 类型中某一部分的引用
    let s = String::from("hello world");
    // 创建切片的语法：[开始索引..终止索引]，右半开区间
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);

    // 如果想要截取完整的 string 切片
    let len = s.len();
    let slice1 = &s[0..len];
    let slice2 = &s[..];
    println!("slice1: {}, slice2: {}", slice1, slice2);
    assert_eq!(slice1 == slice2, true);
}
