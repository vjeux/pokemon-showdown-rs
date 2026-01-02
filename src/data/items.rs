// All items are now loaded from data/items.json
// This file is kept for compatibility with existing code that references item constants
//
// Item data comes from:
// 1. data/items.json - base item data (name, desc, fling power, etc.)
// 2. src/data/item_callbacks/ - custom event handlers per item
//
// Access items via Battle.dex.items().get(name) or Battle.dex.items

// Re-export item data type from dex
pub use crate::dex::ItemData;

// Item category for organization
/// TODO: Not in JavaScript - Rust-specific enum for categorizing items
/// JavaScript doesn't have explicit item categories
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
