#![expect(unused)]

use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Index;

///////////
// Stats //
///////////

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum StatKind {
    Bool,
    Int,
}

#[derive(Debug, Clone)]
enum StatValue {
    Bool(bool),
    Int(i16),
}

impl StatValue {
    pub fn kind(&self) -> StatKind {
        match self {
            StatValue::Bool(_) => StatKind::Bool,
            StatValue::Int(_) => StatKind::Int,
        }
    }
}

#[expect(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum BoolStatName {
    CRIT_BOOST,
    /// Attack first override.
    VANTAGE,
    WARY_FIGHTER,
}

#[expect(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum IntStatName {
    // Attributes
    VITALITY,
    STRENGTH,
    MAGIC,
    DEXTERITY,
    AGILITY,
    LUCK,
    FORTITUDE,
    RESILIENCE,
    // Growths
    VITALITY_ATTR_GROWTH,
    STRENGTH_ATTR_GROWTH,
    MAGIC_ATTR_GROWTH,
    DEXTERITY_ATTR_GROWTH,
    AGILITY_ATTR_GROWTH,
    LUCK_ATTR_GROWTH,
    FORTITUDE_ATTR_GROWTH,
    RESILIENCE_ATTR_GROWTH,
    // Other
    MAXIMUM_LIFE,
    PHYSICAL_DAMAGE,
    PHYSICAL_DEFENSE,
    MAGICAL_DAMAGE,
    MAGICAL_DEFENSE,
    ATTACK_SPEED,
    HIT,
    AVOID,
    CRIT_HIT,
    CRIT_AVOID,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum StatName {
    Bool(BoolStatName),
    Int(IntStatName),
}

impl StatName {
    pub fn kind(&self) -> StatKind {
        match self {
            StatName::Bool(_) => StatKind::Bool,
            StatName::Int(_) => StatKind::Int,
        }
    }
}

pub mod stat_names {
    pub use super::BoolStatName::*;
    pub use super::IntStatName::*;
}

///////////
// Table //
///////////

struct StatTable {
    pub data: HashMap<StatName, StatValue>,
}

impl StatTable {
    pub fn new() -> Self { StatTable { data: HashMap::new() } }

    pub fn get_any(&self, key: StatName) -> Option<StatValue> {
        self.data.get(&key).cloned()
    }

    pub fn set_any(&mut self, key: StatName, value: StatValue) {
        debug_assert!(
            key.kind() == value.kind(),
            "kinds disagree for {key:?} and {value:?}"
        );
        self.data.insert(key, value);
    }
}

trait Extractable {
    type Output;
    fn extract(&self, table: &StatTable) -> Self::Output;

    fn insert(self, table: &mut StatTable, value: Self::Output);
}

macro_rules! impl_extractable {
    ($type:ident, $variant:ident, $output:ty) => {
        impl Extractable for $type {
            type Output = $output;

            fn extract(&self, table: &StatTable) -> Self::Output {
                match table.get_any(StatName::$variant(*self)) {
                    Some(StatValue::$variant(x)) => x,
                    _ => Default::default(),
                }
            }

            fn insert(self, table: &mut StatTable, value: Self::Output) {
                table.set_any(
                    StatName::$variant(self),
                    StatValue::$variant(value),
                );
            }
        }
    };
}

impl_extractable!(BoolStatName, Bool, bool);
impl_extractable!(IntStatName, Int, i16);

impl StatTable {
    pub fn get<E: Extractable>(&self, name: E) -> E::Output {
        name.extract(self)
    }

    pub fn set<E: Extractable>(&mut self, name: E, value: E::Output) {
        name.insert(self, value)
    }
}

///////////////
// Modifiers //
///////////////

// Some kind of trait for picking the correct modifiers?
// And what of incorrect modifiers: do we Err? debug_assert? println?
// TO BE CONTINUED

struct PassiveModifier {}

struct ProportionalModifier {}

struct ConditionalModifier {}

struct StatModifierSet {}

impl StatModifierSet {
    pub fn new() -> Self { StatModifierSet {} }
}

impl StatTable {
    pub fn compute(&self, modifiers: &StatModifierSet) -> StatTable {
        todo!();
    }
}

pub struct CachedModifierSet {
    was_modified: bool,
}

/////////////
// Example //
/////////////

fn test() {
    use stat_names::*;
    let table = StatTable::new();
    let mods = StatModifierSet::new();

    let do_vantage = table.get(VANTAGE);
    VANTAGE.extract(&table);

    let str = table.get(STRENGTH);

    todo!()
}
