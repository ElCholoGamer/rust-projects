fn main() {
    let mut s1 = String::from("Hello there");

    let len = calculate_length(&mut s1);

    // Can still use s1, it had been borrowed
    println!("The length of \"{}\" is {}", s1, len);

    // Can't use both inmutable and
    // mutable references in the
    // same scope if they're being
    // used simultaneously
}

// Takes a mutable string reference
fn calculate_length(s: &mut String) -> usize {
    s.push_str("...");
    s.len()
}
