const items = require("/home/builder/workspace/pokemon-showdown-js/dist/data/items").Items;

// Helper to escape string for Rust
function escapeRust(s) {
  if (!s) return null;
  if (typeof s !== 'string') return String(s);
  return s.replace(/\\/g, "\\\\").replace(/"/g, '\\"');
}

// Helper to format item users as Rust vec
function formatItemUser(users) {
  if (!users || users.length === 0) return "None";
  return `Some(vec![${users.map(u => `"${escapeRust(u)}".to_string()`).join(", ")}])`;
}

// Helper to format boosts as Rust code
function formatBoosts(boosts) {
  if (!boosts) return "None";
  const entries = Object.entries(boosts);
  if (entries.length === 0) return "None";
  return `Some(boosts(&[${entries.map(([k, v]) => `("${k}", ${v})`).join(", ")}]))`;
}

// Generate Rust code
let output = `//! Data-driven Item Definitions
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This module defines items as data structures with their properties,
//! following the pattern from data/items.ts in the JS codebase.

use std::collections::HashMap;
use once_cell::sync::Lazy;
use crate::dex_data::ID;

/// Helper to create boosts HashMap
fn boosts(entries: &[(&str, i8)]) -> HashMap<String, i8> {
    entries.iter().map(|(k, v)| (k.to_string(), *v)).collect()
}

/// Item category for organization
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

/// Item definition - data-driven item with all properties
#[derive(Debug, Clone, Default)]
pub struct ItemDef {
    /// Unique ID
    pub id: ID,
    /// Display name
    pub name: String,
    /// Item number
    pub num: i32,
    /// Generation introduced
    pub gen: u8,
    /// Item category
    pub category: ItemCategory,
    /// Fling base power
    pub fling_power: Option<u32>,
    /// Is this item consumable (one-time use)
    pub is_consumable: bool,
    /// Is this a berry
    pub is_berry: bool,
    /// Is this a choice item
    pub is_choice: bool,
    /// Is a gem (consumed on first matching type move)
    pub is_gem: bool,
    /// Is a Pokeball
    pub is_pokeball: bool,
    /// Is nonstandard (Past, Future, CAP, etc.)
    pub is_nonstandard: Option<String>,

    // === Mega Evolution ===
    /// Mega stone forme name
    pub mega_stone: Option<String>,
    /// Species that can mega evolve with this stone
    pub mega_evolves: Option<String>,
    /// Restricted users (species that can use this item)
    pub item_user: Option<Vec<String>>,

    // === Z-Crystal ===
    /// Z-Crystal move type
    pub z_move_type: Option<String>,
    /// Z-Move name for signature Z-moves
    pub z_move: Option<String>,
    /// Move that transforms into Z-Move
    pub z_move_from: Option<String>,

    // === Forme changing ===
    /// Forces holder to a specific forme
    pub forced_forme: Option<String>,

    // === Stat modifiers ===
    /// Boost damage of specific type by multiplier
    pub type_boost: Option<(String, f64)>,
    /// Boost a specific stat by multiplier
    pub stat_boost: Option<(String, f64)>,
    /// Choice item stat boost (1.5x but locks move)
    pub choice_boost: Option<String>,
    /// Generic boosts map (like Absorb Bulb spa+1)
    pub boosts: Option<HashMap<String, i8>>,

    // === Damage modifiers ===
    /// Damage multiplier for all moves (Life Orb)
    pub damage_multiplier: Option<f64>,
    /// Recoil to user when dealing damage (Life Orb: 0.1)
    pub recoil_on_attack: Option<f64>,

    // === HP/Status effects ===
    /// Residual healing per turn (fraction of max HP)
    pub residual_heal: Option<f64>,
    /// Residual damage per turn (fraction of max HP) - Black Sludge on non-Poison
    pub residual_damage: Option<f64>,
    /// Heal on HP threshold (threshold_fraction, heal_fraction)
    pub heal_on_low_hp: Option<(f64, f64)>,
    /// Status immunity (prevents specific statuses)
    pub status_immunity: Option<Vec<String>>,
    /// Status cure (cures specific statuses when afflicted)
    pub status_cure: Option<Vec<String>>,
    /// Cures all non-volatile status
    pub cures_all_status: bool,

    // === Natural Gift ===
    /// Natural Gift base power (for berries)
    pub natural_gift_power: Option<u32>,
    /// Natural Gift type (for berries)
    pub natural_gift_type: Option<String>,

    // === Type plates/memories/drives ===
    /// Type granted by Plate
    pub on_plate: Option<String>,
    /// Type granted by Memory
    pub on_memory: Option<String>,
    /// Type granted by Drive
    pub on_drive: Option<String>,

    // === Type-specific effects ===
    /// Required type for effect (Black Sludge requires Poison)
    pub required_type: Option<String>,
    /// Immunity to type (Air Balloon = Ground immunity)
    pub type_immunity: Option<String>,
    /// Grounds holder (Iron Ball)
    pub grounds_holder: bool,

    // === Priority/Speed effects ===
    /// Speed multiplier
    pub speed_multiplier: Option<f64>,
    /// Priority boost for specific type moves (Quick Claw gives random +1)
    pub priority_boost: Option<i8>,
    /// Quick Claw chance (0-100)
    pub quick_claw_chance: Option<u8>,

    // === Move effects ===
    /// Boosts critical hit ratio
    pub crit_boost: Option<i8>,
    /// Focus Energy effect
    pub focus_energy: bool,
    /// Prevents flinching (Inner Focus-like effect)
    pub prevents_flinch: bool,
    /// Boosts multi-hit moves to 5 hits
    pub loaded_dice: bool,

    // === Defensive effects ===
    /// Reduces damage from specific type by fraction
    pub resist_type: Option<(String, f64)>,
    /// Focus Sash effect (survive at 1 HP from full)
    pub focus_sash: bool,
    /// Sturdy-like effect
    pub sturdy: bool,

    // === Misc ===
    /// Contact damage to attacker (Rocky Helmet)
    pub contact_damage: Option<f64>,
    /// Mental Herb effect (cures infatuation and taunt)
    pub mental_herb: bool,
    /// White Herb effect (restores lowered stats)
    pub white_herb: bool,
    /// Power Herb effect (skip charge turn)
    pub power_herb: bool,
    /// Eject Button effect
    pub eject_button: bool,
    /// Red Card effect
    pub red_card: bool,
    /// Prevents switching (Shed Shell bypasses)
    pub trapping: bool,
    /// Can switch despite trapping
    pub escape_trapping: bool,
}

impl ItemDef {
    pub fn new(id: &str, name: &str) -> Self {
        Self {
            id: ID::new(id),
            name: name.to_string(),
            ..Default::default()
        }
    }

    /// Check if item boosts a type
    pub fn boosts_type(&self, move_type: &str) -> Option<f64> {
        self.type_boost.as_ref()
            .filter(|(t, _)| t.eq_ignore_ascii_case(move_type))
            .map(|(_, mult)| *mult)
    }

    /// Check if item grants type immunity
    pub fn grants_type_immunity(&self, move_type: &str) -> bool {
        self.type_immunity.as_ref().map_or(false, |t| t.eq_ignore_ascii_case(move_type))
    }
}

/// Static registry of all items
pub static ITEMS: Lazy<HashMap<ID, ItemDef>> = Lazy::new(|| {
    let mut map = HashMap::new();

`;

// Generate all items
const sortedKeys = Object.keys(items).sort();
for (const id of sortedKeys) {
  const item = items[id];

  // Determine category
  let category = "ItemCategory::HeldItem";
  if (item.isBerry) category = "ItemCategory::Berry";
  else if (item.megaStone) category = "ItemCategory::Mega";
  else if (item.zMoveType || item.zMove) category = "ItemCategory::ZCrystal";
  else if (item.onPlate) category = "ItemCategory::Plate";
  else if (item.onDrive) category = "ItemCategory::Drive";
  else if (item.onMemory) category = "ItemCategory::Memory";
  else if (item.isGem) category = "ItemCategory::Gem";
  else if (item.isPokeball) category = "ItemCategory::Pokeball";
  else if (id.startsWith("tm") && id.match(/^tm\d+$/)) category = "ItemCategory::TM";
  else if (id.startsWith("tr") && id.match(/^tr\d+$/)) category = "ItemCategory::TR";
  else if (id.includes("fossil")) category = "ItemCategory::Fossil";

  let code = `    map.insert(ID::new("${id}"), ItemDef {
        id: ID::new("${id}"),
        name: "${escapeRust(item.name)}".to_string(),
        num: ${item.num || 0},
        gen: ${item.gen || 0},
        category: ${category},`;

  if (item.fling && item.fling.basePower) {
    code += `\n        fling_power: Some(${item.fling.basePower}),`;
  }
  if (item.isBerry) {
    code += `\n        is_berry: true,`;
    code += `\n        is_consumable: true,`;
  }
  if (item.isGem) {
    code += `\n        is_gem: true,`;
    code += `\n        is_consumable: true,`;
  }
  if (item.isPokeball) {
    code += `\n        is_pokeball: true,`;
  }
  if (item.isNonstandard) {
    code += `\n        is_nonstandard: Some("${item.isNonstandard}".to_string()),`;
  }
  if (item.megaStone) {
    code += `\n        mega_stone: Some("${escapeRust(item.megaStone)}".to_string()),`;
  }
  if (item.megaEvolves) {
    code += `\n        mega_evolves: Some("${escapeRust(item.megaEvolves)}".to_string()),`;
  }
  if (item.itemUser && item.itemUser.length > 0) {
    code += `\n        item_user: ${formatItemUser(item.itemUser)},`;
  }
  if (item.zMoveType) {
    code += `\n        z_move_type: Some("${item.zMoveType}".to_string()),`;
  }
  if (item.zMove && typeof item.zMove === 'string') {
    code += `\n        z_move: Some("${escapeRust(item.zMove)}".to_string()),`;
  }
  if (item.zMoveFrom) {
    code += `\n        z_move_from: Some("${escapeRust(item.zMoveFrom)}".to_string()),`;
  }
  if (item.forcedForme) {
    code += `\n        forced_forme: Some("${escapeRust(item.forcedForme)}".to_string()),`;
  }
  if (item.boosts) {
    code += `\n        boosts: ${formatBoosts(item.boosts)},`;
  }
  if (item.naturalGift) {
    code += `\n        natural_gift_power: Some(${item.naturalGift.basePower}),`;
    code += `\n        natural_gift_type: Some("${item.naturalGift.type}".to_string()),`;
  }
  if (item.onPlate) {
    code += `\n        on_plate: Some("${item.onPlate}".to_string()),`;
  }
  if (item.onMemory) {
    code += `\n        on_memory: Some("${item.onMemory}".to_string()),`;
  }
  if (item.onDrive) {
    code += `\n        on_drive: Some("${item.onDrive}".to_string()),`;
  }

  code += `\n        ..Default::default()
    });

`;
  output += code;
}

// Closing
output += `    map
});

/// Get item definition by ID
pub fn get_item(id: &ID) -> Option<&'static ItemDef> {
    ITEMS.get(id)
}

/// Check if an item boosts a specific type
pub fn item_boosts_type(item_id: &ID, move_type: &str) -> Option<f64> {
    get_item(item_id).and_then(|item| item.boosts_type(move_type))
}

/// Check if an item is a choice item
pub fn is_choice_item(item_id: &ID) -> bool {
    get_item(item_id).map_or(false, |item| item.is_choice)
}

/// Check if an item is a berry
pub fn is_berry(item_id: &ID) -> bool {
    get_item(item_id).map_or(false, |item| item.is_berry)
}

/// Get residual healing for an item
pub fn get_residual_heal(item_id: &ID) -> Option<f64> {
    get_item(item_id).and_then(|item| item.residual_heal)
}

/// Check if an item grants type immunity
pub fn item_grants_type_immunity(item_id: &ID, move_type: &str) -> bool {
    get_item(item_id).map_or(false, |item| item.grants_type_immunity(move_type))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_count() {
        assert_eq!(ITEMS.len(), 583, "Expected 583 items");
    }

    #[test]
    fn test_leftovers() {
        let leftovers = ITEMS.get(&ID::new("leftovers")).expect("leftovers should exist");
        assert_eq!(leftovers.name, "Leftovers");
        assert_eq!(leftovers.num, 234);
    }

    #[test]
    fn test_choice_items() {
        assert!(ITEMS.get(&ID::new("choiceband")).is_some());
        assert!(ITEMS.get(&ID::new("choicespecs")).is_some());
        assert!(ITEMS.get(&ID::new("choicescarf")).is_some());
    }

    #[test]
    fn test_mega_stones() {
        let venusaurite = ITEMS.get(&ID::new("venusaurite")).expect("venusaurite");
        assert_eq!(venusaurite.mega_stone, Some("Venusaur-Mega".to_string()));
        assert_eq!(venusaurite.mega_evolves, Some("Venusaur".to_string()));
    }

    #[test]
    fn test_z_crystals() {
        let pikashuniumz = ITEMS.get(&ID::new("pikashuniumz")).expect("pikashuniumz");
        assert_eq!(pikashuniumz.z_move, Some("10,000,000 Volt Thunderbolt".to_string()));
    }

    #[test]
    fn test_berries() {
        let sitrus = ITEMS.get(&ID::new("sitrusberry")).expect("sitrusberry");
        assert!(sitrus.is_berry);
        assert_eq!(sitrus.natural_gift_type, Some("Psychic".to_string()));
    }

    #[test]
    fn test_plates() {
        let meadow = ITEMS.get(&ID::new("meadowplate")).expect("meadowplate");
        assert_eq!(meadow.on_plate, Some("Grass".to_string()));
    }
}
`;

console.log(output);
