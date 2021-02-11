enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn send(&self) {
        match self {
            Message::Write(text) => println!("Sending message \"{}\"", text),
            Message::ChangeColor(r, g, b) => {
                println!("Changing color to R: {}, G: {}, B: {}", r, g, b)
            }
            _ => println!("Not doing anything."),
        }
    }
}

fn main() {
    let msg = Message::Write(String::from("hello there"));
    msg.send();

    // Null-ish values with Option<T>
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y; // Can't compile, 'y' might be None
}
