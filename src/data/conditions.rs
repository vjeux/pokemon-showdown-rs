//! Data-driven Condition Definitions
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This module defines conditions (status effects, volatiles, side conditions,
//! weather, terrain) as data structures following the JS architecture.

use std::collections::HashMap;
use once_cell::sync::Lazy;
use crate::dex_data::ID;

/// Type of condition
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConditionType {
    /// Non-volatile status (burn, paralysis, poison, sleep, freeze)
    Status,
    /// Volatile status (confusion, taunt, encore, etc.)
    Volatile,
    /// Side condition (stealth rock, spikes, reflect, etc.)
    SideCondition,
    /// Slot condition (Wish, Healing Wish)
    SlotCondition,
    /// Weather (rain, sun, sand, hail)
    Weather,
    /// Terrain (electric, grassy, psychic, misty)
    Terrain,
    /// Pseudo-weather (Trick Room, Magic Room, etc.)
    PseudoWeather,
}

impl Default for ConditionType {
    fn default() -> Self {
        ConditionType::Volatile
    }
}

/// Condition definition
#[derive(Debug, Clone)]
pub struct ConditionDef {
    /// Unique ID
    pub id: ID,
    /// Display name
    pub name: String,
    /// Condition type
    pub condition_type: ConditionType,
    /// Duration in turns (None = indefinite or until cured)
    pub duration: Option<u32>,
    /// Maximum duration (for variable durations like sleep)
    pub max_duration: Option<u32>,
    /// Minimum duration
    pub min_duration: Option<u32>,
    /// Can be passed by Baton Pass
    pub baton_passable: bool,
    /// Prevents switching
    pub traps: bool,
    /// Maximum layers (for stacking conditions like Spikes)
    pub max_layers: Option<u8>,

    // === Status effects ===
    /// Residual damage per turn (fraction of max HP)
    pub residual_damage: Option<f64>,
    /// Residual damage increases each turn (Toxic)
    pub escalating_damage: bool,
    /// Speed reduction (paralysis = 0.5)
    pub speed_modifier: Option<f64>,
    /// Attack reduction (burn = 0.5 for physical)
    pub attack_modifier: Option<f64>,
    /// Chance to skip turn (paralysis = 25, sleep = 100 until wake)
    pub skip_turn_chance: Option<u8>,
    /// Prevents certain moves (Taunt prevents status moves)
    pub move_restriction: Option<MoveRestriction>,

    // === Volatile effects ===
    /// Damage on each turn (trapped, seeded, etc.)
    pub volatile_damage: Option<f64>,
    /// Heal on each turn (Aqua Ring, Ingrain)
    pub volatile_heal: Option<f64>,
    /// Confusion self-hit chance
    pub confusion_chance: Option<u8>,
    /// Stat boosts while active
    pub stat_boosts: Option<Vec<(String, i8)>>,
    /// Protection from all moves this turn
    pub protection: bool,

    // === Side conditions ===
    /// Entry hazard damage type
    pub hazard_type: Option<String>,
    /// Entry hazard damage (fraction based on type effectiveness for SR)
    pub hazard_damage: Option<f64>,
    /// Screen reduction (Reflect, Light Screen, Aurora Veil)
    pub screen_reduction: Option<f64>,
    /// Applies status on entry (Toxic Spikes)
    pub entry_status: Option<String>,
    /// Speed reduction on entry (Sticky Web)
    pub entry_speed_drop: Option<i8>,

    // === Weather effects ===
    /// Boosts type damage by multiplier
    pub type_boost: Option<(String, f64)>,
    /// Weakens type damage by multiplier
    pub type_weaken: Option<(String, f64)>,
    /// Residual damage to non-immune types
    pub weather_damage: Option<f64>,
    /// Types immune to weather damage
    pub immune_types: Option<Vec<String>>,
    /// Abilities immune to weather damage
    pub immune_abilities: Option<Vec<String>>,

    // === Terrain effects ===
    /// Grounded Pokemon only
    pub grounded_only: bool,
    /// Prevents status
    pub prevents_status: Option<Vec<String>>,
    /// Blocks priority moves against grounded Pokemon
    pub blocks_priority: bool,
}

/// Move restriction type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MoveRestriction {
    /// Taunt: No status moves
    NoStatusMoves,
    /// Encore: Must repeat last move
    EncoreMove(ID),
    /// Disable: Cannot use specific move
    DisabledMove(ID),
    /// Torment: Cannot use same move twice
    NoRepeat,
    /// Heal Block: No healing moves
    NoHealing,
    /// Imprison: Cannot use moves known by imprisoner
    Imprison,
    /// Choice lock: Must use same move
    ChoiceLock(ID),
}

