use crate::game::HitKind;
use crate::game::UnitHandle;

///////////////////
// Unique events //
///////////////////

pub struct HitEvent {
    pub kind: HitKind,
    pub attacker: UnitHandle,
    pub defender: UnitHandle,
    pub is_killing_blow: bool,
    pub damage_dealt: i16,
}

pub struct MissEvent {
    pub attacker: UnitHandle,
    pub defender: UnitHandle,
}

pub struct DamageEvent {
    pub unit: UnitHandle,
    pub amount: i16,
}

pub struct HealEvent {
    pub unit: UnitHandle,
    pub amount: i16,
}

pub struct DeathEvent {
    pub unit: UnitHandle,
}

/////////////////////
// Universal event //
/////////////////////

pub enum AnyEvent {
    HitEvent(HitEvent),
    MissEvent(MissEvent),
    DamageEvent(DamageEvent),
    HealEvent(HealEvent),
    DeathEvent(DeathEvent),
}

macro_rules! isa {
    ($variant:ident, $base:ty) => {
        impl From<$variant> for $base {
            fn from(event: $variant) -> Self { Self::$variant(event) }
        }
    };
}

isa!(HitEvent, AnyEvent);
isa!(MissEvent, AnyEvent);
isa!(DamageEvent, AnyEvent);
isa!(HealEvent, AnyEvent);
isa!(DeathEvent, AnyEvent);
// ...if only we had variant types and coercion rules for them...

///////////////
// Event log //
///////////////

pub struct ObserverInstance {
    events: Vec<AnyEvent>,
}

impl ObserverInstance {
    pub fn new() -> Self { ObserverInstance { events: Vec::new() } }

    pub fn observe(&mut self, event: impl Into<AnyEvent>) {
        self.events.push(event.into())
    }

    pub fn events(&self) -> &[AnyEvent] { &self.events }
}

pub type Observer<'a> = &'a mut ObserverInstance;
