use std::collections::HashMap;
use std::marker::PhantomData;

/*
Trick with CLI parameters was to make the parameter an object, and put the type there


So you can do:
    let my_parameter: CliParameter<T> = ...
    parse(args)[my_parameter] : T

Maybe something like that can work again


Everything passing through f32 feels kinda weird
the alternative is as_value as_chance methods
*/

// More or less a monoid
trait StatValue: Copy {
    fn default() -> Self;
    fn combine(self, other: Self) -> Self;
}

// Deref for these refinement types?

#[derive(Debug, Clone, Copy)]
struct Flag {
    raw: bool,
}

#[allow(non_snake_case)]
fn Flag(raw: bool) -> Flag {
    Flag { raw }
}

impl StatValue for Flag {
    fn default() -> Self {
        Flag(false)
    }

    fn combine(self, other: Self) -> Self {
        Flag(self.raw | other.raw)
    }
}

#[derive(Debug, Clone, Copy)]
struct Value {
    raw: i16,
}

#[allow(non_snake_case)]
fn Value(raw: i16) -> Value {
    Value { raw }
}

impl StatValue for Value {
    fn default() -> Self {
        Value(0)
    }

    fn combine(self, other: Self) -> Self {
        Value(self.raw.saturating_add(other.raw))
    }
}

#[derive(Debug, Clone, Copy)]
struct Chance {
    raw: i16,
}

#[allow(non_snake_case)]
fn Chance(raw: i16) -> Chance {
    Chance { raw }
}

impl StatValue for Chance {
    fn default() -> Self {
        Chance(0)
    }

    fn combine(self, other: Self) -> Self {
        Chance(self.raw.saturating_add(other.raw))
    }
}

enum AnyStat {
    Flag(Flag),
    Value(Value),
    Chance(Chance),
}

impl AnyStat {
    fn combine(lhs: &Option<AnyStat>, rhs: &Option<AnyStat>) -> AnyStat {
        todo!()
        // too
    }
}

type Test = Option<AnyStat>;

struct Stat<T> {
    name: &'static str,
    param: PhantomData<T>,
}

impl<T> Stat<T> {
    pub const fn new(name: &'static str) -> Self {
        Stat {
            name,
            param: PhantomData,
        }
    }
}

struct Stats {
    collection: HashMap<&'static str, AnyStat>,
}

impl Stats {
    fn new() -> Stats {
        Stats {
            collection: HashMap::new(),
        }
    }

    fn include(&mut self, other: &Stats) {
        // Notes:
        // lhs may have keys rhs doesnt and vice versa

        todo!()
    }

    fn get<T: StatValue>(&self, stat: &Stat<T>) -> Option<T> {
        todo!()
    }

    fn set<T: StatValue>(&mut self, stat: &Stat<T>, value: T) {}
}

impl<S: StatValue> Stat<S> {
    fn of(&self, stats: &Stats) -> S {
        stats.get(self).unwrap_or_else(S::default)
    }

    fn entry(&self, stats: &mut Stats) -> &mut S {
        todo!()
    }
}

#[allow(non_upper_case_globals)]
mod stat {
    use super::*;

    pub static maximum_life: Stat<Value> = Stat::new("maximum_life");
    pub static crit: Stat<Chance> = Stat::new("crit");
}

pub fn test() {
    use stat::*;

    let mut stats = Stats::new();
    let stats = &mut stats;

    stats.set(&maximum_life, Value(10));
    *maximum_life.entry(stats) = Value(10);

    let hp = maximum_life.of(stats);
    let my_crit = crit.of(stats).raw;

    todo!()
}
