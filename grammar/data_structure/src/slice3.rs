fn main() {
    let result = vec![1, 2, 3, 4]
        .iter()
        .map(|v| v * v)
        .filter(|v| *v < 12)
        .take(2)
        .collect::<Vec<_>>();

    println!("{:?}", result);
}