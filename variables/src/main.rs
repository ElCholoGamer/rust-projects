fn main() {
    // Variable shadowing
    let x = -5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let mut spaces = "   ";
    // spaces = spaces.len(); // Error: Cannot change the type of spaces
    println!("Spaces: {}", spaces);

    // Tuples
    let tup = (500, 6.4, 1);
    let (first, second, third) = tup; // Tuple destructuring

    println!("Elements: {}, {}, {}", first, second, third);
    println!("The first element is: {}", tup.0);

    // Arrays
    let five_unsigned_ints: [u32; 5] = [1, 5, 2, 3, 7];
    let three_fours = [4; 3];

    let first = five_unsigned_ints[0];
    let second = five_unsigned_ints[1];

    // Access an element by index
    let index = 2;
    let element = three_fours[index];

    println!("Element at index {}: {}", index, element);
}
