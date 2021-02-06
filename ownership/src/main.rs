fn main() {
    let s1 = String::from("Hello there");
    let s2 = s1;

    // Can't use s1 anymore, it has been "moved" to s2
    println!("s2 = {}", s2);

    let s3 = s2.clone();

    // Can still use s2, it hasn't been "moved"
    println!("s2 = {}, s3 = {}", s2, s3);

    // Some copiable types:
    // - integers (i32, u64, etc)
    // - floating point numbers (f64, etc)
    // - bool
    // - char
    // - tuples (If all of its elements are copiable)

    let s4 = String::from("kekw");
    takes_ownership(s4);
    // Can't use s4 anymore

    let num = 5;
    makes_copy(5);
    println!("I can still use num, its value is: {}", num);

    // Reset shit
    drop(s2);
    drop(s3);
    drop(num);

    let s1 = gives_ownership();
    let s2 = String::from("Poggers");

    let s3 = takes_and_gives_back(s2);
    // Can't use s2 anymore,
    // it has been moved to
    // the function and back to s3
}

fn takes_and_gives_back(string: String) -> String {
    string
}

fn takes_ownership(string: String) {
    println!("I took ownership of: {}", string);
}

fn makes_copy(num: i32) {
    println!("I received a copy of number: {}", num)
}

fn gives_ownership() -> String {
    let some_string = String::from("You are a bold one");
    some_string
}
