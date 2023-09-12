use serde::Deserialize;
use std::borrow::Cow;

#[derive(Debug, Deserialize)]
struct User<'input> {
    #[serde[borrow]]
    name: Cow<'input, str>,
    age: u8,
}

fn main() {
    let input = r#"{"name": "QuakeWang", "age": "23"}"#;
    let user: User = serde_json::from_str(input).unwrap();

    match user.name {
        Cow::Borrowed(x) => println!("borrowed {}", x),
        Cow::Owned(x) => println!("owned {}", x),
    }
}