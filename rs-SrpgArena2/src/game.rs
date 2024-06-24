use std::ops::{AddAssign, BitOrAssign};

use rand::{thread_rng, Rng};

use crate::core::slice_get_many_mut;

////////////////////////
// Shared definitions //
////////////////////////

pub struct Resources {
    pub level: i16,
    pub life: i16,
    pub energy: i16,
}

impl Default for Resources {
    fn default() -> Self {
        Resources {
            level: 1,
            life: 1,
            energy: 0,
        }
    }
}

#[derive(Default, Clone)]
pub struct Stats {
    is_final: bool,
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
            is_final,

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
        assert!(!is_final, "tried to add final stats");

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

impl Stats {
    fn compute(&mut self) {
        assert!(!self.is_final);

        self.maximum_life += 1 + 2 * self.vitality;

        self.phys_damage += self.strength;
        self.phys_defense += self.defense;
        self.mag_damage += self.magic;
        self.mag_defense += self.resistance;

        self.attack_speed += self.speed;

        self.hit += self.skill;
        self.avoid += self.speed;
        self.crit += self.luck / 2;
        self.crit_avoid += self.luck / 4;

        self.is_final = true;
    }
}

#[derive(Clone)]
pub enum EquipmentSlot {
    Hand,
    Chest,
    Finger,
}

#[derive(Clone)]
pub struct EquipmentItem {
    name: String,
    pub slot: EquipmentSlot,
    pub attributes: Stats,
}

impl EquipmentItem {
    pub fn name(&self) -> &str {
        &self.name
    }
}

// TODO: Support equipped and unequipped items
// TODO: Support non-equipment items
// #[derive(Default, Clone, Copy)]
// pub enum EquippedState {
//     #[default]
//     Unequipped,
//     Equipped,
// }

// We have more kinds of equipment after all
const INVENTORY_SIZE_LIMIT: usize = 8;

#[derive(Default)]
pub struct UnitInventory {
    items: [Option<EquipmentItem>; INVENTORY_SIZE_LIMIT],
}

impl UnitInventory {
    pub fn new() -> Self {
        UnitInventory {
            items: [const { None }; INVENTORY_SIZE_LIMIT],
        }
    }

    pub fn equipped_items(&self) -> impl Iterator<Item = &EquipmentItem> {
        self.items.iter().filter_map(|item| item.as_ref())
    }

    // Somehow, I need references to specific items
    // ...but we don't have specific slots...
}

trait HitFn {
    fn did_hit<R: Rng>(rng: &mut R, chance: i16) -> bool;

    fn overflowing_chance_hit<R: Rng>(rng: &mut R, chance: i16) -> i16 {
        if chance > 100 {
            1 + Self::overflowing_chance_hit(rng, chance - 100)
        } else {
            if Self::did_hit(rng, chance) {
                1
            } else {
                0
            }
        }
    }
}

struct SingleHit;

impl HitFn for SingleHit {
    fn did_hit<R: Rng>(rng: &mut R, chance: i16) -> bool {
        rng.gen_range(0..100) < chance
    }
}

struct DoubleHit;

impl HitFn for DoubleHit {
    fn did_hit<R: Rng>(rng: &mut R, chance: i16) -> bool {
        let mut roll = || rng.gen_range(0..100);
        let average = (roll() + roll()) / 2;
        average < chance
    }
}

fn single_rng_hit<R: Rng>(random: &mut R, chance: i16) -> bool {
    random.gen_range(0..100) < chance
}

fn double_rng_hit<R: Rng>(random: &mut R, chance: i16) -> bool {
    let mut roll = || random.gen_range(0..100);
    let average = (roll() + roll()) / 2;
    average < chance
}

fn overflowing_chance_hit<R: Rng>(random: &mut R, mut chance: i16) -> i16 {
    let mut result = 0;
    loop {
        if chance >= 100 {
            result += 1;
            chance -= 100;
        } else {
            if single_rng_hit(random, chance) {
                result += 1;
            }
            return result;
        }
    }
}

//////////////////
// Observer API //
//////////////////

#[derive(Clone, Copy)]
pub enum HitKind {
    Hit,
    Crit,
}

impl HitKind {
    fn from_did_crit(did_crit: bool) -> Self {
        if did_crit {
            HitKind::Crit
        } else {
            HitKind::Hit
        }
    }
}

pub struct HitEvent<'a> {
    pub kind: HitKind,
    pub attacker: &'a Unit,
    pub defender: &'a Unit,
    pub is_killing_blow: bool,
    pub damage_dealt: i16,
}

