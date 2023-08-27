#[derive(Clone, Debug)]
struct Developer {
    name: String,
    age: u8,
    lang: Language,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
enum Language {
    Rust,
    Java,
    Python,
    Lua,
    Golang,
    Scala,
}

fn main() {
    let dev = Developer {
        name: "QuakeWang".to_string(),
        age: 23,
        lang: Language::Rust,
    };
    let dev1 = dev.clone();
    println!("dev: {:?}, addr of dev name: {:p}", dev, dev.name.as_str());
    println!(
        "dev1: {:?}, addr of dev1 name: {:p}",
        dev1,
        dev1.name.as_str()
    );
}
