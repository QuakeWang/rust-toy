use std::fs;

// 把错误封装在 Result<T, E> 类型中，同时提供了 ? 操作符来传播错误，
// Result<T, E> 类型是一个泛型数据结构，T 代表成功执行返回的结果类型，E 代表错误类型
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://www.rust-lang.org/";
    let output = "rust.md";
    println!("Fetching url: {}", url);

    let body = reqwest::blocking::get(url)?.text()?;

    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes())?;
    println!("Converting markdown has benn saved in {}.", output);

    Ok(())
}
