use std::io;
use rand::Rng;

pub fn read() -> String {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    input.trim().to_string()

}

pub fn randint(start: i32, end: i32) -> i32 {

    rand::thread_rng().gen_range(start..=end)

}
