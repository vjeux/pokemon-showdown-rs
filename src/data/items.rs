//! Data-driven Item Definitions
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This module defines items as data structures with their properties,
//! following the pattern from data/items.ts in the JS codebase.

use std::collections::HashMap;
use once_cell::sync::Lazy;
use crate::dex_data::ID;

/// Item category for organization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    HeldItem,
}

impl Default for ItemCategory {
    fn default() -> Self {
        ItemCategory::HeldItem
    }
}

/// Item definition - data-driven item with all properties
#[derive(Debug, Clone)]
pub struct ItemDef {
    /// Unique ID
    pub id: ID,
    /// Display name
    pub name: String,
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

    // === Stat modifiers ===
    /// Boost damage of specific type by multiplier
    pub type_boost: Option<(String, f64)>,
    /// Boost a specific stat by multiplier
    pub stat_boost: Option<(String, f64)>,
    /// Choice item stat boost (1.5x but locks move)
    pub choice_boost: Option<String>,

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

    // === Mega/Z/Special ===
    /// Mega stone for species
    pub mega_stone: Option<String>,
    /// Z-Crystal move type
    pub z_move_type: Option<String>,
    /// Z-Crystal specific move
    pub z_move_for: Option<String>,

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

impl Default for ItemDef {
    fn default() -> Self {
        Self {
            id: ID::empty(),
            name: String::new(),
            category: ItemCategory::HeldItem,
            fling_power: None,
            is_consumable: false,
            is_berry: false,
            is_choice: false,
            is_gem: false,
            type_boost: None,
            stat_boost: None,
            choice_boost: None,
            damage_multiplier: None,
            recoil_on_attack: None,
            residual_heal: None,
            residual_damage: None,
            heal_on_low_hp: None,
            status_immunity: None,
            status_cure: None,
            cures_all_status: false,
            required_type: None,
            type_immunity: None,
            grounds_holder: false,
            speed_multiplier: None,
            priority_boost: None,
            quick_claw_chance: None,
            crit_boost: None,
            focus_energy: false,
            prevents_flinch: false,
            loaded_dice: false,
            resist_type: None,
            focus_sash: false,
            sturdy: false,
            mega_stone: None,
            z_move_type: None,
            z_move_for: None,
            contact_damage: None,
            mental_herb: false,
            white_herb: false,
            power_herb: false,
            eject_button: false,
            red_card: false,
            trapping: false,
            escape_trapping: false,
        }
    }
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

    // === Healing Items ===
    map.insert(ID::new("leftovers"), ItemDef {
        id: ID::new("leftovers"),
        name: "Leftovers".to_string(),
        fling_power: Some(10),
        residual_heal: Some(1.0 / 16.0),
        ..Default::default()
    });

    map.insert(ID::new("blacksludge"), ItemDef {
        id: ID::new("blacksludge"),
        name: "Black Sludge".to_string(),
        fling_power: Some(30),
        residual_heal: Some(1.0 / 16.0),
        residual_damage: Some(1.0 / 8.0), // Damage to non-Poison types
        required_type: Some("Poison".to_string()),
        ..Default::default()
    });

    // === Choice Items ===
    map.insert(ID::new("choiceband"), ItemDef {
        id: ID::new("choiceband"),
        name: "Choice Band".to_string(),
        fling_power: Some(10),
        is_choice: true,
        choice_boost: Some("atk".to_string()),
        stat_boost: Some(("atk".to_string(), 1.5)),
        ..Default::default()
    });

    map.insert(ID::new("choicespecs"), ItemDef {
        id: ID::new("choicespecs"),
        name: "Choice Specs".to_string(),
        fling_power: Some(10),
        is_choice: true,
        choice_boost: Some("spa".to_string()),
        stat_boost: Some(("spa".to_string(), 1.5)),
        ..Default::default()
    });

