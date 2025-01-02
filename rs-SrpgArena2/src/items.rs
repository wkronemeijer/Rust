use crate::stats::Stats;

#[derive(Debug, Clone)]
pub enum EquipmentSlot {
    Hand,
    Chest,
    Finger,
}

#[derive(Debug, Clone)]
pub struct EquipmentItem {
    name: String,
    pub slot: EquipmentSlot,
    pub attributes: Stats,
}

impl EquipmentItem {
    pub fn name(&self) -> &str { &self.name }
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
    items: [Option<EquipmentItem>; INVENTORY_SIZE_LIMIT],
}

impl UnitInventory {
    pub fn new() -> Self {
        UnitInventory { items: [const { None }; INVENTORY_SIZE_LIMIT] }
    }

    pub fn equipped_items(&self) -> impl Iterator<Item = &EquipmentItem> {
        self.items.iter().filter_map(|item| item.as_ref())
    }

    // Somehow, I need references to specific items
    // ...but we don't have specific slots...
}
