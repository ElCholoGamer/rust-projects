enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let myCoin = Coin::Dime;

    let money = match myCoin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    };

    let num = plus_one(Some(2));

    if let Some(i) = num {
        println!("Num is {}", i);
    }
}

fn plus_one(num: Option<i32>) -> Option<i32> {
    match num {
        Some(i) => Some(i + 1),
        None => None,
    }
}
