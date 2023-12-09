macro_rules! add {
    ($($a:expr),*) => {
        {
            0
            $(+$a)*
        }
    };
}

fn main() {
    let sum = add!(1, 2, 3);
    println!("sum: {}", sum);
}
