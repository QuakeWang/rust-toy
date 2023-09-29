fn main() {
    // 使用 + 或者 += 連接字符串
    let string_append = String::from("hello ");
    let string_rust = String::from("rust");
    // &string_rust 会自动解引用为 &str
    let result = string_append + &string_rust;
    let mut result = result + "!"; // `result + "!"` 中的 `result` 是不可变的
    result += "!!!";

    println!("连接字符串 + -> {}", result);

    // 使用 format! 連接字符串
    let s1 = "hello";
    let s2 = String::from("rust");
    let s = format!("{} {}!", s1, s2);
    println!("{}", s);
}