pub struct MissEvent<'a> {
    pub attacker: &'a Unit,
    pub defender: &'a Unit,
}

pub struct DamageEvent<'a> {
    pub unit: &'a Unit,
    pub amount: i16,
}

pub struct HealEvent<'a> {
    pub unit: &'a Unit,
    pub amount: i16,
}

pub struct DeathEvent<'a> {
    pub unit: &'a Unit,
}

pub trait EventDelegate {
    // more or less "Arena Event Delegate"
    fn on_hit(&mut self, event: HitEvent);
    fn on_miss(&mut self, event: MissEvent);
    fn on_life_lost(&mut self, event: DamageEvent);
    fn on_life_gained(&mut self, event: HealEvent);
    fn on_death(&mut self, event: DeathEvent);
}

pub type Observer<'a> = &'a mut dyn EventDelegate;

pub struct PrintLnDelegate;

impl EventDelegate for PrintLnDelegate {
    fn on_hit(&mut self, event: HitEvent) {
        match event.kind {
            HitKind::Hit => println!(
                "{} hits {} for {} damage",
                event.attacker.name, event.defender.name, event.damage_dealt
            ),
            HitKind::Crit => println!(
                "{} crits {} for {} damage!",
                event.attacker.name, event.defender.name, event.damage_dealt
            ),
        }
    }

    fn on_miss(&mut self, event: MissEvent) {
        println!("{} misses {}", event.attacker.name, event.defender.name);
    }

    fn on_life_lost(&mut self, event: DamageEvent) {
        println!("{} is left with {} HP", event.unit.name, event.unit.resources().life);
    }

    fn on_life_gained(&mut self, event: HealEvent) {
        println!(
            "{} is healed back up to {} HP",
            event.unit.name,
            event.unit.resources().life
        );
    }

    fn on_death(&mut self, event: DeathEvent) {
        println!("{} has died.", event.unit.name);
    }
}

//////////
// Unit //
//////////

#[derive(Default)]
pub struct Unit {
    name: String,
    is_alive: bool, // seperate because you can heal a corpse
    resources: Resources,
    attributes: Stats,
    pub equipment: Box<UnitInventory>,
}

impl Unit {
    pub fn new(name: impl Into<String>, attributes: Stats) -> Self {
        let mut result = Unit {
            name: name.into(),
            is_alive: false,
            resources: Resources::default(),
            attributes,
            equipment: Box::new(UnitInventory::new()),
        };
        result.revive();
        result
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_alive(&self) -> bool {
        self.is_alive
    }

    pub fn resources(&self) -> &Resources {
        &self.resources
    }

    pub fn attributes(&self) -> &Stats {
        &self.attributes
    }

    fn collect_stats(&self) -> Stats {
        let mut stats = Stats::default();
        stats.include(&self.attributes);
        for item in self.equipment.equipped_items() {
            stats.include(&item.attributes);
        }
        stats
    }

    fn final_stats(&self) -> Stats {
        let mut stats = self.collect_stats();
        stats.compute();
        stats
    }

    pub fn revive(&mut self) {
        let stats = self.final_stats();
        self.resources.life = stats.maximum_life;
        self.is_alive = true;
    }
}

pub enum UnitRelationKind {
    Ally,
    Neutral,
    Enemy,
}

impl Unit {
    pub fn relation(&self, _: &Unit) -> UnitRelationKind {
        // KILL THEM ALL MUHAHAHA
        UnitRelationKind::Enemy
    }

