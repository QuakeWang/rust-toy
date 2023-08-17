fn main() {
    match_case();

    let s1 = String::from("hello world");
    let (s2, len) = calculate_length(s1);
    println!("The value of '{}' is {}", s2, len);
}

// 使用模式匹配结构元组
fn match_case() {
    let tup = (100, 3.14, 1);
    let (x, y, z) = tup;
    println!("Ths value of x: {}, y: {}, z: {}", x, y, z);
}

// calculate_length 函数接收 s1 字符串的所有权，然后计算字符串的长度，接着把字符串所有权和字符串长度再返回给 s2 和 len 变量。
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
