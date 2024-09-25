fn main() {
    let some_number = Some(5);
    let no_number: Option<i32> = None;

    // Using Option in a match statement
    match some_number {
        Some(x) => println!("The number is: {}", x),
        None => println!("No number found"),
    }

    match no_number {
        Some(x) => println!("The number is: {}", x),
        None => println!("No number found"),
    }
}
