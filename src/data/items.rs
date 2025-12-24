//! Data-driven Item Definitions
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This module defines items as data structures with their properties,
//! following the pattern from data/items.ts in the JS codebase.
//!
//! Items use a hybrid approach:
//! - Most items (~80%) use data-driven ItemEffect for their behavior
//! - Complex items (~20%) use handler functions in item_handlers.rs

use std::collections::HashMap;
use once_cell::sync::Lazy;
use crate::dex_data::ID;
use crate::event::EventType;
use super::item_effects::ItemEffect;

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

    // === Effects System (New) ===
    /// Data-driven effects for this item
    /// Most items use this for their behavior
    pub effects: Vec<ItemEffectEntry>,
    /// Flag indicating this item has custom handlers in item_handlers.rs
    /// When true, battle should check item_handlers first
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
        self.type_immunity.as_ref().map_or(false, |t| t.eq_ignore_ascii_case(move_type))
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

    /// Add an effect to this item
    pub fn add_effect(&mut self, event: EventType, effect: ItemEffect) {
        self.effects.push(ItemEffectEntry::new(event, effect));
    }

    /// Add an effect with priority
    pub fn add_effect_with_priority(&mut self, event: EventType, effect: ItemEffect, priority: i32) {
        self.effects.push(ItemEffectEntry::with_priority(event, effect, priority));
    }
}

/// Static registry of all items
pub static ITEMS: Lazy<HashMap<ID, ItemDef>> = Lazy::new(|| {
    let mut map = HashMap::new();

    map.insert(ID::new("abilityshield"), ItemDef {
        id: ID::new("abilityshield"),
        name: "Ability Shield".to_string(),
        num: 1881,
        gen: 9,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        ..Default::default()
    });

    map.insert(ID::new("abomasite"), ItemDef {
        id: ID::new("abomasite"),
        name: "Abomasite".to_string(),
        num: 674,
        gen: 6,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Past".to_string()),
        mega_stone: Some("Abomasnow-Mega".to_string()),
        mega_evolves: Some("Abomasnow".to_string()),
        item_user: Some(vec!["Abomasnow".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("absolite"), ItemDef {
        id: ID::new("absolite"),
        name: "Absolite".to_string(),
        num: 677,
        gen: 6,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Past".to_string()),
        mega_stone: Some("Absol-Mega".to_string()),
        mega_evolves: Some("Absol".to_string()),
        item_user: Some(vec!["Absol".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("absolitez"), ItemDef {
        id: ID::new("absolitez"),
        name: "Absolite Z".to_string(),
        num: 2588,
        gen: 9,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Future".to_string()),
        mega_stone: Some("Absol-Mega-Z".to_string()),
        mega_evolves: Some("Absol".to_string()),
        item_user: Some(vec!["Absol".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("absorbbulb"), ItemDef {
        id: ID::new("absorbbulb"),
        name: "Absorb Bulb".to_string(),
        num: 545,
        gen: 5,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        boosts: Some(boosts(&[("spa", 1)])),
        ..Default::default()
    });

    map.insert(ID::new("zoomlens"), ItemDef {
        id: ID::new("zoomlens"),
        name: "Zoom Lens".to_string(),
        num: 276,
        gen: 4,
        category: ItemCategory::HeldItem,
        fling_power: Some(10),
        ..Default::default()
    });

    map.insert(ID::new("zygardite"), ItemDef {
        id: ID::new("zygardite"),
        name: "Zygardite".to_string(),
        num: 2584,
        gen: 9,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Future".to_string()),
        mega_stone: Some("Zygarde-Mega".to_string()),
        mega_evolves: Some("Zygarde-Complete".to_string()),
        item_user: Some(vec!["Zygarde-Complete".to_string()]),
        ..Default::default()
    });

    map
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
    fn test_item_registry_exists() {
        // Basic sanity check that the registry works
        assert!(!ITEMS.is_empty());
    }

    #[test]
    fn test_get_item() {
        let item = get_item(&ID::new("abilityshield"));
        assert!(item.is_some());
        assert_eq!(item.unwrap().name, "Ability Shield");
    }

    #[test]
    fn test_mega_stone() {
        let item = ITEMS.get(&ID::new("abomasite")).expect("abomasite should exist");
        assert_eq!(item.category, ItemCategory::Mega);
        assert_eq!(item.mega_stone, Some("Abomasnow-Mega".to_string()));
        assert_eq!(item.mega_evolves, Some("Abomasnow".to_string()));
    }

    #[test]
    fn test_item_user() {
        let item = ITEMS.get(&ID::new("absolite")).expect("absolite should exist");
        assert!(item.item_user.is_some());
        assert!(item.item_user.as_ref().unwrap().contains(&"Absol".to_string()));
    }

    #[test]
    fn test_boosts() {
        let item = ITEMS.get(&ID::new("absorbbulb")).expect("absorbbulb should exist");
        assert!(item.boosts.is_some());
        let boosts = item.boosts.as_ref().unwrap();
        assert_eq!(boosts.get("spa"), Some(&1));
    }

    #[test]
    fn test_fling_power() {
        let item = ITEMS.get(&ID::new("abilityshield")).expect("abilityshield should exist");
        assert_eq!(item.fling_power, Some(30));
    }

    #[test]
    fn test_is_nonstandard() {
        let past_item = ITEMS.get(&ID::new("abomasite")).expect("abomasite should exist");
        assert_eq!(past_item.is_nonstandard, Some("Past".to_string()));

        let future_item = ITEMS.get(&ID::new("absolitez")).expect("absolitez should exist");
        assert_eq!(future_item.is_nonstandard, Some("Future".to_string()));
    }

    #[test]
    fn test_item_def_new() {
        let item = ItemDef::new("testitem", "Test Item");
        assert_eq!(item.id.as_str(), "testitem");
        assert_eq!(item.name, "Test Item");
        assert!(item.effects.is_empty());
    }

    #[test]
    fn test_add_effect() {
        let mut item = ItemDef::new("testitem", "Test Item");
        item.add_effect(EventType::Residual, ItemEffect::ResidualHeal { fraction: 0.0625 });

        assert_eq!(item.effects.len(), 1);
        assert!(item.has_effect_for_event(&EventType::Residual));
        assert!(!item.has_effect_for_event(&EventType::Start));
    }

    #[test]
    fn test_get_effects_for_event() {
        let mut item = ItemDef::new("testitem", "Test Item");
        item.add_effect(EventType::Residual, ItemEffect::ResidualHeal { fraction: 0.0625 });
        item.add_effect(EventType::DamagingHit, ItemEffect::ConsumeItem);

        let residual_effects = item.get_effects_for_event(&EventType::Residual);
        assert_eq!(residual_effects.len(), 1);

        let damaging_effects = item.get_effects_for_event(&EventType::DamagingHit);
        assert_eq!(damaging_effects.len(), 1);
    }

    #[test]
    fn test_effect_entry_priority() {
        let entry = ItemEffectEntry::with_priority(
            EventType::ModifyAtk,
            ItemEffect::ModifyStat {
                stat: super::super::item_effects::Stat::Atk,
                multiplier: 1.5,
            },
            1,
        );
        assert_eq!(entry.priority, 1);
    }
}

