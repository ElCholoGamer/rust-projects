#[allow(unused)]

fn main() {
    let s = String::from("hello world");
    let word = first_word(&s);

    println!("The first word is \"{}\"", word);

    let mut arr = [1, 2, 3, 4, 5, 6];
    let part = &arr[..2];

    // Can't modify array because it
    // has been borrowed first
    // arr[0] = 7;

    println!("Array part: {:?}", part);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