    pub fn can_help(&self, other: &Unit) -> bool {
        match self.relation(other) {
            UnitRelationKind::Enemy => false,
            _ => true,
        }
    }

    pub fn can_harm(&self, other: &Unit) -> bool {
        match self.relation(other) {
            UnitRelationKind::Ally => false,
            _ => true,
        }
    }
}

///////////////
// Constants //
///////////////

const LEVEL_CAP: i16 = 40;

const MIN_LIFE: i16 = 0;
const MAX_LIFE: i16 = 99;

const ENERGY_ACT_THRESHOLD: i16 = 100;
const MIN_ENERGY: i16 = 0;
const MAX_ENERGY: i16 = 999;

const MIN_DAMAGE: i16 = 0;
const MAX_DAMAGE: i16 = 999;

const USE_TRUE_HIT: bool = true;
const MIN_HIT: i16 = 0;
const MAX_HIT: i16 = 100;
const CRIT_FORCES_HIT: bool = true;
const NORMAL_CRIT_MULTI: i16 = 2;
const EXTRA_CRIT_MULTI: i16 = 3;

// TODO: Read https://github.com/zakirullin/cognitive-load

impl Unit {
    fn grow_attributes(&mut self) {
        let random = &mut thread_rng();
        let stats = self.final_stats();

        // Feels like more reason to split out attributes as a AddToStats instance

        self.attributes.vitality += overflowing_chance_hit(random, stats.vitality_growth);
        self.attributes.strength += overflowing_chance_hit(random, stats.strength_growth);
        self.attributes.magic += overflowing_chance_hit(random, stats.magic_growth);
        self.attributes.skill += overflowing_chance_hit(random, stats.skill_growth);
        self.attributes.speed += overflowing_chance_hit(random, stats.speed_growth);
        self.attributes.luck += overflowing_chance_hit(random, stats.luck_growth);
        self.attributes.defense += overflowing_chance_hit(random, stats.defense_growth);
        self.attributes.resistance += overflowing_chance_hit(random, stats.resistance_growth);
    }

    pub fn level_up(&mut self) -> bool {
        let old_level = self.resources.level;
        let new_level = (old_level + 1).clamp(1, LEVEL_CAP);
        if new_level > old_level {
            self.resources.level = new_level;
            self.grow_attributes();
            true
        } else {
            false
        }
    }

    pub fn energize(&mut self) {
        let delta = if self.is_alive() {
            self.final_stats().attack_speed.clamp(1, 100)
        } else {
            -1
        };
        self.resources.energy = (self.resources.energy + delta).clamp(MIN_ENERGY, MAX_ENERGY);
    }

    pub fn try_consume_energy(&mut self) -> bool {
        if self.resources.energy > ENERGY_ACT_THRESHOLD {
            self.resources.energy -= ENERGY_ACT_THRESHOLD;
            true
        } else {
            false
        }
    }

