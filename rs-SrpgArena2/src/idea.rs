#![expect(unused)]
/*
Trick with CLI parameters was to make the parameter an object, and put the type there

So you can do:
    let my_parameter: CliParameter<T> = ...
    parse(args).get(my_parameter) : T

Everything passing through f32 feels kinda weird
the alternative is as_value as_chance methods
Ooth, i16 in an enum makes it 4 bytes already...

STAT.of(stats).as_bool() is what we get then


Goal is
*/
use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Deref;

// Another idea:

// More or less a monoid
trait StatValue: Any + Clone + Debug + Default + 'static {
    fn combine(self, other: Self) -> Self;
}

// Deref for these refinement types?

#[derive(Debug, Clone, Copy, Default)]
struct Flag(bool);

impl StatValue for Flag {
    fn combine(self, other: Self) -> Self { Flag(self.0 | other.0) }
}

#[derive(Debug, Clone, Copy, Default)]
struct AttributePoints(pub i16);

impl StatValue for AttributePoints {
    fn combine(self, other: Self) -> Self {
        AttributePoints(self.0.saturating_add(other.0))
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct HitPoints(pub i16);

impl StatValue for HitPoints {
    fn combine(self, other: Self) -> Self {
        HitPoints(self.0.saturating_add(other.0))
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct Percent(i16);

impl StatValue for Percent {
    fn combine(self, other: Self) -> Self {
        Percent(self.0.saturating_add(other.0))
    }
}

enum AnyStatValue {
    Flag(Flag),
    Percent(Percent),
    HitPoints(HitPoints),
    AttributePoints(AttributePoints),
}

type StatName = &'static str;

struct Stat<S: StatValue> {
    name: StatName,
    data: PhantomData<S>,
}

impl<S: StatValue> Stat<S> {
    pub const fn new(name: StatName) -> Self {
        Stat { name, data: PhantomData }
    }

    fn of(&self, stats: &StatTable) -> S {
        stats.get(self).unwrap_or_else(S::default)
    }

    fn entry(&self, stats: &mut StatTable) -> &mut S { todo!() }
}

#[derive(Debug)]
struct StatTable {
    collection: HashMap<StatName, Box<dyn Any>>,
}

impl StatTable {
    pub fn new() -> StatTable { StatTable { collection: HashMap::new() } }

    pub fn get<T: StatValue>(&self, stat: &Stat<T>) -> Option<T> {
        Some(self.collection.get(stat.name)?.downcast_ref::<T>()?.clone())
    }

    pub fn set<T: StatValue>(&mut self, stat: &Stat<T>, value: T) {
        self.collection.insert(stat.name, Box::new(value));
    }

    pub fn combine<T: StatValue>(&mut self, stat: &Stat<T>, value: T) {
        self.set(stat, stat.of(self).combine(value));
    }

    pub fn include(&mut self, other: &StatTable) {
        let all_keys = self
            .collection
            .keys()
            .chain(other.collection.keys())
            .map(Deref::deref);

        for key in all_keys {
            let left_value = self.collection.get(key);
            let right_value = self.collection.get(key);

            todo!();
        }
    }
}

impl Clone for StatTable {
    fn clone(&self) -> Self {
        let mut copy = Self::new();
        copy.include(self);
        copy
    }
}

mod stat {
    use super::*;

    macro_rules! define_stat {
        ($name:ident, $type:ident) => {
            pub static $name: Stat<$type> = Stat::new(stringify!($name));
        };
    }

    define_stat!(VITALITY, AttributePoints);
    define_stat!(STRENGTH, AttributePoints);
    define_stat!(MAGIC, AttributePoints);
    define_stat!(DEXTERITY, AttributePoints);
    define_stat!(AGILITY, AttributePoints);
    define_stat!(LUCK, AttributePoints);
    define_stat!(DEFENSE, AttributePoints);
    define_stat!(RESISTANCE, AttributePoints);

    define_stat!(MAXIMUM_LIFE, HitPoints);
    define_stat!(PHYSICAL_DAMAGE, HitPoints);
    define_stat!(PHYSICAL_DEFENSE, HitPoints);
    define_stat!(MAGICAL_DAMAGE, HitPoints);
    define_stat!(MAGICAL_DEFENSE, HitPoints);
    define_stat!(ATTACK_SPEED, HitPoints);

    define_stat!(HIT, Percent);
    define_stat!(AVOID, Percent);
    define_stat!(CRIT, Percent);
    define_stat!(CRIT_AVOID, Percent);

    define_stat!(CRIT_BOOST, Flag);
}

#[allow(unused_variables)]
pub fn test() {
    use stat::*;

    let ref mut stats = StatTable::new();

    stats.set(&MAXIMUM_LIFE, HitPoints(10));

    let hp = MAXIMUM_LIFE.of(stats);
    let my_crit = CRIT.of(stats).0;

    todo!();
}
