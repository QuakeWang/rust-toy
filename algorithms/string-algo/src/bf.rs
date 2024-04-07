fn bf(primary: &str, pattern: &str) -> i32 {
    
    if primary.is_empty() || pattern.is_empty() || pattern.len() > primary.len() {
        return -1;
    }

    let primary_chars = primary.chars().collect::<Vec<char>>();
    let pattern_chars = pattern.chars().collect::<Vec<char>>();
    for i in 0..(primary.len() - pattern.len() + 1) {
        if pattern_chars == primary_chars[i..i + pattern.len()].to_vec() {
            return i as i32;
        }
    }
    -1
}

fn main() {
    let primary = "thequickbrownfoxjumpsoverthelazydog";
    let pattern = "lazy";
    let result = bf(primary, pattern);
    println!("{}", result);
}
