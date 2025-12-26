// All items are now loaded from data/items.json
// This file is kept for compatibility with existing code that references item constants
//
// Item data comes from:
// 1. data/items.json - base item data (name, desc, fling power, etc.)
// 2. src/data/item_callbacks/ - custom event handlers per item
//
// Access items via Battle.dex.get_item(name) or Battle.dex.items

use crate::dex_data::ID;

// Re-export item data type from dex
pub use crate::dex::ItemData;

// Item category for organization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ItemCategory {
    Berry,
    Choice,
    Mega,
    ZCrystal,
    Plate,
    Drive,
    Memory,
    Orb,
    Gem,
    Mail,
    Medicine,
    StatusCure,
    TypeEnhancing,
    EvolutionItem,
    #[default]
    HeldItem,
    Pokeball,
    TM,
    TR,
    Fossil,
}

// Minimal compatibility struct for existing code
#[derive(Debug, Clone)]
pub struct ItemDef {
    pub id: ID,
    pub name: String,
    pub is_choice: bool,
}

impl ItemDef {
    pub fn from_id(id: ID, name: String, is_choice: bool) -> Self {
        Self { id, name, is_choice }
    }
}

/// Check if an item boosts a specific type
pub fn item_boosts_type(_item_id: &ID, _move_type: &str) -> bool {
    // TODO: Implement by checking item data
    false
}

/// Check if an item is a choice item
pub fn is_choice_item(item_id: &ID) -> bool {
    // Simplified check - just check if name starts with "Choice"
    let name = item_id.as_str();
    name.starts_with("choice")
}

/// Check if an item is a berry
pub fn is_berry(item_id: &ID) -> bool {
    // Berry items have names ending in "berry"
    item_id.as_str().ends_with("berry")
}

/// Get residual heal fraction for an item (Leftovers, etc.)
pub fn get_residual_heal(_item_id: &ID) -> Option<f64> {
    // TODO: Implement by checking item data
    None
}

/// Check if an item grants type immunity
pub fn item_grants_type_immunity(_item_id: &ID, _move_type: &str) -> bool {
    // TODO: Implement by checking item data (e.g., Air Balloon for Ground immunity)
    false
}
