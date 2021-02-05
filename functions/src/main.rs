use std::io;

fn main() {
    print_numbers(24, 12);
}

// Function parameters
fn print_numbers(x: i32, y: i32) {
    println!("The value of X is: {}", x);
    println!("The value of Y is: {}", y);

    println!("Type a number:");

    let x = read_number();
    println!("You entered: {}", x);
}

// Functions with return values
fn read_number() -> i32 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");

    input.trim().parse().unwrap_or(0) // Tail return value
}
