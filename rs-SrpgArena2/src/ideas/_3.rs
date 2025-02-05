#![expect(unused)]

use std::collections::HashMap;
use std::collections::hash_map;
use std::fmt;
use std::ops::AddAssign;
use std::ops::Deref;
use std::ops::Mul;
use std::ops::MulAssign;

use num_traits::Float;

///////////
// Stats //
///////////

#[expect(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Stat {
    ////////////////
    // Attributes //
    ////////////////
    VITALITY = 1,
    STRENGTH,
    MAGIC,
    DEXTERITY,
    AGILITY,
    LUCK,
    FORTITUDE,
    RESILIENCE,

    /////////////
    // Growths //
    /////////////
    VITALITY_ATTR_GROWTH,
    STRENGTH_ATTR_GROWTH,
    MAGIC_ATTR_GROWTH,
    DEXTERITY_ATTR_GROWTH,
    AGILITY_ATTR_GROWTH,
    LUCK_ATTR_GROWTH,
    FORTITUDE_ATTR_GROWTH,
    RESILIENCE_ATTR_GROWTH,

    ///////////
    // Other //
    ///////////
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

    ///////////////
    // Keystones //
    ///////////////
    CRIT_BOOST,
    /// Attack first override.
    VANTAGE,
    WARY_FIGHTER,
}

impl fmt::Display for Stat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: use undo_screaming_snake_case
        match self {
            Self::VITALITY => "vitality",
            Self::STRENGTH => "strength",
            Self::MAGIC => "magic",
            Self::DEXTERITY => "dexterity",
            Self::AGILITY => "agility",
            Self::LUCK => "luck",
            Self::FORTITUDE => "fortitude",
            Self::RESILIENCE => "resilience",

            Self::VITALITY_ATTR_GROWTH => "vitality growth",
            Self::STRENGTH_ATTR_GROWTH => "strength growth",
            Self::MAGIC_ATTR_GROWTH => "magic growth",
            Self::DEXTERITY_ATTR_GROWTH => "dexterity growth",
            Self::AGILITY_ATTR_GROWTH => "agility growth",
            Self::LUCK_ATTR_GROWTH => "luck growth",
            Self::FORTITUDE_ATTR_GROWTH => "fortitude growth",
            Self::RESILIENCE_ATTR_GROWTH => "resilience growth",

            Self::MAXIMUM_LIFE => "maximum life",
            Self::PHYSICAL_DAMAGE => "physical damage",
            Self::PHYSICAL_DEFENSE => "physical defense",
            Self::MAGICAL_DAMAGE => "magical damage",
            Self::MAGICAL_DEFENSE => "magical defense",
            Self::ATTACK_SPEED => "attack speed",
            Self::HIT => "hit",
            Self::AVOID => "avoid",
            Self::CRIT_HIT => "critical hit",
            Self::CRIT_AVOID => "critical avoid",
            Self::CRIT_BOOST => "critical boost",

            Self::VANTAGE => "Vantage",
            Self::WARY_FIGHTER => "Wary Fighter",
        }
        .fmt(f)
    }
}

///////////////
// StatValue //
///////////////

// Maybe swap for f64 and see what happens
pub type StatValue = f32;

///////////////////
// Stat Modifier //
///////////////////

struct ModifierCondition {
    is_low_life: bool,
}

struct CompleteModifier {
    pub target: (StatValue, ModifierKind, Stat),
    pub ratio: Option<(StatValue, Stat)>,
    pub condition: Option<ModifierCondition>,
}

impl CompleteModifier {
    pub fn new(value: StatValue, kind: ModifierKind, name: Stat) -> Self {
        todo!()
    }
}

impl From<(StatValue, ModifierKind, Stat)> for CompleteModifier {
    fn from(value: (StatValue, ModifierKind, Stat)) -> Self {
        CompleteModifier { target: value, ratio: None, condition: None }
    }
}

//////////////////////////////////
// Stat Modifier Partial Result //
//////////////////////////////////

#[derive(Debug, Clone, Copy)]
pub enum ModifierKind {
    Added,
    Increased,
    More,
    Final,
}