    map.insert(ID::new("choicescarf"), ItemDef {
        id: ID::new("choicescarf"),
        name: "Choice Scarf".to_string(),
        fling_power: Some(10),
        is_choice: true,
        choice_boost: Some("spe".to_string()),
        speed_multiplier: Some(1.5),
        ..Default::default()
    });

    // === Life Orb ===
    map.insert(ID::new("lifeorb"), ItemDef {
        id: ID::new("lifeorb"),
        name: "Life Orb".to_string(),
        fling_power: Some(30),
        damage_multiplier: Some(1.3),
        recoil_on_attack: Some(0.1),
        ..Default::default()
    });

    // === Type-boosting Items ===
    map.insert(ID::new("mysticwater"), ItemDef {
        id: ID::new("mysticwater"),
        name: "Mystic Water".to_string(),
        category: ItemCategory::TypeEnhancing,
        fling_power: Some(30),
        type_boost: Some(("Water".to_string(), 1.2)),
        ..Default::default()
    });

    map.insert(ID::new("charcoal"), ItemDef {
        id: ID::new("charcoal"),
        name: "Charcoal".to_string(),
        category: ItemCategory::TypeEnhancing,
        fling_power: Some(30),
        type_boost: Some(("Fire".to_string(), 1.2)),
        ..Default::default()
    });

    map.insert(ID::new("magnet"), ItemDef {
        id: ID::new("magnet"),
        name: "Magnet".to_string(),
        category: ItemCategory::TypeEnhancing,
        fling_power: Some(30),
        type_boost: Some(("Electric".to_string(), 1.2)),
        ..Default::default()
    });

    map.insert(ID::new("miracleseed"), ItemDef {
        id: ID::new("miracleseed"),
        name: "Miracle Seed".to_string(),
        category: ItemCategory::TypeEnhancing,
        fling_power: Some(30),
        type_boost: Some(("Grass".to_string(), 1.2)),
        ..Default::default()
    });

    map.insert(ID::new("nevermeltice"), ItemDef {
        id: ID::new("nevermeltice"),
        name: "Never-Melt Ice".to_string(),
        category: ItemCategory::TypeEnhancing,
        fling_power: Some(30),
        type_boost: Some(("Ice".to_string(), 1.2)),
        ..Default::default()
    });

    map.insert(ID::new("blackbelt"), ItemDef {
        id: ID::new("blackbelt"),
        name: "Black Belt".to_string(),
        category: ItemCategory::TypeEnhancing,
        fling_power: Some(30),
        type_boost: Some(("Fighting".to_string(), 1.2)),
        ..Default::default()
    });

    map.insert(ID::new("poisonbarb"), ItemDef {
        id: ID::new("poisonbarb"),
        name: "Poison Barb".to_string(),
        category: ItemCategory::TypeEnhancing,
        fling_power: Some(70),
        type_boost: Some(("Poison".to_string(), 1.2)),
        ..Default::default()
    });

    map.insert(ID::new("softsand"), ItemDef {
        id: ID::new("softsand"),
        name: "Soft Sand".to_string(),
        category: ItemCategory::TypeEnhancing,
        fling_power: Some(10),
        type_boost: Some(("Ground".to_string(), 1.2)),
        ..Default::default()
    });

    map.insert(ID::new("sharpbeak"), ItemDef {
        id: ID::new("sharpbeak"),
        name: "Sharp Beak".to_string(),
        category: ItemCategory::TypeEnhancing,
        fling_power: Some(50),
        type_boost: Some(("Flying".to_string(), 1.2)),
        ..Default::default()
    });

    map.insert(ID::new("twistedspoon"), ItemDef {
        id: ID::new("twistedspoon"),
        name: "Twisted Spoon".to_string(),
        category: ItemCategory::TypeEnhancing,
        fling_power: Some(30),
        type_boost: Some(("Psychic".to_string(), 1.2)),
        ..Default::default()
    });

    map.insert(ID::new("silverpowder"), ItemDef {
        id: ID::new("silverpowder"),
        name: "Silver Powder".to_string(),
        category: ItemCategory::TypeEnhancing,
        fling_power: Some(10),
        type_boost: Some(("Bug".to_string(), 1.2)),
        ..Default::default()
    });

