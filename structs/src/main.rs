struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(username: String, email: String) -> User {
    User {
        username: String::from(username),
        email: String::from(email),
        sign_in_count: 1,
        active: true,
    }
}

// Tuple struct
struct Color(u8, u8, u8);

fn main() {
    println!("Hello, world!");

    let user1 = build_user(
        String::from("ElCholoGamer"),
        String::from("email@example.com"),
    );

    let user2 = User {
        username: String::from("AnotherUsername"),
        email: String::from("anotheremail@example.com"),
        ..user1
    };

    let red = Color(255, 0, 0);
    let white = Color(255, 255, 255);
    let black = Color(0, 0, 0);

    // Destructure a tuple struct
    let Color(r, g, b) = red;
}