#[derive(Debug, Clone)]
struct ModifierPartialResult {
    // 3 floats + 1 float?
    // TODO: Use simd::f32x4, storing None as NaN
    pub added: StatValue,
    pub increased: StatValue,
    pub more: StatValue,
    pub r#final: Option<StatValue>,
}

impl Default for ModifierPartialResult {
    fn default() -> Self {
        ModifierPartialResult {
            added: 0.0,
            increased: 0.0,
            more: 1.0,
            r#final: None,
        }
    }
}

fn unify_final(lhs: StatValue, rhs: StatValue) -> Option<StatValue> {
    // What is the logic here?
    // We could also panic here
    Some(lhs.max(rhs))
}

impl ModifierPartialResult {
    pub fn from_kind(kind: ModifierKind, value: StatValue) -> Self {
        let mut result = ModifierPartialResult::default();
        match kind {
            ModifierKind::Added => result.added += value,
            ModifierKind::Increased => result.increased += value,
            ModifierKind::More => result.more *= 1.0 + value,
            ModifierKind::Final => result.r#final = Some(value),
        }
        result
    }

    pub fn merge(&mut self, other: &ModifierPartialResult) {
        self.added += other.added;
        self.increased += other.increased;
        self.more *= other.more;

        self.r#final = match (self.r#final, other.r#final) {
            (None, None) => None,
            (None, Some(r)) => Some(r),
            (Some(l), None) => Some(l),
            (Some(l), Some(r)) => unify_final(l, r),
        }
    }

    pub fn finalize(self) -> StatValue {
        match self.r#final {
            Some(value) => value,
            None => self.added * (1.0 + self.increased) * self.more,
        }
    }
}

///////////////////////
// Stat Modifier Set //
///////////////////////

#[derive(Debug, Clone)]
struct ModifierSet {
    modifiers: HashMap<Stat, ModifierPartialResult>,
    // TODO: How to implement `+5 X per 2 y` mods?
}

impl ModifierSet {
    pub fn new() -> Self { ModifierSet { modifier_by_name: HashMap::new() } }

    pub fn add(&mut self, name: impl Into<CompleteModifier>) {
        use hash_map::Entry::*;
        let CompleteModifier { target, ratio, condition } = name.into();
        match self.modifier_by_name.entry(target) {
            Vacant(entry) => {
                entry.insert(effect);
            },
            Occupied(mut entry) => {
                entry.get_mut().merge(&effect);
            },
        }
    }

    fn get(&self, name: Stat) -> StatValue {
        match self.modifier_by_name.get(&name) {
            Some(effect) => effect.finalize(),
            None => StatValue::ZERO,
        }
    }
}

trait Fiddle {
    fn added(self, stat: Stat) -> CompleteModifier;
    fn increased(self, stat: Stat) -> CompleteModifier;
    fn more(self, stat: Stat) -> CompleteModifier;
}

impl Fiddle for f64 {
    fn added(self, stat: Stat) -> CompleteModifier {
        CompleteModifier::new(self, ModifierKind::Added, stat)
    }
    fn increased(self, stat: Stat) -> CompleteModifier {
        CompleteModifier::new(self, ModifierKind::Increased, stat)
    }
    fn more(self, stat: Stat) -> CompleteModifier {
        CompleteModifier::new(self, ModifierKind::More, stat)
    }
}

macro_rules! percent {
    ($l:literal %) => {
        StatValue::from_cents($l as f64)
    };
    ($l:literal) => {
        StatValue::from_whole($l as f64)
    };
}

#[cfg(test)]
mod tests {
    use super::ModifierKind::*;
    use super::Stat::*;
    use super::*;

    #[test]
    fn test() {
        let modifier = CompleteModifier::new;
        let mut mods = ModifierSet::new();

        mods.add(modifier(5.0, Added, STRENGTH));
        mods.add(modifier(0.25, Increased, STRENGTH));
        mods.add(modifier(0.10, More, STRENGTH));
        mods.add(modifier(1.0, Final, VANTAGE));
        mods.add(1.0.added(STRENGTH));
        mods.add(percent!(15%).increased(STRENGTH));

        let value = percent!(15);

        let str = mods.get(STRENGTH);

        assert_eq!(*str, 5.0);
    }
}