    pub fn change_life(&mut self, delta: i16, obs: Observer) {
        let old_life = self.resources.life;
        let new_life = (old_life + delta).clamp(MIN_LIFE, MAX_LIFE);
        let life_gained = new_life - old_life;
        let life_lost = old_life - new_life;

        self.resources.life = new_life;
        if new_life == 0 {
            self.is_alive = false;
            obs.on_death(DeathEvent { unit: self });
        } else if life_gained > 0 {
            obs.on_life_gained(HealEvent {
                unit: self,
                amount: life_gained,
            })
        } else if life_lost > 0 {
            obs.on_life_lost(DamageEvent {
                unit: self,
                amount: life_lost,
            })
        }
    }
}

impl Unit {
    pub fn attack(&mut self, defender: &mut Unit, obs: Observer) {
        let random = &mut thread_rng();
        let mut hit_func = |c: i16| -> bool {
            if USE_TRUE_HIT {
                double_rng_hit(random, c)
            } else {
                single_rng_hit(random, c)
            }
        };

        let attacker = self;

        let alice = attacker.final_stats();
        let bob = defender.final_stats();

        let hit_chance = (alice.hit - bob.avoid).clamp(MIN_HIT, MAX_HIT);
        let crit_chance = (alice.crit - bob.crit_avoid).clamp(MIN_HIT, MAX_HIT);

        let did_hit = hit_func(hit_chance);
        let did_crit = hit_func(crit_chance);
        if did_hit || (CRIT_FORCES_HIT && did_crit) {
            let phys_dmg = (alice.phys_damage - bob.phys_defense).clamp(MIN_DAMAGE, MAX_DAMAGE);
            let mag_dmg = (alice.mag_damage - bob.mag_defense).clamp(MIN_DAMAGE, MAX_DAMAGE);
            let mut all_dmg = (phys_dmg + mag_dmg).clamp(MIN_DAMAGE, MAX_DAMAGE);
            if did_crit {
                all_dmg *= if alice.critical_boost {
                    EXTRA_CRIT_MULTI
                } else {
                    NORMAL_CRIT_MULTI
                };
            }
            let damage_dealt = all_dmg;
            let is_killing_blow = defender.resources.life < damage_dealt;

            obs.on_hit(HitEvent {
                kind: HitKind::from_did_crit(did_crit),
                attacker,
                defender,
                is_killing_blow,
                damage_dealt,
            });
            defender.change_life(-damage_dealt, obs);
        } else {
            obs.on_miss(MissEvent { attacker, defender });
        }
    }
}

///////////
// Arena //
///////////

pub struct Arena {
    combatants: Vec<Unit>,
}

#[must_use]
pub struct ArenaResult<'a> {
    pub victor: Option<&'a Unit>,
}

impl Arena {
    pub fn new() -> Self {
        Arena { combatants: Vec::new() }
    }

    pub fn add(&mut self, unit: Unit) {
        self.combatants.push(unit);
    }

    fn alive_units(&self) -> impl Iterator<Item = &Unit> {
        self.combatants.iter().filter(|unit| unit.is_alive())
    }

    fn find_opponent(&mut self, i: usize) -> Option<usize> {
        let fixed = &self.combatants[i];

        let candidates: Vec<_> = self
            .combatants
            .iter()
            .enumerate()
            .filter(|(idx, _)| *idx != i)
            .collect();

        // FIXME: picks first alive and harmable target
        // Pitfalls:
        // - you can (but usually don't want to) hit corpses
        // - you will always target first in the list (in a threeway this means the last man is always ignored)
        for (idx, unit) in candidates {
            if unit.is_alive() && fixed.can_harm(unit) {
                return Some(idx);
            }
        }
        None
    }

    pub fn fight_to_the_death(&mut self, obs: Observer) -> ArenaResult {
        'game_loop: loop {
            self.combatants.iter_mut().for_each(Unit::energize);
            let mut living = 0;

            for subject_idx in 0..self.combatants.len() {
                let subject = &mut self.combatants[subject_idx];

                if subject.is_alive() {
                    living += 1;
                }
                if subject.try_consume_energy() {
                    if let Some(object_idx) = self.find_opponent(subject_idx) {
                        let (subject, object) = slice_get_many_mut(&mut self.combatants, subject_idx, object_idx);
                        subject.attack(object, obs);
                    }
                }
            }
            if living < 2 {
                break 'game_loop;
            }
        }
        let victor = self.alive_units().next();
        ArenaResult { victor }
    }
}

//////////////////////
// Generating units //
//////////////////////

// #[derive(Default)]
// struct ItemGenOptions {
//     pub name: Option<String>,
// }

// #[derive(Default)]
// struct UnitGenOptions {
//     pub name: Option<String>,

//     pub level: i16,
// }

// const UNIT_NAME_POOL: &[&str] = &["John the Mad", "Jim the Wise", "Spike the Sorrowful"];

// pub fn gen_unit(options: UnitGenOptions) -> Unit {
//     todo!()
// }
