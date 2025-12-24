//! Data-driven Item Definitions (Generated)
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

    map.insert(ID::new("abilityshield"), ItemDef {
        id: ID::new("abilityshield"),
        name: "Ability Shield".to_string(),
        num: 1881,
        gen: 9,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        ignore_klutz: true,
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

    map.insert(ID::new("adamantcrystal"), ItemDef {
        id: ID::new("adamantcrystal"),
        name: "Adamant Crystal".to_string(),
        num: 1777,
        gen: 8,
        category: ItemCategory::HeldItem,
        item_user: Some(vec!["Dialga-Origin".to_string()]),
        forced_forme: Some("Dialga-Origin".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("adamantorb"), ItemDef {
        id: ID::new("adamantorb"),
        name: "Adamant Orb".to_string(),
        num: 135,
        gen: 4,
        category: ItemCategory::HeldItem,
        fling_power: Some(60),
        item_user: Some(vec!["Dialga".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("adrenalineorb"), ItemDef {
        id: ID::new("adrenalineorb"),
        name: "Adrenaline Orb".to_string(),
        num: 846,
        gen: 7,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        boosts: Some(boosts(&[("spe", 1)])),
        ..Default::default()
    });

    map.insert(ID::new("aerodactylite"), ItemDef {
        id: ID::new("aerodactylite"),
        name: "Aerodactylite".to_string(),
        num: 672,
        gen: 6,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Past".to_string()),
        mega_stone: Some("Aerodactyl-Mega".to_string()),
        mega_evolves: Some("Aerodactyl".to_string()),
        item_user: Some(vec!["Aerodactyl".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("aggronite"), ItemDef {
        id: ID::new("aggronite"),
        name: "Aggronite".to_string(),
        num: 667,
        gen: 6,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Past".to_string()),
        mega_stone: Some("Aggron-Mega".to_string()),
        mega_evolves: Some("Aggron".to_string()),
        item_user: Some(vec!["Aggron".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("aguavberry"), ItemDef {
        id: ID::new("aguavberry"),
        name: "Aguav Berry".to_string(),
        num: 162,
        gen: 3,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(80),
        natural_gift_type: Some("Dragon".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::Update, ItemEffect::Triggered {
            trigger: Trigger::OnHPBelowGluttony(0.25),
            effect: Box::new(ItemEffect::Compound { effects: vec![
                ItemEffect::HealOnThreshold { threshold: 0.25, heal_fraction: 0.333 },
                ItemEffect::ConsumeItem,
            ] }),
        }, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("airballoon"), ItemDef {
        id: ID::new("airballoon"),
        name: "Air Balloon".to_string(),
        num: 541,
        gen: 5,
        category: ItemCategory::HeldItem,
        fling_power: Some(10),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::SwitchIn, ItemEffect::Airborne, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("alakazite"), ItemDef {
        id: ID::new("alakazite"),
        name: "Alakazite".to_string(),
        num: 679,
        gen: 6,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Past".to_string()),
        mega_stone: Some("Alakazam-Mega".to_string()),
        mega_evolves: Some("Alakazam".to_string()),
        item_user: Some(vec!["Alakazam".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("aloraichiumz"), ItemDef {
        id: ID::new("aloraichiumz"),
        name: "Aloraichium Z".to_string(),
        num: 803,
        gen: 7,
        category: ItemCategory::ZCrystal,
        is_nonstandard: Some("Past".to_string()),
        item_user: Some(vec!["Raichu-Alola".to_string()]),
        z_move: Some("Stoked Sparksurfer".to_string()),
        z_move_from: Some("Thunderbolt".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("altarianite"), ItemDef {
        id: ID::new("altarianite"),
        name: "Altarianite".to_string(),
        num: 755,
        gen: 6,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Past".to_string()),
        mega_stone: Some("Altaria-Mega".to_string()),
        mega_evolves: Some("Altaria".to_string()),
        item_user: Some(vec!["Altaria".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("ampharosite"), ItemDef {
        id: ID::new("ampharosite"),
        name: "Ampharosite".to_string(),
        num: 658,
        gen: 6,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Past".to_string()),
        mega_stone: Some("Ampharos-Mega".to_string()),
        mega_evolves: Some("Ampharos".to_string()),
        item_user: Some(vec!["Ampharos".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("apicotberry"), ItemDef {
        id: ID::new("apicotberry"),
        name: "Apicot Berry".to_string(),
        num: 205,
        gen: 3,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(100),
        natural_gift_type: Some("Ground".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("armorfossil"), ItemDef {
        id: ID::new("armorfossil"),
        name: "Armor Fossil".to_string(),
        num: 104,
        gen: 4,
        category: ItemCategory::Fossil,
        fling_power: Some(100),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("aspearberry"), ItemDef {
        id: ID::new("aspearberry"),
        name: "Aspear Berry".to_string(),
        num: 153,
        gen: 3,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(80),
        natural_gift_type: Some("Ice".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::Update, ItemEffect::Compound { effects: vec![
                ItemEffect::CureStatus { status: "frz".to_string() },
                ItemEffect::ConsumeItem,
            ] }, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("assaultvest"), ItemDef {
        id: ID::new("assaultvest"),
        name: "Assault Vest".to_string(),
        num: 640,
        gen: 6,
        category: ItemCategory::HeldItem,
        fling_power: Some(80),
        has_custom_handler: true,
        ..Default::default()
    });

    map.insert(ID::new("audinite"), ItemDef {
        id: ID::new("audinite"),
        name: "Audinite".to_string(),
        num: 757,
        gen: 6,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Past".to_string()),
        mega_stone: Some("Audino-Mega".to_string()),
        mega_evolves: Some("Audino".to_string()),
        item_user: Some(vec!["Audino".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("auspiciousarmor"), ItemDef {
        id: ID::new("auspiciousarmor"),
        name: "Auspicious Armor".to_string(),
        num: 2344,
        gen: 9,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        ..Default::default()
    });

    map.insert(ID::new("babiriberry"), ItemDef {
        id: ID::new("babiriberry"),
        name: "Babiri Berry".to_string(),
        num: 199,
        gen: 4,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(80),
        natural_gift_type: Some("Steel".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::SourceModifyDamage, ItemEffect::Compound { effects: vec![
                ItemEffect::ResistSuperEffective { resist_type: "Steel".to_string(), multiplier: 0.5 },
                ItemEffect::ConsumeItem,
            ] }, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("banettite"), ItemDef {
        id: ID::new("banettite"),
        name: "Banettite".to_string(),
        num: 668,
        gen: 6,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Past".to_string()),
        mega_stone: Some("Banette-Mega".to_string()),
        mega_evolves: Some("Banette".to_string()),
        item_user: Some(vec!["Banette".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("barbaracite"), ItemDef {
        id: ID::new("barbaracite"),
        name: "Barbaracite".to_string(),
        num: 2581,
        gen: 9,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Future".to_string()),
        mega_stone: Some("Barbaracle-Mega".to_string()),
        mega_evolves: Some("Barbaracle".to_string()),
        item_user: Some(vec!["Barbaracle".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("baxcalibrite"), ItemDef {
        id: ID::new("baxcalibrite"),
        name: "Baxcalibrite".to_string(),
        num: 2601,
        gen: 9,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Future".to_string()),
        mega_stone: Some("Baxcalibur-Mega".to_string()),
        mega_evolves: Some("Baxcalibur".to_string()),
        item_user: Some(vec!["Baxcalibur".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("beastball"), ItemDef {
        id: ID::new("beastball"),
        name: "Beast Ball".to_string(),
        num: 851,
        gen: 7,
        category: ItemCategory::Pokeball,
        is_pokeball: true,
        ..Default::default()
    });

    map.insert(ID::new("beedrillite"), ItemDef {
        id: ID::new("beedrillite"),
        name: "Beedrillite".to_string(),
        num: 770,
        gen: 6,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Past".to_string()),
        mega_stone: Some("Beedrill-Mega".to_string()),
        mega_evolves: Some("Beedrill".to_string()),
        item_user: Some(vec!["Beedrill".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("belueberry"), ItemDef {
        id: ID::new("belueberry"),
        name: "Belue Berry".to_string(),
        num: 183,
        gen: 3,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        is_nonstandard: Some("Past".to_string()),
        natural_gift_power: Some(100),
        natural_gift_type: Some("Electric".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("berry"), ItemDef {
        id: ID::new("berry"),
        name: "Berry".to_string(),
        num: 155,
        gen: 2,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        is_nonstandard: Some("Past".to_string()),
        natural_gift_power: Some(80),
        natural_gift_type: Some("Poison".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("berryjuice"), ItemDef {
        id: ID::new("berryjuice"),
        name: "Berry Juice".to_string(),
        num: 43,
        gen: 2,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("berrysweet"), ItemDef {
        id: ID::new("berrysweet"),
        name: "Berry Sweet".to_string(),
        num: 1111,
        gen: 8,
        category: ItemCategory::HeldItem,
        fling_power: Some(10),
        ..Default::default()
    });

    map.insert(ID::new("berserkgene"), ItemDef {
        id: ID::new("berserkgene"),
        name: "Berserk Gene".to_string(),
        num: 0,
        gen: 2,
        category: ItemCategory::HeldItem,
        is_nonstandard: Some("Past".to_string()),
        boosts: Some(boosts(&[("atk", 2)])),
        ..Default::default()
    });

    map.insert(ID::new("bignugget"), ItemDef {
        id: ID::new("bignugget"),
        name: "Big Nugget".to_string(),
        num: 581,
        gen: 5,
        category: ItemCategory::HeldItem,
        fling_power: Some(130),
        ..Default::default()
    });

    map.insert(ID::new("bigroot"), ItemDef {
        id: ID::new("bigroot"),
        name: "Big Root".to_string(),
        num: 296,
        gen: 4,
        category: ItemCategory::HeldItem,
        fling_power: Some(10),
        has_custom_handler: true,
        ..Default::default()
    });

    map.insert(ID::new("bindingband"), ItemDef {
        id: ID::new("bindingband"),
        name: "Binding Band".to_string(),
        num: 544,
        gen: 5,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        ..Default::default()
    });

    map.insert(ID::new("bitterberry"), ItemDef {
        id: ID::new("bitterberry"),
        name: "Bitter Berry".to_string(),
        num: 156,
        gen: 2,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        is_nonstandard: Some("Past".to_string()),
        natural_gift_power: Some(80),
        natural_gift_type: Some("Ground".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("blackbelt"), ItemDef {
        id: ID::new("blackbelt"),
        name: "Black Belt".to_string(),
        num: 241,
        gen: 2,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::BasePower, ItemEffect::BoostType { move_type: "Fighting".to_string(), multiplier: 1.2 }, 15),
        ],
        ..Default::default()
    });

    map.insert(ID::new("blackglasses"), ItemDef {
        id: ID::new("blackglasses"),
        name: "Black Glasses".to_string(),
        num: 240,
        gen: 2,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::BasePower, ItemEffect::BoostType { move_type: "Dark".to_string(), multiplier: 1.2 }, 15),
        ],
        ..Default::default()
    });

    map.insert(ID::new("blacksludge"), ItemDef {
        id: ID::new("blacksludge"),
        name: "Black Sludge".to_string(),
        num: 281,
        gen: 4,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::Residual, ItemEffect::ResidualHealOrDamage { required_type: "Poison".to_string(), heal_fraction: 0.0625, damage_fraction: 0.125 }, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("blastoisinite"), ItemDef {
        id: ID::new("blastoisinite"),
        name: "Blastoisinite".to_string(),
        num: 661,
        gen: 6,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Past".to_string()),
        mega_stone: Some("Blastoise-Mega".to_string()),
        mega_evolves: Some("Blastoise".to_string()),
        item_user: Some(vec!["Blastoise".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("blazikenite"), ItemDef {
        id: ID::new("blazikenite"),
        name: "Blazikenite".to_string(),
        num: 664,
        gen: 6,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Past".to_string()),
        mega_stone: Some("Blaziken-Mega".to_string()),
        mega_evolves: Some("Blaziken".to_string()),
        item_user: Some(vec!["Blaziken".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("blueorb"), ItemDef {
        id: ID::new("blueorb"),
        name: "Blue Orb".to_string(),
        num: 535,
        gen: 6,
        category: ItemCategory::HeldItem,
        is_nonstandard: Some("Past".to_string()),
        item_user: Some(vec!["Kyogre".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("blukberry"), ItemDef {
        id: ID::new("blukberry"),
        name: "Bluk Berry".to_string(),
        num: 165,
        gen: 3,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        is_nonstandard: Some("Past".to_string()),
        natural_gift_power: Some(90),
        natural_gift_type: Some("Fire".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("blunderpolicy"), ItemDef {
        id: ID::new("blunderpolicy"),
        name: "Blunder Policy".to_string(),
        num: 1121,
        gen: 8,
        category: ItemCategory::HeldItem,
        fling_power: Some(80),
        has_custom_handler: true,
        ..Default::default()
    });

    map.insert(ID::new("boosterenergy"), ItemDef {
        id: ID::new("boosterenergy"),
        name: "Booster Energy".to_string(),
        num: 1880,
        gen: 9,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        has_custom_handler: true,
        ..Default::default()
    });

    map.insert(ID::new("bottlecap"), ItemDef {
        id: ID::new("bottlecap"),
        name: "Bottle Cap".to_string(),
        num: 795,
        gen: 7,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        ..Default::default()
    });

    map.insert(ID::new("brightpowder"), ItemDef {
        id: ID::new("brightpowder"),
        name: "Bright Powder".to_string(),
        num: 213,
        gen: 2,
        category: ItemCategory::HeldItem,
        fling_power: Some(10),
        has_custom_handler: true,
        ..Default::default()
    });

    map.insert(ID::new("buggem"), ItemDef {
        id: ID::new("buggem"),
        name: "Bug Gem".to_string(),
        num: 558,
        gen: 5,
        category: ItemCategory::Gem,
        is_gem: true,
        is_consumable: true,
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("buginiumz"), ItemDef {
        id: ID::new("buginiumz"),
        name: "Buginium Z".to_string(),
        num: 787,
        gen: 7,
        category: ItemCategory::ZCrystal,
        is_nonstandard: Some("Past".to_string()),
        z_move_type: Some("Bug".to_string()),
        forced_forme: Some("Arceus-Bug".to_string()),
        on_plate: Some("Bug".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("bugmemory"), ItemDef {
        id: ID::new("bugmemory"),
        name: "Bug Memory".to_string(),
        num: 909,
        gen: 7,
        category: ItemCategory::Memory,
        is_nonstandard: Some("Past".to_string()),
        item_user: Some(vec!["Silvally-Bug".to_string()]),
        forced_forme: Some("Silvally-Bug".to_string()),
        on_memory: Some("Bug".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("burndrive"), ItemDef {
        id: ID::new("burndrive"),
        name: "Burn Drive".to_string(),
        num: 118,
        gen: 5,
        category: ItemCategory::Drive,
        is_nonstandard: Some("Past".to_string()),
        item_user: Some(vec!["Genesect-Burn".to_string()]),
        forced_forme: Some("Genesect-Burn".to_string()),
        on_drive: Some("Fire".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("burntberry"), ItemDef {
        id: ID::new("burntberry"),
        name: "Burnt Berry".to_string(),
        num: 153,
        gen: 2,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        is_nonstandard: Some("Past".to_string()),
        natural_gift_power: Some(80),
        natural_gift_type: Some("Ice".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("cameruptite"), ItemDef {
        id: ID::new("cameruptite"),
        name: "Cameruptite".to_string(),
        num: 767,
        gen: 6,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Past".to_string()),
        mega_stone: Some("Camerupt-Mega".to_string()),
        mega_evolves: Some("Camerupt".to_string()),
        item_user: Some(vec!["Camerupt".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("cellbattery"), ItemDef {
        id: ID::new("cellbattery"),
        name: "Cell Battery".to_string(),
        num: 546,
        gen: 5,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        boosts: Some(boosts(&[("atk", 1)])),
        ..Default::default()
    });

    map.insert(ID::new("chandelurite"), ItemDef {
        id: ID::new("chandelurite"),
        name: "Chandelurite".to_string(),
        num: 2574,
        gen: 9,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Future".to_string()),
        mega_stone: Some("Chandelure-Mega".to_string()),
        mega_evolves: Some("Chandelure".to_string()),
        item_user: Some(vec!["Chandelure".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("charcoal"), ItemDef {
        id: ID::new("charcoal"),
        name: "Charcoal".to_string(),
        num: 249,
        gen: 2,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::BasePower, ItemEffect::BoostType { move_type: "Fire".to_string(), multiplier: 1.2 }, 15),
        ],
        ..Default::default()
    });

    map.insert(ID::new("charizarditex"), ItemDef {
        id: ID::new("charizarditex"),
        name: "Charizardite X".to_string(),
        num: 660,
        gen: 6,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Past".to_string()),
        mega_stone: Some("Charizard-Mega-X".to_string()),
        mega_evolves: Some("Charizard".to_string()),
        item_user: Some(vec!["Charizard".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("charizarditey"), ItemDef {
        id: ID::new("charizarditey"),
        name: "Charizardite Y".to_string(),
        num: 678,
        gen: 6,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Past".to_string()),
        mega_stone: Some("Charizard-Mega-Y".to_string()),
        mega_evolves: Some("Charizard".to_string()),
        item_user: Some(vec!["Charizard".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("chartiberry"), ItemDef {
        id: ID::new("chartiberry"),
        name: "Charti Berry".to_string(),
        num: 195,
        gen: 4,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(80),
        natural_gift_type: Some("Rock".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::SourceModifyDamage, ItemEffect::Compound { effects: vec![
                ItemEffect::ResistSuperEffective { resist_type: "Rock".to_string(), multiplier: 0.5 },
                ItemEffect::ConsumeItem,
            ] }, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("cheriberry"), ItemDef {
        id: ID::new("cheriberry"),
        name: "Cheri Berry".to_string(),
        num: 149,
        gen: 3,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(80),
        natural_gift_type: Some("Fire".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::Update, ItemEffect::Compound { effects: vec![
                ItemEffect::CureStatus { status: "par".to_string() },
                ItemEffect::ConsumeItem,
            ] }, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("cherishball"), ItemDef {
        id: ID::new("cherishball"),
        name: "Cherish Ball".to_string(),
        num: 16,
        gen: 4,
        category: ItemCategory::Pokeball,
        is_pokeball: true,
        is_nonstandard: Some("Unobtainable".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("chesnaughtite"), ItemDef {
        id: ID::new("chesnaughtite"),
        name: "Chesnaughtite".to_string(),
        num: 2575,
        gen: 9,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Future".to_string()),
        mega_stone: Some("Chesnaught-Mega".to_string()),
        mega_evolves: Some("Chesnaught".to_string()),
        item_user: Some(vec!["Chesnaught".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("chestoberry"), ItemDef {
        id: ID::new("chestoberry"),
        name: "Chesto Berry".to_string(),
        num: 150,
        gen: 3,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(80),
        natural_gift_type: Some("Water".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::Update, ItemEffect::Compound { effects: vec![
                ItemEffect::CureStatus { status: "slp".to_string() },
                ItemEffect::ConsumeItem,
            ] }, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("chilanberry"), ItemDef {
        id: ID::new("chilanberry"),
        name: "Chilan Berry".to_string(),
        num: 200,
        gen: 4,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(80),
        natural_gift_type: Some("Normal".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::SourceModifyDamage, ItemEffect::Compound { effects: vec![
                ItemEffect::ResistSuperEffective { resist_type: "Normal".to_string(), multiplier: 0.5 },
                ItemEffect::ConsumeItem,
            ] }, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("chilldrive"), ItemDef {
        id: ID::new("chilldrive"),
        name: "Chill Drive".to_string(),
        num: 119,
        gen: 5,
        category: ItemCategory::Drive,
        is_nonstandard: Some("Past".to_string()),
        item_user: Some(vec!["Genesect-Chill".to_string()]),
        forced_forme: Some("Genesect-Chill".to_string()),
        on_drive: Some("Ice".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("chimechite"), ItemDef {
        id: ID::new("chimechite"),
        name: "Chimechite".to_string(),
        num: 2587,
        gen: 9,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Future".to_string()),
        mega_stone: Some("Chimecho-Mega".to_string()),
        mega_evolves: Some("Chimecho".to_string()),
        item_user: Some(vec!["Chimecho".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("chippedpot"), ItemDef {
        id: ID::new("chippedpot"),
        name: "Chipped Pot".to_string(),
        num: 1254,
        gen: 8,
        category: ItemCategory::HeldItem,
        fling_power: Some(80),
        ..Default::default()
    });

    map.insert(ID::new("choiceband"), ItemDef {
        id: ID::new("choiceband"),
        name: "Choice Band".to_string(),
        num: 220,
        gen: 3,
        category: ItemCategory::Choice,
        fling_power: Some(10),
        is_choice: true,
        effects: vec![
            ItemEffectEntry::with_priority(EventType::ModifyMove, ItemEffect::ChoiceLock { stat: Stat::Atk, multiplier: 1.5 }, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("choicescarf"), ItemDef {
        id: ID::new("choicescarf"),
        name: "Choice Scarf".to_string(),
        num: 287,
        gen: 4,
        category: ItemCategory::Choice,
        fling_power: Some(10),
        is_choice: true,
        effects: vec![
            ItemEffectEntry::with_priority(EventType::ModifyMove, ItemEffect::ChoiceLock { stat: Stat::Spe, multiplier: 1.5 }, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("choicespecs"), ItemDef {
        id: ID::new("choicespecs"),
        name: "Choice Specs".to_string(),
        num: 297,
        gen: 4,
        category: ItemCategory::Choice,
        fling_power: Some(10),
        is_choice: true,
        effects: vec![
            ItemEffectEntry::with_priority(EventType::ModifyMove, ItemEffect::ChoiceLock { stat: Stat::SpA, multiplier: 1.5 }, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("chopleberry"), ItemDef {
        id: ID::new("chopleberry"),
        name: "Chople Berry".to_string(),
        num: 189,
        gen: 4,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(80),
        natural_gift_type: Some("Fighting".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::SourceModifyDamage, ItemEffect::Compound { effects: vec![
                ItemEffect::ResistSuperEffective { resist_type: "Fighting".to_string(), multiplier: 0.5 },
                ItemEffect::ConsumeItem,
            ] }, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("clawfossil"), ItemDef {
        id: ID::new("clawfossil"),
        name: "Claw Fossil".to_string(),
        num: 100,
        gen: 3,
        category: ItemCategory::Fossil,
        fling_power: Some(100),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("clearamulet"), ItemDef {
        id: ID::new("clearamulet"),
        name: "Clear Amulet".to_string(),
        num: 1882,
        gen: 9,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        has_custom_handler: true,
        ..Default::default()
    });

    map.insert(ID::new("clefablite"), ItemDef {
        id: ID::new("clefablite"),
        name: "Clefablite".to_string(),
        num: 2559,
        gen: 9,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Future".to_string()),
        mega_stone: Some("Clefable-Mega".to_string()),
        mega_evolves: Some("Clefable".to_string()),
        item_user: Some(vec!["Clefable".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("cloversweet"), ItemDef {
        id: ID::new("cloversweet"),
        name: "Clover Sweet".to_string(),
        num: 1112,
        gen: 8,
        category: ItemCategory::HeldItem,
        fling_power: Some(10),
        ..Default::default()
    });

    map.insert(ID::new("cobaberry"), ItemDef {
        id: ID::new("cobaberry"),
        name: "Coba Berry".to_string(),
        num: 192,
        gen: 4,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(80),
        natural_gift_type: Some("Flying".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::SourceModifyDamage, ItemEffect::Compound { effects: vec![
                ItemEffect::ResistSuperEffective { resist_type: "Flying".to_string(), multiplier: 0.5 },
                ItemEffect::ConsumeItem,
            ] }, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("colburberry"), ItemDef {
        id: ID::new("colburberry"),
        name: "Colbur Berry".to_string(),
        num: 198,
        gen: 4,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(80),
        natural_gift_type: Some("Dark".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::SourceModifyDamage, ItemEffect::Compound { effects: vec![
                ItemEffect::ResistSuperEffective { resist_type: "Dark".to_string(), multiplier: 0.5 },
                ItemEffect::ConsumeItem,
            ] }, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("cornerstonemask"), ItemDef {
        id: ID::new("cornerstonemask"),
        name: "Cornerstone Mask".to_string(),
        num: 2406,
        gen: 9,
        category: ItemCategory::HeldItem,
        fling_power: Some(60),
        item_user: Some(vec!["Ogerpon-Cornerstone".to_string()]),
        forced_forme: Some("Ogerpon-Cornerstone".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("cornnberry"), ItemDef {
        id: ID::new("cornnberry"),
        name: "Cornn Berry".to_string(),
        num: 175,
        gen: 3,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        is_nonstandard: Some("Past".to_string()),
        natural_gift_power: Some(90),
        natural_gift_type: Some("Bug".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("coverfossil"), ItemDef {
        id: ID::new("coverfossil"),
        name: "Cover Fossil".to_string(),
        num: 572,
        gen: 5,
        category: ItemCategory::Fossil,
        fling_power: Some(100),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("covertcloak"), ItemDef {
        id: ID::new("covertcloak"),
        name: "Covert Cloak".to_string(),
        num: 1885,
        gen: 9,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        ..Default::default()
    });

    map.insert(ID::new("crabominite"), ItemDef {
        id: ID::new("crabominite"),
        name: "Crabominite".to_string(),
        num: 2595,
        gen: 9,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Future".to_string()),
        mega_stone: Some("Crabominable-Mega".to_string()),
        mega_evolves: Some("Crabominable".to_string()),
        item_user: Some(vec!["Crabominable".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("crackedpot"), ItemDef {
        id: ID::new("crackedpot"),
        name: "Cracked Pot".to_string(),
        num: 1253,
        gen: 8,
        category: ItemCategory::HeldItem,
        fling_power: Some(80),
        ..Default::default()
    });

    map.insert(ID::new("crucibellite"), ItemDef {
        id: ID::new("crucibellite"),
        name: "Crucibellite".to_string(),
        num: -1,
        gen: 6,
        category: ItemCategory::Mega,
        is_nonstandard: Some("CAP".to_string()),
        mega_stone: Some("Crucibelle-Mega".to_string()),
        mega_evolves: Some("Crucibelle".to_string()),
        item_user: Some(vec!["Crucibelle".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("custapberry"), ItemDef {
        id: ID::new("custapberry"),
        name: "Custap Berry".to_string(),
        num: 210,
        gen: 4,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(100),
        natural_gift_type: Some("Ghost".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("damprock"), ItemDef {
        id: ID::new("damprock"),
        name: "Damp Rock".to_string(),
        num: 285,
        gen: 4,
        category: ItemCategory::HeldItem,
        fling_power: Some(60),
        ..Default::default()
    });

    map.insert(ID::new("darkgem"), ItemDef {
        id: ID::new("darkgem"),
        name: "Dark Gem".to_string(),
        num: 562,
        gen: 5,
        category: ItemCategory::Gem,
        is_gem: true,
        is_consumable: true,
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("darkiniumz"), ItemDef {
        id: ID::new("darkiniumz"),
        name: "Darkinium Z".to_string(),
        num: 791,
        gen: 7,
        category: ItemCategory::ZCrystal,
        is_nonstandard: Some("Past".to_string()),
        z_move_type: Some("Dark".to_string()),
        forced_forme: Some("Arceus-Dark".to_string()),
        on_plate: Some("Dark".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("darkmemory"), ItemDef {
        id: ID::new("darkmemory"),
        name: "Dark Memory".to_string(),
        num: 919,
        gen: 7,
        category: ItemCategory::Memory,
        is_nonstandard: Some("Past".to_string()),
        item_user: Some(vec!["Silvally-Dark".to_string()]),
        forced_forme: Some("Silvally-Dark".to_string()),
        on_memory: Some("Dark".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("darkranite"), ItemDef {
        id: ID::new("darkranite"),
        name: "Darkranite".to_string(),
        num: 2593,
        gen: 9,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Future".to_string()),
        mega_stone: Some("Darkrai-Mega".to_string()),
        mega_evolves: Some("Darkrai".to_string()),
        item_user: Some(vec!["Darkrai".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("dawnstone"), ItemDef {
        id: ID::new("dawnstone"),
        name: "Dawn Stone".to_string(),
        num: 109,
        gen: 4,
        category: ItemCategory::HeldItem,
        fling_power: Some(80),
        ..Default::default()
    });

    map.insert(ID::new("decidiumz"), ItemDef {
        id: ID::new("decidiumz"),
        name: "Decidium Z".to_string(),
        num: 798,
        gen: 7,
        category: ItemCategory::ZCrystal,
        is_nonstandard: Some("Past".to_string()),
        item_user: Some(vec!["Decidueye".to_string()]),
        z_move: Some("Sinister Arrow Raid".to_string()),
        z_move_from: Some("Spirit Shackle".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("deepseascale"), ItemDef {
        id: ID::new("deepseascale"),
        name: "Deep Sea Scale".to_string(),
        num: 227,
        gen: 3,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        is_nonstandard: Some("Past".to_string()),
        item_user: Some(vec!["Clamperl".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("deepseatooth"), ItemDef {
        id: ID::new("deepseatooth"),
        name: "Deep Sea Tooth".to_string(),
        num: 226,
        gen: 3,
        category: ItemCategory::HeldItem,
        fling_power: Some(90),
        is_nonstandard: Some("Past".to_string()),
        item_user: Some(vec!["Clamperl".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("delphoxite"), ItemDef {
        id: ID::new("delphoxite"),
        name: "Delphoxite".to_string(),
        num: 2576,
        gen: 9,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Future".to_string()),
        mega_stone: Some("Delphox-Mega".to_string()),
        mega_evolves: Some("Delphox".to_string()),
        item_user: Some(vec!["Delphox".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("destinyknot"), ItemDef {
        id: ID::new("destinyknot"),
        name: "Destiny Knot".to_string(),
        num: 280,
        gen: 4,
        category: ItemCategory::HeldItem,
        fling_power: Some(10),
        ..Default::default()
    });

    map.insert(ID::new("diancite"), ItemDef {
        id: ID::new("diancite"),
        name: "Diancite".to_string(),
        num: 764,
        gen: 6,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Past".to_string()),
        mega_stone: Some("Diancie-Mega".to_string()),
        mega_evolves: Some("Diancie".to_string()),
        item_user: Some(vec!["Diancie".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("diveball"), ItemDef {
        id: ID::new("diveball"),
        name: "Dive Ball".to_string(),
        num: 7,
        gen: 3,
        category: ItemCategory::Pokeball,
        is_pokeball: true,
        ..Default::default()
    });

    map.insert(ID::new("domefossil"), ItemDef {
        id: ID::new("domefossil"),
        name: "Dome Fossil".to_string(),
        num: 102,
        gen: 3,
        category: ItemCategory::Fossil,
        fling_power: Some(100),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("dousedrive"), ItemDef {
        id: ID::new("dousedrive"),
        name: "Douse Drive".to_string(),
        num: 116,
        gen: 5,
        category: ItemCategory::Drive,
        is_nonstandard: Some("Past".to_string()),
        item_user: Some(vec!["Genesect-Douse".to_string()]),
        forced_forme: Some("Genesect-Douse".to_string()),
        on_drive: Some("Water".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("dracoplate"), ItemDef {
        id: ID::new("dracoplate"),
        name: "Draco Plate".to_string(),
        num: 311,
        gen: 4,
        category: ItemCategory::Plate,
        forced_forme: Some("Arceus-Dragon".to_string()),
        on_plate: Some("Dragon".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::BasePower, ItemEffect::BoostType { move_type: "Dragon".to_string(), multiplier: 1.2 }, 15),
        ],
        ..Default::default()
    });

    map.insert(ID::new("dragalgite"), ItemDef {
        id: ID::new("dragalgite"),
        name: "Dragalgite".to_string(),
        num: 2582,
        gen: 9,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Future".to_string()),
        mega_stone: Some("Dragalge-Mega".to_string()),
        mega_evolves: Some("Dragalge".to_string()),
        item_user: Some(vec!["Dragalge".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("dragonfang"), ItemDef {
        id: ID::new("dragonfang"),
        name: "Dragon Fang".to_string(),
        num: 250,
        gen: 2,
        category: ItemCategory::HeldItem,
        fling_power: Some(70),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::BasePower, ItemEffect::BoostType { move_type: "Dragon".to_string(), multiplier: 1.2 }, 15),
        ],
        ..Default::default()
    });

    map.insert(ID::new("dragongem"), ItemDef {
        id: ID::new("dragongem"),
        name: "Dragon Gem".to_string(),
        num: 561,
        gen: 5,
        category: ItemCategory::Gem,
        is_gem: true,
        is_consumable: true,
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("dragoninite"), ItemDef {
        id: ID::new("dragoninite"),
        name: "Dragoninite".to_string(),
        num: 2562,
        gen: 9,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Future".to_string()),
        mega_stone: Some("Dragonite-Mega".to_string()),
        mega_evolves: Some("Dragonite".to_string()),
        item_user: Some(vec!["Dragonite".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("dragoniumz"), ItemDef {
        id: ID::new("dragoniumz"),
        name: "Dragonium Z".to_string(),
        num: 790,
        gen: 7,
        category: ItemCategory::ZCrystal,
        is_nonstandard: Some("Past".to_string()),
        z_move_type: Some("Dragon".to_string()),
        forced_forme: Some("Arceus-Dragon".to_string()),
        on_plate: Some("Dragon".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("dragonmemory"), ItemDef {
        id: ID::new("dragonmemory"),
        name: "Dragon Memory".to_string(),
        num: 918,
        gen: 7,
        category: ItemCategory::Memory,
        is_nonstandard: Some("Past".to_string()),
        item_user: Some(vec!["Silvally-Dragon".to_string()]),
        forced_forme: Some("Silvally-Dragon".to_string()),
        on_memory: Some("Dragon".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("dragonscale"), ItemDef {
        id: ID::new("dragonscale"),
        name: "Dragon Scale".to_string(),
        num: 235,
        gen: 2,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        ..Default::default()
    });

    map.insert(ID::new("drampanite"), ItemDef {
        id: ID::new("drampanite"),
        name: "Drampanite".to_string(),
        num: 2585,
        gen: 9,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Future".to_string()),
        mega_stone: Some("Drampa-Mega".to_string()),
        mega_evolves: Some("Drampa".to_string()),
        item_user: Some(vec!["Drampa".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("dreadplate"), ItemDef {
        id: ID::new("dreadplate"),
        name: "Dread Plate".to_string(),
        num: 312,
        gen: 4,
        category: ItemCategory::Plate,
        forced_forme: Some("Arceus-Dark".to_string()),
        on_plate: Some("Dark".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::BasePower, ItemEffect::BoostType { move_type: "Dark".to_string(), multiplier: 1.2 }, 15),
        ],
        ..Default::default()
    });

    map.insert(ID::new("dreamball"), ItemDef {
        id: ID::new("dreamball"),
        name: "Dream Ball".to_string(),
        num: 576,
        gen: 5,
        category: ItemCategory::Pokeball,
        is_pokeball: true,
        ..Default::default()
    });

    map.insert(ID::new("dubiousdisc"), ItemDef {
        id: ID::new("dubiousdisc"),
        name: "Dubious Disc".to_string(),
        num: 324,
        gen: 4,
        category: ItemCategory::HeldItem,
        fling_power: Some(50),
        ..Default::default()
    });

    map.insert(ID::new("durinberry"), ItemDef {
        id: ID::new("durinberry"),
        name: "Durin Berry".to_string(),
        num: 182,
        gen: 3,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        is_nonstandard: Some("Past".to_string()),
        natural_gift_power: Some(100),
        natural_gift_type: Some("Water".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("duskball"), ItemDef {
        id: ID::new("duskball"),
        name: "Dusk Ball".to_string(),
        num: 13,
        gen: 4,
        category: ItemCategory::Pokeball,
        is_pokeball: true,
        ..Default::default()
    });

    map.insert(ID::new("duskstone"), ItemDef {
        id: ID::new("duskstone"),
        name: "Dusk Stone".to_string(),
        num: 108,
        gen: 4,
        category: ItemCategory::HeldItem,
        fling_power: Some(80),
        ..Default::default()
    });

    map.insert(ID::new("earthplate"), ItemDef {
        id: ID::new("earthplate"),
        name: "Earth Plate".to_string(),
        num: 305,
        gen: 4,
        category: ItemCategory::Plate,
        forced_forme: Some("Arceus-Ground".to_string()),
        on_plate: Some("Ground".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::BasePower, ItemEffect::BoostType { move_type: "Ground".to_string(), multiplier: 1.2 }, 15),
        ],
        ..Default::default()
    });

    map.insert(ID::new("eelektrossite"), ItemDef {
        id: ID::new("eelektrossite"),
        name: "Eelektrossite".to_string(),
        num: 2573,
        gen: 9,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Future".to_string()),
        mega_stone: Some("Eelektross-Mega".to_string()),
        mega_evolves: Some("Eelektross".to_string()),
        item_user: Some(vec!["Eelektross".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("eeviumz"), ItemDef {
        id: ID::new("eeviumz"),
        name: "Eevium Z".to_string(),
        num: 805,
        gen: 7,
        category: ItemCategory::ZCrystal,
        is_nonstandard: Some("Past".to_string()),
        item_user: Some(vec!["Eevee".to_string()]),
        z_move: Some("Extreme Evoboost".to_string()),
        z_move_from: Some("Last Resort".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("ejectbutton"), ItemDef {
        id: ID::new("ejectbutton"),
        name: "Eject Button".to_string(),
        num: 547,
        gen: 5,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::DamagingHit, ItemEffect::Compound { effects: vec![ItemEffect::EjectOnHit, ItemEffect::ConsumeItem] }, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("ejectpack"), ItemDef {
        id: ID::new("ejectpack"),
        name: "Eject Pack".to_string(),
        num: 1119,
        gen: 8,
        category: ItemCategory::HeldItem,
        fling_power: Some(50),
        ..Default::default()
    });

    map.insert(ID::new("electirizer"), ItemDef {
        id: ID::new("electirizer"),
        name: "Electirizer".to_string(),
        num: 322,
        gen: 4,
        category: ItemCategory::HeldItem,
        fling_power: Some(80),
        ..Default::default()
    });

    map.insert(ID::new("electricgem"), ItemDef {
        id: ID::new("electricgem"),
        name: "Electric Gem".to_string(),
        num: 550,
        gen: 5,
        category: ItemCategory::Gem,
        is_gem: true,
        is_consumable: true,
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("electricmemory"), ItemDef {
        id: ID::new("electricmemory"),
        name: "Electric Memory".to_string(),
        num: 915,
        gen: 7,
        category: ItemCategory::Memory,
        is_nonstandard: Some("Past".to_string()),
        item_user: Some(vec!["Silvally-Electric".to_string()]),
        forced_forme: Some("Silvally-Electric".to_string()),
        on_memory: Some("Electric".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("electricseed"), ItemDef {
        id: ID::new("electricseed"),
        name: "Electric Seed".to_string(),
        num: 881,
        gen: 7,
        category: ItemCategory::HeldItem,
        fling_power: Some(10),
        boosts: Some(boosts(&[("def", 1)])),
        ..Default::default()
    });

    map.insert(ID::new("electriumz"), ItemDef {
        id: ID::new("electriumz"),
        name: "Electrium Z".to_string(),
        num: 779,
        gen: 7,
        category: ItemCategory::ZCrystal,
        is_nonstandard: Some("Past".to_string()),
        z_move_type: Some("Electric".to_string()),
        forced_forme: Some("Arceus-Electric".to_string()),
        on_plate: Some("Electric".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("emboarite"), ItemDef {
        id: ID::new("emboarite"),
        name: "Emboarite".to_string(),
        num: 2569,
        gen: 9,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Future".to_string()),
        mega_stone: Some("Emboar-Mega".to_string()),
        mega_evolves: Some("Emboar".to_string()),
        item_user: Some(vec!["Emboar".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("enigmaberry"), ItemDef {
        id: ID::new("enigmaberry"),
        name: "Enigma Berry".to_string(),
        num: 208,
        gen: 3,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(100),
        natural_gift_type: Some("Bug".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("eviolite"), ItemDef {
        id: ID::new("eviolite"),
        name: "Eviolite".to_string(),
        num: 538,
        gen: 5,
        category: ItemCategory::HeldItem,
        fling_power: Some(40),
        has_custom_handler: true,
        ..Default::default()
    });

    map.insert(ID::new("excadrite"), ItemDef {
        id: ID::new("excadrite"),
        name: "Excadrite".to_string(),
        num: 2570,
        gen: 9,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Future".to_string()),
        mega_stone: Some("Excadrill-Mega".to_string()),
        mega_evolves: Some("Excadrill".to_string()),
        item_user: Some(vec!["Excadrill".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("expertbelt"), ItemDef {
        id: ID::new("expertbelt"),
        name: "Expert Belt".to_string(),
        num: 268,
        gen: 4,
        category: ItemCategory::HeldItem,
        fling_power: Some(10),
        has_custom_handler: true,
        ..Default::default()
    });

    map.insert(ID::new("fairiumz"), ItemDef {
        id: ID::new("fairiumz"),
        name: "Fairium Z".to_string(),
        num: 793,
        gen: 7,
        category: ItemCategory::ZCrystal,
        is_nonstandard: Some("Past".to_string()),
        z_move_type: Some("Fairy".to_string()),
        forced_forme: Some("Arceus-Fairy".to_string()),
        on_plate: Some("Fairy".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("fairyfeather"), ItemDef {
        id: ID::new("fairyfeather"),
        name: "Fairy Feather".to_string(),
        num: 2401,
        gen: 9,
        category: ItemCategory::HeldItem,
        fling_power: Some(10),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::BasePower, ItemEffect::BoostType { move_type: "Fairy".to_string(), multiplier: 1.2 }, 15),
        ],
        ..Default::default()
    });

    map.insert(ID::new("fairygem"), ItemDef {
        id: ID::new("fairygem"),
        name: "Fairy Gem".to_string(),
        num: 715,
        gen: 6,
        category: ItemCategory::Gem,
        is_gem: true,
        is_consumable: true,
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("fairymemory"), ItemDef {
        id: ID::new("fairymemory"),
        name: "Fairy Memory".to_string(),
        num: 920,
        gen: 7,
        category: ItemCategory::Memory,
        is_nonstandard: Some("Past".to_string()),
        item_user: Some(vec!["Silvally-Fairy".to_string()]),
        forced_forme: Some("Silvally-Fairy".to_string()),
        on_memory: Some("Fairy".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("falinksite"), ItemDef {
        id: ID::new("falinksite"),
        name: "Falinksite".to_string(),
        num: 2586,
        gen: 9,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Future".to_string()),
        mega_stone: Some("Falinks-Mega".to_string()),
        mega_evolves: Some("Falinks".to_string()),
        item_user: Some(vec!["Falinks".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("fastball"), ItemDef {
        id: ID::new("fastball"),
        name: "Fast Ball".to_string(),
        num: 492,
        gen: 2,
        category: ItemCategory::Pokeball,
        is_pokeball: true,
        ..Default::default()
    });

    map.insert(ID::new("feraligite"), ItemDef {
        id: ID::new("feraligite"),
        name: "Feraligite".to_string(),
        num: 2564,
        gen: 9,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Future".to_string()),
        mega_stone: Some("Feraligatr-Mega".to_string()),
        mega_evolves: Some("Feraligatr".to_string()),
        item_user: Some(vec!["Feraligatr".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("fightinggem"), ItemDef {
        id: ID::new("fightinggem"),
        name: "Fighting Gem".to_string(),
        num: 553,
        gen: 5,
        category: ItemCategory::Gem,
        is_gem: true,
        is_consumable: true,
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("fightingmemory"), ItemDef {
        id: ID::new("fightingmemory"),
        name: "Fighting Memory".to_string(),
        num: 904,
        gen: 7,
        category: ItemCategory::Memory,
        is_nonstandard: Some("Past".to_string()),
        item_user: Some(vec!["Silvally-Fighting".to_string()]),
        forced_forme: Some("Silvally-Fighting".to_string()),
        on_memory: Some("Fighting".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("fightiniumz"), ItemDef {
        id: ID::new("fightiniumz"),
        name: "Fightinium Z".to_string(),
        num: 782,
        gen: 7,
        category: ItemCategory::ZCrystal,
        is_nonstandard: Some("Past".to_string()),
        z_move_type: Some("Fighting".to_string()),
        forced_forme: Some("Arceus-Fighting".to_string()),
        on_plate: Some("Fighting".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("figyberry"), ItemDef {
        id: ID::new("figyberry"),
        name: "Figy Berry".to_string(),
        num: 159,
        gen: 3,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(80),
        natural_gift_type: Some("Bug".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::Update, ItemEffect::Triggered {
            trigger: Trigger::OnHPBelowGluttony(0.25),
            effect: Box::new(ItemEffect::Compound { effects: vec![
                ItemEffect::HealOnThreshold { threshold: 0.25, heal_fraction: 0.333 },
                ItemEffect::ConsumeItem,
            ] }),
        }, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("firegem"), ItemDef {
        id: ID::new("firegem"),
        name: "Fire Gem".to_string(),
        num: 548,
        gen: 5,
        category: ItemCategory::Gem,
        is_gem: true,
        is_consumable: true,
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("firememory"), ItemDef {
        id: ID::new("firememory"),
        name: "Fire Memory".to_string(),
        num: 912,
        gen: 7,
        category: ItemCategory::Memory,
        is_nonstandard: Some("Past".to_string()),
        item_user: Some(vec!["Silvally-Fire".to_string()]),
        forced_forme: Some("Silvally-Fire".to_string()),
        on_memory: Some("Fire".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("firestone"), ItemDef {
        id: ID::new("firestone"),
        name: "Fire Stone".to_string(),
        num: 82,
        gen: 1,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        ..Default::default()
    });

    map.insert(ID::new("firiumz"), ItemDef {
        id: ID::new("firiumz"),
        name: "Firium Z".to_string(),
        num: 777,
        gen: 7,
        category: ItemCategory::ZCrystal,
        is_nonstandard: Some("Past".to_string()),
        z_move_type: Some("Fire".to_string()),
        forced_forme: Some("Arceus-Fire".to_string()),
        on_plate: Some("Fire".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("fistplate"), ItemDef {
        id: ID::new("fistplate"),
        name: "Fist Plate".to_string(),
        num: 303,
        gen: 4,
        category: ItemCategory::Plate,
        forced_forme: Some("Arceus-Fighting".to_string()),
        on_plate: Some("Fighting".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::BasePower, ItemEffect::BoostType { move_type: "Fighting".to_string(), multiplier: 1.2 }, 15),
        ],
        ..Default::default()
    });

    map.insert(ID::new("flameorb"), ItemDef {
        id: ID::new("flameorb"),
        name: "Flame Orb".to_string(),
        num: 273,
        gen: 4,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        ..Default::default()
    });

    map.insert(ID::new("flameplate"), ItemDef {
        id: ID::new("flameplate"),
        name: "Flame Plate".to_string(),
        num: 298,
        gen: 4,
        category: ItemCategory::Plate,
        forced_forme: Some("Arceus-Fire".to_string()),
        on_plate: Some("Fire".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::BasePower, ItemEffect::BoostType { move_type: "Fire".to_string(), multiplier: 1.2 }, 15),
        ],
        ..Default::default()
    });

    map.insert(ID::new("floatstone"), ItemDef {
        id: ID::new("floatstone"),
        name: "Float Stone".to_string(),
        num: 539,
        gen: 5,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        ..Default::default()
    });

    map.insert(ID::new("floettite"), ItemDef {
        id: ID::new("floettite"),
        name: "Floettite".to_string(),
        num: 2579,
        gen: 9,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Future".to_string()),
        mega_stone: Some("Floette-Mega".to_string()),
        mega_evolves: Some("Floette-Eternal".to_string()),
        item_user: Some(vec!["Floette-Eternal".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("flowersweet"), ItemDef {
        id: ID::new("flowersweet"),
        name: "Flower Sweet".to_string(),
        num: 1113,
        gen: 8,
        category: ItemCategory::HeldItem,
        ..Default::default()
    });

    map.insert(ID::new("flyinggem"), ItemDef {
        id: ID::new("flyinggem"),
        name: "Flying Gem".to_string(),
        num: 556,
        gen: 5,
        category: ItemCategory::Gem,
        is_gem: true,
        is_consumable: true,
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("flyingmemory"), ItemDef {
        id: ID::new("flyingmemory"),
        name: "Flying Memory".to_string(),
        num: 905,
        gen: 7,
        category: ItemCategory::Memory,
        is_nonstandard: Some("Past".to_string()),
        item_user: Some(vec!["Silvally-Flying".to_string()]),
        forced_forme: Some("Silvally-Flying".to_string()),
        on_memory: Some("Flying".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("flyiniumz"), ItemDef {
        id: ID::new("flyiniumz"),
        name: "Flyinium Z".to_string(),
        num: 785,
        gen: 7,
        category: ItemCategory::ZCrystal,
        is_nonstandard: Some("Past".to_string()),
        z_move_type: Some("Flying".to_string()),
        forced_forme: Some("Arceus-Flying".to_string()),
        on_plate: Some("Flying".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("focusband"), ItemDef {
        id: ID::new("focusband"),
        name: "Focus Band".to_string(),
        num: 230,
        gen: 2,
        category: ItemCategory::HeldItem,
        fling_power: Some(10),
        ..Default::default()
    });

    map.insert(ID::new("focussash"), ItemDef {
        id: ID::new("focussash"),
        name: "Focus Sash".to_string(),
        num: 275,
        gen: 4,
        category: ItemCategory::HeldItem,
        fling_power: Some(10),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::Damage, ItemEffect::Compound { effects: vec![ItemEffect::FocusSash, ItemEffect::ConsumeItem] }, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("fossilizedbird"), ItemDef {
        id: ID::new("fossilizedbird"),
        name: "Fossilized Bird".to_string(),
        num: 1105,
        gen: 8,
        category: ItemCategory::Fossil,
        fling_power: Some(100),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("fossilizeddino"), ItemDef {
        id: ID::new("fossilizeddino"),
        name: "Fossilized Dino".to_string(),
        num: 1108,
        gen: 8,
        category: ItemCategory::Fossil,
        fling_power: Some(100),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("fossilizeddrake"), ItemDef {
        id: ID::new("fossilizeddrake"),
        name: "Fossilized Drake".to_string(),
        num: 1107,
        gen: 8,
        category: ItemCategory::Fossil,
        fling_power: Some(100),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("fossilizedfish"), ItemDef {
        id: ID::new("fossilizedfish"),
        name: "Fossilized Fish".to_string(),
        num: 1106,
        gen: 8,
        category: ItemCategory::Fossil,
        fling_power: Some(100),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("friendball"), ItemDef {
        id: ID::new("friendball"),
        name: "Friend Ball".to_string(),
        num: 497,
        gen: 2,
        category: ItemCategory::Pokeball,
        is_pokeball: true,
        ..Default::default()
    });

    map.insert(ID::new("froslassite"), ItemDef {
        id: ID::new("froslassite"),
        name: "Froslassite".to_string(),
        num: 2566,
        gen: 9,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Future".to_string()),
        mega_stone: Some("Froslass-Mega".to_string()),
        mega_evolves: Some("Froslass".to_string()),
        item_user: Some(vec!["Froslass".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("fullincense"), ItemDef {
        id: ID::new("fullincense"),
        name: "Full Incense".to_string(),
        num: 316,
        gen: 4,
        category: ItemCategory::HeldItem,
        fling_power: Some(10),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("galaricacuff"), ItemDef {
        id: ID::new("galaricacuff"),
        name: "Galarica Cuff".to_string(),
        num: 1582,
        gen: 8,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        ..Default::default()
    });

    map.insert(ID::new("galaricawreath"), ItemDef {
        id: ID::new("galaricawreath"),
        name: "Galarica Wreath".to_string(),
        num: 1592,
        gen: 8,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        ..Default::default()
    });

    map.insert(ID::new("galladite"), ItemDef {
        id: ID::new("galladite"),
        name: "Galladite".to_string(),
        num: 756,
        gen: 6,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Past".to_string()),
        mega_stone: Some("Gallade-Mega".to_string()),
        mega_evolves: Some("Gallade".to_string()),
        item_user: Some(vec!["Gallade".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("ganlonberry"), ItemDef {
        id: ID::new("ganlonberry"),
        name: "Ganlon Berry".to_string(),
        num: 202,
        gen: 3,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(100),
        natural_gift_type: Some("Ice".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("garchompite"), ItemDef {
        id: ID::new("garchompite"),
        name: "Garchompite".to_string(),
        num: 683,
        gen: 6,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Past".to_string()),
        mega_stone: Some("Garchomp-Mega".to_string()),
        mega_evolves: Some("Garchomp".to_string()),
        item_user: Some(vec!["Garchomp".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("garchompitez"), ItemDef {
        id: ID::new("garchompitez"),
        name: "Garchompite Z".to_string(),
        num: 2590,
        gen: 9,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Future".to_string()),
        mega_stone: Some("Garchomp-Mega-Z".to_string()),
        mega_evolves: Some("Garchomp".to_string()),
        item_user: Some(vec!["Garchomp".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("gardevoirite"), ItemDef {
        id: ID::new("gardevoirite"),
        name: "Gardevoirite".to_string(),
        num: 657,
        gen: 6,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Past".to_string()),
        mega_stone: Some("Gardevoir-Mega".to_string()),
        mega_evolves: Some("Gardevoir".to_string()),
        item_user: Some(vec!["Gardevoir".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("gengarite"), ItemDef {
        id: ID::new("gengarite"),
        name: "Gengarite".to_string(),
        num: 656,
        gen: 6,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Past".to_string()),
        mega_stone: Some("Gengar-Mega".to_string()),
        mega_evolves: Some("Gengar".to_string()),
        item_user: Some(vec!["Gengar".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("ghostgem"), ItemDef {
        id: ID::new("ghostgem"),
        name: "Ghost Gem".to_string(),
        num: 560,
        gen: 5,
        category: ItemCategory::Gem,
        is_gem: true,
        is_consumable: true,
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("ghostiumz"), ItemDef {
        id: ID::new("ghostiumz"),
        name: "Ghostium Z".to_string(),
        num: 789,
        gen: 7,
        category: ItemCategory::ZCrystal,
        is_nonstandard: Some("Past".to_string()),
        z_move_type: Some("Ghost".to_string()),
        forced_forme: Some("Arceus-Ghost".to_string()),
        on_plate: Some("Ghost".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("ghostmemory"), ItemDef {
        id: ID::new("ghostmemory"),
        name: "Ghost Memory".to_string(),
        num: 910,
        gen: 7,
        category: ItemCategory::Memory,
        is_nonstandard: Some("Past".to_string()),
        item_user: Some(vec!["Silvally-Ghost".to_string()]),
        forced_forme: Some("Silvally-Ghost".to_string()),
        on_memory: Some("Ghost".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("glalitite"), ItemDef {
        id: ID::new("glalitite"),
        name: "Glalitite".to_string(),
        num: 763,
        gen: 6,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Past".to_string()),
        mega_stone: Some("Glalie-Mega".to_string()),
        mega_evolves: Some("Glalie".to_string()),
        item_user: Some(vec!["Glalie".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("glimmoranite"), ItemDef {
        id: ID::new("glimmoranite"),
        name: "Glimmoranite".to_string(),
        num: 2600,
        gen: 9,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Future".to_string()),
        mega_stone: Some("Glimmora-Mega".to_string()),
        mega_evolves: Some("Glimmora".to_string()),
        item_user: Some(vec!["Glimmora".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("goldberry"), ItemDef {
        id: ID::new("goldberry"),
        name: "Gold Berry".to_string(),
        num: 158,
        gen: 2,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        is_nonstandard: Some("Past".to_string()),
        natural_gift_power: Some(80),
        natural_gift_type: Some("Psychic".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("goldbottlecap"), ItemDef {
        id: ID::new("goldbottlecap"),
        name: "Gold Bottle Cap".to_string(),
        num: 796,
        gen: 7,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        ..Default::default()
    });

    map.insert(ID::new("golisopite"), ItemDef {
        id: ID::new("golisopite"),
        name: "Golisopite".to_string(),
        num: 2596,
        gen: 9,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Future".to_string()),
        mega_stone: Some("Golisopod-Mega".to_string()),
        mega_evolves: Some("Golisopod".to_string()),
        item_user: Some(vec!["Golisopod".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("golurkite"), ItemDef {
        id: ID::new("golurkite"),
        name: "Golurkite".to_string(),
        num: 2594,
        gen: 9,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Future".to_string()),
        mega_stone: Some("Golurk-Mega".to_string()),
        mega_evolves: Some("Golurk".to_string()),
        item_user: Some(vec!["Golurk".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("grassgem"), ItemDef {
        id: ID::new("grassgem"),
        name: "Grass Gem".to_string(),
        num: 551,
        gen: 5,
        category: ItemCategory::Gem,
        is_gem: true,
        is_consumable: true,
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("grassiumz"), ItemDef {
        id: ID::new("grassiumz"),
        name: "Grassium Z".to_string(),
        num: 780,
        gen: 7,
        category: ItemCategory::ZCrystal,
        is_nonstandard: Some("Past".to_string()),
        z_move_type: Some("Grass".to_string()),
        forced_forme: Some("Arceus-Grass".to_string()),
        on_plate: Some("Grass".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("grassmemory"), ItemDef {
        id: ID::new("grassmemory"),
        name: "Grass Memory".to_string(),
        num: 914,
        gen: 7,
        category: ItemCategory::Memory,
        is_nonstandard: Some("Past".to_string()),
        item_user: Some(vec!["Silvally-Grass".to_string()]),
        forced_forme: Some("Silvally-Grass".to_string()),
        on_memory: Some("Grass".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("grassyseed"), ItemDef {
        id: ID::new("grassyseed"),
        name: "Grassy Seed".to_string(),
        num: 884,
        gen: 7,
        category: ItemCategory::HeldItem,
        fling_power: Some(10),
        boosts: Some(boosts(&[("def", 1)])),
        ..Default::default()
    });

    map.insert(ID::new("greatball"), ItemDef {
        id: ID::new("greatball"),
        name: "Great Ball".to_string(),
        num: 3,
        gen: 1,
        category: ItemCategory::Pokeball,
        is_pokeball: true,
        ..Default::default()
    });

    map.insert(ID::new("greninjite"), ItemDef {
        id: ID::new("greninjite"),
        name: "Greninjite".to_string(),
        num: 2577,
        gen: 9,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Future".to_string()),
        mega_stone: Some("Greninja-Mega".to_string()),
        mega_evolves: Some("Greninja".to_string()),
        item_user: Some(vec!["Greninja".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("grepaberry"), ItemDef {
        id: ID::new("grepaberry"),
        name: "Grepa Berry".to_string(),
        num: 173,
        gen: 3,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(90),
        natural_gift_type: Some("Flying".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("gripclaw"), ItemDef {
        id: ID::new("gripclaw"),
        name: "Grip Claw".to_string(),
        num: 286,
        gen: 4,
        category: ItemCategory::HeldItem,
        fling_power: Some(90),
        ..Default::default()
    });

    map.insert(ID::new("griseouscore"), ItemDef {
        id: ID::new("griseouscore"),
        name: "Griseous Core".to_string(),
        num: 1779,
        gen: 8,
        category: ItemCategory::HeldItem,
        item_user: Some(vec!["Giratina-Origin".to_string()]),
        forced_forme: Some("Giratina-Origin".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("griseousorb"), ItemDef {
        id: ID::new("griseousorb"),
        name: "Griseous Orb".to_string(),
        num: 112,
        gen: 4,
        category: ItemCategory::HeldItem,
        fling_power: Some(60),
        item_user: Some(vec!["Giratina".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("groundgem"), ItemDef {
        id: ID::new("groundgem"),
        name: "Ground Gem".to_string(),
        num: 555,
        gen: 5,
        category: ItemCategory::Gem,
        is_gem: true,
        is_consumable: true,
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("groundiumz"), ItemDef {
        id: ID::new("groundiumz"),
        name: "Groundium Z".to_string(),
        num: 784,
        gen: 7,
        category: ItemCategory::ZCrystal,
        is_nonstandard: Some("Past".to_string()),
        z_move_type: Some("Ground".to_string()),
        forced_forme: Some("Arceus-Ground".to_string()),
        on_plate: Some("Ground".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("groundmemory"), ItemDef {
        id: ID::new("groundmemory"),
        name: "Ground Memory".to_string(),
        num: 907,
        gen: 7,
        category: ItemCategory::Memory,
        is_nonstandard: Some("Past".to_string()),
        item_user: Some(vec!["Silvally-Ground".to_string()]),
        forced_forme: Some("Silvally-Ground".to_string()),
        on_memory: Some("Ground".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("gyaradosite"), ItemDef {
        id: ID::new("gyaradosite"),
        name: "Gyaradosite".to_string(),
        num: 676,
        gen: 6,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Past".to_string()),
        mega_stone: Some("Gyarados-Mega".to_string()),
        mega_evolves: Some("Gyarados".to_string()),
        item_user: Some(vec!["Gyarados".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("habanberry"), ItemDef {
        id: ID::new("habanberry"),
        name: "Haban Berry".to_string(),
        num: 197,
        gen: 4,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(80),
        natural_gift_type: Some("Dragon".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::SourceModifyDamage, ItemEffect::Compound { effects: vec![
                ItemEffect::ResistSuperEffective { resist_type: "Dragon".to_string(), multiplier: 0.5 },
                ItemEffect::ConsumeItem,
            ] }, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("hardstone"), ItemDef {
        id: ID::new("hardstone"),
        name: "Hard Stone".to_string(),
        num: 238,
        gen: 2,
        category: ItemCategory::HeldItem,
        fling_power: Some(100),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::BasePower, ItemEffect::BoostType { move_type: "Rock".to_string(), multiplier: 1.2 }, 15),
        ],
        ..Default::default()
    });

    map.insert(ID::new("hawluchanite"), ItemDef {
        id: ID::new("hawluchanite"),
        name: "Hawluchanite".to_string(),
        num: 2583,
        gen: 9,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Future".to_string()),
        mega_stone: Some("Hawlucha-Mega".to_string()),
        mega_evolves: Some("Hawlucha".to_string()),
        item_user: Some(vec!["Hawlucha".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("healball"), ItemDef {
        id: ID::new("healball"),
        name: "Heal Ball".to_string(),
        num: 14,
        gen: 4,
        category: ItemCategory::Pokeball,
        is_pokeball: true,
        ..Default::default()
    });

    map.insert(ID::new("hearthflamemask"), ItemDef {
        id: ID::new("hearthflamemask"),
        name: "Hearthflame Mask".to_string(),
        num: 2408,
        gen: 9,
        category: ItemCategory::HeldItem,
        fling_power: Some(60),
        item_user: Some(vec!["Ogerpon-Hearthflame".to_string()]),
        forced_forme: Some("Ogerpon-Hearthflame".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("heatranite"), ItemDef {
        id: ID::new("heatranite"),
        name: "Heatranite".to_string(),
        num: 2592,
        gen: 9,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Future".to_string()),
        mega_stone: Some("Heatran-Mega".to_string()),
        mega_evolves: Some("Heatran".to_string()),
        item_user: Some(vec!["Heatran".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("heatrock"), ItemDef {
        id: ID::new("heatrock"),
        name: "Heat Rock".to_string(),
        num: 284,
        gen: 4,
        category: ItemCategory::HeldItem,
        fling_power: Some(60),
        ..Default::default()
    });

    map.insert(ID::new("heavyball"), ItemDef {
        id: ID::new("heavyball"),
        name: "Heavy Ball".to_string(),
        num: 495,
        gen: 2,
        category: ItemCategory::Pokeball,
        is_pokeball: true,
        ..Default::default()
    });

    map.insert(ID::new("heavydutyboots"), ItemDef {
        id: ID::new("heavydutyboots"),
        name: "Heavy-Duty Boots".to_string(),
        num: 1120,
        gen: 8,
        category: ItemCategory::HeldItem,
        fling_power: Some(80),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::SwitchIn, ItemEffect::HazardImmunity, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("helixfossil"), ItemDef {
        id: ID::new("helixfossil"),
        name: "Helix Fossil".to_string(),
        num: 101,
        gen: 3,
        category: ItemCategory::Fossil,
        fling_power: Some(100),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("heracronite"), ItemDef {
        id: ID::new("heracronite"),
        name: "Heracronite".to_string(),
        num: 680,
        gen: 6,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Past".to_string()),
        mega_stone: Some("Heracross-Mega".to_string()),
        mega_evolves: Some("Heracross".to_string()),
        item_user: Some(vec!["Heracross".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("hondewberry"), ItemDef {
        id: ID::new("hondewberry"),
        name: "Hondew Berry".to_string(),
        num: 172,
        gen: 3,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(90),
        natural_gift_type: Some("Ground".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("houndoominite"), ItemDef {
        id: ID::new("houndoominite"),
        name: "Houndoominite".to_string(),
        num: 666,
        gen: 6,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Past".to_string()),
        mega_stone: Some("Houndoom-Mega".to_string()),
        mega_evolves: Some("Houndoom".to_string()),
        item_user: Some(vec!["Houndoom".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("iapapaberry"), ItemDef {
        id: ID::new("iapapaberry"),
        name: "Iapapa Berry".to_string(),
        num: 163,
        gen: 3,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(80),
        natural_gift_type: Some("Dark".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::Update, ItemEffect::Triggered {
            trigger: Trigger::OnHPBelowGluttony(0.25),
            effect: Box::new(ItemEffect::Compound { effects: vec![
                ItemEffect::HealOnThreshold { threshold: 0.25, heal_fraction: 0.333 },
                ItemEffect::ConsumeItem,
            ] }),
        }, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("iceberry"), ItemDef {
        id: ID::new("iceberry"),
        name: "Ice Berry".to_string(),
        num: 152,
        gen: 2,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        is_nonstandard: Some("Past".to_string()),
        natural_gift_power: Some(80),
        natural_gift_type: Some("Grass".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("icegem"), ItemDef {
        id: ID::new("icegem"),
        name: "Ice Gem".to_string(),
        num: 552,
        gen: 5,
        category: ItemCategory::Gem,
        is_gem: true,
        is_consumable: true,
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("icememory"), ItemDef {
        id: ID::new("icememory"),
        name: "Ice Memory".to_string(),
        num: 917,
        gen: 7,
        category: ItemCategory::Memory,
        is_nonstandard: Some("Past".to_string()),
        item_user: Some(vec!["Silvally-Ice".to_string()]),
        forced_forme: Some("Silvally-Ice".to_string()),
        on_memory: Some("Ice".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("icestone"), ItemDef {
        id: ID::new("icestone"),
        name: "Ice Stone".to_string(),
        num: 849,
        gen: 7,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        ..Default::default()
    });

    map.insert(ID::new("icicleplate"), ItemDef {
        id: ID::new("icicleplate"),
        name: "Icicle Plate".to_string(),
        num: 302,
        gen: 4,
        category: ItemCategory::Plate,
        forced_forme: Some("Arceus-Ice".to_string()),
        on_plate: Some("Ice".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::BasePower, ItemEffect::BoostType { move_type: "Ice".to_string(), multiplier: 1.2 }, 15),
        ],
        ..Default::default()
    });

    map.insert(ID::new("iciumz"), ItemDef {
        id: ID::new("iciumz"),
        name: "Icium Z".to_string(),
        num: 781,
        gen: 7,
        category: ItemCategory::ZCrystal,
        is_nonstandard: Some("Past".to_string()),
        z_move_type: Some("Ice".to_string()),
        forced_forme: Some("Arceus-Ice".to_string()),
        on_plate: Some("Ice".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("icyrock"), ItemDef {
        id: ID::new("icyrock"),
        name: "Icy Rock".to_string(),
        num: 282,
        gen: 4,
        category: ItemCategory::HeldItem,
        fling_power: Some(40),
        ..Default::default()
    });

    map.insert(ID::new("inciniumz"), ItemDef {
        id: ID::new("inciniumz"),
        name: "Incinium Z".to_string(),
        num: 799,
        gen: 7,
        category: ItemCategory::ZCrystal,
        is_nonstandard: Some("Past".to_string()),
        item_user: Some(vec!["Incineroar".to_string()]),
        z_move: Some("Malicious Moonsault".to_string()),
        z_move_from: Some("Darkest Lariat".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("insectplate"), ItemDef {
        id: ID::new("insectplate"),
        name: "Insect Plate".to_string(),
        num: 308,
        gen: 4,
        category: ItemCategory::Plate,
        forced_forme: Some("Arceus-Bug".to_string()),
        on_plate: Some("Bug".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::BasePower, ItemEffect::BoostType { move_type: "Bug".to_string(), multiplier: 1.2 }, 15),
        ],
        ..Default::default()
    });

    map.insert(ID::new("ironball"), ItemDef {
        id: ID::new("ironball"),
        name: "Iron Ball".to_string(),
        num: 278,
        gen: 4,
        category: ItemCategory::HeldItem,
        fling_power: Some(130),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::SwitchIn, ItemEffect::GroundHolder, 0),
            ItemEffectEntry::with_priority(EventType::ModifySpe, ItemEffect::ModifyStat { stat: Stat::Spe, multiplier: 0.5 }, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("ironplate"), ItemDef {
        id: ID::new("ironplate"),
        name: "Iron Plate".to_string(),
        num: 313,
        gen: 4,
        category: ItemCategory::Plate,
        forced_forme: Some("Arceus-Steel".to_string()),
        on_plate: Some("Steel".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::BasePower, ItemEffect::BoostType { move_type: "Steel".to_string(), multiplier: 1.2 }, 15),
        ],
        ..Default::default()
    });

    map.insert(ID::new("jabocaberry"), ItemDef {
        id: ID::new("jabocaberry"),
        name: "Jaboca Berry".to_string(),
        num: 211,
        gen: 4,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(100),
        natural_gift_type: Some("Dragon".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("jawfossil"), ItemDef {
        id: ID::new("jawfossil"),
        name: "Jaw Fossil".to_string(),
        num: 710,
        gen: 6,
        category: ItemCategory::Fossil,
        fling_power: Some(100),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("kangaskhanite"), ItemDef {
        id: ID::new("kangaskhanite"),
        name: "Kangaskhanite".to_string(),
        num: 675,
        gen: 6,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Past".to_string()),
        mega_stone: Some("Kangaskhan-Mega".to_string()),
        mega_evolves: Some("Kangaskhan".to_string()),
        item_user: Some(vec!["Kangaskhan".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("kasibberry"), ItemDef {
        id: ID::new("kasibberry"),
        name: "Kasib Berry".to_string(),
        num: 196,
        gen: 4,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(80),
        natural_gift_type: Some("Ghost".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::SourceModifyDamage, ItemEffect::Compound { effects: vec![
                ItemEffect::ResistSuperEffective { resist_type: "Ghost".to_string(), multiplier: 0.5 },
                ItemEffect::ConsumeItem,
            ] }, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("kebiaberry"), ItemDef {
        id: ID::new("kebiaberry"),
        name: "Kebia Berry".to_string(),
        num: 190,
        gen: 4,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(80),
        natural_gift_type: Some("Poison".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::SourceModifyDamage, ItemEffect::Compound { effects: vec![
                ItemEffect::ResistSuperEffective { resist_type: "Poison".to_string(), multiplier: 0.5 },
                ItemEffect::ConsumeItem,
            ] }, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("keeberry"), ItemDef {
        id: ID::new("keeberry"),
        name: "Kee Berry".to_string(),
        num: 687,
        gen: 6,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(100),
        natural_gift_type: Some("Fairy".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("kelpsyberry"), ItemDef {
        id: ID::new("kelpsyberry"),
        name: "Kelpsy Berry".to_string(),
        num: 170,
        gen: 3,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(90),
        natural_gift_type: Some("Fighting".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("kingsrock"), ItemDef {
        id: ID::new("kingsrock"),
        name: "King's Rock".to_string(),
        num: 221,
        gen: 2,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        ..Default::default()
    });

    map.insert(ID::new("kommoniumz"), ItemDef {
        id: ID::new("kommoniumz"),
        name: "Kommonium Z".to_string(),
        num: 926,
        gen: 7,
        category: ItemCategory::ZCrystal,
        is_nonstandard: Some("Past".to_string()),
        item_user: Some(vec!["Kommo-o".to_string(), "Kommo-o-Totem".to_string()]),
        z_move: Some("Clangorous Soulblaze".to_string()),
        z_move_from: Some("Clanging Scales".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("laggingtail"), ItemDef {
        id: ID::new("laggingtail"),
        name: "Lagging Tail".to_string(),
        num: 279,
        gen: 4,
        category: ItemCategory::HeldItem,
        fling_power: Some(10),
        ..Default::default()
    });

    map.insert(ID::new("lansatberry"), ItemDef {
        id: ID::new("lansatberry"),
        name: "Lansat Berry".to_string(),
        num: 206,
        gen: 3,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(100),
        natural_gift_type: Some("Flying".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("latiasite"), ItemDef {
        id: ID::new("latiasite"),
        name: "Latiasite".to_string(),
        num: 684,
        gen: 6,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Past".to_string()),
        mega_stone: Some("Latias-Mega".to_string()),
        mega_evolves: Some("Latias".to_string()),
        item_user: Some(vec!["Latias".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("latiosite"), ItemDef {
        id: ID::new("latiosite"),
        name: "Latiosite".to_string(),
        num: 685,
        gen: 6,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Past".to_string()),
        mega_stone: Some("Latios-Mega".to_string()),
        mega_evolves: Some("Latios".to_string()),
        item_user: Some(vec!["Latios".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("laxincense"), ItemDef {
        id: ID::new("laxincense"),
        name: "Lax Incense".to_string(),
        num: 255,
        gen: 3,
        category: ItemCategory::HeldItem,
        fling_power: Some(10),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("leafstone"), ItemDef {
        id: ID::new("leafstone"),
        name: "Leaf Stone".to_string(),
        num: 85,
        gen: 1,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        ..Default::default()
    });

    map.insert(ID::new("leek"), ItemDef {
        id: ID::new("leek"),
        name: "Leek".to_string(),
        num: 259,
        gen: 8,
        category: ItemCategory::HeldItem,
        fling_power: Some(60),
        is_nonstandard: Some("Past".to_string()),
        item_user: Some(vec!["Farfetch’d".to_string(), "Farfetch’d-Galar".to_string(), "Sirfetch’d".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("leftovers"), ItemDef {
        id: ID::new("leftovers"),
        name: "Leftovers".to_string(),
        num: 234,
        gen: 2,
        category: ItemCategory::HeldItem,
        fling_power: Some(10),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::Residual, ItemEffect::ResidualHeal { fraction: 0.0625 }, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("leppaberry"), ItemDef {
        id: ID::new("leppaberry"),
        name: "Leppa Berry".to_string(),
        num: 154,
        gen: 3,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(80),
        natural_gift_type: Some("Fighting".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("levelball"), ItemDef {
        id: ID::new("levelball"),
        name: "Level Ball".to_string(),
        num: 493,
        gen: 2,
        category: ItemCategory::Pokeball,
        is_pokeball: true,
        ..Default::default()
    });

    map.insert(ID::new("liechiberry"), ItemDef {
        id: ID::new("liechiberry"),
        name: "Liechi Berry".to_string(),
        num: 201,
        gen: 3,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(100),
        natural_gift_type: Some("Grass".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("lifeorb"), ItemDef {
        id: ID::new("lifeorb"),
        name: "Life Orb".to_string(),
        num: 270,
        gen: 4,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::ModifyDamage, ItemEffect::BoostDamage { multiplier: 1.3 }, 0),
            ItemEffectEntry::with_priority(EventType::AfterHit, ItemEffect::RecoilOnAttack { fraction: 0.1 }, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("lightball"), ItemDef {
        id: ID::new("lightball"),
        name: "Light Ball".to_string(),
        num: 236,
        gen: 2,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        item_user: Some(vec!["Pikachu".to_string(), "Pikachu-Cosplay".to_string(), "Pikachu-Rock-Star".to_string(), "Pikachu-Belle".to_string(), "Pikachu-Pop-Star".to_string(), "Pikachu-PhD".to_string(), "Pikachu-Libre".to_string(), "Pikachu-Original".to_string(), "Pikachu-Hoenn".to_string(), "Pikachu-Sinnoh".to_string(), "Pikachu-Unova".to_string(), "Pikachu-Kalos".to_string(), "Pikachu-Alola".to_string(), "Pikachu-Partner".to_string(), "Pikachu-Starter".to_string(), "Pikachu-World".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("lightclay"), ItemDef {
        id: ID::new("lightclay"),
        name: "Light Clay".to_string(),
        num: 269,
        gen: 4,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        ..Default::default()
    });

    map.insert(ID::new("loadeddice"), ItemDef {
        id: ID::new("loadeddice"),
        name: "Loaded Dice".to_string(),
        num: 1886,
        gen: 9,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::ModifyMove, ItemEffect::MaxMultiHit, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("lopunnite"), ItemDef {
        id: ID::new("lopunnite"),
        name: "Lopunnite".to_string(),
        num: 768,
        gen: 6,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Past".to_string()),
        mega_stone: Some("Lopunny-Mega".to_string()),
        mega_evolves: Some("Lopunny".to_string()),
        item_user: Some(vec!["Lopunny".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("loveball"), ItemDef {
        id: ID::new("loveball"),
        name: "Love Ball".to_string(),
        num: 496,
        gen: 2,
        category: ItemCategory::Pokeball,
        is_pokeball: true,
        ..Default::default()
    });

    map.insert(ID::new("lovesweet"), ItemDef {
        id: ID::new("lovesweet"),
        name: "Love Sweet".to_string(),
        num: 1110,
        gen: 8,
        category: ItemCategory::HeldItem,
        fling_power: Some(10),
        ..Default::default()
    });

    map.insert(ID::new("lucarionite"), ItemDef {
        id: ID::new("lucarionite"),
        name: "Lucarionite".to_string(),
        num: 673,
        gen: 6,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Past".to_string()),
        mega_stone: Some("Lucario-Mega".to_string()),
        mega_evolves: Some("Lucario".to_string()),
        item_user: Some(vec!["Lucario".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("lucarionitez"), ItemDef {
        id: ID::new("lucarionitez"),
        name: "Lucarionite Z".to_string(),
        num: 2591,
        gen: 9,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Future".to_string()),
        mega_stone: Some("Lucario-Mega-Z".to_string()),
        mega_evolves: Some("Lucario".to_string()),
        item_user: Some(vec!["Lucario".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("luckypunch"), ItemDef {
        id: ID::new("luckypunch"),
        name: "Lucky Punch".to_string(),
        num: 256,
        gen: 2,
        category: ItemCategory::HeldItem,
        fling_power: Some(40),
        is_nonstandard: Some("Past".to_string()),
        item_user: Some(vec!["Chansey".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("lumberry"), ItemDef {
        id: ID::new("lumberry"),
        name: "Lum Berry".to_string(),
        num: 157,
        gen: 3,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(80),
        natural_gift_type: Some("Flying".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::Update, ItemEffect::Compound { effects: vec![ItemEffect::CureAllStatus, ItemEffect::ConsumeItem] }, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("luminousmoss"), ItemDef {
        id: ID::new("luminousmoss"),
        name: "Luminous Moss".to_string(),
        num: 648,
        gen: 6,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        boosts: Some(boosts(&[("spd", 1)])),
        ..Default::default()
    });

    map.insert(ID::new("lunaliumz"), ItemDef {
        id: ID::new("lunaliumz"),
        name: "Lunalium Z".to_string(),
        num: 922,
        gen: 7,
        category: ItemCategory::ZCrystal,
        is_nonstandard: Some("Past".to_string()),
        item_user: Some(vec!["Lunala".to_string(), "Necrozma-Dawn-Wings".to_string()]),
        z_move: Some("Menacing Moonraze Maelstrom".to_string()),
        z_move_from: Some("Moongeist Beam".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("lureball"), ItemDef {
        id: ID::new("lureball"),
        name: "Lure Ball".to_string(),
        num: 494,
        gen: 2,
        category: ItemCategory::Pokeball,
        is_pokeball: true,
        ..Default::default()
    });

    map.insert(ID::new("lustrousglobe"), ItemDef {
        id: ID::new("lustrousglobe"),
        name: "Lustrous Globe".to_string(),
        num: 1778,
        gen: 8,
        category: ItemCategory::HeldItem,
        item_user: Some(vec!["Palkia-Origin".to_string()]),
        forced_forme: Some("Palkia-Origin".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("lustrousorb"), ItemDef {
        id: ID::new("lustrousorb"),
        name: "Lustrous Orb".to_string(),
        num: 136,
        gen: 4,
        category: ItemCategory::HeldItem,
        fling_power: Some(60),
        item_user: Some(vec!["Palkia".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("luxuryball"), ItemDef {
        id: ID::new("luxuryball"),
        name: "Luxury Ball".to_string(),
        num: 11,
        gen: 3,
        category: ItemCategory::Pokeball,
        is_pokeball: true,
        ..Default::default()
    });

    map.insert(ID::new("lycaniumz"), ItemDef {
        id: ID::new("lycaniumz"),
        name: "Lycanium Z".to_string(),
        num: 925,
        gen: 7,
        category: ItemCategory::ZCrystal,
        is_nonstandard: Some("Past".to_string()),
        item_user: Some(vec!["Lycanroc".to_string(), "Lycanroc-Midnight".to_string(), "Lycanroc-Dusk".to_string()]),
        z_move: Some("Splintered Stormshards".to_string()),
        z_move_from: Some("Stone Edge".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("machobrace"), ItemDef {
        id: ID::new("machobrace"),
        name: "Macho Brace".to_string(),
        num: 215,
        gen: 3,
        category: ItemCategory::HeldItem,
        fling_power: Some(60),
        is_nonstandard: Some("Past".to_string()),
        ignore_klutz: true,
        ..Default::default()
    });

    map.insert(ID::new("magearnite"), ItemDef {
        id: ID::new("magearnite"),
        name: "Magearnite".to_string(),
        num: 2597,
        gen: 9,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Future".to_string()),
        mega_stone: Some("Magearna-Mega,Magearna-Original-Mega".to_string()),
        mega_evolves: Some("Magearna,Magearna-Original".to_string()),
        item_user: Some(vec!["Magearna".to_string(), "Magearna-Original".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("magmarizer"), ItemDef {
        id: ID::new("magmarizer"),
        name: "Magmarizer".to_string(),
        num: 323,
        gen: 4,
        category: ItemCategory::HeldItem,
        fling_power: Some(80),
        ..Default::default()
    });

    map.insert(ID::new("magnet"), ItemDef {
        id: ID::new("magnet"),
        name: "Magnet".to_string(),
        num: 242,
        gen: 2,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::BasePower, ItemEffect::BoostType { move_type: "Electric".to_string(), multiplier: 1.2 }, 15),
        ],
        ..Default::default()
    });

    map.insert(ID::new("magoberry"), ItemDef {
        id: ID::new("magoberry"),
        name: "Mago Berry".to_string(),
        num: 161,
        gen: 3,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(80),
        natural_gift_type: Some("Ghost".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::Update, ItemEffect::Triggered {
            trigger: Trigger::OnHPBelowGluttony(0.25),
            effect: Box::new(ItemEffect::Compound { effects: vec![
                ItemEffect::HealOnThreshold { threshold: 0.25, heal_fraction: 0.333 },
                ItemEffect::ConsumeItem,
            ] }),
        }, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("magostberry"), ItemDef {
        id: ID::new("magostberry"),
        name: "Magost Berry".to_string(),
        num: 176,
        gen: 3,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        is_nonstandard: Some("Past".to_string()),
        natural_gift_power: Some(90),
        natural_gift_type: Some("Rock".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("mail"), ItemDef {
        id: ID::new("mail"),
        name: "Mail".to_string(),
        num: 137,
        gen: 2,
        category: ItemCategory::HeldItem,
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("malamarite"), ItemDef {
        id: ID::new("malamarite"),
        name: "Malamarite".to_string(),
        num: 2580,
        gen: 9,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Future".to_string()),
        mega_stone: Some("Malamar-Mega".to_string()),
        mega_evolves: Some("Malamar".to_string()),
        item_user: Some(vec!["Malamar".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("maliciousarmor"), ItemDef {
        id: ID::new("maliciousarmor"),
        name: "Malicious Armor".to_string(),
        num: 1861,
        gen: 9,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        ..Default::default()
    });

    map.insert(ID::new("manectite"), ItemDef {
        id: ID::new("manectite"),
        name: "Manectite".to_string(),
        num: 682,
        gen: 6,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Past".to_string()),
        mega_stone: Some("Manectric-Mega".to_string()),
        mega_evolves: Some("Manectric".to_string()),
        item_user: Some(vec!["Manectric".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("marangaberry"), ItemDef {
        id: ID::new("marangaberry"),
        name: "Maranga Berry".to_string(),
        num: 688,
        gen: 6,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(100),
        natural_gift_type: Some("Dark".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("marshadiumz"), ItemDef {
        id: ID::new("marshadiumz"),
        name: "Marshadium Z".to_string(),
        num: 802,
        gen: 7,
        category: ItemCategory::ZCrystal,
        is_nonstandard: Some("Past".to_string()),
        item_user: Some(vec!["Marshadow".to_string()]),
        z_move: Some("Soul-Stealing 7-Star Strike".to_string()),
        z_move_from: Some("Spectral Thief".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("masterball"), ItemDef {
        id: ID::new("masterball"),
        name: "Master Ball".to_string(),
        num: 1,
        gen: 1,
        category: ItemCategory::Pokeball,
        is_pokeball: true,
        ..Default::default()
    });

    map.insert(ID::new("masterpieceteacup"), ItemDef {
        id: ID::new("masterpieceteacup"),
        name: "Masterpiece Teacup".to_string(),
        num: 2404,
        gen: 9,
        category: ItemCategory::HeldItem,
        fling_power: Some(80),
        ..Default::default()
    });

    map.insert(ID::new("mawilite"), ItemDef {
        id: ID::new("mawilite"),
        name: "Mawilite".to_string(),
        num: 681,
        gen: 6,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Past".to_string()),
        mega_stone: Some("Mawile-Mega".to_string()),
        mega_evolves: Some("Mawile".to_string()),
        item_user: Some(vec!["Mawile".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("meadowplate"), ItemDef {
        id: ID::new("meadowplate"),
        name: "Meadow Plate".to_string(),
        num: 301,
        gen: 4,
        category: ItemCategory::Plate,
        forced_forme: Some("Arceus-Grass".to_string()),
        on_plate: Some("Grass".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::BasePower, ItemEffect::BoostType { move_type: "Grass".to_string(), multiplier: 1.2 }, 15),
        ],
        ..Default::default()
    });

    map.insert(ID::new("medichamite"), ItemDef {
        id: ID::new("medichamite"),
        name: "Medichamite".to_string(),
        num: 665,
        gen: 6,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Past".to_string()),
        mega_stone: Some("Medicham-Mega".to_string()),
        mega_evolves: Some("Medicham".to_string()),
        item_user: Some(vec!["Medicham".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("meganiumite"), ItemDef {
        id: ID::new("meganiumite"),
        name: "Meganiumite".to_string(),
        num: 2563,
        gen: 9,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Future".to_string()),
        mega_stone: Some("Meganium-Mega".to_string()),
        mega_evolves: Some("Meganium".to_string()),
        item_user: Some(vec!["Meganium".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("mentalherb"), ItemDef {
        id: ID::new("mentalherb"),
        name: "Mental Herb".to_string(),
        num: 219,
        gen: 3,
        category: ItemCategory::HeldItem,
        fling_power: Some(10),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::Update, ItemEffect::Compound { effects: vec![ItemEffect::MentalHerb, ItemEffect::ConsumeItem] }, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("meowsticite"), ItemDef {
        id: ID::new("meowsticite"),
        name: "Meowsticite".to_string(),
        num: 2594,
        gen: 9,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Future".to_string()),
        mega_stone: Some("Meowstic-M-Mega,Meowstic-F-Mega".to_string()),
        mega_evolves: Some("Meowstic,Meowstic-F".to_string()),
        item_user: Some(vec!["Meowstic".to_string(), "Meowstic-F".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("metagrossite"), ItemDef {
        id: ID::new("metagrossite"),
        name: "Metagrossite".to_string(),
        num: 758,
        gen: 6,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Past".to_string()),
        mega_stone: Some("Metagross-Mega".to_string()),
        mega_evolves: Some("Metagross".to_string()),
        item_user: Some(vec!["Metagross".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("metalalloy"), ItemDef {
        id: ID::new("metalalloy"),
        name: "Metal Alloy".to_string(),
        num: 2482,
        gen: 9,
        category: ItemCategory::HeldItem,
        ..Default::default()
    });

    map.insert(ID::new("metalcoat"), ItemDef {
        id: ID::new("metalcoat"),
        name: "Metal Coat".to_string(),
        num: 233,
        gen: 2,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::BasePower, ItemEffect::BoostType { move_type: "Steel".to_string(), multiplier: 1.2 }, 15),
        ],
        ..Default::default()
    });

    map.insert(ID::new("metalpowder"), ItemDef {
        id: ID::new("metalpowder"),
        name: "Metal Powder".to_string(),
        num: 257,
        gen: 2,
        category: ItemCategory::HeldItem,
        fling_power: Some(10),
        is_nonstandard: Some("Past".to_string()),
        item_user: Some(vec!["Ditto".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("metronome"), ItemDef {
        id: ID::new("metronome"),
        name: "Metronome".to_string(),
        num: 277,
        gen: 4,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        has_custom_handler: true,
        ..Default::default()
    });

    map.insert(ID::new("mewniumz"), ItemDef {
        id: ID::new("mewniumz"),
        name: "Mewnium Z".to_string(),
        num: 806,
        gen: 7,
        category: ItemCategory::ZCrystal,
        is_nonstandard: Some("Past".to_string()),
        item_user: Some(vec!["Mew".to_string()]),
        z_move: Some("Genesis Supernova".to_string()),
        z_move_from: Some("Psychic".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("mewtwonitex"), ItemDef {
        id: ID::new("mewtwonitex"),
        name: "Mewtwonite X".to_string(),
        num: 662,
        gen: 6,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Past".to_string()),
        mega_stone: Some("Mewtwo-Mega-X".to_string()),
        mega_evolves: Some("Mewtwo".to_string()),
        item_user: Some(vec!["Mewtwo".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("mewtwonitey"), ItemDef {
        id: ID::new("mewtwonitey"),
        name: "Mewtwonite Y".to_string(),
        num: 663,
        gen: 6,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Past".to_string()),
        mega_stone: Some("Mewtwo-Mega-Y".to_string()),
        mega_evolves: Some("Mewtwo".to_string()),
        item_user: Some(vec!["Mewtwo".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("micleberry"), ItemDef {
        id: ID::new("micleberry"),
        name: "Micle Berry".to_string(),
        num: 209,
        gen: 4,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(100),
        natural_gift_type: Some("Rock".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("mimikiumz"), ItemDef {
        id: ID::new("mimikiumz"),
        name: "Mimikium Z".to_string(),
        num: 924,
        gen: 7,
        category: ItemCategory::ZCrystal,
        is_nonstandard: Some("Past".to_string()),
        item_user: Some(vec!["Mimikyu".to_string(), "Mimikyu-Busted".to_string(), "Mimikyu-Totem".to_string(), "Mimikyu-Busted-Totem".to_string()]),
        z_move: Some("Let's Snuggle Forever".to_string()),
        z_move_from: Some("Play Rough".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("mindplate"), ItemDef {
        id: ID::new("mindplate"),
        name: "Mind Plate".to_string(),
        num: 307,
        gen: 4,
        category: ItemCategory::Plate,
        forced_forme: Some("Arceus-Psychic".to_string()),
        on_plate: Some("Psychic".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::BasePower, ItemEffect::BoostType { move_type: "Psychic".to_string(), multiplier: 1.2 }, 15),
        ],
        ..Default::default()
    });

    map.insert(ID::new("mintberry"), ItemDef {
        id: ID::new("mintberry"),
        name: "Mint Berry".to_string(),
        num: 150,
        gen: 2,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        is_nonstandard: Some("Past".to_string()),
        natural_gift_power: Some(80),
        natural_gift_type: Some("Water".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("miracleberry"), ItemDef {
        id: ID::new("miracleberry"),
        name: "Miracle Berry".to_string(),
        num: 157,
        gen: 2,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        is_nonstandard: Some("Past".to_string()),
        natural_gift_power: Some(80),
        natural_gift_type: Some("Flying".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("miracleseed"), ItemDef {
        id: ID::new("miracleseed"),
        name: "Miracle Seed".to_string(),
        num: 239,
        gen: 2,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::BasePower, ItemEffect::BoostType { move_type: "Grass".to_string(), multiplier: 1.2 }, 15),
        ],
        ..Default::default()
    });

    map.insert(ID::new("mirrorherb"), ItemDef {
        id: ID::new("mirrorherb"),
        name: "Mirror Herb".to_string(),
        num: 1883,
        gen: 9,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        ..Default::default()
    });

    map.insert(ID::new("mistyseed"), ItemDef {
        id: ID::new("mistyseed"),
        name: "Misty Seed".to_string(),
        num: 883,
        gen: 7,
        category: ItemCategory::HeldItem,
        fling_power: Some(10),
        boosts: Some(boosts(&[("spd", 1)])),
        ..Default::default()
    });

    map.insert(ID::new("moonball"), ItemDef {
        id: ID::new("moonball"),
        name: "Moon Ball".to_string(),
        num: 498,
        gen: 2,
        category: ItemCategory::Pokeball,
        is_pokeball: true,
        ..Default::default()
    });

    map.insert(ID::new("moonstone"), ItemDef {
        id: ID::new("moonstone"),
        name: "Moon Stone".to_string(),
        num: 81,
        gen: 1,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        ..Default::default()
    });

    map.insert(ID::new("muscleband"), ItemDef {
        id: ID::new("muscleband"),
        name: "Muscle Band".to_string(),
        num: 266,
        gen: 4,
        category: ItemCategory::HeldItem,
        fling_power: Some(10),
        ..Default::default()
    });

    map.insert(ID::new("mysteryberry"), ItemDef {
        id: ID::new("mysteryberry"),
        name: "Mystery Berry".to_string(),
        num: 154,
        gen: 2,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        is_nonstandard: Some("Past".to_string()),
        natural_gift_power: Some(80),
        natural_gift_type: Some("Fighting".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("mysticwater"), ItemDef {
        id: ID::new("mysticwater"),
        name: "Mystic Water".to_string(),
        num: 243,
        gen: 2,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::BasePower, ItemEffect::BoostType { move_type: "Water".to_string(), multiplier: 1.2 }, 15),
        ],
        ..Default::default()
    });

    map.insert(ID::new("nanabberry"), ItemDef {
        id: ID::new("nanabberry"),
        name: "Nanab Berry".to_string(),
        num: 166,
        gen: 3,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        is_nonstandard: Some("Past".to_string()),
        natural_gift_power: Some(90),
        natural_gift_type: Some("Water".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("nestball"), ItemDef {
        id: ID::new("nestball"),
        name: "Nest Ball".to_string(),
        num: 8,
        gen: 3,
        category: ItemCategory::Pokeball,
        is_pokeball: true,
        ..Default::default()
    });

    map.insert(ID::new("netball"), ItemDef {
        id: ID::new("netball"),
        name: "Net Ball".to_string(),
        num: 6,
        gen: 3,
        category: ItemCategory::Pokeball,
        is_pokeball: true,
        ..Default::default()
    });

    map.insert(ID::new("nevermeltice"), ItemDef {
        id: ID::new("nevermeltice"),
        name: "Never-Melt Ice".to_string(),
        num: 246,
        gen: 2,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::BasePower, ItemEffect::BoostType { move_type: "Ice".to_string(), multiplier: 1.2 }, 15),
        ],
        ..Default::default()
    });

    map.insert(ID::new("nomelberry"), ItemDef {
        id: ID::new("nomelberry"),
        name: "Nomel Berry".to_string(),
        num: 178,
        gen: 3,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        is_nonstandard: Some("Past".to_string()),
        natural_gift_power: Some(90),
        natural_gift_type: Some("Dragon".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("normalgem"), ItemDef {
        id: ID::new("normalgem"),
        name: "Normal Gem".to_string(),
        num: 564,
        gen: 5,
        category: ItemCategory::Gem,
        is_gem: true,
        is_consumable: true,
        ..Default::default()
    });

    map.insert(ID::new("normaliumz"), ItemDef {
        id: ID::new("normaliumz"),
        name: "Normalium Z".to_string(),
        num: 776,
        gen: 7,
        category: ItemCategory::ZCrystal,
        is_nonstandard: Some("Past".to_string()),
        z_move_type: Some("Normal".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("occaberry"), ItemDef {
        id: ID::new("occaberry"),
        name: "Occa Berry".to_string(),
        num: 184,
        gen: 4,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(80),
        natural_gift_type: Some("Fire".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::SourceModifyDamage, ItemEffect::Compound { effects: vec![
                ItemEffect::ResistSuperEffective { resist_type: "Fire".to_string(), multiplier: 0.5 },
                ItemEffect::ConsumeItem,
            ] }, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("oddincense"), ItemDef {
        id: ID::new("oddincense"),
        name: "Odd Incense".to_string(),
        num: 314,
        gen: 4,
        category: ItemCategory::HeldItem,
        fling_power: Some(10),
        is_nonstandard: Some("Past".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::BasePower, ItemEffect::BoostType { move_type: "Psychic".to_string(), multiplier: 1.2 }, 15),
        ],
        ..Default::default()
    });

    map.insert(ID::new("oldamber"), ItemDef {
        id: ID::new("oldamber"),
        name: "Old Amber".to_string(),
        num: 103,
        gen: 3,
        category: ItemCategory::HeldItem,
        fling_power: Some(100),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("oranberry"), ItemDef {
        id: ID::new("oranberry"),
        name: "Oran Berry".to_string(),
        num: 155,
        gen: 3,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(80),
        natural_gift_type: Some("Poison".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("ovalstone"), ItemDef {
        id: ID::new("ovalstone"),
        name: "Oval Stone".to_string(),
        num: 110,
        gen: 4,
        category: ItemCategory::HeldItem,
        fling_power: Some(80),
        ..Default::default()
    });

    map.insert(ID::new("pamtreberry"), ItemDef {
        id: ID::new("pamtreberry"),
        name: "Pamtre Berry".to_string(),
        num: 180,
        gen: 3,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        is_nonstandard: Some("Past".to_string()),
        natural_gift_power: Some(90),
        natural_gift_type: Some("Steel".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("parkball"), ItemDef {
        id: ID::new("parkball"),
        name: "Park Ball".to_string(),
        num: 500,
        gen: 4,
        category: ItemCategory::Pokeball,
        is_pokeball: true,
        is_nonstandard: Some("Unobtainable".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("passhoberry"), ItemDef {
        id: ID::new("passhoberry"),
        name: "Passho Berry".to_string(),
        num: 185,
        gen: 4,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(80),
        natural_gift_type: Some("Water".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::SourceModifyDamage, ItemEffect::Compound { effects: vec![
                ItemEffect::ResistSuperEffective { resist_type: "Water".to_string(), multiplier: 0.5 },
                ItemEffect::ConsumeItem,
            ] }, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("payapaberry"), ItemDef {
        id: ID::new("payapaberry"),
        name: "Payapa Berry".to_string(),
        num: 193,
        gen: 4,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(80),
        natural_gift_type: Some("Psychic".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::SourceModifyDamage, ItemEffect::Compound { effects: vec![
                ItemEffect::ResistSuperEffective { resist_type: "Psychic".to_string(), multiplier: 0.5 },
                ItemEffect::ConsumeItem,
            ] }, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("pechaberry"), ItemDef {
        id: ID::new("pechaberry"),
        name: "Pecha Berry".to_string(),
        num: 151,
        gen: 3,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(80),
        natural_gift_type: Some("Electric".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::Update, ItemEffect::Compound { effects: vec![
                ItemEffect::CureStatus { status: "psn".to_string() },
                ItemEffect::ConsumeItem,
            ] }, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("persimberry"), ItemDef {
        id: ID::new("persimberry"),
        name: "Persim Berry".to_string(),
        num: 156,
        gen: 3,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(80),
        natural_gift_type: Some("Ground".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("petayaberry"), ItemDef {
        id: ID::new("petayaberry"),
        name: "Petaya Berry".to_string(),
        num: 204,
        gen: 3,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(100),
        natural_gift_type: Some("Poison".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("pidgeotite"), ItemDef {
        id: ID::new("pidgeotite"),
        name: "Pidgeotite".to_string(),
        num: 762,
        gen: 6,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Past".to_string()),
        mega_stone: Some("Pidgeot-Mega".to_string()),
        mega_evolves: Some("Pidgeot".to_string()),
        item_user: Some(vec!["Pidgeot".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("pikaniumz"), ItemDef {
        id: ID::new("pikaniumz"),
        name: "Pikanium Z".to_string(),
        num: 794,
        gen: 7,
        category: ItemCategory::ZCrystal,
        is_nonstandard: Some("Past".to_string()),
        item_user: Some(vec!["Pikachu".to_string()]),
        z_move: Some("Catastropika".to_string()),
        z_move_from: Some("Volt Tackle".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("pikashuniumz"), ItemDef {
        id: ID::new("pikashuniumz"),
        name: "Pikashunium Z".to_string(),
        num: 836,
        gen: 7,
        category: ItemCategory::ZCrystal,
        is_nonstandard: Some("Past".to_string()),
        item_user: Some(vec!["Pikachu-Original".to_string(), "Pikachu-Hoenn".to_string(), "Pikachu-Sinnoh".to_string(), "Pikachu-Unova".to_string(), "Pikachu-Kalos".to_string(), "Pikachu-Alola".to_string(), "Pikachu-Partner".to_string()]),
        z_move: Some("10,000,000 Volt Thunderbolt".to_string()),
        z_move_from: Some("Thunderbolt".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("pinapberry"), ItemDef {
        id: ID::new("pinapberry"),
        name: "Pinap Berry".to_string(),
        num: 168,
        gen: 3,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        is_nonstandard: Some("Past".to_string()),
        natural_gift_power: Some(90),
        natural_gift_type: Some("Grass".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("pinkbow"), ItemDef {
        id: ID::new("pinkbow"),
        name: "Pink Bow".to_string(),
        num: 251,
        gen: 2,
        category: ItemCategory::HeldItem,
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("pinsirite"), ItemDef {
        id: ID::new("pinsirite"),
        name: "Pinsirite".to_string(),
        num: 671,
        gen: 6,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Past".to_string()),
        mega_stone: Some("Pinsir-Mega".to_string()),
        mega_evolves: Some("Pinsir".to_string()),
        item_user: Some(vec!["Pinsir".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("pixieplate"), ItemDef {
        id: ID::new("pixieplate"),
        name: "Pixie Plate".to_string(),
        num: 644,
        gen: 6,
        category: ItemCategory::Plate,
        forced_forme: Some("Arceus-Fairy".to_string()),
        on_plate: Some("Fairy".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::BasePower, ItemEffect::BoostType { move_type: "Fairy".to_string(), multiplier: 1.2 }, 15),
        ],
        ..Default::default()
    });

    map.insert(ID::new("plumefossil"), ItemDef {
        id: ID::new("plumefossil"),
        name: "Plume Fossil".to_string(),
        num: 573,
        gen: 5,
        category: ItemCategory::Fossil,
        fling_power: Some(100),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("poisonbarb"), ItemDef {
        id: ID::new("poisonbarb"),
        name: "Poison Barb".to_string(),
        num: 245,
        gen: 2,
        category: ItemCategory::HeldItem,
        fling_power: Some(70),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::BasePower, ItemEffect::BoostType { move_type: "Poison".to_string(), multiplier: 1.2 }, 15),
        ],
        ..Default::default()
    });

    map.insert(ID::new("poisongem"), ItemDef {
        id: ID::new("poisongem"),
        name: "Poison Gem".to_string(),
        num: 554,
        gen: 5,
        category: ItemCategory::Gem,
        is_gem: true,
        is_consumable: true,
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("poisoniumz"), ItemDef {
        id: ID::new("poisoniumz"),
        name: "Poisonium Z".to_string(),
        num: 783,
        gen: 7,
        category: ItemCategory::ZCrystal,
        is_nonstandard: Some("Past".to_string()),
        z_move_type: Some("Poison".to_string()),
        forced_forme: Some("Arceus-Poison".to_string()),
        on_plate: Some("Poison".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("poisonmemory"), ItemDef {
        id: ID::new("poisonmemory"),
        name: "Poison Memory".to_string(),
        num: 906,
        gen: 7,
        category: ItemCategory::Memory,
        is_nonstandard: Some("Past".to_string()),
        item_user: Some(vec!["Silvally-Poison".to_string()]),
        forced_forme: Some("Silvally-Poison".to_string()),
        on_memory: Some("Poison".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("pokeball"), ItemDef {
        id: ID::new("pokeball"),
        name: "Poke Ball".to_string(),
        num: 4,
        gen: 1,
        category: ItemCategory::Pokeball,
        is_pokeball: true,
        ..Default::default()
    });

    map.insert(ID::new("polkadotbow"), ItemDef {
        id: ID::new("polkadotbow"),
        name: "Polkadot Bow".to_string(),
        num: 251,
        gen: 2,
        category: ItemCategory::HeldItem,
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("pomegberry"), ItemDef {
        id: ID::new("pomegberry"),
        name: "Pomeg Berry".to_string(),
        num: 169,
        gen: 3,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(90),
        natural_gift_type: Some("Ice".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("poweranklet"), ItemDef {
        id: ID::new("poweranklet"),
        name: "Power Anklet".to_string(),
        num: 293,
        gen: 4,
        category: ItemCategory::HeldItem,
        fling_power: Some(70),
        ignore_klutz: true,
        ..Default::default()
    });

    map.insert(ID::new("powerband"), ItemDef {
        id: ID::new("powerband"),
        name: "Power Band".to_string(),
        num: 292,
        gen: 4,
        category: ItemCategory::HeldItem,
        fling_power: Some(70),
        ignore_klutz: true,
        ..Default::default()
    });

    map.insert(ID::new("powerbelt"), ItemDef {
        id: ID::new("powerbelt"),
        name: "Power Belt".to_string(),
        num: 290,
        gen: 4,
        category: ItemCategory::HeldItem,
        fling_power: Some(70),
        ignore_klutz: true,
        ..Default::default()
    });

    map.insert(ID::new("powerbracer"), ItemDef {
        id: ID::new("powerbracer"),
        name: "Power Bracer".to_string(),
        num: 289,
        gen: 4,
        category: ItemCategory::HeldItem,
        fling_power: Some(70),
        ignore_klutz: true,
        ..Default::default()
    });

    map.insert(ID::new("powerherb"), ItemDef {
        id: ID::new("powerherb"),
        name: "Power Herb".to_string(),
        num: 271,
        gen: 4,
        category: ItemCategory::HeldItem,
        fling_power: Some(10),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::BeforeMove, ItemEffect::Compound { effects: vec![ItemEffect::SkipChargeTurn, ItemEffect::ConsumeItem] }, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("powerlens"), ItemDef {
        id: ID::new("powerlens"),
        name: "Power Lens".to_string(),
        num: 291,
        gen: 4,
        category: ItemCategory::HeldItem,
        fling_power: Some(70),
        ignore_klutz: true,
        ..Default::default()
    });

    map.insert(ID::new("powerweight"), ItemDef {
        id: ID::new("powerweight"),
        name: "Power Weight".to_string(),
        num: 294,
        gen: 4,
        category: ItemCategory::HeldItem,
        fling_power: Some(70),
        ignore_klutz: true,
        ..Default::default()
    });

    map.insert(ID::new("premierball"), ItemDef {
        id: ID::new("premierball"),
        name: "Premier Ball".to_string(),
        num: 12,
        gen: 3,
        category: ItemCategory::Pokeball,
        is_pokeball: true,
        ..Default::default()
    });

    map.insert(ID::new("prettyfeather"), ItemDef {
        id: ID::new("prettyfeather"),
        name: "Pretty Feather".to_string(),
        num: 571,
        gen: 5,
        category: ItemCategory::HeldItem,
        fling_power: Some(20),
        ..Default::default()
    });

    map.insert(ID::new("primariumz"), ItemDef {
        id: ID::new("primariumz"),
        name: "Primarium Z".to_string(),
        num: 800,
        gen: 7,
        category: ItemCategory::ZCrystal,
        is_nonstandard: Some("Past".to_string()),
        item_user: Some(vec!["Primarina".to_string()]),
        z_move: Some("Oceanic Operetta".to_string()),
        z_move_from: Some("Sparkling Aria".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("prismscale"), ItemDef {
        id: ID::new("prismscale"),
        name: "Prism Scale".to_string(),
        num: 537,
        gen: 5,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        ..Default::default()
    });

    map.insert(ID::new("protectivepads"), ItemDef {
        id: ID::new("protectivepads"),
        name: "Protective Pads".to_string(),
        num: 880,
        gen: 7,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        has_custom_handler: true,
        ..Default::default()
    });

    map.insert(ID::new("protector"), ItemDef {
        id: ID::new("protector"),
        name: "Protector".to_string(),
        num: 321,
        gen: 4,
        category: ItemCategory::HeldItem,
        fling_power: Some(80),
        ..Default::default()
    });

    map.insert(ID::new("przcureberry"), ItemDef {
        id: ID::new("przcureberry"),
        name: "PRZ Cure Berry".to_string(),
        num: 149,
        gen: 2,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        is_nonstandard: Some("Past".to_string()),
        natural_gift_power: Some(80),
        natural_gift_type: Some("Fire".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("psncureberry"), ItemDef {
        id: ID::new("psncureberry"),
        name: "PSN Cure Berry".to_string(),
        num: 151,
        gen: 2,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        is_nonstandard: Some("Past".to_string()),
        natural_gift_power: Some(80),
        natural_gift_type: Some("Electric".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("psychicgem"), ItemDef {
        id: ID::new("psychicgem"),
        name: "Psychic Gem".to_string(),
        num: 557,
        gen: 5,
        category: ItemCategory::Gem,
        is_gem: true,
        is_consumable: true,
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("psychicmemory"), ItemDef {
        id: ID::new("psychicmemory"),
        name: "Psychic Memory".to_string(),
        num: 916,
        gen: 7,
        category: ItemCategory::Memory,
        is_nonstandard: Some("Past".to_string()),
        item_user: Some(vec!["Silvally-Psychic".to_string()]),
        forced_forme: Some("Silvally-Psychic".to_string()),
        on_memory: Some("Psychic".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("psychicseed"), ItemDef {
        id: ID::new("psychicseed"),
        name: "Psychic Seed".to_string(),
        num: 882,
        gen: 7,
        category: ItemCategory::HeldItem,
        fling_power: Some(10),
        boosts: Some(boosts(&[("spd", 1)])),
        ..Default::default()
    });

    map.insert(ID::new("psychiumz"), ItemDef {
        id: ID::new("psychiumz"),
        name: "Psychium Z".to_string(),
        num: 786,
        gen: 7,
        category: ItemCategory::ZCrystal,
        is_nonstandard: Some("Past".to_string()),
        z_move_type: Some("Psychic".to_string()),
        forced_forme: Some("Arceus-Psychic".to_string()),
        on_plate: Some("Psychic".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("punchingglove"), ItemDef {
        id: ID::new("punchingglove"),
        name: "Punching Glove".to_string(),
        num: 1884,
        gen: 9,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        ..Default::default()
    });

    map.insert(ID::new("pyroarite"), ItemDef {
        id: ID::new("pyroarite"),
        name: "Pyroarite".to_string(),
        num: 2578,
        gen: 9,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Future".to_string()),
        mega_stone: Some("Pyroar-Mega".to_string()),
        mega_evolves: Some("Pyroar".to_string()),
        item_user: Some(vec!["Pyroar".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("qualotberry"), ItemDef {
        id: ID::new("qualotberry"),
        name: "Qualot Berry".to_string(),
        num: 171,
        gen: 3,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(90),
        natural_gift_type: Some("Poison".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("quickball"), ItemDef {
        id: ID::new("quickball"),
        name: "Quick Ball".to_string(),
        num: 15,
        gen: 4,
        category: ItemCategory::Pokeball,
        is_pokeball: true,
        ..Default::default()
    });

    map.insert(ID::new("quickclaw"), ItemDef {
        id: ID::new("quickclaw"),
        name: "Quick Claw".to_string(),
        num: 217,
        gen: 2,
        category: ItemCategory::HeldItem,
        fling_power: Some(80),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::BeforeMove, ItemEffect::PriorityChance { chance: 0.2 }, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("quickpowder"), ItemDef {
        id: ID::new("quickpowder"),
        name: "Quick Powder".to_string(),
        num: 274,
        gen: 4,
        category: ItemCategory::HeldItem,
        fling_power: Some(10),
        is_nonstandard: Some("Past".to_string()),
        item_user: Some(vec!["Ditto".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("rabutaberry"), ItemDef {
        id: ID::new("rabutaberry"),
        name: "Rabuta Berry".to_string(),
        num: 177,
        gen: 3,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        is_nonstandard: Some("Past".to_string()),
        natural_gift_power: Some(90),
        natural_gift_type: Some("Ghost".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("raichunitex"), ItemDef {
        id: ID::new("raichunitex"),
        name: "Raichunite X".to_string(),
        num: 2585,
        gen: 9,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Future".to_string()),
        mega_stone: Some("Raichu-Mega-X".to_string()),
        mega_evolves: Some("Raichu".to_string()),
        item_user: Some(vec!["Raichu".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("raichunitey"), ItemDef {
        id: ID::new("raichunitey"),
        name: "Raichunite Y".to_string(),
        num: 2586,
        gen: 9,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Future".to_string()),
        mega_stone: Some("Raichu-Mega-Y".to_string()),
        mega_evolves: Some("Raichu".to_string()),
        item_user: Some(vec!["Raichu".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("rarebone"), ItemDef {
        id: ID::new("rarebone"),
        name: "Rare Bone".to_string(),
        num: 106,
        gen: 4,
        category: ItemCategory::HeldItem,
        fling_power: Some(100),
        ..Default::default()
    });

    map.insert(ID::new("rawstberry"), ItemDef {
        id: ID::new("rawstberry"),
        name: "Rawst Berry".to_string(),
        num: 152,
        gen: 3,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(80),
        natural_gift_type: Some("Grass".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::Update, ItemEffect::Compound { effects: vec![
                ItemEffect::CureStatus { status: "brn".to_string() },
                ItemEffect::ConsumeItem,
            ] }, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("razorclaw"), ItemDef {
        id: ID::new("razorclaw"),
        name: "Razor Claw".to_string(),
        num: 326,
        gen: 4,
        category: ItemCategory::HeldItem,
        fling_power: Some(80),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::ModifyCritRatio, ItemEffect::BoostCritRatio { stages: 1 }, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("razorfang"), ItemDef {
        id: ID::new("razorfang"),
        name: "Razor Fang".to_string(),
        num: 327,
        gen: 4,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        ..Default::default()
    });

    map.insert(ID::new("razzberry"), ItemDef {
        id: ID::new("razzberry"),
        name: "Razz Berry".to_string(),
        num: 164,
        gen: 3,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        is_nonstandard: Some("Past".to_string()),
        natural_gift_power: Some(80),
        natural_gift_type: Some("Steel".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("reapercloth"), ItemDef {
        id: ID::new("reapercloth"),
        name: "Reaper Cloth".to_string(),
        num: 325,
        gen: 4,
        category: ItemCategory::HeldItem,
        fling_power: Some(10),
        ..Default::default()
    });

    map.insert(ID::new("redcard"), ItemDef {
        id: ID::new("redcard"),
        name: "Red Card".to_string(),
        num: 542,
        gen: 5,
        category: ItemCategory::HeldItem,
        fling_power: Some(10),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::DamagingHit, ItemEffect::Compound { effects: vec![ItemEffect::ForceOpponentSwitch, ItemEffect::ConsumeItem] }, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("redorb"), ItemDef {
        id: ID::new("redorb"),
        name: "Red Orb".to_string(),
        num: 534,
        gen: 6,
        category: ItemCategory::HeldItem,
        is_nonstandard: Some("Past".to_string()),
        item_user: Some(vec!["Groudon".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("repeatball"), ItemDef {
        id: ID::new("repeatball"),
        name: "Repeat Ball".to_string(),
        num: 9,
        gen: 3,
        category: ItemCategory::Pokeball,
        is_pokeball: true,
        ..Default::default()
    });

    map.insert(ID::new("ribbonsweet"), ItemDef {
        id: ID::new("ribbonsweet"),
        name: "Ribbon Sweet".to_string(),
        num: 1115,
        gen: 8,
        category: ItemCategory::HeldItem,
        fling_power: Some(10),
        ..Default::default()
    });

    map.insert(ID::new("rindoberry"), ItemDef {
        id: ID::new("rindoberry"),
        name: "Rindo Berry".to_string(),
        num: 187,
        gen: 4,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(80),
        natural_gift_type: Some("Grass".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::SourceModifyDamage, ItemEffect::Compound { effects: vec![
                ItemEffect::ResistSuperEffective { resist_type: "Grass".to_string(), multiplier: 0.5 },
                ItemEffect::ConsumeItem,
            ] }, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("ringtarget"), ItemDef {
        id: ID::new("ringtarget"),
        name: "Ring Target".to_string(),
        num: 543,
        gen: 5,
        category: ItemCategory::HeldItem,
        fling_power: Some(10),
        ..Default::default()
    });

    map.insert(ID::new("rockgem"), ItemDef {
        id: ID::new("rockgem"),
        name: "Rock Gem".to_string(),
        num: 559,
        gen: 5,
        category: ItemCategory::Gem,
        is_gem: true,
        is_consumable: true,
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("rockincense"), ItemDef {
        id: ID::new("rockincense"),
        name: "Rock Incense".to_string(),
        num: 315,
        gen: 4,
        category: ItemCategory::HeldItem,
        fling_power: Some(10),
        is_nonstandard: Some("Past".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::BasePower, ItemEffect::BoostType { move_type: "Rock".to_string(), multiplier: 1.2 }, 15),
        ],
        ..Default::default()
    });

    map.insert(ID::new("rockiumz"), ItemDef {
        id: ID::new("rockiumz"),
        name: "Rockium Z".to_string(),
        num: 788,
        gen: 7,
        category: ItemCategory::ZCrystal,
        is_nonstandard: Some("Past".to_string()),
        z_move_type: Some("Rock".to_string()),
        forced_forme: Some("Arceus-Rock".to_string()),
        on_plate: Some("Rock".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("rockmemory"), ItemDef {
        id: ID::new("rockmemory"),
        name: "Rock Memory".to_string(),
        num: 908,
        gen: 7,
        category: ItemCategory::Memory,
        is_nonstandard: Some("Past".to_string()),
        item_user: Some(vec!["Silvally-Rock".to_string()]),
        forced_forme: Some("Silvally-Rock".to_string()),
        on_memory: Some("Rock".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("rockyhelmet"), ItemDef {
        id: ID::new("rockyhelmet"),
        name: "Rocky Helmet".to_string(),
        num: 540,
        gen: 5,
        category: ItemCategory::HeldItem,
        fling_power: Some(60),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::DamagingHit, ItemEffect::ContactDamage { fraction: 0.167 }, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("roomservice"), ItemDef {
        id: ID::new("roomservice"),
        name: "Room Service".to_string(),
        num: 1122,
        gen: 8,
        category: ItemCategory::HeldItem,
        fling_power: Some(100),
        boosts: Some(boosts(&[("spe", -1)])),
        ..Default::default()
    });

    map.insert(ID::new("rootfossil"), ItemDef {
        id: ID::new("rootfossil"),
        name: "Root Fossil".to_string(),
        num: 99,
        gen: 3,
        category: ItemCategory::Fossil,
        fling_power: Some(100),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("roseincense"), ItemDef {
        id: ID::new("roseincense"),
        name: "Rose Incense".to_string(),
        num: 318,
        gen: 4,
        category: ItemCategory::HeldItem,
        fling_power: Some(10),
        is_nonstandard: Some("Past".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::BasePower, ItemEffect::BoostType { move_type: "Grass".to_string(), multiplier: 1.2 }, 15),
        ],
        ..Default::default()
    });

    map.insert(ID::new("roseliberry"), ItemDef {
        id: ID::new("roseliberry"),
        name: "Roseli Berry".to_string(),
        num: 686,
        gen: 6,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(80),
        natural_gift_type: Some("Fairy".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::SourceModifyDamage, ItemEffect::Compound { effects: vec![
                ItemEffect::ResistSuperEffective { resist_type: "Fairy".to_string(), multiplier: 0.5 },
                ItemEffect::ConsumeItem,
            ] }, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("rowapberry"), ItemDef {
        id: ID::new("rowapberry"),
        name: "Rowap Berry".to_string(),
        num: 212,
        gen: 4,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(100),
        natural_gift_type: Some("Dark".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("rustedshield"), ItemDef {
        id: ID::new("rustedshield"),
        name: "Rusted Shield".to_string(),
        num: 1104,
        gen: 8,
        category: ItemCategory::HeldItem,
        item_user: Some(vec!["Zamazenta-Crowned".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("rustedsword"), ItemDef {
        id: ID::new("rustedsword"),
        name: "Rusted Sword".to_string(),
        num: 1103,
        gen: 8,
        category: ItemCategory::HeldItem,
        item_user: Some(vec!["Zacian-Crowned".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("sablenite"), ItemDef {
        id: ID::new("sablenite"),
        name: "Sablenite".to_string(),
        num: 754,
        gen: 6,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Past".to_string()),
        mega_stone: Some("Sableye-Mega".to_string()),
        mega_evolves: Some("Sableye".to_string()),
        item_user: Some(vec!["Sableye".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("sachet"), ItemDef {
        id: ID::new("sachet"),
        name: "Sachet".to_string(),
        num: 647,
        gen: 6,
        category: ItemCategory::HeldItem,
        fling_power: Some(80),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("safariball"), ItemDef {
        id: ID::new("safariball"),
        name: "Safari Ball".to_string(),
        num: 5,
        gen: 1,
        category: ItemCategory::Pokeball,
        is_pokeball: true,
        ..Default::default()
    });

    map.insert(ID::new("safetygoggles"), ItemDef {
        id: ID::new("safetygoggles"),
        name: "Safety Goggles".to_string(),
        num: 650,
        gen: 6,
        category: ItemCategory::HeldItem,
        fling_power: Some(80),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::Residual, ItemEffect::WeatherImmunity, 0),
            ItemEffectEntry::with_priority(EventType::TryHit, ItemEffect::PowderImmunity, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("sailfossil"), ItemDef {
        id: ID::new("sailfossil"),
        name: "Sail Fossil".to_string(),
        num: 711,
        gen: 6,
        category: ItemCategory::Fossil,
        fling_power: Some(100),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("salacberry"), ItemDef {
        id: ID::new("salacberry"),
        name: "Salac Berry".to_string(),
        num: 203,
        gen: 3,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(100),
        natural_gift_type: Some("Fighting".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("salamencite"), ItemDef {
        id: ID::new("salamencite"),
        name: "Salamencite".to_string(),
        num: 769,
        gen: 6,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Past".to_string()),
        mega_stone: Some("Salamence-Mega".to_string()),
        mega_evolves: Some("Salamence".to_string()),
        item_user: Some(vec!["Salamence".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("sceptilite"), ItemDef {
        id: ID::new("sceptilite"),
        name: "Sceptilite".to_string(),
        num: 753,
        gen: 6,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Past".to_string()),
        mega_stone: Some("Sceptile-Mega".to_string()),
        mega_evolves: Some("Sceptile".to_string()),
        item_user: Some(vec!["Sceptile".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("scizorite"), ItemDef {
        id: ID::new("scizorite"),
        name: "Scizorite".to_string(),
        num: 670,
        gen: 6,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Past".to_string()),
        mega_stone: Some("Scizor-Mega".to_string()),
        mega_evolves: Some("Scizor".to_string()),
        item_user: Some(vec!["Scizor".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("scolipite"), ItemDef {
        id: ID::new("scolipite"),
        name: "Scolipite".to_string(),
        num: 2571,
        gen: 9,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Future".to_string()),
        mega_stone: Some("Scolipede-Mega".to_string()),
        mega_evolves: Some("Scolipede".to_string()),
        item_user: Some(vec!["Scolipede".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("scopelens"), ItemDef {
        id: ID::new("scopelens"),
        name: "Scope Lens".to_string(),
        num: 232,
        gen: 2,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::ModifyCritRatio, ItemEffect::BoostCritRatio { stages: 1 }, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("scovillainite"), ItemDef {
        id: ID::new("scovillainite"),
        name: "Scovillainite".to_string(),
        num: 2599,
        gen: 9,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Future".to_string()),
        mega_stone: Some("Scovillain-Mega".to_string()),
        mega_evolves: Some("Scovillain".to_string()),
        item_user: Some(vec!["Scovillain".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("scraftinite"), ItemDef {
        id: ID::new("scraftinite"),
        name: "Scraftinite".to_string(),
        num: 2572,
        gen: 9,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Future".to_string()),
        mega_stone: Some("Scrafty-Mega".to_string()),
        mega_evolves: Some("Scrafty".to_string()),
        item_user: Some(vec!["Scrafty".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("seaincense"), ItemDef {
        id: ID::new("seaincense"),
        name: "Sea Incense".to_string(),
        num: 254,
        gen: 3,
        category: ItemCategory::HeldItem,
        fling_power: Some(10),
        is_nonstandard: Some("Past".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::BasePower, ItemEffect::BoostType { move_type: "Water".to_string(), multiplier: 1.2 }, 15),
        ],
        ..Default::default()
    });

    map.insert(ID::new("sharpbeak"), ItemDef {
        id: ID::new("sharpbeak"),
        name: "Sharp Beak".to_string(),
        num: 244,
        gen: 2,
        category: ItemCategory::HeldItem,
        fling_power: Some(50),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::BasePower, ItemEffect::BoostType { move_type: "Flying".to_string(), multiplier: 1.2 }, 15),
        ],
        ..Default::default()
    });

    map.insert(ID::new("sharpedonite"), ItemDef {
        id: ID::new("sharpedonite"),
        name: "Sharpedonite".to_string(),
        num: 759,
        gen: 6,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Past".to_string()),
        mega_stone: Some("Sharpedo-Mega".to_string()),
        mega_evolves: Some("Sharpedo".to_string()),
        item_user: Some(vec!["Sharpedo".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("shedshell"), ItemDef {
        id: ID::new("shedshell"),
        name: "Shed Shell".to_string(),
        num: 295,
        gen: 4,
        category: ItemCategory::HeldItem,
        fling_power: Some(10),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::SwitchOut, ItemEffect::EscapeTrapping, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("shellbell"), ItemDef {
        id: ID::new("shellbell"),
        name: "Shell Bell".to_string(),
        num: 253,
        gen: 3,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        ..Default::default()
    });

    map.insert(ID::new("shinystone"), ItemDef {
        id: ID::new("shinystone"),
        name: "Shiny Stone".to_string(),
        num: 107,
        gen: 4,
        category: ItemCategory::HeldItem,
        fling_power: Some(80),
        ..Default::default()
    });

    map.insert(ID::new("shockdrive"), ItemDef {
        id: ID::new("shockdrive"),
        name: "Shock Drive".to_string(),
        num: 117,
        gen: 5,
        category: ItemCategory::Drive,
        is_nonstandard: Some("Past".to_string()),
        item_user: Some(vec!["Genesect-Shock".to_string()]),
        forced_forme: Some("Genesect-Shock".to_string()),
        on_drive: Some("Electric".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("shucaberry"), ItemDef {
        id: ID::new("shucaberry"),
        name: "Shuca Berry".to_string(),
        num: 191,
        gen: 4,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(80),
        natural_gift_type: Some("Ground".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::SourceModifyDamage, ItemEffect::Compound { effects: vec![
                ItemEffect::ResistSuperEffective { resist_type: "Ground".to_string(), multiplier: 0.5 },
                ItemEffect::ConsumeItem,
            ] }, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("silkscarf"), ItemDef {
        id: ID::new("silkscarf"),
        name: "Silk Scarf".to_string(),
        num: 251,
        gen: 3,
        category: ItemCategory::HeldItem,
        fling_power: Some(10),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::BasePower, ItemEffect::BoostType { move_type: "Normal".to_string(), multiplier: 1.2 }, 15),
        ],
        ..Default::default()
    });

    map.insert(ID::new("silverpowder"), ItemDef {
        id: ID::new("silverpowder"),
        name: "Silver Powder".to_string(),
        num: 222,
        gen: 2,
        category: ItemCategory::HeldItem,
        fling_power: Some(10),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::BasePower, ItemEffect::BoostType { move_type: "Bug".to_string(), multiplier: 1.2 }, 15),
        ],
        ..Default::default()
    });

    map.insert(ID::new("sitrusberry"), ItemDef {
        id: ID::new("sitrusberry"),
        name: "Sitrus Berry".to_string(),
        num: 158,
        gen: 3,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(80),
        natural_gift_type: Some("Psychic".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::Update, ItemEffect::Triggered {
            trigger: Trigger::OnHPBelow(0.5),
            effect: Box::new(ItemEffect::Compound { effects: vec![
                ItemEffect::HealOnThreshold { threshold: 0.5, heal_fraction: 0.25 },
                ItemEffect::ConsumeItem,
            ] }),
        }, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("skarmorite"), ItemDef {
        id: ID::new("skarmorite"),
        name: "Skarmorite".to_string(),
        num: 2565,
        gen: 9,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Future".to_string()),
        mega_stone: Some("Skarmory-Mega".to_string()),
        mega_evolves: Some("Skarmory".to_string()),
        item_user: Some(vec!["Skarmory".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("skullfossil"), ItemDef {
        id: ID::new("skullfossil"),
        name: "Skull Fossil".to_string(),
        num: 105,
        gen: 4,
        category: ItemCategory::Fossil,
        fling_power: Some(100),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("skyplate"), ItemDef {
        id: ID::new("skyplate"),
        name: "Sky Plate".to_string(),
        num: 306,
        gen: 4,
        category: ItemCategory::Plate,
        forced_forme: Some("Arceus-Flying".to_string()),
        on_plate: Some("Flying".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::BasePower, ItemEffect::BoostType { move_type: "Flying".to_string(), multiplier: 1.2 }, 15),
        ],
        ..Default::default()
    });

    map.insert(ID::new("slowbronite"), ItemDef {
        id: ID::new("slowbronite"),
        name: "Slowbronite".to_string(),
        num: 760,
        gen: 6,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Past".to_string()),
        mega_stone: Some("Slowbro-Mega".to_string()),
        mega_evolves: Some("Slowbro".to_string()),
        item_user: Some(vec!["Slowbro".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("smoothrock"), ItemDef {
        id: ID::new("smoothrock"),
        name: "Smooth Rock".to_string(),
        num: 283,
        gen: 4,
        category: ItemCategory::HeldItem,
        fling_power: Some(10),
        ..Default::default()
    });

    map.insert(ID::new("snorliumz"), ItemDef {
        id: ID::new("snorliumz"),
        name: "Snorlium Z".to_string(),
        num: 804,
        gen: 7,
        category: ItemCategory::ZCrystal,
        is_nonstandard: Some("Past".to_string()),
        item_user: Some(vec!["Snorlax".to_string()]),
        z_move: Some("Pulverizing Pancake".to_string()),
        z_move_from: Some("Giga Impact".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("snowball"), ItemDef {
        id: ID::new("snowball"),
        name: "Snowball".to_string(),
        num: 649,
        gen: 6,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        boosts: Some(boosts(&[("atk", 1)])),
        ..Default::default()
    });

    map.insert(ID::new("softsand"), ItemDef {
        id: ID::new("softsand"),
        name: "Soft Sand".to_string(),
        num: 237,
        gen: 2,
        category: ItemCategory::HeldItem,
        fling_power: Some(10),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::BasePower, ItemEffect::BoostType { move_type: "Ground".to_string(), multiplier: 1.2 }, 15),
        ],
        ..Default::default()
    });

    map.insert(ID::new("solganiumz"), ItemDef {
        id: ID::new("solganiumz"),
        name: "Solganium Z".to_string(),
        num: 921,
        gen: 7,
        category: ItemCategory::ZCrystal,
        is_nonstandard: Some("Past".to_string()),
        item_user: Some(vec!["Solgaleo".to_string(), "Necrozma-Dusk-Mane".to_string()]),
        z_move: Some("Searing Sunraze Smash".to_string()),
        z_move_from: Some("Sunsteel Strike".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("souldew"), ItemDef {
        id: ID::new("souldew"),
        name: "Soul Dew".to_string(),
        num: 225,
        gen: 3,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        item_user: Some(vec!["Latios".to_string(), "Latias".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("spelltag"), ItemDef {
        id: ID::new("spelltag"),
        name: "Spell Tag".to_string(),
        num: 247,
        gen: 2,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::BasePower, ItemEffect::BoostType { move_type: "Ghost".to_string(), multiplier: 1.2 }, 15),
        ],
        ..Default::default()
    });

    map.insert(ID::new("spelonberry"), ItemDef {
        id: ID::new("spelonberry"),
        name: "Spelon Berry".to_string(),
        num: 179,
        gen: 3,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        is_nonstandard: Some("Past".to_string()),
        natural_gift_power: Some(90),
        natural_gift_type: Some("Dark".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("splashplate"), ItemDef {
        id: ID::new("splashplate"),
        name: "Splash Plate".to_string(),
        num: 299,
        gen: 4,
        category: ItemCategory::Plate,
        forced_forme: Some("Arceus-Water".to_string()),
        on_plate: Some("Water".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::BasePower, ItemEffect::BoostType { move_type: "Water".to_string(), multiplier: 1.2 }, 15),
        ],
        ..Default::default()
    });

    map.insert(ID::new("spookyplate"), ItemDef {
        id: ID::new("spookyplate"),
        name: "Spooky Plate".to_string(),
        num: 310,
        gen: 4,
        category: ItemCategory::Plate,
        forced_forme: Some("Arceus-Ghost".to_string()),
        on_plate: Some("Ghost".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::BasePower, ItemEffect::BoostType { move_type: "Ghost".to_string(), multiplier: 1.2 }, 15),
        ],
        ..Default::default()
    });

    map.insert(ID::new("sportball"), ItemDef {
        id: ID::new("sportball"),
        name: "Sport Ball".to_string(),
        num: 499,
        gen: 2,
        category: ItemCategory::Pokeball,
        is_pokeball: true,
        ..Default::default()
    });

    map.insert(ID::new("staraptite"), ItemDef {
        id: ID::new("staraptite"),
        name: "Staraptite".to_string(),
        num: 2589,
        gen: 9,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Future".to_string()),
        mega_stone: Some("Staraptor-Mega".to_string()),
        mega_evolves: Some("Staraptor".to_string()),
        item_user: Some(vec!["Staraptor".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("starfberry"), ItemDef {
        id: ID::new("starfberry"),
        name: "Starf Berry".to_string(),
        num: 207,
        gen: 3,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(100),
        natural_gift_type: Some("Psychic".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("starminite"), ItemDef {
        id: ID::new("starminite"),
        name: "Starminite".to_string(),
        num: 2561,
        gen: 9,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Future".to_string()),
        mega_stone: Some("Starmie-Mega".to_string()),
        mega_evolves: Some("Starmie".to_string()),
        item_user: Some(vec!["Starmie".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("starsweet"), ItemDef {
        id: ID::new("starsweet"),
        name: "Star Sweet".to_string(),
        num: 1114,
        gen: 8,
        category: ItemCategory::HeldItem,
        fling_power: Some(10),
        ..Default::default()
    });

    map.insert(ID::new("steelgem"), ItemDef {
        id: ID::new("steelgem"),
        name: "Steel Gem".to_string(),
        num: 563,
        gen: 5,
        category: ItemCategory::Gem,
        is_gem: true,
        is_consumable: true,
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("steeliumz"), ItemDef {
        id: ID::new("steeliumz"),
        name: "Steelium Z".to_string(),
        num: 792,
        gen: 7,
        category: ItemCategory::ZCrystal,
        is_nonstandard: Some("Past".to_string()),
        z_move_type: Some("Steel".to_string()),
        forced_forme: Some("Arceus-Steel".to_string()),
        on_plate: Some("Steel".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("steelixite"), ItemDef {
        id: ID::new("steelixite"),
        name: "Steelixite".to_string(),
        num: 761,
        gen: 6,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Past".to_string()),
        mega_stone: Some("Steelix-Mega".to_string()),
        mega_evolves: Some("Steelix".to_string()),
        item_user: Some(vec!["Steelix".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("steelmemory"), ItemDef {
        id: ID::new("steelmemory"),
        name: "Steel Memory".to_string(),
        num: 911,
        gen: 7,
        category: ItemCategory::Memory,
        is_nonstandard: Some("Past".to_string()),
        item_user: Some(vec!["Silvally-Steel".to_string()]),
        forced_forme: Some("Silvally-Steel".to_string()),
        on_memory: Some("Steel".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("stick"), ItemDef {
        id: ID::new("stick"),
        name: "Stick".to_string(),
        num: 259,
        gen: 2,
        category: ItemCategory::HeldItem,
        fling_power: Some(60),
        is_nonstandard: Some("Past".to_string()),
        item_user: Some(vec!["Farfetch’d".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("stickybarb"), ItemDef {
        id: ID::new("stickybarb"),
        name: "Sticky Barb".to_string(),
        num: 288,
        gen: 4,
        category: ItemCategory::HeldItem,
        fling_power: Some(80),
        ..Default::default()
    });

    map.insert(ID::new("stoneplate"), ItemDef {
        id: ID::new("stoneplate"),
        name: "Stone Plate".to_string(),
        num: 309,
        gen: 4,
        category: ItemCategory::Plate,
        forced_forme: Some("Arceus-Rock".to_string()),
        on_plate: Some("Rock".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::BasePower, ItemEffect::BoostType { move_type: "Rock".to_string(), multiplier: 1.2 }, 15),
        ],
        ..Default::default()
    });

    map.insert(ID::new("strangeball"), ItemDef {
        id: ID::new("strangeball"),
        name: "Strange Ball".to_string(),
        num: 1785,
        gen: 8,
        category: ItemCategory::Pokeball,
        is_pokeball: true,
        is_nonstandard: Some("Unobtainable".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("strawberrysweet"), ItemDef {
        id: ID::new("strawberrysweet"),
        name: "Strawberry Sweet".to_string(),
        num: 1109,
        gen: 8,
        category: ItemCategory::HeldItem,
        fling_power: Some(10),
        ..Default::default()
    });

    map.insert(ID::new("sunstone"), ItemDef {
        id: ID::new("sunstone"),
        name: "Sun Stone".to_string(),
        num: 80,
        gen: 2,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        ..Default::default()
    });

    map.insert(ID::new("swampertite"), ItemDef {
        id: ID::new("swampertite"),
        name: "Swampertite".to_string(),
        num: 752,
        gen: 6,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Past".to_string()),
        mega_stone: Some("Swampert-Mega".to_string()),
        mega_evolves: Some("Swampert".to_string()),
        item_user: Some(vec!["Swampert".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("sweetapple"), ItemDef {
        id: ID::new("sweetapple"),
        name: "Sweet Apple".to_string(),
        num: 1116,
        gen: 8,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        ..Default::default()
    });

    map.insert(ID::new("syrupyapple"), ItemDef {
        id: ID::new("syrupyapple"),
        name: "Syrupy Apple".to_string(),
        num: 2402,
        gen: 9,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        ..Default::default()
    });

    map.insert(ID::new("tamatoberry"), ItemDef {
        id: ID::new("tamatoberry"),
        name: "Tamato Berry".to_string(),
        num: 174,
        gen: 3,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(90),
        natural_gift_type: Some("Psychic".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tangaberry"), ItemDef {
        id: ID::new("tangaberry"),
        name: "Tanga Berry".to_string(),
        num: 194,
        gen: 4,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(80),
        natural_gift_type: Some("Bug".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::SourceModifyDamage, ItemEffect::Compound { effects: vec![
                ItemEffect::ResistSuperEffective { resist_type: "Bug".to_string(), multiplier: 0.5 },
                ItemEffect::ConsumeItem,
            ] }, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("tapuniumz"), ItemDef {
        id: ID::new("tapuniumz"),
        name: "Tapunium Z".to_string(),
        num: 801,
        gen: 7,
        category: ItemCategory::ZCrystal,
        is_nonstandard: Some("Past".to_string()),
        item_user: Some(vec!["Tapu Koko".to_string(), "Tapu Lele".to_string(), "Tapu Bulu".to_string(), "Tapu Fini".to_string()]),
        z_move: Some("Guardian of Alola".to_string()),
        z_move_from: Some("Nature's Madness".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tartapple"), ItemDef {
        id: ID::new("tartapple"),
        name: "Tart Apple".to_string(),
        num: 1117,
        gen: 8,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        ..Default::default()
    });

    map.insert(ID::new("tatsugirinite"), ItemDef {
        id: ID::new("tatsugirinite"),
        name: "Tatsugirinite".to_string(),
        num: 2601,
        gen: 9,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Future".to_string()),
        mega_stone: Some("Tatsugiri-Curly-Mega,Tatsugiri-Droopy-Mega,Tatsugiri-Stretchy-Mega".to_string()),
        mega_evolves: Some("Tatsugiri,Tatsugiri-Droopy,Tatsugiri-Stretchy".to_string()),
        item_user: Some(vec!["Tatsugiri".to_string(), "Tatsugiri-Droopy".to_string(), "Tatsugiri-Stretchy".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("terrainextender"), ItemDef {
        id: ID::new("terrainextender"),
        name: "Terrain Extender".to_string(),
        num: 879,
        gen: 7,
        category: ItemCategory::HeldItem,
        fling_power: Some(60),
        ..Default::default()
    });

    map.insert(ID::new("thickclub"), ItemDef {
        id: ID::new("thickclub"),
        name: "Thick Club".to_string(),
        num: 258,
        gen: 2,
        category: ItemCategory::HeldItem,
        fling_power: Some(90),
        is_nonstandard: Some("Past".to_string()),
        item_user: Some(vec!["Marowak".to_string(), "Marowak-Alola".to_string(), "Marowak-Alola-Totem".to_string(), "Cubone".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("throatspray"), ItemDef {
        id: ID::new("throatspray"),
        name: "Throat Spray".to_string(),
        num: 1118,
        gen: 8,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        boosts: Some(boosts(&[("spa", 1)])),
        ..Default::default()
    });

    map.insert(ID::new("thunderstone"), ItemDef {
        id: ID::new("thunderstone"),
        name: "Thunder Stone".to_string(),
        num: 83,
        gen: 1,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        ..Default::default()
    });

    map.insert(ID::new("timerball"), ItemDef {
        id: ID::new("timerball"),
        name: "Timer Ball".to_string(),
        num: 10,
        gen: 3,
        category: ItemCategory::Pokeball,
        is_pokeball: true,
        ..Default::default()
    });

    map.insert(ID::new("toxicorb"), ItemDef {
        id: ID::new("toxicorb"),
        name: "Toxic Orb".to_string(),
        num: 272,
        gen: 4,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        ..Default::default()
    });

    map.insert(ID::new("toxicplate"), ItemDef {
        id: ID::new("toxicplate"),
        name: "Toxic Plate".to_string(),
        num: 304,
        gen: 4,
        category: ItemCategory::Plate,
        forced_forme: Some("Arceus-Poison".to_string()),
        on_plate: Some("Poison".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::BasePower, ItemEffect::BoostType { move_type: "Poison".to_string(), multiplier: 1.2 }, 15),
        ],
        ..Default::default()
    });

    map.insert(ID::new("tr00"), ItemDef {
        id: ID::new("tr00"),
        name: "TR00".to_string(),
        num: 1130,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(10),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr01"), ItemDef {
        id: ID::new("tr01"),
        name: "TR01".to_string(),
        num: 1131,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(85),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr02"), ItemDef {
        id: ID::new("tr02"),
        name: "TR02".to_string(),
        num: 1132,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(90),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr03"), ItemDef {
        id: ID::new("tr03"),
        name: "TR03".to_string(),
        num: 1133,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(110),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr04"), ItemDef {
        id: ID::new("tr04"),
        name: "TR04".to_string(),
        num: 1134,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(90),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr05"), ItemDef {
        id: ID::new("tr05"),
        name: "TR05".to_string(),
        num: 1135,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(90),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr06"), ItemDef {
        id: ID::new("tr06"),
        name: "TR06".to_string(),
        num: 1136,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(110),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr07"), ItemDef {
        id: ID::new("tr07"),
        name: "TR07".to_string(),
        num: 1137,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(10),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr08"), ItemDef {
        id: ID::new("tr08"),
        name: "TR08".to_string(),
        num: 1138,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(90),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr09"), ItemDef {
        id: ID::new("tr09"),
        name: "TR09".to_string(),
        num: 1139,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(110),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr10"), ItemDef {
        id: ID::new("tr10"),
        name: "TR10".to_string(),
        num: 1140,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(100),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr11"), ItemDef {
        id: ID::new("tr11"),
        name: "TR11".to_string(),
        num: 1141,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(90),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr12"), ItemDef {
        id: ID::new("tr12"),
        name: "TR12".to_string(),
        num: 1142,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(10),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr13"), ItemDef {
        id: ID::new("tr13"),
        name: "TR13".to_string(),
        num: 1143,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(10),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr14"), ItemDef {
        id: ID::new("tr14"),
        name: "TR14".to_string(),
        num: 1144,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(10),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr15"), ItemDef {
        id: ID::new("tr15"),
        name: "TR15".to_string(),
        num: 1145,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(110),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr16"), ItemDef {
        id: ID::new("tr16"),
        name: "TR16".to_string(),
        num: 1146,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(80),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr17"), ItemDef {
        id: ID::new("tr17"),
        name: "TR17".to_string(),
        num: 1147,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(10),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr18"), ItemDef {
        id: ID::new("tr18"),
        name: "TR18".to_string(),
        num: 1148,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(80),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr19"), ItemDef {
        id: ID::new("tr19"),
        name: "TR19".to_string(),
        num: 1149,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(80),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr20"), ItemDef {
        id: ID::new("tr20"),
        name: "TR20".to_string(),
        num: 1150,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(10),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr21"), ItemDef {
        id: ID::new("tr21"),
        name: "TR21".to_string(),
        num: 1151,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(10),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr22"), ItemDef {
        id: ID::new("tr22"),
        name: "TR22".to_string(),
        num: 1152,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(90),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr23"), ItemDef {
        id: ID::new("tr23"),
        name: "TR23".to_string(),
        num: 1153,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(10),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr24"), ItemDef {
        id: ID::new("tr24"),
        name: "TR24".to_string(),
        num: 1154,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(120),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr25"), ItemDef {
        id: ID::new("tr25"),
        name: "TR25".to_string(),
        num: 1155,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(80),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr26"), ItemDef {
        id: ID::new("tr26"),
        name: "TR26".to_string(),
        num: 1156,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(10),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr27"), ItemDef {
        id: ID::new("tr27"),
        name: "TR27".to_string(),
        num: 1157,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(10),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr28"), ItemDef {
        id: ID::new("tr28"),
        name: "TR28".to_string(),
        num: 1158,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(120),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr29"), ItemDef {
        id: ID::new("tr29"),
        name: "TR29".to_string(),
        num: 1159,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(10),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr30"), ItemDef {
        id: ID::new("tr30"),
        name: "TR30".to_string(),
        num: 1160,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(10),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr31"), ItemDef {
        id: ID::new("tr31"),
        name: "TR31".to_string(),
        num: 1161,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(100),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr32"), ItemDef {
        id: ID::new("tr32"),
        name: "TR32".to_string(),
        num: 1162,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(80),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr33"), ItemDef {
        id: ID::new("tr33"),
        name: "TR33".to_string(),
        num: 1163,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(80),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr34"), ItemDef {
        id: ID::new("tr34"),
        name: "TR34".to_string(),
        num: 1164,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(120),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr35"), ItemDef {
        id: ID::new("tr35"),
        name: "TR35".to_string(),
        num: 1165,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(90),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr36"), ItemDef {
        id: ID::new("tr36"),
        name: "TR36".to_string(),
        num: 1166,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(95),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr37"), ItemDef {
        id: ID::new("tr37"),
        name: "TR37".to_string(),
        num: 1167,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(10),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr38"), ItemDef {
        id: ID::new("tr38"),
        name: "TR38".to_string(),
        num: 1168,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(10),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr39"), ItemDef {
        id: ID::new("tr39"),
        name: "TR39".to_string(),
        num: 1169,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(120),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr40"), ItemDef {
        id: ID::new("tr40"),
        name: "TR40".to_string(),
        num: 1170,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(10),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr41"), ItemDef {
        id: ID::new("tr41"),
        name: "TR41".to_string(),
        num: 1171,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(85),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr42"), ItemDef {
        id: ID::new("tr42"),
        name: "TR42".to_string(),
        num: 1172,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(90),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr43"), ItemDef {
        id: ID::new("tr43"),
        name: "TR43".to_string(),
        num: 1173,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(130),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr44"), ItemDef {
        id: ID::new("tr44"),
        name: "TR44".to_string(),
        num: 1174,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(10),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr45"), ItemDef {
        id: ID::new("tr45"),
        name: "TR45".to_string(),
        num: 1175,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(90),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr46"), ItemDef {
        id: ID::new("tr46"),
        name: "TR46".to_string(),
        num: 1176,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(10),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr47"), ItemDef {
        id: ID::new("tr47"),
        name: "TR47".to_string(),
        num: 1177,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(80),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr48"), ItemDef {
        id: ID::new("tr48"),
        name: "TR48".to_string(),
        num: 1178,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(10),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr49"), ItemDef {
        id: ID::new("tr49"),
        name: "TR49".to_string(),
        num: 1179,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(10),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr50"), ItemDef {
        id: ID::new("tr50"),
        name: "TR50".to_string(),
        num: 1180,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(90),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr51"), ItemDef {
        id: ID::new("tr51"),
        name: "TR51".to_string(),
        num: 1181,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(10),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr52"), ItemDef {
        id: ID::new("tr52"),
        name: "TR52".to_string(),
        num: 1182,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(10),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr53"), ItemDef {
        id: ID::new("tr53"),
        name: "TR53".to_string(),
        num: 1183,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(120),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr54"), ItemDef {
        id: ID::new("tr54"),
        name: "TR54".to_string(),
        num: 1184,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(10),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr55"), ItemDef {
        id: ID::new("tr55"),
        name: "TR55".to_string(),
        num: 1185,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(120),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr56"), ItemDef {
        id: ID::new("tr56"),
        name: "TR56".to_string(),
        num: 1186,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(80),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr57"), ItemDef {
        id: ID::new("tr57"),
        name: "TR57".to_string(),
        num: 1187,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(80),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr58"), ItemDef {
        id: ID::new("tr58"),
        name: "TR58".to_string(),
        num: 1188,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(80),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr59"), ItemDef {
        id: ID::new("tr59"),
        name: "TR59".to_string(),
        num: 1189,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(80),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr60"), ItemDef {
        id: ID::new("tr60"),
        name: "TR60".to_string(),
        num: 1190,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(80),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr61"), ItemDef {
        id: ID::new("tr61"),
        name: "TR61".to_string(),
        num: 1191,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(90),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr62"), ItemDef {
        id: ID::new("tr62"),
        name: "TR62".to_string(),
        num: 1192,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(85),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr63"), ItemDef {
        id: ID::new("tr63"),
        name: "TR63".to_string(),
        num: 1193,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(80),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr64"), ItemDef {
        id: ID::new("tr64"),
        name: "TR64".to_string(),
        num: 1194,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(120),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr65"), ItemDef {
        id: ID::new("tr65"),
        name: "TR65".to_string(),
        num: 1195,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(90),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr66"), ItemDef {
        id: ID::new("tr66"),
        name: "TR66".to_string(),
        num: 1196,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(120),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr67"), ItemDef {
        id: ID::new("tr67"),
        name: "TR67".to_string(),
        num: 1197,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(90),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr68"), ItemDef {
        id: ID::new("tr68"),
        name: "TR68".to_string(),
        num: 1198,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(10),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr69"), ItemDef {
        id: ID::new("tr69"),
        name: "TR69".to_string(),
        num: 1199,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(80),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr70"), ItemDef {
        id: ID::new("tr70"),
        name: "TR70".to_string(),
        num: 1200,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(80),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr71"), ItemDef {
        id: ID::new("tr71"),
        name: "TR71".to_string(),
        num: 1201,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(130),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr72"), ItemDef {
        id: ID::new("tr72"),
        name: "TR72".to_string(),
        num: 1202,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(120),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr73"), ItemDef {
        id: ID::new("tr73"),
        name: "TR73".to_string(),
        num: 1203,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(120),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr74"), ItemDef {
        id: ID::new("tr74"),
        name: "TR74".to_string(),
        num: 1204,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(80),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr75"), ItemDef {
        id: ID::new("tr75"),
        name: "TR75".to_string(),
        num: 1205,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(100),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr76"), ItemDef {
        id: ID::new("tr76"),
        name: "TR76".to_string(),
        num: 1206,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(10),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr77"), ItemDef {
        id: ID::new("tr77"),
        name: "TR77".to_string(),
        num: 1207,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(10),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr78"), ItemDef {
        id: ID::new("tr78"),
        name: "TR78".to_string(),
        num: 1208,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(95),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr79"), ItemDef {
        id: ID::new("tr79"),
        name: "TR79".to_string(),
        num: 1209,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(10),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr80"), ItemDef {
        id: ID::new("tr80"),
        name: "TR80".to_string(),
        num: 1210,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(10),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr81"), ItemDef {
        id: ID::new("tr81"),
        name: "TR81".to_string(),
        num: 1211,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(95),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr82"), ItemDef {
        id: ID::new("tr82"),
        name: "TR82".to_string(),
        num: 1212,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(20),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr83"), ItemDef {
        id: ID::new("tr83"),
        name: "TR83".to_string(),
        num: 1213,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(10),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr84"), ItemDef {
        id: ID::new("tr84"),
        name: "TR84".to_string(),
        num: 1214,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(80),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr85"), ItemDef {
        id: ID::new("tr85"),
        name: "TR85".to_string(),
        num: 1215,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(10),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr86"), ItemDef {
        id: ID::new("tr86"),
        name: "TR86".to_string(),
        num: 1216,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(90),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr87"), ItemDef {
        id: ID::new("tr87"),
        name: "TR87".to_string(),
        num: 1217,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(80),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr88"), ItemDef {
        id: ID::new("tr88"),
        name: "TR88".to_string(),
        num: 1218,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(10),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr89"), ItemDef {
        id: ID::new("tr89"),
        name: "TR89".to_string(),
        num: 1219,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(110),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr90"), ItemDef {
        id: ID::new("tr90"),
        name: "TR90".to_string(),
        num: 1220,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(90),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr91"), ItemDef {
        id: ID::new("tr91"),
        name: "TR91".to_string(),
        num: 1221,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(10),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr92"), ItemDef {
        id: ID::new("tr92"),
        name: "TR92".to_string(),
        num: 1222,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(80),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr93"), ItemDef {
        id: ID::new("tr93"),
        name: "TR93".to_string(),
        num: 1223,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(85),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr94"), ItemDef {
        id: ID::new("tr94"),
        name: "TR94".to_string(),
        num: 1224,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(95),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr95"), ItemDef {
        id: ID::new("tr95"),
        name: "TR95".to_string(),
        num: 1225,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(80),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr96"), ItemDef {
        id: ID::new("tr96"),
        name: "TR96".to_string(),
        num: 1226,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(90),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr97"), ItemDef {
        id: ID::new("tr97"),
        name: "TR97".to_string(),
        num: 1227,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(85),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr98"), ItemDef {
        id: ID::new("tr98"),
        name: "TR98".to_string(),
        num: 1228,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(85),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("tr99"), ItemDef {
        id: ID::new("tr99"),
        name: "TR99".to_string(),
        num: 1229,
        gen: 8,
        category: ItemCategory::TR,
        fling_power: Some(80),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("twistedspoon"), ItemDef {
        id: ID::new("twistedspoon"),
        name: "Twisted Spoon".to_string(),
        num: 248,
        gen: 2,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::BasePower, ItemEffect::BoostType { move_type: "Psychic".to_string(), multiplier: 1.2 }, 15),
        ],
        ..Default::default()
    });

    map.insert(ID::new("tyranitarite"), ItemDef {
        id: ID::new("tyranitarite"),
        name: "Tyranitarite".to_string(),
        num: 669,
        gen: 6,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Past".to_string()),
        mega_stone: Some("Tyranitar-Mega".to_string()),
        mega_evolves: Some("Tyranitar".to_string()),
        item_user: Some(vec!["Tyranitar".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("ultraball"), ItemDef {
        id: ID::new("ultraball"),
        name: "Ultra Ball".to_string(),
        num: 2,
        gen: 1,
        category: ItemCategory::Pokeball,
        is_pokeball: true,
        ..Default::default()
    });

    map.insert(ID::new("ultranecroziumz"), ItemDef {
        id: ID::new("ultranecroziumz"),
        name: "Ultranecrozium Z".to_string(),
        num: 923,
        gen: 7,
        category: ItemCategory::ZCrystal,
        is_nonstandard: Some("Past".to_string()),
        item_user: Some(vec!["Necrozma-Ultra".to_string()]),
        z_move: Some("Light That Burns the Sky".to_string()),
        z_move_from: Some("Photon Geyser".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("unremarkableteacup"), ItemDef {
        id: ID::new("unremarkableteacup"),
        name: "Unremarkable Teacup".to_string(),
        num: 2403,
        gen: 9,
        category: ItemCategory::HeldItem,
        fling_power: Some(80),
        ..Default::default()
    });

    map.insert(ID::new("upgrade"), ItemDef {
        id: ID::new("upgrade"),
        name: "Up-Grade".to_string(),
        num: 252,
        gen: 2,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        ..Default::default()
    });

    map.insert(ID::new("utilityumbrella"), ItemDef {
        id: ID::new("utilityumbrella"),
        name: "Utility Umbrella".to_string(),
        num: 1123,
        gen: 8,
        category: ItemCategory::HeldItem,
        fling_power: Some(60),
        ..Default::default()
    });

    map.insert(ID::new("venusaurite"), ItemDef {
        id: ID::new("venusaurite"),
        name: "Venusaurite".to_string(),
        num: 659,
        gen: 6,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Past".to_string()),
        mega_stone: Some("Venusaur-Mega".to_string()),
        mega_evolves: Some("Venusaur".to_string()),
        item_user: Some(vec!["Venusaur".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("victreebelite"), ItemDef {
        id: ID::new("victreebelite"),
        name: "Victreebelite".to_string(),
        num: 2560,
        gen: 9,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Future".to_string()),
        mega_stone: Some("Victreebel-Mega".to_string()),
        mega_evolves: Some("Victreebel".to_string()),
        item_user: Some(vec!["Victreebel".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("vilevial"), ItemDef {
        id: ID::new("vilevial"),
        name: "Vile Vial".to_string(),
        num: -2,
        gen: 8,
        category: ItemCategory::HeldItem,
        fling_power: Some(60),
        is_nonstandard: Some("CAP".to_string()),
        item_user: Some(vec!["Venomicon-Epilogue".to_string()]),
        forced_forme: Some("Venomicon-Epilogue".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("wacanberry"), ItemDef {
        id: ID::new("wacanberry"),
        name: "Wacan Berry".to_string(),
        num: 186,
        gen: 4,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(80),
        natural_gift_type: Some("Electric".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::SourceModifyDamage, ItemEffect::Compound { effects: vec![
                ItemEffect::ResistSuperEffective { resist_type: "Electric".to_string(), multiplier: 0.5 },
                ItemEffect::ConsumeItem,
            ] }, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("watergem"), ItemDef {
        id: ID::new("watergem"),
        name: "Water Gem".to_string(),
        num: 549,
        gen: 5,
        category: ItemCategory::Gem,
        is_gem: true,
        is_consumable: true,
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("wateriumz"), ItemDef {
        id: ID::new("wateriumz"),
        name: "Waterium Z".to_string(),
        num: 778,
        gen: 7,
        category: ItemCategory::ZCrystal,
        is_nonstandard: Some("Past".to_string()),
        z_move_type: Some("Water".to_string()),
        forced_forme: Some("Arceus-Water".to_string()),
        on_plate: Some("Water".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("watermemory"), ItemDef {
        id: ID::new("watermemory"),
        name: "Water Memory".to_string(),
        num: 913,
        gen: 7,
        category: ItemCategory::Memory,
        is_nonstandard: Some("Past".to_string()),
        item_user: Some(vec!["Silvally-Water".to_string()]),
        forced_forme: Some("Silvally-Water".to_string()),
        on_memory: Some("Water".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("waterstone"), ItemDef {
        id: ID::new("waterstone"),
        name: "Water Stone".to_string(),
        num: 84,
        gen: 1,
        category: ItemCategory::HeldItem,
        fling_power: Some(30),
        ..Default::default()
    });

    map.insert(ID::new("watmelberry"), ItemDef {
        id: ID::new("watmelberry"),
        name: "Watmel Berry".to_string(),
        num: 181,
        gen: 3,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        is_nonstandard: Some("Past".to_string()),
        natural_gift_power: Some(100),
        natural_gift_type: Some("Fire".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("waveincense"), ItemDef {
        id: ID::new("waveincense"),
        name: "Wave Incense".to_string(),
        num: 317,
        gen: 4,
        category: ItemCategory::HeldItem,
        fling_power: Some(10),
        is_nonstandard: Some("Past".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::BasePower, ItemEffect::BoostType { move_type: "Water".to_string(), multiplier: 1.2 }, 15),
        ],
        ..Default::default()
    });

    map.insert(ID::new("weaknesspolicy"), ItemDef {
        id: ID::new("weaknesspolicy"),
        name: "Weakness Policy".to_string(),
        num: 639,
        gen: 6,
        category: ItemCategory::HeldItem,
        fling_power: Some(80),
        boosts: Some(boosts(&[("atk", 2), ("spa", 2)])),
        ..Default::default()
    });

    map.insert(ID::new("wellspringmask"), ItemDef {
        id: ID::new("wellspringmask"),
        name: "Wellspring Mask".to_string(),
        num: 2407,
        gen: 9,
        category: ItemCategory::HeldItem,
        fling_power: Some(60),
        item_user: Some(vec!["Ogerpon-Wellspring".to_string()]),
        forced_forme: Some("Ogerpon-Wellspring".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("wepearberry"), ItemDef {
        id: ID::new("wepearberry"),
        name: "Wepear Berry".to_string(),
        num: 167,
        gen: 3,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        is_nonstandard: Some("Past".to_string()),
        natural_gift_power: Some(90),
        natural_gift_type: Some("Electric".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("whippeddream"), ItemDef {
        id: ID::new("whippeddream"),
        name: "Whipped Dream".to_string(),
        num: 646,
        gen: 6,
        category: ItemCategory::HeldItem,
        fling_power: Some(80),
        is_nonstandard: Some("Past".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("whiteherb"), ItemDef {
        id: ID::new("whiteherb"),
        name: "White Herb".to_string(),
        num: 214,
        gen: 3,
        category: ItemCategory::HeldItem,
        fling_power: Some(10),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::AfterBoost, ItemEffect::Compound { effects: vec![ItemEffect::RestoreLoweredStats, ItemEffect::ConsumeItem] }, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("widelens"), ItemDef {
        id: ID::new("widelens"),
        name: "Wide Lens".to_string(),
        num: 265,
        gen: 4,
        category: ItemCategory::HeldItem,
        fling_power: Some(10),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::ModifyAccuracy, ItemEffect::BoostAccuracy { multiplier: 1.1 }, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("wikiberry"), ItemDef {
        id: ID::new("wikiberry"),
        name: "Wiki Berry".to_string(),
        num: 160,
        gen: 3,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(80),
        natural_gift_type: Some("Rock".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::Update, ItemEffect::Triggered {
            trigger: Trigger::OnHPBelowGluttony(0.25),
            effect: Box::new(ItemEffect::Compound { effects: vec![
                ItemEffect::HealOnThreshold { threshold: 0.25, heal_fraction: 0.333 },
                ItemEffect::ConsumeItem,
            ] }),
        }, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("wiseglasses"), ItemDef {
        id: ID::new("wiseglasses"),
        name: "Wise Glasses".to_string(),
        num: 267,
        gen: 4,
        category: ItemCategory::HeldItem,
        fling_power: Some(10),
        ..Default::default()
    });

    map.insert(ID::new("yacheberry"), ItemDef {
        id: ID::new("yacheberry"),
        name: "Yache Berry".to_string(),
        num: 188,
        gen: 4,
        category: ItemCategory::Berry,
        is_berry: true,
        is_consumable: true,
        natural_gift_power: Some(80),
        natural_gift_type: Some("Ice".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::SourceModifyDamage, ItemEffect::Compound { effects: vec![
                ItemEffect::ResistSuperEffective { resist_type: "Ice".to_string(), multiplier: 0.5 },
                ItemEffect::ConsumeItem,
            ] }, 0),
        ],
        ..Default::default()
    });

    map.insert(ID::new("zapplate"), ItemDef {
        id: ID::new("zapplate"),
        name: "Zap Plate".to_string(),
        num: 300,
        gen: 4,
        category: ItemCategory::Plate,
        forced_forme: Some("Arceus-Electric".to_string()),
        on_plate: Some("Electric".to_string()),
        effects: vec![
            ItemEffectEntry::with_priority(EventType::BasePower, ItemEffect::BoostType { move_type: "Electric".to_string(), multiplier: 1.2 }, 15),
        ],
        ..Default::default()
    });

    map.insert(ID::new("zeraorite"), ItemDef {
        id: ID::new("zeraorite"),
        name: "Zeraorite".to_string(),
        num: 2598,
        gen: 9,
        category: ItemCategory::Mega,
        is_nonstandard: Some("Future".to_string()),
        mega_stone: Some("Zeraora-Mega".to_string()),
        mega_evolves: Some("Zeraora".to_string()),
        item_user: Some(vec!["Zeraora".to_string()]),
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

