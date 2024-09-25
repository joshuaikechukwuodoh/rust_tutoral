enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn move_character(direction: Direction) {
    match direction {
        Direction::Up => println!("Moving up!"),
        Direction::Down => println!("Moving down!"),
        Direction::Left => println!("Moving left!"),
        Direction::Right => println!("Moving right!"),
    }
}

fn main() {
    let my_direction = Direction::Up;
    move_character(my_direction); // Output: Moving up!
}

