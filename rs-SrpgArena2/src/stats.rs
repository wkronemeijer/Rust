use std::marker::PhantomData;
use std::ops::AddAssign;
use std::ops::BitOrAssign;

///////////////
// Resources //
///////////////

#[derive(Debug)]
pub struct Resources {
    pub level: i16,
    pub life: i16,
    pub energy: i16,
}

impl Default for Resources {
    fn default() -> Self { Resources { level: 1, life: 1, energy: 0 } }
}

///////////
// Stats //
///////////

#[derive(Debug, Default, Clone)]
pub struct Stats {
    // Why merge attributes and stats?
    // ...so we can do skill checks with the modified attributes
    pub vitality: i16,
    pub strength: i16,
    pub magic: i16,
    pub skill: i16,
    pub speed: i16,
    pub luck: i16,
    pub defense: i16,
    pub resistance: i16,

    // Why put growths here?
    // Because items can influence growths
    // (FE5 crusader scrolls come to mind)
    pub vitality_growth: i16,
    pub strength_growth: i16,
    pub magic_growth: i16,
    pub skill_growth: i16,
    pub speed_growth: i16,
    pub luck_growth: i16,
    pub defense_growth: i16,
    pub resistance_growth: i16,

    pub maximum_life: i16,

    pub phys_damage: i16,
    pub phys_defense: i16,
    pub mag_damage: i16,
    pub mag_defense: i16,

    pub attack_speed: i16,

    pub hit: i16,
    pub avoid: i16,
    pub crit: i16,
    pub crit_avoid: i16,

    pub critical_boost: bool,
}

// If stats grows too big, we can split up data and store only a fraction
// This is how it merges data.
pub trait AddToStats {
    fn add_to_stats(&self, stats: &mut Stats);
}

impl Stats {
    pub fn include(&mut self, other: &impl AddToStats) {
        other.add_to_stats(self);
    }
}

impl AddToStats for Stats {
    fn add_to_stats(&self, stats: &mut Stats) {
        // Not much shorted, but should be less error-prone
        macro_rules! once {
            ($owner:ident, $op:ident, $field:ident) => {
                $owner.$field.$op($field);
                // replace field so duplicates cause an error
                #[allow(unused_variables)]
                let $field = ();
            };
        }

        let Stats {
            vitality,
            strength,
            magic,
            skill,
            speed,
            luck,
            defense,
            resistance,

            vitality_growth,
            strength_growth,
            magic_growth,
            skill_growth,
            speed_growth,
            luck_growth,
            defense_growth,
            resistance_growth,

            maximum_life,
            phys_damage,
            phys_defense,
            mag_damage,
            mag_defense,
            attack_speed,
            hit,
            avoid,
            crit,
            crit_avoid,

            critical_boost,
        } = self;

        once!(stats, add_assign, vitality);
        once!(stats, add_assign, strength);
        once!(stats, add_assign, magic);
        once!(stats, add_assign, skill);
        once!(stats, add_assign, speed);
        once!(stats, add_assign, luck);
        once!(stats, add_assign, defense);
        once!(stats, add_assign, resistance);

        once!(stats, add_assign, vitality_growth);
        once!(stats, add_assign, strength_growth);
        once!(stats, add_assign, magic_growth);
        once!(stats, add_assign, skill_growth);
        once!(stats, add_assign, speed_growth);
        once!(stats, add_assign, luck_growth);
        once!(stats, add_assign, defense_growth);
        once!(stats, add_assign, resistance_growth);

        once!(stats, add_assign, maximum_life);
        once!(stats, add_assign, phys_damage);
        once!(stats, add_assign, phys_defense);
        once!(stats, add_assign, mag_damage);
        once!(stats, add_assign, mag_defense);
        once!(stats, add_assign, attack_speed);
        once!(stats, add_assign, hit);
        once!(stats, add_assign, avoid);
        once!(stats, add_assign, crit);
        once!(stats, add_assign, crit_avoid);

        once!(stats, bitor_assign, critical_boost);
    }
}

pub struct FinalStats<'a> {
    source: PhantomData<&'a ()>,
    stats: Stats,
}

impl<'a> FinalStats<'a> {
    pub fn stats(&self) -> &Stats { &self.stats }
}

impl Stats {
    pub(crate) fn compute(self: &mut Stats) -> FinalStats {
        let mut this = self.clone();

        this.maximum_life += 1 + 2 * this.vitality;

        this.phys_damage += this.strength;
        this.phys_defense += this.defense;
        this.mag_damage += this.magic;
        this.mag_defense += this.resistance;

        this.attack_speed += this.speed;

        this.hit += this.skill;
        this.avoid += this.speed;
        this.crit += this.luck / 2;
        this.crit_avoid += this.luck / 4;

        FinalStats { source: PhantomData, stats: this }
    }
}
