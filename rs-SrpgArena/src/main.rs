mod error;
mod game;

use std::io::{stdin, BufRead};

fn main() {
    let mut lines = stdin().lock().lines();
    loop {
        let Some(Ok(line)) = lines.next() else {
            break;
        };
        if line == ".exit" {
            break;
        }
        println!("Hello, \x1B[1m{}\x1b[0m!", line);
    }
}
