/**
 * Item Data Generator for Pokemon Showdown Rust Port
 *
 * This script generates Rust code for items using the hybrid approach:
 * - Data-driven effects for simple items
 * - Handler flags for complex items
 *
 * Run with: node generate_items.js > src/data/items_generated.rs
 */

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

// Items that have complex handlers in item_handlers.rs
const COMPLEX_ITEMS = new Set([
  'boosterenergy',
  'clearamulet',
  'assaultvest',
  'eviolite',
  'expertbelt',
  'bigroot',
  'brightpowder',
  'protectivepads',
  'metronome', // The item, not the move
  'blunderpolicy',
]);

// Type-boosting items mapping
const TYPE_BOOST_ITEMS = {
  'charcoal': 'Fire',
  'flameplate': 'Fire',
  'mysticwater': 'Water',
  'splashplate': 'Water',
  'seaincense': 'Water',
  'waveincense': 'Water',
  'miracleseed': 'Grass',
  'meadowplate': 'Grass',
  'roseincense': 'Grass',
  'magnet': 'Electric',
  'zapplate': 'Electric',
  'nevermeltice': 'Ice',
  'icicleplate': 'Ice',
  'blackbelt': 'Fighting',
  'fistplate': 'Fighting',
  'poisonbarb': 'Poison',
  'toxicplate': 'Poison',
  'softsand': 'Ground',
  'earthplate': 'Ground',
  'sharpbeak': 'Flying',
  'skyplate': 'Flying',
  'twistedspoon': 'Psychic',
  'mindplate': 'Psychic',
  'oddincense': 'Psychic',
  'silverpowder': 'Bug',
  'insectplate': 'Bug',
  'hardstone': 'Rock',
  'stoneplate': 'Rock',
  'rockincense': 'Rock',
  'spelltag': 'Ghost',
  'spookyplate': 'Ghost',
  'dragonfang': 'Dragon',
  'dracoplate': 'Dragon',
  'blackglasses': 'Dark',
  'dreadplate': 'Dark',
  'metalcoat': 'Steel',
  'ironplate': 'Steel',
  'silkscarf': 'Normal',
  'pixieplate': 'Fairy',
  'fairyfeather': 'Fairy',
};

// Type-resist berries
const TYPE_RESIST_BERRIES = {
  'occaberry': 'Fire',
  'passhoberry': 'Water',
  'wacanberry': 'Electric',
  'rindoberry': 'Grass',
  'yacheberry': 'Ice',
  'chopleberry': 'Fighting',
  'kebiaberry': 'Poison',
  'shucaberry': 'Ground',
  'cobaberry': 'Flying',
  'payapaberry': 'Psychic',
  'tangaberry': 'Bug',
  'chartiberry': 'Rock',
  'kasibberry': 'Ghost',
  'habanberry': 'Dragon',
  'colburberry': 'Dark',
  'babiriberry': 'Steel',
  'chilanberry': 'Normal',
  'roseliberry': 'Fairy',
};

// Status-cure berries
const STATUS_CURE_BERRIES = {
  'cheriberry': 'par',
  'chestoberry': 'slp',
  'pechaberry': 'psn',
  'rawstberry': 'brn',
  'aspearberry': 'frz',
  'lumberry': 'all', // Special: cures any status
};