    map.insert(ID::new("hardstone"), ItemDef {
        id: ID::new("hardstone"),
        name: "Hard Stone".to_string(),
        category: ItemCategory::TypeEnhancing,
        fling_power: Some(100),
        type_boost: Some(("Rock".to_string(), 1.2)),
        ..Default::default()
    });

    map.insert(ID::new("spelltag"), ItemDef {
        id: ID::new("spelltag"),
        name: "Spell Tag".to_string(),
        category: ItemCategory::TypeEnhancing,
        fling_power: Some(30),
        type_boost: Some(("Ghost".to_string(), 1.2)),
        ..Default::default()
    });

    map.insert(ID::new("dragonfang"), ItemDef {
        id: ID::new("dragonfang"),
        name: "Dragon Fang".to_string(),
        category: ItemCategory::TypeEnhancing,
        fling_power: Some(70),
        type_boost: Some(("Dragon".to_string(), 1.2)),
        ..Default::default()
    });

    map.insert(ID::new("blackglasses"), ItemDef {
        id: ID::new("blackglasses"),
        name: "Black Glasses".to_string(),
        category: ItemCategory::TypeEnhancing,
        fling_power: Some(30),
        type_boost: Some(("Dark".to_string(), 1.2)),
        ..Default::default()
    });

    map.insert(ID::new("metalcoat"), ItemDef {
        id: ID::new("metalcoat"),
        name: "Metal Coat".to_string(),
        category: ItemCategory::TypeEnhancing,
        fling_power: Some(30),
        type_boost: Some(("Steel".to_string(), 1.2)),
        ..Default::default()
    });

    map.insert(ID::new("silkscarf"), ItemDef {
        id: ID::new("silkscarf"),
        name: "Silk Scarf".to_string(),
        category: ItemCategory::TypeEnhancing,
        fling_power: Some(10),
        type_boost: Some(("Normal".to_string(), 1.2)),
        ..Default::default()
    });

    // === Berries ===
    map.insert(ID::new("sitrusberry"), ItemDef {
        id: ID::new("sitrusberry"),
        name: "Sitrus Berry".to_string(),
        category: ItemCategory::Berry,
        fling_power: Some(10),
        is_berry: true,
        is_consumable: true,
        heal_on_low_hp: Some((0.5, 0.25)), // Heal 25% when below 50%
        ..Default::default()
    });

    map.insert(ID::new("oranberry"), ItemDef {
        id: ID::new("oranberry"),
        name: "Oran Berry".to_string(),
        category: ItemCategory::Berry,
        fling_power: Some(10),
        is_berry: true,
        is_consumable: true,
        heal_on_low_hp: Some((0.5, 0.10)), // Fixed 10 HP, approximated
        ..Default::default()
    });

    // Pinch berries (33% heal at 25% HP)
    for (id, name) in [
        ("figyberry", "Figy Berry"),
        ("aguavberry", "Aguav Berry"),
        ("iapapaberry", "Iapapa Berry"),
        ("magoberry", "Mago Berry"),
        ("wikiberry", "Wiki Berry"),
    ] {
        map.insert(ID::new(id), ItemDef {
            id: ID::new(id),
            name: name.to_string(),
            category: ItemCategory::Berry,
            fling_power: Some(10),
            is_berry: true,
            is_consumable: true,
            heal_on_low_hp: Some((0.25, 1.0 / 3.0)),
            ..Default::default()
        });
    }

    // Status cure berries
    map.insert(ID::new("lumberry"), ItemDef {
        id: ID::new("lumberry"),
        name: "Lum Berry".to_string(),
        category: ItemCategory::Berry,
        fling_power: Some(10),
        is_berry: true,
        is_consumable: true,
        cures_all_status: true,
        ..Default::default()
    });

