//! Item data from the Dex

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::StringOrVec;

/// Fling data for items that can be flung
#[derive(Debug, Clone, Serialize, Deserialize)]
/// JavaScript equivalent: FlingData (sim/dex-items.ts)
/// 4 fields in JavaScript
pub struct FlingData {
    /// Base power of Fling when using this item
    /// JavaScript: basePower: number
    #[serde(rename = "basePower", default)]
    pub base_power: i32,
    /// Effect when item is flung
    /// JavaScript: effect?: string
    #[serde(default)]
    pub effect: Option<String>,
    /// Status inflicted by Fling
    /// JavaScript: status?: string
    #[serde(default)]
    pub status: Option<String>,
    /// Volatile status inflicted by Fling
    /// JavaScript: volatileStatus?: string
    #[serde(rename = "volatileStatus", default)]
    pub volatile_status: Option<String>,
}

/// Item data from the Dex
#[derive(Debug, Clone, Serialize, Deserialize)]
/// JavaScript equivalent: ItemData (sim/dex-items.ts)
/// 30+ fields in JavaScript (many are callbacks)
pub struct ItemData {
    /// Item number
    /// JavaScript: num: number
    #[serde(default)]
    pub num: i32,
    /// Item name
    /// JavaScript: name: string
    pub name: String,
    /// Description
    /// JavaScript: desc?: string
    #[serde(default)]
    pub desc: Option<String>,
    /// Is this a Choice item? (Choice Band, Scarf, Specs)
    /// JavaScript: isChoice?: boolean
    #[serde(rename = "isChoice", default)]
    pub is_choice: bool,
    /// Is this a Berry?
    /// JavaScript: isBerry?: boolean
    #[serde(rename = "isBerry", default)]
    pub is_berry: bool,
    /// Is this a Gem? (Fire Gem, etc.)
    /// JavaScript: isGem?: boolean
    #[serde(rename = "isGem", default)]
    pub is_gem: bool,
    /// Fling data
    /// JavaScript: fling?: FlingData
    #[serde(default)]
    pub fling: Option<FlingData>,
    /// Natural Gift data (for berries)
    /// JavaScript: naturalGift?: { basePower: number, type: string }
    #[serde(rename = "naturalGift", default)]
    pub natural_gift: Option<serde_json::Value>,
    /// Type for Plate items (e.g., "Fire" for Flame Plate)
    /// JavaScript: onPlate?: string
    #[serde(rename = "onPlate", default)]
    pub on_plate: Option<String>,
    /// Type for Memory items (e.g., "Fire" for Fire Memory)
    /// JavaScript: onMemory?: string
    #[serde(rename = "onMemory", default)]
    pub on_memory: Option<String>,
    /// Type for Drive items (e.g., "Water" for Douse Drive)
    /// JavaScript: onDrive?: string
    #[serde(rename = "onDrive", default)]
    pub on_drive: Option<String>,
    /// Z-Move compatibility
    /// JavaScript: zMove?: string | true | ZMoveOptions
    /// TODO: Rust uses Option<serde_json::Value>, JavaScript uses union type
    #[serde(rename = "zMove", default)]
    pub z_move: Option<serde_json::Value>,
    /// Mega Evolution stone target (e.g., "Froslass-Mega")
    /// JavaScript: megaStone?: string
    /// TODO: Rust uses StringOrVec, JavaScript uses string
    #[serde(rename = "megaStone", default)]
    pub mega_stone: Option<StringOrVec>,
    /// Pokemon species that can use this mega stone (e.g., "Froslass")
    /// JavaScript: megaEvolves?: string
    /// TODO: Rust uses StringOrVec, JavaScript uses string
    #[serde(rename = "megaEvolves", default)]
    pub mega_evolves: Option<StringOrVec>,
    /// Species that can use this item
    /// JavaScript: itemUser?: string[]
    #[serde(rename = "itemUser", default)]
    pub item_user: Option<Vec<String>>,
    /// Stat boosts when item is used (e.g., for Cell Battery)
    /// JavaScript: boosts?: SparseBoostsTable
    #[serde(default)]
    pub boosts: Option<std::collections::HashMap<String, i32>>,
    /// Sprite number for UI display
    /// JavaScript: spritenum?: number
    #[serde(default)]
    pub spritenum: Option<i32>,
    /// Generation introduced
    /// JavaScript: gen?: number
    #[serde(default)]
    pub gen: Option<u8>,
    /// Is this a Poké Ball?
    /// JavaScript: isPokeball?: boolean
    #[serde(rename = "isPokeball", default)]
    pub is_pokeball: bool,
    /// Does this item ignore the Klutz ability?
    /// JavaScript: ignoreKlutz?: boolean
    #[serde(rename = "ignoreKlutz", default)]
    pub ignore_klutz: bool,
    /// Nonstandard status (Past, Future, Unobtainable, etc.)
    /// JavaScript: isNonstandard?: Nonstandard | null
    /// TODO: Rust uses Option<String>, JavaScript uses Nonstandard union type
    #[serde(rename = "isNonstandard", default)]
    pub is_nonstandard: Option<String>,
    /// Extra fields (like onResidualOrder, onResidualSubOrder, etc.)
    /// JavaScript: handler.order = (handler.effect as any)[`${callbackName}Order`]
    /// Note: JavaScript has many callback methods (onStart, onEnd, etc.) that cannot be stored in data
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl ItemData {
    /// Get effect type
    /// JavaScript equivalent: item.effectType (always 'Item')
    /// In JavaScript, this is set in the Item constructor: this.effectType = 'Item'
    pub fn effect_type(&self) -> &'static str {
        "Item"
    }
}
