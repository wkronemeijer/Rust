use derive_builder::Builder;
use std::collections::HashMap;
use BaseStat::*;

struct ClampedU8(u8);

struct Percentage(u8);

impl Percentage {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum BaseStat {
    Constitution,
    Strength,
    Magic,
    Skill,
    Speed,
    Luck,
    Defense,
    Resistance,
}

// can't iterate over plain enum values at any rate...

impl BaseStat {
    fn abbreviation(self) -> &'static str {
        match self {
            Constitution => "CON",
            Strength => "STR",
            Magic => "MAG",
            Skill => "SKL",
            Speed => "SPD",
            Luck => "LCK",
            Defense => "DEF",
            Resistance => "RES",
        }
    }
}

#[derive(Debug, Clone)]
struct AttributeInfo {
    map: HashMap<BaseStat, i32>,
}

enum UnitResource {
    Life,
    Energy,
    Stamina,
    Mana,
}

#[derive(Debug, Builder)]
struct Unit {
    name: String,
    life: i32,
    attributes: AttributeInfo,
}

fn test() {
    let my_unit = UnitBuilder::default().name("lel".to_string());
}
