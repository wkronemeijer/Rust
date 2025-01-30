#![expect(unused)]

use std::collections::HashMap;
use std::collections::hash_map;
use std::ops::AddAssign;
use std::ops::Deref;
use std::ops::Mul;
use std::ops::MulAssign;

use num_traits::Float;

#[expect(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StatName {
    CRIT_BOOST,
    /// Attack first override.
    VANTAGE,
    WARY_FIGHTER,
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

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct StatValue(f64);

impl StatValue {
    pub const ZERO: StatValue = StatValue(0.0);

    pub fn as_bool(self) -> bool { *self != 0.0 }
}

impl Deref for StatValue {
    type Target = f64;
    fn deref(&self) -> &Self::Target { &self.0 }
}

/////////
// Log //
/////////

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Log<F: Float> {
    /// Internally stored as a log of the constructed value.
    inner: F,
}

#[expect(
    non_snake_case,
    reason = "works like a tuple but must maintain invariant"
)]
pub fn Log<F: Float>(value: F) -> Log<F> { Log::new(value) }

impl<F: Float> Log<F> {
    pub fn new(value: F) -> Self { Log { inner: value.log2() } }

    pub fn linear(self) -> F { self.inner.exp2() }
}

impl<F: Float> From<F> for Log<F> {
    fn from(value: F) -> Self { Self::new(value) }
}

impl<F: Float + AddAssign> MulAssign for Log<F> {
    fn mul_assign(&mut self, rhs: Self) { self.inner += rhs.inner; }
}

impl<F: Float + AddAssign> Mul for Log<F> {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self.mul_assign(rhs);
        self
    }
}

////////////////////////////
// Stat Modifier (effect) //
////////////////////////////

#[derive(Debug, Clone, Copy)]
pub enum StatModifierKind {
    Added,
    Increased,
    More,
    Final,
}

#[derive(Debug, Clone, Default)]
struct StatModifierEffect {
    pub added: f64,
    pub increased: f64,
    pub more: Log<f64>,
    pub r#final: Option<f64>,
}

fn unify_final(lhs: f64, rhs: f64) -> Option<f64> {
    // What is the logic here?
    // We could also panic here
    Some(lhs.max(rhs))
}

impl StatModifierEffect {
    pub fn from_kind(kind: StatModifierKind, value: f64) -> Self {
        let mut result = StatModifierEffect::default();
        match kind {
            StatModifierKind::Added => result.added = value,
            StatModifierKind::Increased => result.increased = value,
            StatModifierKind::More => result.more = Log(value),
            StatModifierKind::Final => result.r#final = Some(value),
        }
        result
    }

    pub fn compute(&self) -> StatValue {
        StatValue(match self.r#final {
            Some(value) => value,
            // TODO: Replace (1 + x) formula with (1 + |x|)^sgn(x)
            None => self.added * (1.0 + self.increased) * self.more.linear(),
        })
    }

    pub fn merge(&mut self, other: &StatModifierEffect) {
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
}

#[derive(Debug, Clone)]
struct StatModifier {
    pub target: StatName,
    pub effect: StatModifierEffect,
}

impl StatModifier {
    pub fn new(value: f64, kind: StatModifierKind, name: StatName) -> Self {
        StatModifier {
            target: name,
            effect: StatModifierEffect::from_kind(kind, value),
        }
    }
}

#[derive(Debug, Clone)]
struct StatModifierSet {
    modifier_by_name: HashMap<StatName, StatModifierEffect>,
    // TODO: How to implement `+5 X per 2 y` mods?
}

impl StatModifierSet {
    pub fn new() -> Self {
        StatModifierSet { modifier_by_name: HashMap::new() }
    }

    pub fn add(&mut self, name: StatModifier) {
        use hash_map::Entry::*;
        let StatModifier { target, effect } = name;
        match self.modifier_by_name.entry(target) {
            Vacant(entry) => {
                entry.insert(effect);
            },
            Occupied(mut entry) => {
                entry.get_mut().merge(&effect);
            },
        }
    }

    fn get(&self, name: StatName) -> StatValue {
        match self.modifier_by_name.get(&name) {
            Some(effect) => effect.compute(),
            None => StatValue::ZERO,
        }
    }
}

// #[cfg(test)]
// #[test]
// fn test() {
//     use StatModifierKind::*;
//     use StatName::*;

//     let modifier = StatModifier::new;
//     let mut mods = StatModifierSet::new();

//     mods.add(modifier(5.0, Added, STRENGTH));
//     mods.add(modifier(0.25, Increased, STRENGTH));
//     mods.add(modifier(0.10, More, STRENGTH));
//     mods.add(modifier(1.0, Final, VANTAGE));

//     let str = mods.get(STRENGTH);

//     assert_eq!(*str, 5.0);
// }