// Generate effects for an item
function generateEffects(id, item) {
  const effects = [];

  // Choice items
  if (item.isChoice) {
    let stat = 'Spe';
    if (id === 'choiceband') stat = 'Atk';
    else if (id === 'choicespecs') stat = 'SpA';
    effects.push({
      event: 'ModifyMove',
      effect: `ItemEffect::ChoiceLock { stat: Stat::${stat}, multiplier: 1.5 }`,
    });
  }

  // Leftovers
  if (id === 'leftovers') {
    effects.push({
      event: 'Residual',
      effect: 'ItemEffect::ResidualHeal { fraction: 0.0625 }',
    });
  }

  // Black Sludge
  if (id === 'blacksludge') {
    effects.push({
      event: 'Residual',
      effect: 'ItemEffect::ResidualHealOrDamage { required_type: "Poison".to_string(), heal_fraction: 0.0625, damage_fraction: 0.125 }',
    });
  }

  // Life Orb
  if (id === 'lifeorb') {
    effects.push({
      event: 'ModifyDamage',
      effect: 'ItemEffect::BoostDamage { multiplier: 1.3 }',
    });
    effects.push({
      event: 'AfterHit',
      effect: 'ItemEffect::RecoilOnAttack { fraction: 0.1 }',
    });
  }

  // Type-boosting items
  if (TYPE_BOOST_ITEMS[id]) {
    effects.push({
      event: 'BasePower',
      effect: `ItemEffect::BoostType { move_type: "${TYPE_BOOST_ITEMS[id]}".to_string(), multiplier: 1.2 }`,
      priority: 15,
    });
  }

  // Type-resist berries
  if (TYPE_RESIST_BERRIES[id]) {
    effects.push({
      event: 'SourceModifyDamage',
      effect: `ItemEffect::Compound { effects: vec![
                ItemEffect::ResistSuperEffective { resist_type: "${TYPE_RESIST_BERRIES[id]}".to_string(), multiplier: 0.5 },
                ItemEffect::ConsumeItem,
            ] }`,
    });
  }

  // Status-cure berries
  if (STATUS_CURE_BERRIES[id]) {
    const status = STATUS_CURE_BERRIES[id];
    if (status === 'all') {
      effects.push({
        event: 'Update',
        effect: 'ItemEffect::Compound { effects: vec![ItemEffect::CureAllStatus, ItemEffect::ConsumeItem] }',
      });
    } else {
      effects.push({
        event: 'Update',
        effect: `ItemEffect::Compound { effects: vec![
                ItemEffect::CureStatus { status: "${status}".to_string() },
                ItemEffect::ConsumeItem,
            ] }`,
      });
    }
  }

  // Sitrus Berry
  if (id === 'sitrusberry') {
    effects.push({
      event: 'Update',
      effect: `ItemEffect::Triggered {
            trigger: Trigger::OnHPBelow(0.5),
            effect: Box::new(ItemEffect::Compound { effects: vec![
                ItemEffect::HealOnThreshold { threshold: 0.5, heal_fraction: 0.25 },
                ItemEffect::ConsumeItem,
            ] }),
        }`,
    });
  }

  // Pinch berries (Figy, Aguav, etc.)
  const pinchBerries = ['figyberry', 'aguavberry', 'iapapaberry', 'magoberry', 'wikiberry'];
  if (pinchBerries.includes(id)) {
    effects.push({
      event: 'Update',
      effect: `ItemEffect::Triggered {
            trigger: Trigger::OnHPBelowGluttony(0.25),
            effect: Box::new(ItemEffect::Compound { effects: vec![
                ItemEffect::HealOnThreshold { threshold: 0.25, heal_fraction: 0.333 },
                ItemEffect::ConsumeItem,
            ] }),
        }`,
    });
  }

  // Rocky Helmet
  if (id === 'rockyhelmet') {
    effects.push({
      event: 'DamagingHit',
      effect: 'ItemEffect::ContactDamage { fraction: 0.167 }', // 1/6
    });
  }

  // Focus Sash
  if (id === 'focussash') {
    effects.push({
      event: 'Damage',
      effect: 'ItemEffect::Compound { effects: vec![ItemEffect::FocusSash, ItemEffect::ConsumeItem] }',
    });
  }

  // Scope Lens / Razor Claw
  if (id === 'scopelens' || id === 'razorclaw') {
    effects.push({
      event: 'ModifyCritRatio',
      effect: 'ItemEffect::BoostCritRatio { stages: 1 }',
    });
  }

  // Wide Lens
  if (id === 'widelens') {
    effects.push({
      event: 'ModifyAccuracy',
      effect: 'ItemEffect::BoostAccuracy { multiplier: 1.1 }',
    });
  }

  // Quick Claw
  if (id === 'quickclaw') {
    effects.push({
      event: 'BeforeMove',
      effect: 'ItemEffect::PriorityChance { chance: 0.2 }', // 20% chance
    });
  }

  // Heavy-Duty Boots
  if (id === 'heavydutyboots') {
    effects.push({
      event: 'SwitchIn',
      effect: 'ItemEffect::HazardImmunity',
    });
  }

  // Safety Goggles
  if (id === 'safetygoggles') {
    effects.push({
      event: 'Residual',
      effect: 'ItemEffect::WeatherImmunity',
    });
    effects.push({
      event: 'TryHit',
      effect: 'ItemEffect::PowderImmunity',
    });
  }

  // Air Balloon
  if (id === 'airballoon') {
    effects.push({
      event: 'SwitchIn',
      effect: 'ItemEffect::Airborne',
    });
  }

  // Iron Ball
  if (id === 'ironball') {
    effects.push({
      event: 'SwitchIn',
      effect: 'ItemEffect::GroundHolder',
    });
    effects.push({
      event: 'ModifySpe',
      effect: 'ItemEffect::ModifyStat { stat: Stat::Spe, multiplier: 0.5 }',
    });
  }

  // Shed Shell
  if (id === 'shedshell') {
    effects.push({
      event: 'SwitchOut',
      effect: 'ItemEffect::EscapeTrapping',
    });
  }

  // Eject Button
  if (id === 'ejectbutton') {
    effects.push({
      event: 'DamagingHit',
      effect: 'ItemEffect::Compound { effects: vec![ItemEffect::EjectOnHit, ItemEffect::ConsumeItem] }',
    });
  }

  // Red Card
  if (id === 'redcard') {
    effects.push({
      event: 'DamagingHit',
      effect: 'ItemEffect::Compound { effects: vec![ItemEffect::ForceOpponentSwitch, ItemEffect::ConsumeItem] }',
    });
  }

  // Mental Herb
  if (id === 'mentalherb') {
    effects.push({
      event: 'Update',
      effect: 'ItemEffect::Compound { effects: vec![ItemEffect::MentalHerb, ItemEffect::ConsumeItem] }',
    });
  }

  // White Herb
  if (id === 'whiteherb') {
    effects.push({
      event: 'AfterBoost',
      effect: 'ItemEffect::Compound { effects: vec![ItemEffect::RestoreLoweredStats, ItemEffect::ConsumeItem] }',
    });
  }

  // Power Herb
  if (id === 'powerherb') {
    effects.push({
      event: 'BeforeMove',
      effect: 'ItemEffect::Compound { effects: vec![ItemEffect::SkipChargeTurn, ItemEffect::ConsumeItem] }',
    });
  }

  // Loaded Dice
  if (id === 'loadeddice') {
    effects.push({
      event: 'ModifyMove',
      effect: 'ItemEffect::MaxMultiHit',
    });
  }

  return effects;
}

