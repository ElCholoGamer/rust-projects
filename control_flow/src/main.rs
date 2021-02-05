use std::io::stdin;

fn main() {
    let num = read_number("Type a number:");

    // if Expressions
    if num != 0 {
        println!("The number is not zero")
    }

    if num < 5 {
        println!("The number is less than 5");
    } else {
        println!("The number is greated than 5");
    }

    // "Ternary" operators (ew)
    let is_big = if num > 100 { "yes" } else { "no" };
    println!("Is the number big? {}", is_big);

    // Infinite loop till a 'break' statement
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter >= 10 {
            break counter * 2;
        }
    };
    println!("Result after loop: {}", result);

    // while Loops
    counter = 5;
    while counter > 0 {
        println!("Iteration #{}", counter);
        counter -= 1;
    }

    let nums = [5, 2, 3, 6, 1];

    for num in nums.iter() {
        println!("Number: {}", num);
    }
}

fn read_number(message: &str) -> i32 {
    if message != "" {
        println!("{}", message);
    }

    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Failed to read number");

    input.trim().parse().unwrap_or(0)
}
