use std::{
    collections::HashMap,
    io::{stdin, BufRead},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum BaseStat {
    MaximumLife,
    Strength,
    Magic,
    Skill,
    Speed,
    Luck,
    Defense,
    Resistance,
}

impl BaseStat {
    fn abbreviation(self) -> &'static str {
        match self {
            BaseStat::MaximumLife => "CON",
            BaseStat::Strength => "STR",
            BaseStat::Magic => "MAG",
            BaseStat::Skill => "SKL",
            BaseStat::Speed => "SPD",
            BaseStat::Luck => "LCK",
            BaseStat::Defense => "DEF",
            BaseStat::Resistance => "RES",
        }
    }
}

struct AttributeInfo {
    map: HashMap<BaseStat, i32>,
}

struct Unit<'a> {
    name: &'a str,
    life: i32,
    attributes: AttributeInfo,
}

fn main() {
    let input = stdin();
    for maybe_line in input.lock().lines() {
        let Ok(line) = maybe_line else {
            break;
        };
        if line == ".exit" {
            break;
        }

        let message = format!("Hello, {}", line);

        println!("Hello, \x1B[1m{}\x1b[0m!", line);
    }
}
