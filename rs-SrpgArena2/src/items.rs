use arrayvec::ArrayVec;

use crate::stats::Stats;

#[derive(Debug, Clone, Copy)]
pub enum EquipmentSlot {
    Hand,
    Chest,
    Finger,
}

#[derive(Debug, Clone)]
pub struct EquipmentItem {
    name: String,
    slot: EquipmentSlot,
    pub attributes: Box<Stats>,
}

impl EquipmentItem {
    pub fn name(&self) -> &str { &self.name }

    pub fn slot(&self) -> EquipmentSlot { self.slot }
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

#[derive(Debug, Default)]
pub struct UnitInventory {
    items: ArrayVec<EquipmentItem, INVENTORY_SIZE_LIMIT>,
}

impl UnitInventory {
    pub fn new() -> Self { UnitInventory { items: ArrayVec::new() } }

    pub fn iter(&self) -> impl Iterator<Item = &EquipmentItem> {
        self.items.iter()
    }

    // Somehow, I need references to specific items
    // ...but we don't have specific slots...
}