// Format effects as Rust code
function formatEffects(effects) {
  if (effects.length === 0) {
    return '';
  }

  let code = '\n        effects: vec![';
  for (const e of effects) {
    const priority = e.priority !== undefined ? e.priority : 0;
    code += `\n            ItemEffectEntry::with_priority(EventType::${e.event}, ${e.effect}, ${priority}),`;
  }
  code += '\n        ],';
  return code;
}

// Generate Rust code
let output = `//! Data-driven Item Definitions (Generated)
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This module defines items as data structures with their properties,
//! following the pattern from data/items.ts in the JS codebase.
//!
//! Items use a hybrid approach:
//! - Most items (~80%) use data-driven ItemEffect for their behavior
//! - Complex items (~20%) use handler functions in item_handlers.rs
//!
//! GENERATED FILE - Do not edit directly. Run generate_items.js to regenerate.

use std::collections::HashMap;
use once_cell::sync::Lazy;
use crate::dex_data::ID;
use crate::event::EventType;
use super::item_effects::{ItemEffect, Stat, Trigger};

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

/// Effect entry - pairs an event type with an effect and optional priority
#[derive(Debug, Clone)]
pub struct ItemEffectEntry {
    /// The event that triggers this effect
    pub event: EventType,
    /// The effect to apply
    pub effect: ItemEffect,
    /// Priority for this handler (higher = earlier)
    pub priority: i32,
}

impl ItemEffectEntry {
    pub fn new(event: EventType, effect: ItemEffect) -> Self {
        Self {
            event,
            effect,
            priority: 0,
        }
    }

    pub fn with_priority(event: EventType, effect: ItemEffect, priority: i32) -> Self {
        Self {
            event,
            effect,
            priority,
        }
    }
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

    // === Effects System ===
    /// Data-driven effects for this item
    pub effects: Vec<ItemEffectEntry>,
    /// Flag indicating this item has custom handlers in item_handlers.rs
    pub has_custom_handler: bool,
    /// Ignore Klutz ability
    pub ignore_klutz: bool,
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
        // Check effects for TypeImmunity
        for entry in &self.effects {
            if let ItemEffect::TypeImmunity { immune_type } = &entry.effect {
                if immune_type.eq_ignore_ascii_case(move_type) {
                    return true;
                }
            }
        }
        false
    }

    /// Get effects for a specific event
    pub fn get_effects_for_event(&self, event: &EventType) -> Vec<&ItemEffect> {
        self.effects
            .iter()
            .filter(|e| &e.event == event)
            .map(|e| &e.effect)
            .collect()
    }

    /// Check if this item has any effect for a given event
    pub fn has_effect_for_event(&self, event: &EventType) -> bool {
        self.effects.iter().any(|e| &e.event == event)
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
  else if (item.isChoice) category = "ItemCategory::Choice";
  else if (id.startsWith("tm") && id.match(/^tm\d+$/)) category = "ItemCategory::TM";
  else if (id.startsWith("tr") && id.match(/^tr\d+$/)) category = "ItemCategory::TR";
  else if (id.includes("fossil")) category = "ItemCategory::Fossil";

  // Generate effects
  const effects = generateEffects(id, item);
  const hasCustomHandler = COMPLEX_ITEMS.has(id);

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
  if (item.isChoice) {
    code += `\n        is_choice: true,`;
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
  if (item.ignoreKlutz) {
    code += `\n        ignore_klutz: true,`;
  }

  // Add effects
  code += formatEffects(effects);

  // Add custom handler flag
  if (hasCustomHandler) {
    code += `\n        has_custom_handler: true,`;
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

/// Check if an item grants type immunity
pub fn item_grants_type_immunity(item_id: &ID, move_type: &str) -> bool {
    get_item(item_id).map_or(false, |item| item.grants_type_immunity(move_type))
}

/// Get the total number of items
pub fn item_count() -> usize {
    ITEMS.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_count() {
        assert!(item_count() > 500, "Expected at least 500 items");
    }

    #[test]
    fn test_leftovers() {
        let leftovers = ITEMS.get(&ID::new("leftovers")).expect("leftovers should exist");
        assert_eq!(leftovers.name, "Leftovers");
        assert!(!leftovers.effects.is_empty());
    }

    #[test]
    fn test_choice_items() {
        let band = ITEMS.get(&ID::new("choiceband")).expect("choiceband");
        assert!(band.is_choice);
        assert!(!band.effects.is_empty());
    }

    #[test]
    fn test_sitrus_berry() {
        let sitrus = ITEMS.get(&ID::new("sitrusberry")).expect("sitrusberry");
        assert!(sitrus.is_berry);
        assert!(!sitrus.effects.is_empty());
    }

    #[test]
    fn test_complex_items_flagged() {
        let assault_vest = ITEMS.get(&ID::new("assaultvest")).expect("assaultvest");
        assert!(assault_vest.has_custom_handler);
    }
}
`;

console.log(output);
