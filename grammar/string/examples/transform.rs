fn main() {
    // &str 和 String 类型的转换
    let s = String::from("hello world");
    say_hello(&s);
    say_hello(&s[..]);
    say_hello(s.as_str());
}

fn say_hello(s: &str) {
    println!("{}", s)
}
