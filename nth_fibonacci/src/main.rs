use std::io::stdin;

fn main() {
    println!("Welcome to the Fibonacci calculator!");
    println!("Type the Fibonacci position you would like to calculate:");

    let mut pos: i32;

    loop {
        pos = read_number();
        if pos > 0 {
            break;
        } else {
            println!("Type a valid position!")
        }
    }

    let result = fibonacci(pos);
    println!("Result: {}", result);
}

fn fibonacci(pos: i32) -> i32 {
    if pos <= 2 {
        1
    } else {
        fibonacci(pos - 2) + fibonacci(pos - 1)
    }
}

fn read_number() -> i32 {
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Failed to read number");

    input.trim().parse().unwrap_or(0)
}
