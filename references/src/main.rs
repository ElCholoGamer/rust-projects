fn main() {
    let mut s1 = String::from("Hello there");

    let len = calculate_length(&mut s1);

    // Can still use s1, it had been borrowed
    println!("The length of \"{}\" is {}", s1, len);

    let mut s2 = String::from("This is were the fun begins");

    // Can't use both inmutable and
    // mutable references in the
    // same scope if they're being
    // used simultaneously

    let r1 = &s2; // no problem
    let r2 = &s2; // no problem
    // let r3 = &mut s2; // BIG PROBLEM

    // println!("{}, {}, and {}", r1, r2, r3);
}

// Takes a mutable string reference
fn calculate_length(s: &mut String) -> usize {
    s.push_str("...");
    s.len()
}
