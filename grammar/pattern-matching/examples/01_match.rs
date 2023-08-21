enum Direction {
    East,
    West,
    South,
    North,
}

fn main() {
    let dire = Direction::West;
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("North or South");
        }
        _ => println!("West"),
    }
}
