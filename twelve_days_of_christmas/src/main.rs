const PRELUDE: [&str; 2] = [
    "On the Twefth day of Christmas",
    "My good friends brought to me",
];

const PRESENTS: [&str; 12] = [
    "A song and a Christmas tree",
    "Two candy canes",
    "Three boughs of holly",
    "Four colored lights",
    "A shining star",
    "Little silver bells",
    "Candles a-glowing",
    "Gold and silver tinsel",
    "A guardian angel",
    "Some mistletoe",
    "Gifts for one and all",
    "All their good wishes",
];

fn main() {
    for index in 0..PRESENTS.len() {
        for line in PRELUDE.iter() {
            println!("{}", line);
        }

        for line_index in (0..(index + 1)).rev() {
            let line = PRESENTS[line_index];
            println!("{}", line);
        }

        println!();
    }
}