impl Default for ConditionDef {
    fn default() -> Self {
        Self {
            id: ID::empty(),
            name: String::new(),
            condition_type: ConditionType::Volatile,
            duration: None,
            max_duration: None,
            min_duration: None,
            baton_passable: false,
            traps: false,
            max_layers: None,
            residual_damage: None,
            escalating_damage: false,
            speed_modifier: None,
            attack_modifier: None,
            skip_turn_chance: None,
            move_restriction: None,
            volatile_damage: None,
            volatile_heal: None,
            confusion_chance: None,
            stat_boosts: None,
            protection: false,
            hazard_type: None,
            hazard_damage: None,
            screen_reduction: None,
            entry_status: None,
            entry_speed_drop: None,
            type_boost: None,
            type_weaken: None,
            weather_damage: None,
            immune_types: None,
            immune_abilities: None,
            grounded_only: false,
            prevents_status: None,
            blocks_priority: false,
        }
    }
}

impl ConditionDef {
    pub fn new(id: &str, name: &str, condition_type: ConditionType) -> Self {
        Self {
            id: ID::new(id),
            name: name.to_string(),
            condition_type,
            ..Default::default()
        }
    }
}

/// Static registry of all conditions
pub static CONDITIONS: Lazy<HashMap<ID, ConditionDef>> = Lazy::new(|| {
    let mut map = HashMap::new();

    // ==========================================
    // STATUS CONDITIONS (non-volatile)
    // ==========================================

    map.insert(ID::new("brn"), ConditionDef {
        id: ID::new("brn"),
        name: "Burn".to_string(),
        condition_type: ConditionType::Status,
        residual_damage: Some(1.0 / 16.0),
        attack_modifier: Some(0.5), // Physical moves deal half damage
        ..Default::default()
    });

    map.insert(ID::new("par"), ConditionDef {
        id: ID::new("par"),
        name: "Paralysis".to_string(),
        condition_type: ConditionType::Status,
        speed_modifier: Some(0.5),
        skip_turn_chance: Some(25),
        ..Default::default()
    });

    map.insert(ID::new("psn"), ConditionDef {
        id: ID::new("psn"),
        name: "Poison".to_string(),
        condition_type: ConditionType::Status,
        residual_damage: Some(1.0 / 8.0),
        ..Default::default()
    });

    map.insert(ID::new("tox"), ConditionDef {
        id: ID::new("tox"),
        name: "Toxic".to_string(),
        condition_type: ConditionType::Status,
        residual_damage: Some(1.0 / 16.0), // Starts at 1/16, increases
        escalating_damage: true,
        ..Default::default()
    });

    map.insert(ID::new("slp"), ConditionDef {
        id: ID::new("slp"),
        name: "Sleep".to_string(),
        condition_type: ConditionType::Status,
        min_duration: Some(1),
        max_duration: Some(3),
        skip_turn_chance: Some(100), // Can't move while asleep
        ..Default::default()
    });

    map.insert(ID::new("frz"), ConditionDef {
        id: ID::new("frz"),
        name: "Freeze".to_string(),
        condition_type: ConditionType::Status,
        skip_turn_chance: Some(100), // Can't move while frozen
        // 20% chance to thaw each turn - handled in battle.rs
        ..Default::default()
    });

    // ==========================================
    // VOLATILE CONDITIONS
    // ==========================================

    map.insert(ID::new("confusion"), ConditionDef {
        id: ID::new("confusion"),
        name: "Confusion".to_string(),
        condition_type: ConditionType::Volatile,
        min_duration: Some(2),
        max_duration: Some(5),
        confusion_chance: Some(33), // 33% chance to hit self
        baton_passable: false,
        ..Default::default()
    });

    map.insert(ID::new("flinch"), ConditionDef {
        id: ID::new("flinch"),
        name: "Flinch".to_string(),
        condition_type: ConditionType::Volatile,
        duration: Some(1),
        skip_turn_chance: Some(100),
        ..Default::default()
    });

    map.insert(ID::new("taunt"), ConditionDef {
        id: ID::new("taunt"),
        name: "Taunt".to_string(),
        condition_type: ConditionType::Volatile,
        duration: Some(3),
        move_restriction: Some(MoveRestriction::NoStatusMoves),
        ..Default::default()
    });

    map.insert(ID::new("torment"), ConditionDef {
        id: ID::new("torment"),
        name: "Torment".to_string(),
        condition_type: ConditionType::Volatile,
        move_restriction: Some(MoveRestriction::NoRepeat),
        ..Default::default()
    });

    map.insert(ID::new("healblock"), ConditionDef {
        id: ID::new("healblock"),
        name: "Heal Block".to_string(),
        condition_type: ConditionType::Volatile,
        duration: Some(5),
        move_restriction: Some(MoveRestriction::NoHealing),
        ..Default::default()
    });

    map.insert(ID::new("substitute"), ConditionDef {
        id: ID::new("substitute"),
        name: "Substitute".to_string(),
        condition_type: ConditionType::Volatile,
        baton_passable: true,
        // HP stored separately in volatile state
        ..Default::default()
    });

    map.insert(ID::new("protect"), ConditionDef {
        id: ID::new("protect"),
        name: "Protect".to_string(),
        condition_type: ConditionType::Volatile,
        duration: Some(1),
        protection: true,
        ..Default::default()
    });

    map.insert(ID::new("leechseed"), ConditionDef {
        id: ID::new("leechseed"),
        name: "Leech Seed".to_string(),
        condition_type: ConditionType::Volatile,
        volatile_damage: Some(1.0 / 8.0),
        // Heals the seeder - handled in battle.rs
        ..Default::default()
    });

    map.insert(ID::new("aquaring"), ConditionDef {
        id: ID::new("aquaring"),
        name: "Aqua Ring".to_string(),
        condition_type: ConditionType::Volatile,
        volatile_heal: Some(1.0 / 16.0),
        baton_passable: true,
        ..Default::default()
    });

    map.insert(ID::new("ingrain"), ConditionDef {
        id: ID::new("ingrain"),
        name: "Ingrain".to_string(),
        condition_type: ConditionType::Volatile,
        volatile_heal: Some(1.0 / 16.0),
        traps: true, // Can't switch
        baton_passable: true,
        ..Default::default()
    });

    map.insert(ID::new("trapped"), ConditionDef {
        id: ID::new("trapped"),
        name: "Trapped".to_string(),
        condition_type: ConditionType::Volatile,
        traps: true,
        ..Default::default()
    });

    map.insert(ID::new("partiallytrapped"), ConditionDef {
        id: ID::new("partiallytrapped"),
        name: "Partially Trapped".to_string(),
        condition_type: ConditionType::Volatile,
        min_duration: Some(4),
        max_duration: Some(5),
        traps: true,
        volatile_damage: Some(1.0 / 8.0), // 1/8 per turn (Bind, Wrap, etc.)
        ..Default::default()
    });

    map.insert(ID::new("curse"), ConditionDef {
        id: ID::new("curse"),
        name: "Curse".to_string(),
        condition_type: ConditionType::Volatile,
        volatile_damage: Some(0.25),
        ..Default::default()
    });

    map.insert(ID::new("nightmare"), ConditionDef {
        id: ID::new("nightmare"),
        name: "Nightmare".to_string(),
        condition_type: ConditionType::Volatile,
        volatile_damage: Some(0.25),
        // Only works while asleep - handled in battle.rs
        ..Default::default()
    });

    map.insert(ID::new("perishsong"), ConditionDef {
        id: ID::new("perishsong"),
        name: "Perish Song".to_string(),
        condition_type: ConditionType::Volatile,
        duration: Some(4), // Faints when counter reaches 0
        baton_passable: false,
        ..Default::default()
    });

    map.insert(ID::new("attract"), ConditionDef {
        id: ID::new("attract"),
        name: "Infatuation".to_string(),
        condition_type: ConditionType::Volatile,
        skip_turn_chance: Some(50),
        ..Default::default()
    });

    map.insert(ID::new("yawn"), ConditionDef {
        id: ID::new("yawn"),
        name: "Yawn".to_string(),
        condition_type: ConditionType::Volatile,
        duration: Some(2), // Falls asleep at end of next turn
        ..Default::default()
    });

    // Pivot move marker
    map.insert(ID::new("pivotswitch"), ConditionDef {
        id: ID::new("pivotswitch"),
        name: "Pivot Switch".to_string(),
        condition_type: ConditionType::Volatile,
        duration: Some(1),
        ..Default::default()
    });

    // ==========================================
    // SIDE CONDITIONS
    // ==========================================

    map.insert(ID::new("stealthrock"), ConditionDef {
        id: ID::new("stealthrock"),
        name: "Stealth Rock".to_string(),
        condition_type: ConditionType::SideCondition,
        hazard_type: Some("Rock".to_string()),
        // Damage based on type effectiveness: 1/8 neutral, scales with weakness/resistance
        ..Default::default()
    });

    map.insert(ID::new("spikes"), ConditionDef {
        id: ID::new("spikes"),
        name: "Spikes".to_string(),
        condition_type: ConditionType::SideCondition,
        max_layers: Some(3),
        // 1/8, 1/6, 1/4 damage based on layers
        ..Default::default()
    });

    map.insert(ID::new("toxicspikes"), ConditionDef {
        id: ID::new("toxicspikes"),
        name: "Toxic Spikes".to_string(),
        condition_type: ConditionType::SideCondition,
        max_layers: Some(2),
        entry_status: Some("psn".to_string()), // 1 layer = poison, 2 = toxic
        ..Default::default()
    });

    map.insert(ID::new("stickyweb"), ConditionDef {
        id: ID::new("stickyweb"),
        name: "Sticky Web".to_string(),
        condition_type: ConditionType::SideCondition,
        entry_speed_drop: Some(-1),
        ..Default::default()
    });

    map.insert(ID::new("reflect"), ConditionDef {
        id: ID::new("reflect"),
        name: "Reflect".to_string(),
        condition_type: ConditionType::SideCondition,
        duration: Some(5),
        screen_reduction: Some(0.5), // Halves physical damage
        ..Default::default()
    });

    map.insert(ID::new("lightscreen"), ConditionDef {
        id: ID::new("lightscreen"),
        name: "Light Screen".to_string(),
        condition_type: ConditionType::SideCondition,
        duration: Some(5),
        screen_reduction: Some(0.5), // Halves special damage
        ..Default::default()
    });

    map.insert(ID::new("auroraveil"), ConditionDef {
        id: ID::new("auroraveil"),
        name: "Aurora Veil".to_string(),
        condition_type: ConditionType::SideCondition,
        duration: Some(5),
        screen_reduction: Some(0.5), // Halves all damage
        // Requires hail/snow - handled in battle.rs
        ..Default::default()
    });

    map.insert(ID::new("tailwind"), ConditionDef {
        id: ID::new("tailwind"),
        name: "Tailwind".to_string(),
        condition_type: ConditionType::SideCondition,
        duration: Some(4),
        // 2x speed - handled in battle.rs
        ..Default::default()
    });

    // ==========================================
    // WEATHER
    // ==========================================

    map.insert(ID::new("raindance"), ConditionDef {
        id: ID::new("raindance"),
        name: "Rain".to_string(),
        condition_type: ConditionType::Weather,
        duration: Some(5),
        type_boost: Some(("Water".to_string(), 1.5)),
        type_weaken: Some(("Fire".to_string(), 0.5)),
        ..Default::default()
    });

    map.insert(ID::new("sunnyday"), ConditionDef {
        id: ID::new("sunnyday"),
        name: "Sun".to_string(),
        condition_type: ConditionType::Weather,
        duration: Some(5),
        type_boost: Some(("Fire".to_string(), 1.5)),
        type_weaken: Some(("Water".to_string(), 0.5)),
        ..Default::default()
    });

    map.insert(ID::new("sandstorm"), ConditionDef {
        id: ID::new("sandstorm"),
        name: "Sandstorm".to_string(),
        condition_type: ConditionType::Weather,
        duration: Some(5),
        weather_damage: Some(1.0 / 16.0),
        immune_types: Some(vec!["Rock".to_string(), "Ground".to_string(), "Steel".to_string()]),
        immune_abilities: Some(vec!["sandveil".to_string(), "sandrush".to_string(), "sandforce".to_string(), "magicguard".to_string(), "overcoat".to_string()]),
        // Also boosts Rock SpD by 1.5x - handled in battle.rs
        ..Default::default()
    });

    map.insert(ID::new("hail"), ConditionDef {
        id: ID::new("hail"),
        name: "Hail".to_string(),
        condition_type: ConditionType::Weather,
        duration: Some(5),
        weather_damage: Some(1.0 / 16.0),
        immune_types: Some(vec!["Ice".to_string()]),
        immune_abilities: Some(vec!["icebody".to_string(), "snowcloak".to_string(), "magicguard".to_string(), "overcoat".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("snow"), ConditionDef {
        id: ID::new("snow"),
        name: "Snow".to_string(),
        condition_type: ConditionType::Weather,
        duration: Some(5),
        // Gen 9: No damage, but boosts Ice Def by 1.5x
        ..Default::default()
    });

    // Primal weathers (permanent, blocks other weather)
    map.insert(ID::new("primordialsea"), ConditionDef {
        id: ID::new("primordialsea"),
        name: "Primordial Sea".to_string(),
        condition_type: ConditionType::Weather,
        // Permanent, blocks Fire moves entirely
        type_boost: Some(("Water".to_string(), 1.5)),
        ..Default::default()
    });

    map.insert(ID::new("desolateland"), ConditionDef {
        id: ID::new("desolateland"),
        name: "Desolate Land".to_string(),
        condition_type: ConditionType::Weather,
        // Permanent, blocks Water moves entirely
        type_boost: Some(("Fire".to_string(), 1.5)),
        ..Default::default()
    });

    // ==========================================
    // TERRAIN
    // ==========================================

    map.insert(ID::new("electricterrain"), ConditionDef {
        id: ID::new("electricterrain"),
        name: "Electric Terrain".to_string(),
        condition_type: ConditionType::Terrain,
        duration: Some(5),
        grounded_only: true,
        type_boost: Some(("Electric".to_string(), 1.3)),
        prevents_status: Some(vec!["slp".to_string()]),
        ..Default::default()
    });

    map.insert(ID::new("grassyterrain"), ConditionDef {
        id: ID::new("grassyterrain"),
        name: "Grassy Terrain".to_string(),
        condition_type: ConditionType::Terrain,
        duration: Some(5),
        grounded_only: true,
        type_boost: Some(("Grass".to_string(), 1.3)),
        volatile_heal: Some(1.0 / 16.0), // Heals grounded Pokemon
        // Also weakens Earthquake/Bulldoze/Magnitude - handled in battle.rs
        ..Default::default()
    });

    map.insert(ID::new("psychicterrain"), ConditionDef {
        id: ID::new("psychicterrain"),
        name: "Psychic Terrain".to_string(),
        condition_type: ConditionType::Terrain,
        duration: Some(5),
        grounded_only: true,
        type_boost: Some(("Psychic".to_string(), 1.3)),
        blocks_priority: true,
        ..Default::default()
    });

    map.insert(ID::new("mistyterrain"), ConditionDef {
        id: ID::new("mistyterrain"),
        name: "Misty Terrain".to_string(),
        condition_type: ConditionType::Terrain,
        duration: Some(5),
        grounded_only: true,
        prevents_status: Some(vec!["brn".to_string(), "par".to_string(), "psn".to_string(), "tox".to_string(), "slp".to_string(), "frz".to_string()]),
        // Also halves Dragon damage - handled in battle.rs
        ..Default::default()
    });

    // ==========================================
    // PSEUDO-WEATHER
    // ==========================================

    map.insert(ID::new("trickroom"), ConditionDef {
        id: ID::new("trickroom"),
        name: "Trick Room".to_string(),
        condition_type: ConditionType::PseudoWeather,
        duration: Some(5),
        // Reverses speed order - handled in battle.rs
        ..Default::default()
    });

    map.insert(ID::new("magicroom"), ConditionDef {
        id: ID::new("magicroom"),
        name: "Magic Room".to_string(),
        condition_type: ConditionType::PseudoWeather,
        duration: Some(5),
        // Suppresses held items - handled in battle.rs
        ..Default::default()
    });

    map.insert(ID::new("wonderroom"), ConditionDef {
        id: ID::new("wonderroom"),
        name: "Wonder Room".to_string(),
        condition_type: ConditionType::PseudoWeather,
        duration: Some(5),
        // Swaps Def and SpD - handled in battle.rs
        ..Default::default()
    });

    map.insert(ID::new("gravity"), ConditionDef {
        id: ID::new("gravity"),
        name: "Gravity".to_string(),
        condition_type: ConditionType::PseudoWeather,
        duration: Some(5),
        // Grounds all Pokemon, prevents fly/bounce/etc - handled in battle.rs
        ..Default::default()
    });

    map
});

/// Get condition definition by ID
pub fn get_condition(id: &ID) -> Option<&'static ConditionDef> {
    CONDITIONS.get(id)
}

/// Check if a condition is a status (non-volatile)
pub fn is_status_condition(id: &ID) -> bool {
    get_condition(id).map_or(false, |c| c.condition_type == ConditionType::Status)
}

/// Check if a condition is a volatile
pub fn is_volatile_condition(id: &ID) -> bool {
    get_condition(id).map_or(false, |c| c.condition_type == ConditionType::Volatile)
}

/// Check if a condition traps the Pokemon
pub fn condition_traps(id: &ID) -> bool {
    get_condition(id).map_or(false, |c| c.traps)
}

/// Get residual damage for a condition
pub fn get_condition_damage(id: &ID) -> Option<f64> {
    get_condition(id).and_then(|c| c.residual_damage.or(c.volatile_damage))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_burn() {
        let burn = get_condition(&ID::new("brn")).unwrap();
        assert_eq!(burn.condition_type, ConditionType::Status);
        assert_eq!(burn.residual_damage, Some(1.0 / 16.0));
        assert_eq!(burn.attack_modifier, Some(0.5));
    }

    #[test]
    fn test_paralysis() {
        let par = get_condition(&ID::new("par")).unwrap();
        assert_eq!(par.speed_modifier, Some(0.5));
        assert_eq!(par.skip_turn_chance, Some(25));
    }

    #[test]
    fn test_toxic() {
        let tox = get_condition(&ID::new("tox")).unwrap();
        assert!(tox.escalating_damage);
    }

    #[test]
    fn test_confusion() {
        let confusion = get_condition(&ID::new("confusion")).unwrap();
        assert_eq!(confusion.condition_type, ConditionType::Volatile);
        assert_eq!(confusion.confusion_chance, Some(33));
    }

    #[test]
    fn test_stealth_rock() {
        let sr = get_condition(&ID::new("stealthrock")).unwrap();
        assert_eq!(sr.condition_type, ConditionType::SideCondition);
        assert_eq!(sr.hazard_type, Some("Rock".to_string()));
    }

    #[test]
    fn test_rain() {
        let rain = get_condition(&ID::new("raindance")).unwrap();
        assert_eq!(rain.condition_type, ConditionType::Weather);
        assert_eq!(rain.type_boost, Some(("Water".to_string(), 1.5)));
        assert_eq!(rain.type_weaken, Some(("Fire".to_string(), 0.5)));
    }

    #[test]
    fn test_electric_terrain() {
        let et = get_condition(&ID::new("electricterrain")).unwrap();
        assert_eq!(et.condition_type, ConditionType::Terrain);
        assert!(et.grounded_only);
        assert_eq!(et.prevents_status, Some(vec!["slp".to_string()]));
    }

    #[test]
    fn test_trick_room() {
        let tr = get_condition(&ID::new("trickroom")).unwrap();
        assert_eq!(tr.condition_type, ConditionType::PseudoWeather);
        assert_eq!(tr.duration, Some(5));
    }

    #[test]
    fn test_trapping() {
        assert!(condition_traps(&ID::new("ingrain")));
        assert!(condition_traps(&ID::new("partiallytrapped")));
        assert!(!condition_traps(&ID::new("confusion")));
    }
}
