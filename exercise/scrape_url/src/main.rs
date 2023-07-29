use std::fs;

fn main() {
    let url = "https://www.rust-lang.org/";
    let output = "rust.md";
    println!("Fetching url: {}", url);

    let body = reqwest::blocking::get(url).unwrap().text().unwrap();
    
    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes()).unwrap();
    println!("Converting markdown has benn saved in {}.", output);
}
