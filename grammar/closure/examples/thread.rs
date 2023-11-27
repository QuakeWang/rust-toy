use std::thread;

fn main() {
    let s = String::from("Hello world");

    let handle = thread::spawn(move || {
        println!("moved: {:?}", s);
    });

    handle.join().unwrap();
}
