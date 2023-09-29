fn main() {
    let name = "Shhhhh.";
    // greet(name); // error code
    greet(name.to_string());
}

fn greet(name: String) {
    println!("Hello, {}", name);
}