    map.insert(ID::new("cheriberry"), ItemDef {
        id: ID::new("cheriberry"),
        name: "Cheri Berry".to_string(),
        category: ItemCategory::Berry,
        fling_power: Some(10),
        is_berry: true,
        is_consumable: true,
        status_cure: Some(vec!["par".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("chestoberry"), ItemDef {
        id: ID::new("chestoberry"),
        name: "Chesto Berry".to_string(),
        category: ItemCategory::Berry,
        fling_power: Some(10),
        is_berry: true,
        is_consumable: true,
        status_cure: Some(vec!["slp".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("pechaberry"), ItemDef {
        id: ID::new("pechaberry"),
        name: "Pecha Berry".to_string(),
        category: ItemCategory::Berry,
        fling_power: Some(10),
        is_berry: true,
        is_consumable: true,
        status_cure: Some(vec!["psn".to_string(), "tox".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("rawstberry"), ItemDef {
        id: ID::new("rawstberry"),
        name: "Rawst Berry".to_string(),
        category: ItemCategory::Berry,
        fling_power: Some(10),
        is_berry: true,
        is_consumable: true,
        status_cure: Some(vec!["brn".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("aspearberry"), ItemDef {
        id: ID::new("aspearberry"),
        name: "Aspear Berry".to_string(),
        category: ItemCategory::Berry,
        fling_power: Some(10),
        is_berry: true,
        is_consumable: true,
        status_cure: Some(vec!["frz".to_string()]),
        ..Default::default()
    });

    // === Defensive Items ===
    map.insert(ID::new("focussash"), ItemDef {
        id: ID::new("focussash"),
        name: "Focus Sash".to_string(),
        fling_power: Some(10),
        is_consumable: true,
        focus_sash: true,
        ..Default::default()
    });

    map.insert(ID::new("rockyhelmet"), ItemDef {
        id: ID::new("rockyhelmet"),
        name: "Rocky Helmet".to_string(),
        fling_power: Some(60),
        contact_damage: Some(1.0 / 6.0),
        ..Default::default()
    });

    map.insert(ID::new("airballoon"), ItemDef {
        id: ID::new("airballoon"),
        name: "Air Balloon".to_string(),
        fling_power: Some(10),
        is_consumable: true,
        type_immunity: Some("Ground".to_string()),
        ..Default::default()
    });

    // === Speed Items ===
    map.insert(ID::new("ironball"), ItemDef {
        id: ID::new("ironball"),
        name: "Iron Ball".to_string(),
        fling_power: Some(130),
        speed_multiplier: Some(0.5),
        grounds_holder: true,
        ..Default::default()
    });

    map.insert(ID::new("quickclaw"), ItemDef {
        id: ID::new("quickclaw"),
        name: "Quick Claw".to_string(),
        fling_power: Some(80),
        quick_claw_chance: Some(20),
        ..Default::default()
    });

    // === Misc Items ===
    map.insert(ID::new("assaultvest"), ItemDef {
        id: ID::new("assaultvest"),
        name: "Assault Vest".to_string(),
        fling_power: Some(80),
        stat_boost: Some(("spd".to_string(), 1.5)),
        // Note: Prevents status moves - handled in battle.rs
        ..Default::default()
    });

    map.insert(ID::new("eviolite"), ItemDef {
        id: ID::new("eviolite"),
        name: "Eviolite".to_string(),
        fling_power: Some(40),
        // 1.5x Def and SpD for Pokemon that can evolve - handled in battle.rs
        ..Default::default()
    });

    map.insert(ID::new("heavydutyboots"), ItemDef {
        id: ID::new("heavydutyboots"),
        name: "Heavy-Duty Boots".to_string(),
        fling_power: Some(80),
        // Prevents hazard damage - handled in battle.rs
        ..Default::default()
    });

    map.insert(ID::new("shedshell"), ItemDef {
        id: ID::new("shedshell"),
        name: "Shed Shell".to_string(),
        fling_power: Some(10),
        escape_trapping: true,
        ..Default::default()
    });

    map.insert(ID::new("scopelens"), ItemDef {
        id: ID::new("scopelens"),
        name: "Scope Lens".to_string(),
        fling_power: Some(30),
        crit_boost: Some(1),
        ..Default::default()
    });

    map.insert(ID::new("razorclaw"), ItemDef {
        id: ID::new("razorclaw"),
        name: "Razor Claw".to_string(),
        fling_power: Some(80),
        crit_boost: Some(1),
        ..Default::default()
    });

    map.insert(ID::new("widelens"), ItemDef {
        id: ID::new("widelens"),
        name: "Wide Lens".to_string(),
        fling_power: Some(10),
        // 1.1x accuracy - handled in battle.rs
        ..Default::default()
    });

    map.insert(ID::new("expertbelt"), ItemDef {
        id: ID::new("expertbelt"),
        name: "Expert Belt".to_string(),
        fling_power: Some(10),
        // 1.2x damage on super effective - handled in battle.rs
        ..Default::default()
    });

    map.insert(ID::new("mentalherb"), ItemDef {
        id: ID::new("mentalherb"),
        name: "Mental Herb".to_string(),
        fling_power: Some(10),
        is_consumable: true,
        mental_herb: true,
        ..Default::default()
    });

    map.insert(ID::new("whiteherb"), ItemDef {
        id: ID::new("whiteherb"),
        name: "White Herb".to_string(),
        fling_power: Some(10),
        is_consumable: true,
        white_herb: true,
        ..Default::default()
    });

    map.insert(ID::new("powerherb"), ItemDef {
        id: ID::new("powerherb"),
        name: "Power Herb".to_string(),
        fling_power: Some(10),
        is_consumable: true,
        power_herb: true,
        ..Default::default()
    });

    map.insert(ID::new("ejectbutton"), ItemDef {
        id: ID::new("ejectbutton"),
        name: "Eject Button".to_string(),
        fling_power: Some(30),
        is_consumable: true,
        eject_button: true,
        ..Default::default()
    });

    map.insert(ID::new("redcard"), ItemDef {
        id: ID::new("redcard"),
        name: "Red Card".to_string(),
        fling_power: Some(10),
        is_consumable: true,
        red_card: true,
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
    fn test_leftovers() {
        let leftovers = ID::new("leftovers");
        assert_eq!(get_residual_heal(&leftovers), Some(1.0 / 16.0));
    }

    #[test]
    fn test_choice_items() {
        assert!(is_choice_item(&ID::new("choiceband")));
        assert!(is_choice_item(&ID::new("choicespecs")));
        assert!(is_choice_item(&ID::new("choicescarf")));
        assert!(!is_choice_item(&ID::new("leftovers")));
    }

    #[test]
    fn test_type_boosting_items() {
        let mystic_water = ID::new("mysticwater");
        assert_eq!(item_boosts_type(&mystic_water, "Water"), Some(1.2));
        assert_eq!(item_boosts_type(&mystic_water, "Fire"), None);
    }

    #[test]
    fn test_berries() {
        assert!(is_berry(&ID::new("sitrusberry")));
        assert!(is_berry(&ID::new("lumberry")));
        assert!(!is_berry(&ID::new("leftovers")));

        let sitrus = get_item(&ID::new("sitrusberry")).unwrap();
        assert_eq!(sitrus.heal_on_low_hp, Some((0.5, 0.25)));
    }

    #[test]
    fn test_air_balloon() {
        let balloon = ID::new("airballoon");
        assert!(item_grants_type_immunity(&balloon, "Ground"));
        assert!(!item_grants_type_immunity(&balloon, "Fire"));
    }

    #[test]
    fn test_life_orb() {
        let life_orb = get_item(&ID::new("lifeorb")).unwrap();
        assert_eq!(life_orb.damage_multiplier, Some(1.3));
        assert_eq!(life_orb.recoil_on_attack, Some(0.1));
    }
}
