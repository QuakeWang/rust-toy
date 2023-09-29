fn _define_statement() {
    let a = 8;
    let b: Vec<f64> = Vec::new();
    let (a, c) = ("hi", false);

    println!("a: {}, b: {}, c: {}", a, b, c);
}

fn _define_expression() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}
