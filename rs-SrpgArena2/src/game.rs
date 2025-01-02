use rand::thread_rng;

use crate::core::slice_index_pair_checked;
use crate::events::DamageEvent;
use crate::events::DeathEvent;
use crate::events::HealEvent;
use crate::events::HitEvent;
use crate::events::MissEvent;
use crate::events::Observer;
use crate::items::UnitInventory;
use crate::rng::double_rng_hit;
use crate::rng::overflowing_chance_hit;
use crate::rng::single_rng_hit;
use crate::stats::Resources;
use crate::stats::Stats;

#[derive(Clone, Copy)]
pub enum HitKind {
    Hit,
    Crit,
}

impl HitKind {
    pub fn from_did_crit(did_crit: bool) -> Self {
        if did_crit { HitKind::Crit } else { HitKind::Hit }
    }
}

//////////////////
// Observer API //
//////////////////

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnitHandle {
    // Not generational...for now
    index: u16,
}

impl UnitHandle {
    pub fn new() -> Self { UnitHandle { index: 0 } }

    pub fn index(self) -> usize { usize::from(self.index) }

    pub fn next(self) -> Option<Self> {
        self.index.checked_add(1).map(|index| UnitHandle { index })
    }
}

pub struct UnitCollection {
    units: Vec<(UnitHandle, Unit)>,
    next_index: Option<UnitHandle>,
}

impl UnitCollection {
    pub fn new() -> Self {
        UnitCollection {
            units: Vec::new(),
            next_index: Some(UnitHandle::new()),
        }
    }

    pub fn len(&self) -> usize { self.units.len() }

    pub fn get(&self, handle: UnitHandle) -> Option<&Unit> {
        self.units.get(handle.index()).map(|(_, u)| u)
    }

    pub fn get_mut(&mut self, handle: UnitHandle) -> Option<&mut Unit> {
        self.units.get_mut(handle.index()).map(|(_, u)| u)
    }

    pub fn get_many_mut(
        &mut self,
        handle_1: UnitHandle,
        handle_2: UnitHandle,
    ) -> Option<(&mut Unit, &mut Unit)> {
        let ((_, unit_1), (_, unit_2)) = slice_index_pair_checked(
            &mut self.units,
            (usize::from(handle_1.index), usize::from(handle_2.index)),
        )?;
        Some((unit_1, unit_2))
    }

    pub fn push(&mut self, mut unit: Unit) -> crate::Result<UnitHandle> {
        if let Some(unit_index) = self.next_index {
            unit.id = unit_index;
            self.units.insert(unit_index.index(), (unit_index, unit));
            self.next_index = unit_index.next();
            Ok(unit_index)
        } else {
            Err("not enough room".into())
        }
    }

    // That's right, there is no pop()!

    ///////////////
    // Iterators //
    ///////////////

    pub fn keys(&self) -> impl Iterator<Item = UnitHandle> {
        self.units.iter().map(|(h, _)| *h)
    }

    pub fn values(&self) -> impl Iterator<Item = &Unit> {
        self.units.iter().map(|(_, u)| u)
    }

    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut Unit> {
        self.units.iter_mut().map(|(_, u)| u)
    }

    pub fn iter(&self) -> impl Iterator<Item = (UnitHandle, &Unit)> {
        self.units.iter().map(|(k, v)| (*k, v))
    }

    pub fn iter_mut(
        &mut self,
    ) -> impl Iterator<Item = (UnitHandle, &mut Unit)> {
        self.units.iter_mut().map(|(k, v)| (*k, v))
    }
}

////////////
// Events //
////////////

// References?
// LOL
// Handle time!

//////////
// Unit //
//////////

#[derive(Debug)]
pub struct Unit {
    id: UnitHandle,
    name: String,
    is_alive: bool, // seperate because you can heal a corpse
    resources: Resources,
    attributes: Stats,
    pub equipment: Box<UnitInventory>,
}

impl Unit {
    pub fn new(name: impl Into<String>, attributes: Stats) -> Self {
        let mut result = Unit {
            id: UnitHandle::new(),
            name: name.into(),
            is_alive: false,
            resources: Resources::default(),
            attributes,
            equipment: Box::new(UnitInventory::new()),
        };
        result.revive();
        result
    }

    pub fn id(&self) -> UnitHandle { self.id }

    pub fn name(&self) -> &str { &self.name }

    pub fn is_alive(&self) -> bool { self.is_alive }

    pub fn resources(&self) -> &Resources { &self.resources }

