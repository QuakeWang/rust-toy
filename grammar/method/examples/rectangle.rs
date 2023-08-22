#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {

    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    } 

    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle::new(5, 6);
    println!("The area of the rectangle is {} square pixels.", rect1.area());
}