    pub fn attributes(&self) -> &Stats { &self.attributes }

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

        self.attributes.vitality +=
            overflowing_chance_hit(random, stats.vitality_growth);
        self.attributes.strength +=
            overflowing_chance_hit(random, stats.strength_growth);
        self.attributes.magic +=
            overflowing_chance_hit(random, stats.magic_growth);
        self.attributes.skill +=
            overflowing_chance_hit(random, stats.skill_growth);
        self.attributes.speed +=
            overflowing_chance_hit(random, stats.speed_growth);
        self.attributes.luck +=
            overflowing_chance_hit(random, stats.luck_growth);
        self.attributes.defense +=
            overflowing_chance_hit(random, stats.defense_growth);
        self.attributes.resistance +=
            overflowing_chance_hit(random, stats.resistance_growth);
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
        self.resources.energy =
            (self.resources.energy + delta).clamp(MIN_ENERGY, MAX_ENERGY);
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
            obs.observe(DeathEvent { unit: self.id });
        } else if life_gained > 0 {
            obs.observe(HealEvent { unit: self.id, amount: life_gained })
        } else if life_lost > 0 {
            obs.observe(DamageEvent { unit: self.id, amount: life_lost })
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
            let phys_dmg = (alice.phys_damage - bob.phys_defense)
                .clamp(MIN_DAMAGE, MAX_DAMAGE);
            let mag_dmg = (alice.mag_damage - bob.mag_defense)
                .clamp(MIN_DAMAGE, MAX_DAMAGE);
            let mut all_dmg =
                (phys_dmg + mag_dmg).clamp(MIN_DAMAGE, MAX_DAMAGE);
            if did_crit {
                all_dmg *= if alice.critical_boost {
                    EXTRA_CRIT_MULTI
                } else {
                    NORMAL_CRIT_MULTI
                };
            }
            let damage_dealt = all_dmg;
            let is_killing_blow = defender.resources.life < damage_dealt;

            defender.change_life(-damage_dealt, obs);
            obs.observe(HitEvent {
                kind: HitKind::from_did_crit(did_crit),
                attacker: attacker.id,
                defender: defender.id,
                is_killing_blow,
                damage_dealt,
            });
        } else {
            obs.observe(MissEvent {
                attacker: attacker.id,
                defender: defender.id,
            });
        }
    }
}

///////////
// Arena //
///////////

pub struct Arena {
    pub combatants: UnitCollection,
}

#[must_use]
pub struct ArenaResult {
    pub victor: Option<UnitHandle>,
}

impl Arena {
    pub fn new() -> Self { Arena { combatants: UnitCollection::new() } }

    pub fn combatants(&self) -> &UnitCollection { &self.combatants }

    pub fn add(&mut self, unit: Unit) -> UnitHandle {
        self.combatants.push(unit).expect("failed to add unit")
    }

    fn alive_units(&self) -> impl Iterator<Item = UnitHandle> {
        self.combatants
            .iter()
            .filter(|(_, unit)| unit.is_alive())
            .map(|(x, _)| x)
    }

    fn find_opponent(
        &mut self,
        attacker_idx: UnitHandle,
    ) -> Option<UnitHandle> {
        let source = self.combatants.get(attacker_idx)?;

        let candidates: Vec<_> = self
            .combatants
            .iter()
            .filter(|(idx, _)| attacker_idx != *idx)
            .collect();

        // FIXME: picks first alive and harmable target
        // Pitfalls:
        // - you can (but usually don't want to) hit corpses
        // - you will always target first in the list (in a threeway this means the last man is always ignored)
        for (idx, target) in candidates {
            if target.is_alive() && source.can_harm(target) {
                return Some(idx);
            }
        }
        None
    }

    pub fn fight_to_the_death(&mut self, obs: Observer) -> ArenaResult {
        'game_loop: loop {
            self.combatants.values_mut().for_each(Unit::energize);
            let mut living = 0;

            let handles: Vec<_> = self.combatants.keys().collect();
            for subject_idx in handles {
                let subject = self.combatants.get_mut(subject_idx).unwrap();

                if subject.is_alive() {
                    living += 1;
                }
                if subject.try_consume_energy() {
                    if let Some(object_idx) = self.find_opponent(subject_idx) {
                        if let Some((subject, object)) = self
                            .combatants
                            .get_many_mut(subject_idx, object_idx)
                        {
                            subject.attack(object, obs);
                        }
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

    pub fn advance() {
        todo!();
    }
}
