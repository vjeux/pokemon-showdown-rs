//! Data-driven Move Definitions
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This module defines moves as data structures with their properties,
//! following the pattern from data/moves.ts in the JS codebase.

use std::collections::HashMap;
use once_cell::sync::Lazy;
use crate::dex_data::ID;
use crate::event::MoveFlags;

/// Move category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MoveCategory {
    Physical,
    Special,
    Status,
}

impl Default for MoveCategory {
    fn default() -> Self {
        MoveCategory::Physical
    }
}

/// Move target type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MoveTargetType {
    Normal,          // Single adjacent target
    Self_,           // User only
    AllySide,        // User's side of field
    FoeSide,         // Opponent's side of field
    All,             // All Pokemon on field
    AllAdjacent,     // All adjacent Pokemon
    AllAdjacentFoes, // All adjacent foes
    Ally,            // Single ally
    Any,             // Any single Pokemon
    RandomNormal,    // Random adjacent foe
    AllAllies,       // User and all allies
}

impl Default for MoveTargetType {
    fn default() -> Self {
        MoveTargetType::Normal
    }
}

/// Secondary effect of a move
#[derive(Debug, Clone, Default)]
pub struct SecondaryEffect {
    /// Chance of effect (1-100)
    pub chance: u8,
    /// Status to inflict
    pub status: Option<String>,
    /// Volatile status to add
    pub volatile_status: Option<String>,
    /// Stat boosts to apply (stat, stages)
    pub boosts: Option<Vec<(String, i8)>>,
    /// Self boost (apply to user instead of target)
    pub self_boost: bool,
    /// Flinch
    pub flinch: bool,
}

/// Move definition - data-driven move with all properties
#[derive(Debug, Clone)]
pub struct MoveDef {
    /// Unique ID
    pub id: ID,
    /// Display name
    pub name: String,
    /// Base power (0 for status moves)
    pub base_power: u32,
    /// Accuracy (0 = always hits)
    pub accuracy: u8,
    /// PP (power points)
    pub pp: u8,
    /// Move category
    pub category: MoveCategory,
    /// Move type
    pub move_type: String,
    /// Move flags
    pub flags: MoveFlags,
    /// Priority (-7 to +7)
    pub priority: i8,
    /// Target type
    pub target: MoveTargetType,
    /// Is a Z-move
    pub is_z: bool,
    /// Is a Max move
    pub is_max: bool,
    /// Critical hit ratio stage (0 = normal, 1 = high, 2 = always crits)
    pub crit_ratio: u8,
    /// Multi-hit info (min, max hits)
    pub multi_hit: Option<(u8, u8)>,
    /// Recoil damage (fraction of damage dealt)
    pub recoil: Option<f64>,
    /// Recoil damage based on user's max HP
    pub recoil_hp: Option<f64>,
    /// Drain HP (fraction of damage dealt)
    pub drain: Option<f64>,
    /// Self stat drops after use (stat, stages)
    pub self_drops: Option<Vec<(String, i8)>>,
    /// Self stat boosts after use (stat, stages)
    pub self_boosts: Option<Vec<(String, i8)>>,
    /// Target stat boosts (stat, stages)
    pub boosts: Option<Vec<(String, i8)>>,
    /// Status to inflict
    pub status: Option<String>,
    /// Volatile status to add to target
    pub volatile_status: Option<String>,
    /// Volatile status to add to user
    pub self_volatile: Option<String>,
    /// Side condition to set (on foe's side)
    pub side_condition: Option<String>,
    /// Slot condition to set
    pub slot_condition: Option<String>,
    /// Weather to set
    pub weather: Option<String>,
    /// Terrain to set
    pub terrain: Option<String>,
    /// Pseudo-weather to set
    pub pseudo_weather: Option<String>,
    /// Is a pivot move (switch out after attacking)
    pub is_pivot: bool,
    /// Force switch (roar, whirlwind)
    pub force_switch: bool,
    /// Secondary effects
    pub secondaries: Vec<SecondaryEffect>,
    /// Heal user (fraction of max HP)
    pub heal: Option<f64>,
    /// Heal user by weather (weather, fraction pairs)
    pub heal_by_weather: Option<HashMap<String, f64>>,
    /// OHKO move
    pub ohko: bool,
    /// Thaws the user if frozen
    pub thaws_user: bool,
    /// Ignores target's ability
    pub ignores_ability: bool,
    /// Breaks through Protect
    pub breaks_protect: bool,
    /// Always crits
    pub will_crit: bool,
    /// Ignores type immunity
    pub ignore_immunity: bool,
    /// Self destruct type
    pub self_destruct: Option<String>,
    /// Has crash damage
    pub has_crash_damage: bool,
    /// Forces STAB bonus even if not matching type
    pub force_stab: bool,
    /// Type was changed by an ability like Aerilate/Pixilate/etc.
    pub type_changer_boosted: bool,
}

impl Default for MoveDef {
    fn default() -> Self {
        Self {
            id: ID::empty(),
            name: String::new(),
            base_power: 0,
            accuracy: 100,
            pp: 5,
            category: MoveCategory::Physical,
            move_type: "Normal".to_string(),
            flags: MoveFlags::default(),
            priority: 0,
            target: MoveTargetType::Normal,
            is_z: false,
            is_max: false,
            crit_ratio: 0,
            multi_hit: None,
            recoil: None,
            recoil_hp: None,
            drain: None,
            self_drops: None,
            self_boosts: None,
            boosts: None,
            status: None,
            volatile_status: None,
            self_volatile: None,
            side_condition: None,
            slot_condition: None,
            weather: None,
            terrain: None,
            pseudo_weather: None,
            is_pivot: false,
            force_switch: false,
            secondaries: Vec::new(),
            heal: None,
            heal_by_weather: None,
            ohko: false,
            thaws_user: false,
            ignores_ability: false,
            breaks_protect: false,
            will_crit: false,
            ignore_immunity: false,
            self_destruct: None,
            has_crash_damage: false,
            force_stab: false,
            type_changer_boosted: false,
        }
    }
}

impl MoveDef {
    pub fn new(id: &str, name: &str) -> Self {
        Self {
            id: ID::new(id),
            name: name.to_string(),
            ..Default::default()
        }
    }

    /// Check if move is a status move
    pub fn is_status(&self) -> bool {
        self.category == MoveCategory::Status
    }

    /// Check if move is a damaging move
    pub fn is_damaging(&self) -> bool {
        self.category != MoveCategory::Status
    }

    /// Get effective accuracy (0 = never miss)
    pub fn effective_accuracy(&self) -> u8 {
        if self.accuracy == 0 { 0 } else { self.accuracy }
    }
}

/// Static registry of all moves
pub static MOVES: Lazy<HashMap<ID, MoveDef>> = Lazy::new(|| {
    let mut map = HashMap::new();

    // === Common attacking moves ===
    map.insert(ID::new("tackle"), MoveDef {
        id: ID::new("tackle"),
        name: "Tackle".to_string(),
        base_power: 40,
        accuracy: 100,
        pp: 35,
        category: MoveCategory::Physical,
        move_type: "Normal".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("earthquake"), MoveDef {
        id: ID::new("earthquake"),
        name: "Earthquake".to_string(),
        base_power: 100,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Ground".to_string(),
        target: MoveTargetType::AllAdjacent,
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("thunderbolt"), MoveDef {
        id: ID::new("thunderbolt"),
        name: "Thunderbolt".to_string(),
        base_power: 90,
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Special,
        move_type: "Electric".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 10,
            status: Some("par".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("flamethrower"), MoveDef {
        id: ID::new("flamethrower"),
        name: "Flamethrower".to_string(),
        base_power: 90,
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Special,
        move_type: "Fire".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 10,
            status: Some("brn".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("icebeam"), MoveDef {
        id: ID::new("icebeam"),
        name: "Ice Beam".to_string(),
        base_power: 90,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Special,
        move_type: "Ice".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 10,
            status: Some("frz".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("surf"), MoveDef {
        id: ID::new("surf"),
        name: "Surf".to_string(),
        base_power: 90,
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Special,
        move_type: "Water".to_string(),
        target: MoveTargetType::AllAdjacent,
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("closecombat"), MoveDef {
        id: ID::new("closecombat"),
        name: "Close Combat".to_string(),
        base_power: 120,
        accuracy: 100,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Fighting".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        self_drops: Some(vec![("def".to_string(), -1), ("spd".to_string(), -1)]),
        ..Default::default()
    });

    // === Pivot moves ===
    map.insert(ID::new("uturn"), MoveDef {
        id: ID::new("uturn"),
        name: "U-turn".to_string(),
        base_power: 70,
        accuracy: 100,
        pp: 20,
        category: MoveCategory::Physical,
        move_type: "Bug".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        is_pivot: true,
        ..Default::default()
    });

    map.insert(ID::new("voltswitch"), MoveDef {
        id: ID::new("voltswitch"),
        name: "Volt Switch".to_string(),
        base_power: 70,
        accuracy: 100,
        pp: 20,
        category: MoveCategory::Special,
        move_type: "Electric".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        is_pivot: true,
        ..Default::default()
    });

    map.insert(ID::new("flipturn"), MoveDef {
        id: ID::new("flipturn"),
        name: "Flip Turn".to_string(),
        base_power: 60,
        accuracy: 100,
        pp: 20,
        category: MoveCategory::Physical,
        move_type: "Water".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        is_pivot: true,
        ..Default::default()
    });

    // === Status moves ===
    map.insert(ID::new("thunderwave"), MoveDef {
        id: ID::new("thunderwave"),
        name: "Thunder Wave".to_string(),
        accuracy: 90,
        pp: 20,
        category: MoveCategory::Status,
        move_type: "Electric".to_string(),
        flags: MoveFlags { protect: true, reflectable: true, mirror: true, ..Default::default() },
        status: Some("par".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("willowisp"), MoveDef {
        id: ID::new("willowisp"),
        name: "Will-O-Wisp".to_string(),
        accuracy: 85,
        pp: 15,
        category: MoveCategory::Status,
        move_type: "Fire".to_string(),
        flags: MoveFlags { protect: true, reflectable: true, mirror: true, ..Default::default() },
        status: Some("brn".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("toxic"), MoveDef {
        id: ID::new("toxic"),
        name: "Toxic".to_string(),
        accuracy: 90,
        pp: 10,
        category: MoveCategory::Status,
        move_type: "Poison".to_string(),
        flags: MoveFlags { protect: true, reflectable: true, mirror: true, ..Default::default() },
        status: Some("tox".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("spore"), MoveDef {
        id: ID::new("spore"),
        name: "Spore".to_string(),
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Status,
        move_type: "Grass".to_string(),
        flags: MoveFlags { protect: true, reflectable: true, mirror: true, powder: true, ..Default::default() },
        status: Some("slp".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("sleeppowder"), MoveDef {
        id: ID::new("sleeppowder"),
        name: "Sleep Powder".to_string(),
        accuracy: 75,
        pp: 15,
        category: MoveCategory::Status,
        move_type: "Grass".to_string(),
        flags: MoveFlags { protect: true, reflectable: true, mirror: true, powder: true, ..Default::default() },
        status: Some("slp".to_string()),
        ..Default::default()
    });

    // === Setup moves ===
    map.insert(ID::new("swordsdance"), MoveDef {
        id: ID::new("swordsdance"),
        name: "Swords Dance".to_string(),
        pp: 20,
        category: MoveCategory::Status,
        move_type: "Normal".to_string(),
        target: MoveTargetType::Self_,
        flags: MoveFlags { dance: true, ..Default::default() },
        self_boosts: Some(vec![("atk".to_string(), 2)]),
        ..Default::default()
    });

    map.insert(ID::new("nastyplot"), MoveDef {
        id: ID::new("nastyplot"),
        name: "Nasty Plot".to_string(),
        pp: 20,
        category: MoveCategory::Status,
        move_type: "Dark".to_string(),
        target: MoveTargetType::Self_,
        self_boosts: Some(vec![("spa".to_string(), 2)]),
        ..Default::default()
    });

    map.insert(ID::new("calmmind"), MoveDef {
        id: ID::new("calmmind"),
        name: "Calm Mind".to_string(),
        pp: 20,
        category: MoveCategory::Status,
        move_type: "Psychic".to_string(),
        target: MoveTargetType::Self_,
        self_boosts: Some(vec![("spa".to_string(), 1), ("spd".to_string(), 1)]),
        ..Default::default()
    });

    map.insert(ID::new("agility"), MoveDef {
        id: ID::new("agility"),
        name: "Agility".to_string(),
        pp: 30,
        category: MoveCategory::Status,
        move_type: "Psychic".to_string(),
        target: MoveTargetType::Self_,
        self_boosts: Some(vec![("spe".to_string(), 2)]),
        ..Default::default()
    });

    map.insert(ID::new("bulkup"), MoveDef {
        id: ID::new("bulkup"),
        name: "Bulk Up".to_string(),
        pp: 20,
        category: MoveCategory::Status,
        move_type: "Fighting".to_string(),
        target: MoveTargetType::Self_,
        self_boosts: Some(vec![("atk".to_string(), 1), ("def".to_string(), 1)]),
        ..Default::default()
    });

    map.insert(ID::new("irondefense"), MoveDef {
        id: ID::new("irondefense"),
        name: "Iron Defense".to_string(),
        pp: 15,
        category: MoveCategory::Status,
        move_type: "Steel".to_string(),
        target: MoveTargetType::Self_,
        self_boosts: Some(vec![("def".to_string(), 2)]),
        ..Default::default()
    });

    // === Protection moves ===
    map.insert(ID::new("protect"), MoveDef {
        id: ID::new("protect"),
        name: "Protect".to_string(),
        pp: 10,
        category: MoveCategory::Status,
        move_type: "Normal".to_string(),
        priority: 4,
        target: MoveTargetType::Self_,
        self_volatile: Some("protect".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("detect"), MoveDef {
        id: ID::new("detect"),
        name: "Detect".to_string(),
        pp: 5,
        category: MoveCategory::Status,
        move_type: "Fighting".to_string(),
        priority: 4,
        target: MoveTargetType::Self_,
        self_volatile: Some("protect".to_string()),
        ..Default::default()
    });

    // === Recovery moves ===
    map.insert(ID::new("recover"), MoveDef {
        id: ID::new("recover"),
        name: "Recover".to_string(),
        pp: 10,
        category: MoveCategory::Status,
        move_type: "Normal".to_string(),
        target: MoveTargetType::Self_,
        flags: MoveFlags { heal: true, ..Default::default() },
        heal: Some(0.5),
        ..Default::default()
    });

    map.insert(ID::new("softboiled"), MoveDef {
        id: ID::new("softboiled"),
        name: "Soft-Boiled".to_string(),
        pp: 10,
        category: MoveCategory::Status,
        move_type: "Normal".to_string(),
        target: MoveTargetType::Self_,
        flags: MoveFlags { heal: true, ..Default::default() },
        heal: Some(0.5),
        ..Default::default()
    });

    map.insert(ID::new("roost"), MoveDef {
        id: ID::new("roost"),
        name: "Roost".to_string(),
        pp: 10,
        category: MoveCategory::Status,
        move_type: "Flying".to_string(),
        target: MoveTargetType::Self_,
        flags: MoveFlags { heal: true, ..Default::default() },
        heal: Some(0.5),
        self_volatile: Some("roost".to_string()),
        ..Default::default()
    });

    {
        let mut weather_heal = HashMap::new();
        weather_heal.insert("sunnyday".to_string(), 2.0/3.0);
        weather_heal.insert("raindance".to_string(), 0.25);
        weather_heal.insert("sandstorm".to_string(), 0.25);
        weather_heal.insert("hail".to_string(), 0.25);
        weather_heal.insert("snow".to_string(), 0.25);
        weather_heal.insert("".to_string(), 0.5);

        map.insert(ID::new("moonlight"), MoveDef {
            id: ID::new("moonlight"),
            name: "Moonlight".to_string(),
            pp: 5,
            category: MoveCategory::Status,
            move_type: "Fairy".to_string(),
            target: MoveTargetType::Self_,
            flags: MoveFlags { heal: true, ..Default::default() },
            heal_by_weather: Some(weather_heal.clone()),
            ..Default::default()
        });

        map.insert(ID::new("synthesis"), MoveDef {
            id: ID::new("synthesis"),
            name: "Synthesis".to_string(),
            pp: 5,
            category: MoveCategory::Status,
            move_type: "Grass".to_string(),
            target: MoveTargetType::Self_,
            flags: MoveFlags { heal: true, ..Default::default() },
            heal_by_weather: Some(weather_heal.clone()),
            ..Default::default()
        });

        map.insert(ID::new("morningsun"), MoveDef {
            id: ID::new("morningsun"),
            name: "Morning Sun".to_string(),
            pp: 5,
            category: MoveCategory::Status,
            move_type: "Normal".to_string(),
            target: MoveTargetType::Self_,
            flags: MoveFlags { heal: true, ..Default::default() },
            heal_by_weather: Some(weather_heal),
            ..Default::default()
        });
    }

    // === Hazard moves ===
    map.insert(ID::new("stealthrock"), MoveDef {
        id: ID::new("stealthrock"),
        name: "Stealth Rock".to_string(),
        pp: 20,
        category: MoveCategory::Status,
        move_type: "Rock".to_string(),
        target: MoveTargetType::FoeSide,
        flags: MoveFlags { reflectable: true, ..Default::default() },
        side_condition: Some("stealthrock".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("spikes"), MoveDef {
        id: ID::new("spikes"),
        name: "Spikes".to_string(),
        pp: 20,
        category: MoveCategory::Status,
        move_type: "Ground".to_string(),
        target: MoveTargetType::FoeSide,
        flags: MoveFlags { reflectable: true, ..Default::default() },
        side_condition: Some("spikes".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("toxicspikes"), MoveDef {
        id: ID::new("toxicspikes"),
        name: "Toxic Spikes".to_string(),
        pp: 20,
        category: MoveCategory::Status,
        move_type: "Poison".to_string(),
        target: MoveTargetType::FoeSide,
        flags: MoveFlags { reflectable: true, ..Default::default() },
        side_condition: Some("toxicspikes".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("stickyweb"), MoveDef {
        id: ID::new("stickyweb"),
        name: "Sticky Web".to_string(),
        pp: 20,
        category: MoveCategory::Status,
        move_type: "Bug".to_string(),
        target: MoveTargetType::FoeSide,
        flags: MoveFlags { reflectable: true, ..Default::default() },
        side_condition: Some("stickyweb".to_string()),
        ..Default::default()
    });

    // === Hazard removal ===
    map.insert(ID::new("defog"), MoveDef {
        id: ID::new("defog"),
        name: "Defog".to_string(),
        pp: 15,
        category: MoveCategory::Status,
        move_type: "Flying".to_string(),
        flags: MoveFlags { protect: true, reflectable: true, mirror: true, ..Default::default() },
        boosts: Some(vec![("evasion".to_string(), -1)]),
        // Note: Also removes hazards - handled in battle.rs
        ..Default::default()
    });

    map.insert(ID::new("rapidspin"), MoveDef {
        id: ID::new("rapidspin"),
        name: "Rapid Spin".to_string(),
        base_power: 50,
        accuracy: 100,
        pp: 40,
        category: MoveCategory::Physical,
        move_type: "Normal".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        // Note: Also removes hazards and boosts Speed - handled in battle.rs
        self_boosts: Some(vec![("spe".to_string(), 1)]),
        ..Default::default()
    });

    // === Substitute ===
    map.insert(ID::new("substitute"), MoveDef {
        id: ID::new("substitute"),
        name: "Substitute".to_string(),
        pp: 10,
        category: MoveCategory::Status,
        move_type: "Normal".to_string(),
        target: MoveTargetType::Self_,
        self_volatile: Some("substitute".to_string()),
        // Note: Costs 25% HP - handled in battle.rs
        ..Default::default()
    });

    // === High crit moves ===
    map.insert(ID::new("slash"), MoveDef {
        id: ID::new("slash"),
        name: "Slash".to_string(),
        base_power: 70,
        accuracy: 100,
        pp: 20,
        category: MoveCategory::Physical,
        move_type: "Normal".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, high_crit_ratio: true, ..Default::default() },
        crit_ratio: 1,
        ..Default::default()
    });

    map.insert(ID::new("stoneedge"), MoveDef {
        id: ID::new("stoneedge"),
        name: "Stone Edge".to_string(),
        base_power: 100,
        accuracy: 80,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Rock".to_string(),
        flags: MoveFlags { protect: true, mirror: true, high_crit_ratio: true, ..Default::default() },
        crit_ratio: 1,
        ..Default::default()
    });

    // === Multi-hit moves ===
    map.insert(ID::new("bulletseed"), MoveDef {
        id: ID::new("bulletseed"),
        name: "Bullet Seed".to_string(),
        base_power: 25,
        accuracy: 100,
        pp: 30,
        category: MoveCategory::Physical,
        move_type: "Grass".to_string(),
        flags: MoveFlags { bullet: true, protect: true, mirror: true, ..Default::default() },
        multi_hit: Some((2, 5)),
        ..Default::default()
    });

    map.insert(ID::new("rockblast"), MoveDef {
        id: ID::new("rockblast"),
        name: "Rock Blast".to_string(),
        base_power: 25,
        accuracy: 90,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Rock".to_string(),
        flags: MoveFlags { bullet: true, protect: true, mirror: true, ..Default::default() },
        multi_hit: Some((2, 5)),
        ..Default::default()
    });

    // === Recoil moves ===
    map.insert(ID::new("doubleedge"), MoveDef {
        id: ID::new("doubleedge"),
        name: "Double-Edge".to_string(),
        base_power: 120,
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Physical,
        move_type: "Normal".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        recoil: Some(1.0/3.0),
        ..Default::default()
    });

    map.insert(ID::new("bravebird"), MoveDef {
        id: ID::new("bravebird"),
        name: "Brave Bird".to_string(),
        base_power: 120,
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Physical,
        move_type: "Flying".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        recoil: Some(1.0/3.0),
        ..Default::default()
    });

    map.insert(ID::new("flareblitz"), MoveDef {
        id: ID::new("flareblitz"),
        name: "Flare Blitz".to_string(),
        base_power: 120,
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Physical,
        move_type: "Fire".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, defrost: true, ..Default::default() },
        recoil: Some(1.0/3.0),
        thaws_user: true,
        secondaries: vec![SecondaryEffect {
            chance: 10,
            status: Some("brn".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    });

    // === Trick Room ===
    map.insert(ID::new("trickroom"), MoveDef {
        id: ID::new("trickroom"),
        name: "Trick Room".to_string(),
        pp: 5,
        category: MoveCategory::Status,
        move_type: "Psychic".to_string(),
        priority: -7,
        target: MoveTargetType::All,
        pseudo_weather: Some("trickroom".to_string()),
        ..Default::default()
    });

    // === A moves ===
    map.insert(ID::new("absorb"), MoveDef {
        id: ID::new("absorb"),
        name: "Absorb".to_string(),
        base_power: 20,
        accuracy: 100,
        pp: 25,
        category: MoveCategory::Special,
        move_type: "Grass".to_string(),
        flags: MoveFlags { protect: true, mirror: true, heal: true, ..Default::default() },
        drain: Some(0.5),
        ..Default::default()
    });

    map.insert(ID::new("accelerock"), MoveDef {
        id: ID::new("accelerock"),
        name: "Accelerock".to_string(),
        base_power: 40,
        accuracy: 100,
        pp: 20,
        priority: 1,
        category: MoveCategory::Physical,
        move_type: "Rock".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("acid"), MoveDef {
        id: ID::new("acid"),
        name: "Acid".to_string(),
        base_power: 40,
        accuracy: 100,
        pp: 30,
        category: MoveCategory::Special,
        move_type: "Poison".to_string(),
        target: MoveTargetType::AllAdjacentFoes,
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 10,
            boosts: Some(vec![("spd".to_string(), -1)]),
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("acidarmor"), MoveDef {
        id: ID::new("acidarmor"),
        name: "Acid Armor".to_string(),
        accuracy: 0,
        pp: 20,
        category: MoveCategory::Status,
        move_type: "Poison".to_string(),
        target: MoveTargetType::Self_,
        self_boosts: Some(vec![("def".to_string(), 2)]),
        ..Default::default()
    });

    map.insert(ID::new("aciddownpour"), MoveDef {
        id: ID::new("aciddownpour"),
        name: "Acid Downpour".to_string(),
        base_power: 1,
        accuracy: 0,
        pp: 1,
        category: MoveCategory::Physical,
        move_type: "Poison".to_string(),
        is_z: true,
        ..Default::default()
    });

    map.insert(ID::new("acidspray"), MoveDef {
        id: ID::new("acidspray"),
        name: "Acid Spray".to_string(),
        base_power: 40,
        accuracy: 100,
        pp: 20,
        category: MoveCategory::Special,
        move_type: "Poison".to_string(),
        flags: MoveFlags { protect: true, mirror: true, bullet: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 100,
            boosts: Some(vec![("spd".to_string(), -2)]),
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("acrobatics"), MoveDef {
        id: ID::new("acrobatics"),
        name: "Acrobatics".to_string(),
        base_power: 55,
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Physical,
        move_type: "Flying".to_string(),
        target: MoveTargetType::Any,
        flags: MoveFlags { contact: true, protect: true, mirror: true, distance: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("acupressure"), MoveDef {
        id: ID::new("acupressure"),
        name: "Acupressure".to_string(),
        accuracy: 0,
        pp: 30,
        category: MoveCategory::Status,
        move_type: "Normal".to_string(),
        target: MoveTargetType::Ally,
        ..Default::default()
    });

    map.insert(ID::new("aerialace"), MoveDef {
        id: ID::new("aerialace"),
        name: "Aerial Ace".to_string(),
        base_power: 60,
        accuracy: 0,
        pp: 20,
        category: MoveCategory::Physical,
        move_type: "Flying".to_string(),
        target: MoveTargetType::Any,
        flags: MoveFlags { contact: true, protect: true, mirror: true, distance: true, slicing: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("aeroblast"), MoveDef {
        id: ID::new("aeroblast"),
        name: "Aeroblast".to_string(),
        base_power: 100,
        accuracy: 95,
        pp: 5,
        category: MoveCategory::Special,
        move_type: "Flying".to_string(),
        target: MoveTargetType::Any,
        crit_ratio: 1,
        flags: MoveFlags { protect: true, mirror: true, distance: true, wind: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("afteryou"), MoveDef {
        id: ID::new("afteryou"),
        name: "After You".to_string(),
        accuracy: 0,
        pp: 15,
        category: MoveCategory::Status,
        move_type: "Normal".to_string(),
        flags: MoveFlags { bypasssub: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("aircutter"), MoveDef {
        id: ID::new("aircutter"),
        name: "Air Cutter".to_string(),
        base_power: 60,
        accuracy: 95,
        pp: 25,
        category: MoveCategory::Special,
        move_type: "Flying".to_string(),
        target: MoveTargetType::AllAdjacentFoes,
        crit_ratio: 1,
        flags: MoveFlags { protect: true, mirror: true, slicing: true, wind: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("airslash"), MoveDef {
        id: ID::new("airslash"),
        name: "Air Slash".to_string(),
        base_power: 75,
        accuracy: 95,
        pp: 15,
        category: MoveCategory::Special,
        move_type: "Flying".to_string(),
        target: MoveTargetType::Any,
        flags: MoveFlags { protect: true, mirror: true, distance: true, slicing: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 30,
            volatile_status: Some("flinch".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("alloutpummeling"), MoveDef {
        id: ID::new("alloutpummeling"),
        name: "All-Out Pummeling".to_string(),
        base_power: 1,
        accuracy: 0,
        pp: 1,
        category: MoveCategory::Physical,
        move_type: "Fighting".to_string(),
        is_z: true,
        ..Default::default()
    });

    map.insert(ID::new("alluringvoice"), MoveDef {
        id: ID::new("alluringvoice"),
        name: "Alluring Voice".to_string(),
        base_power: 80,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Special,
        move_type: "Fairy".to_string(),
        flags: MoveFlags { protect: true, mirror: true, sound: true, bypasssub: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("allyswitch"), MoveDef {
        id: ID::new("allyswitch"),
        name: "Ally Switch".to_string(),
        accuracy: 0,
        pp: 15,
        priority: 2,
        category: MoveCategory::Status,
        move_type: "Psychic".to_string(),
        target: MoveTargetType::Self_,
        ..Default::default()
    });

    map.insert(ID::new("amnesia"), MoveDef {
        id: ID::new("amnesia"),
        name: "Amnesia".to_string(),
        accuracy: 0,
        pp: 20,
        category: MoveCategory::Status,
        move_type: "Psychic".to_string(),
        target: MoveTargetType::Self_,
        self_boosts: Some(vec![("spd".to_string(), 2)]),
        ..Default::default()
    });

    map.insert(ID::new("anchorshot"), MoveDef {
        id: ID::new("anchorshot"),
        name: "Anchor Shot".to_string(),
        base_power: 80,
        accuracy: 100,
        pp: 20,
        category: MoveCategory::Physical,
        move_type: "Steel".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 100,
            volatile_status: Some("trapped".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("ancientpower"), MoveDef {
        id: ID::new("ancientpower"),
        name: "Ancient Power".to_string(),
        base_power: 60,
        accuracy: 100,
        pp: 5,
        category: MoveCategory::Special,
        move_type: "Rock".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 10,
            boosts: Some(vec![("atk".to_string(), 1), ("def".to_string(), 1), ("spa".to_string(), 1), ("spd".to_string(), 1), ("spe".to_string(), 1)]),
            self_boost: true,
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("appleacid"), MoveDef {
        id: ID::new("appleacid"),
        name: "Apple Acid".to_string(),
        base_power: 80,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Special,
        move_type: "Grass".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 100,
            boosts: Some(vec![("spd".to_string(), -1)]),
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("aquacutter"), MoveDef {
        id: ID::new("aquacutter"),
        name: "Aqua Cutter".to_string(),
        base_power: 70,
        accuracy: 100,
        pp: 20,
        category: MoveCategory::Physical,
        move_type: "Water".to_string(),
        crit_ratio: 1,
        flags: MoveFlags { protect: true, mirror: true, slicing: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("aquajet"), MoveDef {
        id: ID::new("aquajet"),
        name: "Aqua Jet".to_string(),
        base_power: 40,
        accuracy: 100,
        pp: 20,
        priority: 1,
        category: MoveCategory::Physical,
        move_type: "Water".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("aquaring"), MoveDef {
        id: ID::new("aquaring"),
        name: "Aqua Ring".to_string(),
        accuracy: 0,
        pp: 20,
        category: MoveCategory::Status,
        move_type: "Water".to_string(),
        target: MoveTargetType::Self_,
        self_volatile: Some("aquaring".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("aquastep"), MoveDef {
        id: ID::new("aquastep"),
        name: "Aqua Step".to_string(),
        base_power: 80,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Water".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, dance: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 100,
            boosts: Some(vec![("spe".to_string(), 1)]),
            self_boost: true,
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("aquatail"), MoveDef {
        id: ID::new("aquatail"),
        name: "Aqua Tail".to_string(),
        base_power: 90,
        accuracy: 90,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Water".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("armorcannon"), MoveDef {
        id: ID::new("armorcannon"),
        name: "Armor Cannon".to_string(),
        base_power: 120,
        accuracy: 100,
        pp: 5,
        category: MoveCategory::Special,
        move_type: "Fire".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        self_drops: Some(vec![("def".to_string(), -1), ("spd".to_string(), -1)]),
        ..Default::default()
    });

    map.insert(ID::new("armthrust"), MoveDef {
        id: ID::new("armthrust"),
        name: "Arm Thrust".to_string(),
        base_power: 15,
        accuracy: 100,
        pp: 20,
        category: MoveCategory::Physical,
        move_type: "Fighting".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        multi_hit: Some((2, 5)),
        ..Default::default()
    });

    map.insert(ID::new("aromatherapy"), MoveDef {
        id: ID::new("aromatherapy"),
        name: "Aromatherapy".to_string(),
        accuracy: 0,
        pp: 5,
        category: MoveCategory::Status,
        move_type: "Grass".to_string(),
        target: MoveTargetType::AllySide,
        flags: MoveFlags { distance: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("aromaticmist"), MoveDef {
        id: ID::new("aromaticmist"),
        name: "Aromatic Mist".to_string(),
        accuracy: 0,
        pp: 20,
        category: MoveCategory::Status,
        move_type: "Fairy".to_string(),
        target: MoveTargetType::Ally,
        flags: MoveFlags { bypasssub: true, ..Default::default() },
        boosts: Some(vec![("spd".to_string(), 1)]),
        ..Default::default()
    });

    map.insert(ID::new("assist"), MoveDef {
        id: ID::new("assist"),
        name: "Assist".to_string(),
        accuracy: 0,
        pp: 20,
        category: MoveCategory::Status,
        move_type: "Normal".to_string(),
        target: MoveTargetType::Self_,
        ..Default::default()
    });

    map.insert(ID::new("assurance"), MoveDef {
        id: ID::new("assurance"),
        name: "Assurance".to_string(),
        base_power: 60,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Dark".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("astonish"), MoveDef {
        id: ID::new("astonish"),
        name: "Astonish".to_string(),
        base_power: 30,
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Physical,
        move_type: "Ghost".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 30,
            volatile_status: Some("flinch".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("astralbarrage"), MoveDef {
        id: ID::new("astralbarrage"),
        name: "Astral Barrage".to_string(),
        base_power: 120,
        accuracy: 100,
        pp: 5,
        category: MoveCategory::Special,
        move_type: "Ghost".to_string(),
        target: MoveTargetType::AllAdjacentFoes,
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("attackorder"), MoveDef {
        id: ID::new("attackorder"),
        name: "Attack Order".to_string(),
        base_power: 90,
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Physical,
        move_type: "Bug".to_string(),
        crit_ratio: 1,
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("attract"), MoveDef {
        id: ID::new("attract"),
        name: "Attract".to_string(),
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Status,
        move_type: "Normal".to_string(),
        flags: MoveFlags { protect: true, reflectable: true, mirror: true, bypasssub: true, ..Default::default() },
        volatile_status: Some("attract".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("aurasphere"), MoveDef {
        id: ID::new("aurasphere"),
        name: "Aura Sphere".to_string(),
        base_power: 80,
        accuracy: 0,
        pp: 20,
        category: MoveCategory::Special,
        move_type: "Fighting".to_string(),
        target: MoveTargetType::Any,
        flags: MoveFlags { protect: true, mirror: true, distance: true, bullet: true, pulse: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("aurawheel"), MoveDef {
        id: ID::new("aurawheel"),
        name: "Aura Wheel".to_string(),
        base_power: 110,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Electric".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 100,
            boosts: Some(vec![("spe".to_string(), 1)]),
            self_boost: true,
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("aurorabeam"), MoveDef {
        id: ID::new("aurorabeam"),
        name: "Aurora Beam".to_string(),
        base_power: 65,
        accuracy: 100,
        pp: 20,
        category: MoveCategory::Special,
        move_type: "Ice".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 10,
            boosts: Some(vec![("atk".to_string(), -1)]),
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("auroraveil"), MoveDef {
        id: ID::new("auroraveil"),
        name: "Aurora Veil".to_string(),
        accuracy: 0,
        pp: 20,
        category: MoveCategory::Status,
        move_type: "Ice".to_string(),
        target: MoveTargetType::AllySide,
        side_condition: Some("auroraveil".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("autotomize"), MoveDef {
        id: ID::new("autotomize"),
        name: "Autotomize".to_string(),
        accuracy: 0,
        pp: 15,
        category: MoveCategory::Status,
        move_type: "Steel".to_string(),
        target: MoveTargetType::Self_,
        self_boosts: Some(vec![("spe".to_string(), 2)]),
        ..Default::default()
    });

    map.insert(ID::new("avalanche"), MoveDef {
        id: ID::new("avalanche"),
        name: "Avalanche".to_string(),
        base_power: 60,
        accuracy: 100,
        pp: 10,
        priority: -4,
        category: MoveCategory::Physical,
        move_type: "Ice".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("axekick"), MoveDef {
        id: ID::new("axekick"),
        name: "Axe Kick".to_string(),
        base_power: 120,
        accuracy: 90,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Fighting".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 30,
            volatile_status: Some("confusion".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    });

    // === B moves ===
    map.insert(ID::new("babydolleyes"), MoveDef {
        id: ID::new("babydolleyes"),
        name: "Baby-Doll Eyes".to_string(),
        accuracy: 100,
        pp: 30,
        priority: 1,
        category: MoveCategory::Status,
        move_type: "Fairy".to_string(),
        flags: MoveFlags { protect: true, reflectable: true, mirror: true, ..Default::default() },
        boosts: Some(vec![("atk".to_string(), -1)]),
        ..Default::default()
    });

    map.insert(ID::new("baddybad"), MoveDef {
        id: ID::new("baddybad"),
        name: "Baddy Bad".to_string(),
        base_power: 80,
        accuracy: 95,
        pp: 15,
        category: MoveCategory::Special,
        move_type: "Dark".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        side_condition: Some("reflect".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("banefulbunker"), MoveDef {
        id: ID::new("banefulbunker"),
        name: "Baneful Bunker".to_string(),
        accuracy: 0,
        pp: 10,
        priority: 4,
        category: MoveCategory::Status,
        move_type: "Poison".to_string(),
        target: MoveTargetType::Self_,
        self_volatile: Some("banefulbunker".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("barbbarrage"), MoveDef {
        id: ID::new("barbbarrage"),
        name: "Barb Barrage".to_string(),
        base_power: 60,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Poison".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 50,
            status: Some("psn".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("barrage"), MoveDef {
        id: ID::new("barrage"),
        name: "Barrage".to_string(),
        base_power: 15,
        accuracy: 85,
        pp: 20,
        category: MoveCategory::Physical,
        move_type: "Normal".to_string(),
        flags: MoveFlags { protect: true, mirror: true, bullet: true, ..Default::default() },
        multi_hit: Some((2, 5)),
        ..Default::default()
    });

    map.insert(ID::new("barrier"), MoveDef {
        id: ID::new("barrier"),
        name: "Barrier".to_string(),
        accuracy: 0,
        pp: 20,
        category: MoveCategory::Status,
        move_type: "Psychic".to_string(),
        target: MoveTargetType::Self_,
        self_boosts: Some(vec![("def".to_string(), 2)]),
        ..Default::default()
    });

    map.insert(ID::new("batonpass"), MoveDef {
        id: ID::new("batonpass"),
        name: "Baton Pass".to_string(),
        accuracy: 0,
        pp: 40,
        category: MoveCategory::Status,
        move_type: "Normal".to_string(),
        target: MoveTargetType::Self_,
        is_pivot: true,
        ..Default::default()
    });

    map.insert(ID::new("beakblast"), MoveDef {
        id: ID::new("beakblast"),
        name: "Beak Blast".to_string(),
        base_power: 100,
        accuracy: 100,
        pp: 15,
        priority: -3,
        category: MoveCategory::Physical,
        move_type: "Flying".to_string(),
        flags: MoveFlags { protect: true, bullet: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("beatup"), MoveDef {
        id: ID::new("beatup"),
        name: "Beat Up".to_string(),
        base_power: 0,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Dark".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("behemothbash"), MoveDef {
        id: ID::new("behemothbash"),
        name: "Behemoth Bash".to_string(),
        base_power: 100,
        accuracy: 100,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Steel".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("behemothblade"), MoveDef {
        id: ID::new("behemothblade"),
        name: "Behemoth Blade".to_string(),
        base_power: 100,
        accuracy: 100,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Steel".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, slicing: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("belch"), MoveDef {
        id: ID::new("belch"),
        name: "Belch".to_string(),
        base_power: 120,
        accuracy: 90,
        pp: 10,
        category: MoveCategory::Special,
        move_type: "Poison".to_string(),
        flags: MoveFlags { protect: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("bellydrum"), MoveDef {
        id: ID::new("bellydrum"),
        name: "Belly Drum".to_string(),
        accuracy: 0,
        pp: 10,
        category: MoveCategory::Status,
        move_type: "Normal".to_string(),
        target: MoveTargetType::Self_,
        ..Default::default()
    });

    map.insert(ID::new("bestow"), MoveDef {
        id: ID::new("bestow"),
        name: "Bestow".to_string(),
        accuracy: 0,
        pp: 15,
        category: MoveCategory::Status,
        move_type: "Normal".to_string(),
        flags: MoveFlags { mirror: true, bypasssub: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("bide"), MoveDef {
        id: ID::new("bide"),
        name: "Bide".to_string(),
        base_power: 0,
        accuracy: 0,
        pp: 10,
        priority: 1,
        category: MoveCategory::Physical,
        move_type: "Normal".to_string(),
        target: MoveTargetType::Self_,
        flags: MoveFlags { contact: true, protect: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("bind"), MoveDef {
        id: ID::new("bind"),
        name: "Bind".to_string(),
        base_power: 15,
        accuracy: 85,
        pp: 20,
        category: MoveCategory::Physical,
        move_type: "Normal".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        volatile_status: Some("partiallytrapped".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("bite"), MoveDef {
        id: ID::new("bite"),
        name: "Bite".to_string(),
        base_power: 60,
        accuracy: 100,
        pp: 25,
        category: MoveCategory::Physical,
        move_type: "Dark".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, bite: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 30,
            volatile_status: Some("flinch".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("bitterblade"), MoveDef {
        id: ID::new("bitterblade"),
        name: "Bitter Blade".to_string(),
        base_power: 90,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Fire".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, heal: true, slicing: true, ..Default::default() },
        drain: Some(0.5),
        ..Default::default()
    });

    map.insert(ID::new("bittermalice"), MoveDef {
        id: ID::new("bittermalice"),
        name: "Bitter Malice".to_string(),
        base_power: 75,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Special,
        move_type: "Ghost".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 100,
            boosts: Some(vec![("atk".to_string(), -1)]),
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("blackholeeclipse"), MoveDef {
        id: ID::new("blackholeeclipse"),
        name: "Black Hole Eclipse".to_string(),
        base_power: 1,
        accuracy: 0,
        pp: 1,
        category: MoveCategory::Physical,
        move_type: "Dark".to_string(),
        is_z: true,
        ..Default::default()
    });

    map.insert(ID::new("blastburn"), MoveDef {
        id: ID::new("blastburn"),
        name: "Blast Burn".to_string(),
        base_power: 150,
        accuracy: 90,
        pp: 5,
        category: MoveCategory::Special,
        move_type: "Fire".to_string(),
        flags: MoveFlags { recharge: true, protect: true, mirror: true, ..Default::default() },
        self_volatile: Some("mustrecharge".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("blazekick"), MoveDef {
        id: ID::new("blazekick"),
        name: "Blaze Kick".to_string(),
        base_power: 85,
        accuracy: 90,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Fire".to_string(),
        crit_ratio: 1,
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 10,
            status: Some("brn".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("blazingtorque"), MoveDef {
        id: ID::new("blazingtorque"),
        name: "Blazing Torque".to_string(),
        base_power: 80,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Fire".to_string(),
        flags: MoveFlags { protect: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 30,
            status: Some("brn".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("bleakwindstorm"), MoveDef {
        id: ID::new("bleakwindstorm"),
        name: "Bleakwind Storm".to_string(),
        base_power: 100,
        accuracy: 80,
        pp: 10,
        category: MoveCategory::Special,
        move_type: "Flying".to_string(),
        target: MoveTargetType::AllAdjacentFoes,
        flags: MoveFlags { protect: true, mirror: true, wind: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 30,
            boosts: Some(vec![("spe".to_string(), -1)]),
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("blizzard"), MoveDef {
        id: ID::new("blizzard"),
        name: "Blizzard".to_string(),
        base_power: 110,
        accuracy: 70,
        pp: 5,
        category: MoveCategory::Special,
        move_type: "Ice".to_string(),
        target: MoveTargetType::AllAdjacentFoes,
        flags: MoveFlags { protect: true, mirror: true, wind: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 10,
            status: Some("frz".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("block"), MoveDef {
        id: ID::new("block"),
        name: "Block".to_string(),
        accuracy: 0,
        pp: 5,
        category: MoveCategory::Status,
        move_type: "Normal".to_string(),
        flags: MoveFlags { reflectable: true, mirror: true, ..Default::default() },
        volatile_status: Some("trapped".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("bloodmoon"), MoveDef {
        id: ID::new("bloodmoon"),
        name: "Blood Moon".to_string(),
        base_power: 140,
        accuracy: 100,
        pp: 5,
        category: MoveCategory::Special,
        move_type: "Normal".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("bloomdoom"), MoveDef {
        id: ID::new("bloomdoom"),
        name: "Bloom Doom".to_string(),
        base_power: 1,
        accuracy: 0,
        pp: 1,
        category: MoveCategory::Physical,
        move_type: "Grass".to_string(),
        is_z: true,
        ..Default::default()
    });

    map.insert(ID::new("blueflare"), MoveDef {
        id: ID::new("blueflare"),
        name: "Blue Flare".to_string(),
        base_power: 130,
        accuracy: 85,
        pp: 5,
        category: MoveCategory::Special,
        move_type: "Fire".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 20,
            status: Some("brn".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("bodypress"), MoveDef {
        id: ID::new("bodypress"),
        name: "Body Press".to_string(),
        base_power: 80,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Fighting".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("bodyslam"), MoveDef {
        id: ID::new("bodyslam"),
        name: "Body Slam".to_string(),
        base_power: 85,
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Physical,
        move_type: "Normal".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 30,
            status: Some("par".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("boltbeak"), MoveDef {
        id: ID::new("boltbeak"),
        name: "Bolt Beak".to_string(),
        base_power: 85,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Electric".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("boltstrike"), MoveDef {
        id: ID::new("boltstrike"),
        name: "Bolt Strike".to_string(),
        base_power: 130,
        accuracy: 85,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Electric".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 20,
            status: Some("par".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("boneclub"), MoveDef {
        id: ID::new("boneclub"),
        name: "Bone Club".to_string(),
        base_power: 65,
        accuracy: 85,
        pp: 20,
        category: MoveCategory::Physical,
        move_type: "Ground".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 10,
            volatile_status: Some("flinch".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("bonemerang"), MoveDef {
        id: ID::new("bonemerang"),
        name: "Bonemerang".to_string(),
        base_power: 50,
        accuracy: 90,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Ground".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        multi_hit: Some((2, 2)),
        ..Default::default()
    });

    map.insert(ID::new("bonerush"), MoveDef {
        id: ID::new("bonerush"),
        name: "Bone Rush".to_string(),
        base_power: 25,
        accuracy: 90,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Ground".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        multi_hit: Some((2, 5)),
        ..Default::default()
    });

    map.insert(ID::new("boomburst"), MoveDef {
        id: ID::new("boomburst"),
        name: "Boomburst".to_string(),
        base_power: 140,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Special,
        move_type: "Normal".to_string(),
        target: MoveTargetType::AllAdjacent,
        flags: MoveFlags { protect: true, mirror: true, sound: true, bypasssub: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("bounce"), MoveDef {
        id: ID::new("bounce"),
        name: "Bounce".to_string(),
        base_power: 85,
        accuracy: 85,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Flying".to_string(),
        target: MoveTargetType::Any,
        flags: MoveFlags { contact: true, charge: true, protect: true, mirror: true, gravity: true, distance: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 30,
            status: Some("par".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("bouncybubble"), MoveDef {
        id: ID::new("bouncybubble"),
        name: "Bouncy Bubble".to_string(),
        base_power: 60,
        accuracy: 100,
        pp: 20,
        category: MoveCategory::Special,
        move_type: "Water".to_string(),
        flags: MoveFlags { protect: true, mirror: true, heal: true, ..Default::default() },
        drain: Some(0.5),
        ..Default::default()
    });

    map.insert(ID::new("branchpoke"), MoveDef {
        id: ID::new("branchpoke"),
        name: "Branch Poke".to_string(),
        base_power: 40,
        accuracy: 100,
        pp: 40,
        category: MoveCategory::Physical,
        move_type: "Grass".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("breakingswipe"), MoveDef {
        id: ID::new("breakingswipe"),
        name: "Breaking Swipe".to_string(),
        base_power: 60,
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Physical,
        move_type: "Dragon".to_string(),
        target: MoveTargetType::AllAdjacentFoes,
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 100,
            boosts: Some(vec![("atk".to_string(), -1)]),
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("breakneckblitz"), MoveDef {
        id: ID::new("breakneckblitz"),
        name: "Breakneck Blitz".to_string(),
        base_power: 1,
        accuracy: 0,
        pp: 1,
        category: MoveCategory::Physical,
        move_type: "Normal".to_string(),
        is_z: true,
        ..Default::default()
    });

    map.insert(ID::new("brickbreak"), MoveDef {
        id: ID::new("brickbreak"),
        name: "Brick Break".to_string(),
        base_power: 75,
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Physical,
        move_type: "Fighting".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("brine"), MoveDef {
        id: ID::new("brine"),
        name: "Brine".to_string(),
        base_power: 65,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Special,
        move_type: "Water".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("brutalswing"), MoveDef {
        id: ID::new("brutalswing"),
        name: "Brutal Swing".to_string(),
        base_power: 60,
        accuracy: 100,
        pp: 20,
        category: MoveCategory::Physical,
        move_type: "Dark".to_string(),
        target: MoveTargetType::AllAdjacent,
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("bubble"), MoveDef {
        id: ID::new("bubble"),
        name: "Bubble".to_string(),
        base_power: 40,
        accuracy: 100,
        pp: 30,
        category: MoveCategory::Special,
        move_type: "Water".to_string(),
        target: MoveTargetType::AllAdjacentFoes,
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 10,
            boosts: Some(vec![("spe".to_string(), -1)]),
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("bubblebeam"), MoveDef {
        id: ID::new("bubblebeam"),
        name: "Bubble Beam".to_string(),
        base_power: 65,
        accuracy: 100,
        pp: 20,
        category: MoveCategory::Special,
        move_type: "Water".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 10,
            boosts: Some(vec![("spe".to_string(), -1)]),
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("bugbite"), MoveDef {
        id: ID::new("bugbite"),
        name: "Bug Bite".to_string(),
        base_power: 60,
        accuracy: 100,
        pp: 20,
        category: MoveCategory::Physical,
        move_type: "Bug".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("bugbuzz"), MoveDef {
        id: ID::new("bugbuzz"),
        name: "Bug Buzz".to_string(),
        base_power: 90,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Special,
        move_type: "Bug".to_string(),
        flags: MoveFlags { protect: true, mirror: true, sound: true, bypasssub: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 10,
            boosts: Some(vec![("spd".to_string(), -1)]),
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("bulldoze"), MoveDef {
        id: ID::new("bulldoze"),
        name: "Bulldoze".to_string(),
        base_power: 60,
        accuracy: 100,
        pp: 20,
        category: MoveCategory::Physical,
        move_type: "Ground".to_string(),
        target: MoveTargetType::AllAdjacent,
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 100,
            boosts: Some(vec![("spe".to_string(), -1)]),
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("bulletpunch"), MoveDef {
        id: ID::new("bulletpunch"),
        name: "Bullet Punch".to_string(),
        base_power: 40,
        accuracy: 100,
        pp: 30,
        priority: 1,
        category: MoveCategory::Physical,
        move_type: "Steel".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, punch: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("burningbulwark"), MoveDef {
        id: ID::new("burningbulwark"),
        name: "Burning Bulwark".to_string(),
        accuracy: 0,
        pp: 10,
        priority: 4,
        category: MoveCategory::Status,
        move_type: "Fire".to_string(),
        target: MoveTargetType::Self_,
        self_volatile: Some("burningbulwark".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("burningjealousy"), MoveDef {
        id: ID::new("burningjealousy"),
        name: "Burning Jealousy".to_string(),
        base_power: 70,
        accuracy: 100,
        pp: 5,
        category: MoveCategory::Special,
        move_type: "Fire".to_string(),
        target: MoveTargetType::AllAdjacentFoes,
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("burnup"), MoveDef {
        id: ID::new("burnup"),
        name: "Burn Up".to_string(),
        base_power: 130,
        accuracy: 100,
        pp: 5,
        category: MoveCategory::Special,
        move_type: "Fire".to_string(),
        flags: MoveFlags { protect: true, mirror: true, defrost: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("buzzybuzz"), MoveDef {
        id: ID::new("buzzybuzz"),
        name: "Buzzy Buzz".to_string(),
        base_power: 60,
        accuracy: 100,
        pp: 20,
        category: MoveCategory::Special,
        move_type: "Electric".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 100,
            status: Some("par".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    });

    // === C moves ===
    map.insert(ID::new("camouflage"), MoveDef {
        id: ID::new("camouflage"),
        name: "Camouflage".to_string(),
        accuracy: 0,
        pp: 20,
        category: MoveCategory::Status,
        move_type: "Normal".to_string(),
        target: MoveTargetType::Self_,
        ..Default::default()
    });

    map.insert(ID::new("captivate"), MoveDef {
        id: ID::new("captivate"),
        name: "Captivate".to_string(),
        accuracy: 100,
        pp: 20,
        category: MoveCategory::Status,
        move_type: "Normal".to_string(),
        target: MoveTargetType::AllAdjacentFoes,
        flags: MoveFlags { protect: true, reflectable: true, mirror: true, ..Default::default() },
        boosts: Some(vec![("spa".to_string(), -2)]),
        ..Default::default()
    });

    map.insert(ID::new("catastropika"), MoveDef {
        id: ID::new("catastropika"),
        name: "Catastropika".to_string(),
        base_power: 210,
        accuracy: 0,
        pp: 1,
        category: MoveCategory::Physical,
        move_type: "Electric".to_string(),
        flags: MoveFlags { contact: true, ..Default::default() },
        is_z: true,
        ..Default::default()
    });

    map.insert(ID::new("ceaselessedge"), MoveDef {
        id: ID::new("ceaselessedge"),
        name: "Ceaseless Edge".to_string(),
        base_power: 65,
        accuracy: 90,
        pp: 15,
        category: MoveCategory::Physical,
        move_type: "Dark".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, slicing: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("celebrate"), MoveDef {
        id: ID::new("celebrate"),
        name: "Celebrate".to_string(),
        accuracy: 0,
        pp: 40,
        category: MoveCategory::Status,
        move_type: "Normal".to_string(),
        target: MoveTargetType::Self_,
        ..Default::default()
    });

    map.insert(ID::new("charge"), MoveDef {
        id: ID::new("charge"),
        name: "Charge".to_string(),
        accuracy: 0,
        pp: 20,
        category: MoveCategory::Status,
        move_type: "Electric".to_string(),
        target: MoveTargetType::Self_,
        self_volatile: Some("charge".to_string()),
        self_boosts: Some(vec![("spd".to_string(), 1)]),
        ..Default::default()
    });

    map.insert(ID::new("chargebeam"), MoveDef {
        id: ID::new("chargebeam"),
        name: "Charge Beam".to_string(),
        base_power: 50,
        accuracy: 90,
        pp: 10,
        category: MoveCategory::Special,
        move_type: "Electric".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 70,
            boosts: Some(vec![("spa".to_string(), 1)]),
            self_boost: true,
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("charm"), MoveDef {
        id: ID::new("charm"),
        name: "Charm".to_string(),
        accuracy: 100,
        pp: 20,
        category: MoveCategory::Status,
        move_type: "Fairy".to_string(),
        flags: MoveFlags { protect: true, reflectable: true, mirror: true, ..Default::default() },
        boosts: Some(vec![("atk".to_string(), -2)]),
        ..Default::default()
    });

    map.insert(ID::new("chatter"), MoveDef {
        id: ID::new("chatter"),
        name: "Chatter".to_string(),
        base_power: 65,
        accuracy: 100,
        pp: 20,
        category: MoveCategory::Special,
        move_type: "Flying".to_string(),
        target: MoveTargetType::Any,
        flags: MoveFlags { protect: true, mirror: true, sound: true, distance: true, bypasssub: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 100,
            volatile_status: Some("confusion".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("chillingwater"), MoveDef {
        id: ID::new("chillingwater"),
        name: "Chilling Water".to_string(),
        base_power: 50,
        accuracy: 100,
        pp: 20,
        category: MoveCategory::Special,
        move_type: "Water".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 100,
            boosts: Some(vec![("atk".to_string(), -1)]),
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("chillyreception"), MoveDef {
        id: ID::new("chillyreception"),
        name: "Chilly Reception".to_string(),
        accuracy: 0,
        pp: 10,
        category: MoveCategory::Status,
        move_type: "Ice".to_string(),
        target: MoveTargetType::All,
        weather: Some("snowscape".to_string()),
        is_pivot: true,
        ..Default::default()
    });

    map.insert(ID::new("chipaway"), MoveDef {
        id: ID::new("chipaway"),
        name: "Chip Away".to_string(),
        base_power: 70,
        accuracy: 100,
        pp: 20,
        category: MoveCategory::Physical,
        move_type: "Normal".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        ignores_ability: true,
        ..Default::default()
    });

    map.insert(ID::new("chloroblast"), MoveDef {
        id: ID::new("chloroblast"),
        name: "Chloroblast".to_string(),
        base_power: 150,
        accuracy: 95,
        pp: 5,
        category: MoveCategory::Special,
        move_type: "Grass".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        recoil_hp: Some(0.5),
        ..Default::default()
    });

    map.insert(ID::new("circlethrow"), MoveDef {
        id: ID::new("circlethrow"),
        name: "Circle Throw".to_string(),
        base_power: 60,
        accuracy: 90,
        pp: 10,
        priority: -6,
        category: MoveCategory::Physical,
        move_type: "Fighting".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        force_switch: true,
        ..Default::default()
    });

    map.insert(ID::new("clamp"), MoveDef {
        id: ID::new("clamp"),
        name: "Clamp".to_string(),
        base_power: 35,
        accuracy: 85,
        pp: 15,
        category: MoveCategory::Physical,
        move_type: "Water".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        volatile_status: Some("partiallytrapped".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("clangingscales"), MoveDef {
        id: ID::new("clangingscales"),
        name: "Clanging Scales".to_string(),
        base_power: 110,
        accuracy: 100,
        pp: 5,
        category: MoveCategory::Special,
        move_type: "Dragon".to_string(),
        target: MoveTargetType::AllAdjacentFoes,
        flags: MoveFlags { protect: true, mirror: true, sound: true, bypasssub: true, ..Default::default() },
        self_drops: Some(vec![("def".to_string(), -1)]),
        ..Default::default()
    });

    map.insert(ID::new("clangoroussoul"), MoveDef {
        id: ID::new("clangoroussoul"),
        name: "Clangorous Soul".to_string(),
        accuracy: 100,
        pp: 5,
        category: MoveCategory::Status,
        move_type: "Dragon".to_string(),
        target: MoveTargetType::Self_,
        flags: MoveFlags { sound: true, dance: true, ..Default::default() },
        self_boosts: Some(vec![("atk".to_string(), 1), ("def".to_string(), 1), ("spa".to_string(), 1), ("spd".to_string(), 1), ("spe".to_string(), 1)]),
        ..Default::default()
    });

    map.insert(ID::new("clangoroussoulblaze"), MoveDef {
        id: ID::new("clangoroussoulblaze"),
        name: "Clangorous Soulblaze".to_string(),
        base_power: 185,
        accuracy: 0,
        pp: 1,
        category: MoveCategory::Special,
        move_type: "Dragon".to_string(),
        target: MoveTargetType::AllAdjacentFoes,
        flags: MoveFlags { sound: true, bypasssub: true, ..Default::default() },
        is_z: true,
        self_boosts: Some(vec![("atk".to_string(), 1), ("def".to_string(), 1), ("spa".to_string(), 1), ("spd".to_string(), 1), ("spe".to_string(), 1)]),
        ..Default::default()
    });

    map.insert(ID::new("clearsmog"), MoveDef {
        id: ID::new("clearsmog"),
        name: "Clear Smog".to_string(),
        base_power: 50,
        accuracy: 0,
        pp: 15,
        category: MoveCategory::Special,
        move_type: "Poison".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("coaching"), MoveDef {
        id: ID::new("coaching"),
        name: "Coaching".to_string(),
        accuracy: 0,
        pp: 10,
        category: MoveCategory::Status,
        move_type: "Fighting".to_string(),
        target: MoveTargetType::Ally,
        flags: MoveFlags { bypasssub: true, ..Default::default() },
        boosts: Some(vec![("atk".to_string(), 1), ("def".to_string(), 1)]),
        ..Default::default()
    });

    map.insert(ID::new("coil"), MoveDef {
        id: ID::new("coil"),
        name: "Coil".to_string(),
        accuracy: 0,
        pp: 20,
        category: MoveCategory::Status,
        move_type: "Poison".to_string(),
        target: MoveTargetType::Self_,
        self_boosts: Some(vec![("atk".to_string(), 1), ("def".to_string(), 1)]),
        ..Default::default()
    });

    map.insert(ID::new("collisioncourse"), MoveDef {
        id: ID::new("collisioncourse"),
        name: "Collision Course".to_string(),
        base_power: 100,
        accuracy: 100,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Fighting".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("combattorque"), MoveDef {
        id: ID::new("combattorque"),
        name: "Combat Torque".to_string(),
        base_power: 100,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Fighting".to_string(),
        flags: MoveFlags { protect: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 30,
            status: Some("par".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("cometpunch"), MoveDef {
        id: ID::new("cometpunch"),
        name: "Comet Punch".to_string(),
        base_power: 18,
        accuracy: 85,
        pp: 15,
        category: MoveCategory::Physical,
        move_type: "Normal".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, punch: true, ..Default::default() },
        multi_hit: Some((2, 5)),
        ..Default::default()
    });

    map.insert(ID::new("comeuppance"), MoveDef {
        id: ID::new("comeuppance"),
        name: "Comeuppance".to_string(),
        base_power: 0,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Dark".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("confide"), MoveDef {
        id: ID::new("confide"),
        name: "Confide".to_string(),
        accuracy: 0,
        pp: 20,
        category: MoveCategory::Status,
        move_type: "Normal".to_string(),
        flags: MoveFlags { reflectable: true, mirror: true, sound: true, bypasssub: true, ..Default::default() },
        boosts: Some(vec![("spa".to_string(), -1)]),
        ..Default::default()
    });

    map.insert(ID::new("confuseray"), MoveDef {
        id: ID::new("confuseray"),
        name: "Confuse Ray".to_string(),
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Status,
        move_type: "Ghost".to_string(),
        flags: MoveFlags { protect: true, reflectable: true, mirror: true, ..Default::default() },
        volatile_status: Some("confusion".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("confusion"), MoveDef {
        id: ID::new("confusion"),
        name: "Confusion".to_string(),
        base_power: 50,
        accuracy: 100,
        pp: 25,
        category: MoveCategory::Special,
        move_type: "Psychic".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 10,
            volatile_status: Some("confusion".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("constrict"), MoveDef {
        id: ID::new("constrict"),
        name: "Constrict".to_string(),
        base_power: 10,
        accuracy: 100,
        pp: 35,
        category: MoveCategory::Physical,
        move_type: "Normal".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 10,
            boosts: Some(vec![("spe".to_string(), -1)]),
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("continentalcrush"), MoveDef {
        id: ID::new("continentalcrush"),
        name: "Continental Crush".to_string(),
        base_power: 1,
        accuracy: 0,
        pp: 1,
        category: MoveCategory::Physical,
        move_type: "Rock".to_string(),
        is_z: true,
        ..Default::default()
    });

    map.insert(ID::new("conversion"), MoveDef {
        id: ID::new("conversion"),
        name: "Conversion".to_string(),
        accuracy: 0,
        pp: 30,
        category: MoveCategory::Status,
        move_type: "Normal".to_string(),
        target: MoveTargetType::Self_,
        ..Default::default()
    });

    map.insert(ID::new("conversion2"), MoveDef {
        id: ID::new("conversion2"),
        name: "Conversion 2".to_string(),
        accuracy: 0,
        pp: 30,
        category: MoveCategory::Status,
        move_type: "Normal".to_string(),
        flags: MoveFlags { bypasssub: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("copycat"), MoveDef {
        id: ID::new("copycat"),
        name: "Copycat".to_string(),
        accuracy: 0,
        pp: 20,
        category: MoveCategory::Status,
        move_type: "Normal".to_string(),
        target: MoveTargetType::Self_,
        ..Default::default()
    });

    map.insert(ID::new("coreenforcer"), MoveDef {
        id: ID::new("coreenforcer"),
        name: "Core Enforcer".to_string(),
        base_power: 100,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Special,
        move_type: "Dragon".to_string(),
        target: MoveTargetType::AllAdjacentFoes,
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("corkscrewcrash"), MoveDef {
        id: ID::new("corkscrewcrash"),
        name: "Corkscrew Crash".to_string(),
        base_power: 1,
        accuracy: 0,
        pp: 1,
        category: MoveCategory::Physical,
        move_type: "Steel".to_string(),
        is_z: true,
        ..Default::default()
    });

    map.insert(ID::new("corrosivegas"), MoveDef {
        id: ID::new("corrosivegas"),
        name: "Corrosive Gas".to_string(),
        accuracy: 100,
        pp: 40,
        category: MoveCategory::Status,
        move_type: "Poison".to_string(),
        target: MoveTargetType::AllAdjacent,
        flags: MoveFlags { protect: true, reflectable: true, mirror: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("cosmicpower"), MoveDef {
        id: ID::new("cosmicpower"),
        name: "Cosmic Power".to_string(),
        accuracy: 0,
        pp: 20,
        category: MoveCategory::Status,
        move_type: "Psychic".to_string(),
        target: MoveTargetType::Self_,
        self_boosts: Some(vec![("def".to_string(), 1), ("spd".to_string(), 1)]),
        ..Default::default()
    });

    map.insert(ID::new("cottonguard"), MoveDef {
        id: ID::new("cottonguard"),
        name: "Cotton Guard".to_string(),
        accuracy: 0,
        pp: 10,
        category: MoveCategory::Status,
        move_type: "Grass".to_string(),
        target: MoveTargetType::Self_,
        self_boosts: Some(vec![("def".to_string(), 3)]),
        ..Default::default()
    });

    map.insert(ID::new("cottonspore"), MoveDef {
        id: ID::new("cottonspore"),
        name: "Cotton Spore".to_string(),
        accuracy: 100,
        pp: 40,
        category: MoveCategory::Status,
        move_type: "Grass".to_string(),
        target: MoveTargetType::AllAdjacentFoes,
        flags: MoveFlags { protect: true, reflectable: true, mirror: true, powder: true, ..Default::default() },
        boosts: Some(vec![("spe".to_string(), -2)]),
        ..Default::default()
    });

    map.insert(ID::new("counter"), MoveDef {
        id: ID::new("counter"),
        name: "Counter".to_string(),
        base_power: 0,
        accuracy: 100,
        pp: 20,
        priority: -5,
        category: MoveCategory::Physical,
        move_type: "Fighting".to_string(),
        flags: MoveFlags { contact: true, protect: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("courtchange"), MoveDef {
        id: ID::new("courtchange"),
        name: "Court Change".to_string(),
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Status,
        move_type: "Normal".to_string(),
        target: MoveTargetType::All,
        flags: MoveFlags { mirror: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("covet"), MoveDef {
        id: ID::new("covet"),
        name: "Covet".to_string(),
        base_power: 60,
        accuracy: 100,
        pp: 25,
        category: MoveCategory::Physical,
        move_type: "Normal".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("crabhammer"), MoveDef {
        id: ID::new("crabhammer"),
        name: "Crabhammer".to_string(),
        base_power: 100,
        accuracy: 90,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Water".to_string(),
        crit_ratio: 1,
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("craftyshield"), MoveDef {
        id: ID::new("craftyshield"),
        name: "Crafty Shield".to_string(),
        accuracy: 0,
        pp: 10,
        priority: 3,
        category: MoveCategory::Status,
        move_type: "Fairy".to_string(),
        target: MoveTargetType::AllySide,
        side_condition: Some("craftyshield".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("crosschop"), MoveDef {
        id: ID::new("crosschop"),
        name: "Cross Chop".to_string(),
        base_power: 100,
        accuracy: 80,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Fighting".to_string(),
        crit_ratio: 1,
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("crosspoison"), MoveDef {
        id: ID::new("crosspoison"),
        name: "Cross Poison".to_string(),
        base_power: 70,
        accuracy: 100,
        pp: 20,
        category: MoveCategory::Physical,
        move_type: "Poison".to_string(),
        crit_ratio: 1,
        flags: MoveFlags { contact: true, protect: true, mirror: true, slicing: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 10,
            status: Some("psn".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("crunch"), MoveDef {
        id: ID::new("crunch"),
        name: "Crunch".to_string(),
        base_power: 80,
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Physical,
        move_type: "Dark".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, bite: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 20,
            boosts: Some(vec![("def".to_string(), -1)]),
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("crushclaw"), MoveDef {
        id: ID::new("crushclaw"),
        name: "Crush Claw".to_string(),
        base_power: 75,
        accuracy: 95,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Normal".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 50,
            boosts: Some(vec![("def".to_string(), -1)]),
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("crushgrip"), MoveDef {
        id: ID::new("crushgrip"),
        name: "Crush Grip".to_string(),
        base_power: 0,
        accuracy: 100,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Normal".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("curse"), MoveDef {
        id: ID::new("curse"),
        name: "Curse".to_string(),
        accuracy: 0,
        pp: 10,
        category: MoveCategory::Status,
        move_type: "Ghost".to_string(),
        flags: MoveFlags { bypasssub: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("cut"), MoveDef {
        id: ID::new("cut"),
        name: "Cut".to_string(),
        base_power: 50,
        accuracy: 95,
        pp: 30,
        category: MoveCategory::Physical,
        move_type: "Normal".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, slicing: true, ..Default::default() },
        ..Default::default()
    });

    // === D moves ===
    map.insert(ID::new("darkestlariat"), MoveDef {
        id: ID::new("darkestlariat"),
        name: "Darkest Lariat".to_string(),
        base_power: 85,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Dark".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        ignores_ability: true,
        ..Default::default()
    });

    map.insert(ID::new("darkpulse"), MoveDef {
        id: ID::new("darkpulse"),
        name: "Dark Pulse".to_string(),
        base_power: 80,
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Special,
        move_type: "Dark".to_string(),
        target: MoveTargetType::Any,
        flags: MoveFlags { protect: true, mirror: true, distance: true, pulse: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 20,
            volatile_status: Some("flinch".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("darkvoid"), MoveDef {
        id: ID::new("darkvoid"),
        name: "Dark Void".to_string(),
        accuracy: 50,
        pp: 10,
        category: MoveCategory::Status,
        move_type: "Dark".to_string(),
        target: MoveTargetType::AllAdjacentFoes,
        flags: MoveFlags { protect: true, reflectable: true, mirror: true, ..Default::default() },
        status: Some("slp".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("dazzlinggleam"), MoveDef {
        id: ID::new("dazzlinggleam"),
        name: "Dazzling Gleam".to_string(),
        base_power: 80,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Special,
        move_type: "Fairy".to_string(),
        target: MoveTargetType::AllAdjacentFoes,
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("decorate"), MoveDef {
        id: ID::new("decorate"),
        name: "Decorate".to_string(),
        accuracy: 0,
        pp: 15,
        category: MoveCategory::Status,
        move_type: "Fairy".to_string(),
        boosts: Some(vec![("atk".to_string(), 2), ("spa".to_string(), 2)]),
        ..Default::default()
    });

    map.insert(ID::new("defendorder"), MoveDef {
        id: ID::new("defendorder"),
        name: "Defend Order".to_string(),
        accuracy: 0,
        pp: 10,
        category: MoveCategory::Status,
        move_type: "Bug".to_string(),
        target: MoveTargetType::Self_,
        self_boosts: Some(vec![("def".to_string(), 1), ("spd".to_string(), 1)]),
        ..Default::default()
    });

    map.insert(ID::new("defensecurl"), MoveDef {
        id: ID::new("defensecurl"),
        name: "Defense Curl".to_string(),
        accuracy: 0,
        pp: 40,
        category: MoveCategory::Status,
        move_type: "Normal".to_string(),
        target: MoveTargetType::Self_,
        self_boosts: Some(vec![("def".to_string(), 1)]),
        self_volatile: Some("defensecurl".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("destinybond"), MoveDef {
        id: ID::new("destinybond"),
        name: "Destiny Bond".to_string(),
        accuracy: 0,
        pp: 5,
        category: MoveCategory::Status,
        move_type: "Ghost".to_string(),
        target: MoveTargetType::Self_,
        flags: MoveFlags { bypasssub: true, ..Default::default() },
        self_volatile: Some("destinybond".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("devastatingdrake"), MoveDef {
        id: ID::new("devastatingdrake"),
        name: "Devastating Drake".to_string(),
        base_power: 1,
        accuracy: 0,
        pp: 1,
        category: MoveCategory::Physical,
        move_type: "Dragon".to_string(),
        is_z: true,
        ..Default::default()
    });

    map.insert(ID::new("diamondstorm"), MoveDef {
        id: ID::new("diamondstorm"),
        name: "Diamond Storm".to_string(),
        base_power: 100,
        accuracy: 95,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Rock".to_string(),
        target: MoveTargetType::AllAdjacentFoes,
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 50,
            boosts: Some(vec![("def".to_string(), 2)]),
            self_boost: true,
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("dig"), MoveDef {
        id: ID::new("dig"),
        name: "Dig".to_string(),
        base_power: 80,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Ground".to_string(),
        flags: MoveFlags { contact: true, charge: true, protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("direclaw"), MoveDef {
        id: ID::new("direclaw"),
        name: "Dire Claw".to_string(),
        base_power: 80,
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Physical,
        move_type: "Poison".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("disable"), MoveDef {
        id: ID::new("disable"),
        name: "Disable".to_string(),
        accuracy: 100,
        pp: 20,
        category: MoveCategory::Status,
        move_type: "Normal".to_string(),
        flags: MoveFlags { protect: true, reflectable: true, mirror: true, bypasssub: true, ..Default::default() },
        volatile_status: Some("disable".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("disarmingvoice"), MoveDef {
        id: ID::new("disarmingvoice"),
        name: "Disarming Voice".to_string(),
        base_power: 40,
        accuracy: 0,
        pp: 15,
        category: MoveCategory::Special,
        move_type: "Fairy".to_string(),
        target: MoveTargetType::AllAdjacentFoes,
        flags: MoveFlags { protect: true, mirror: true, sound: true, bypasssub: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("discharge"), MoveDef {
        id: ID::new("discharge"),
        name: "Discharge".to_string(),
        base_power: 80,
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Special,
        move_type: "Electric".to_string(),
        target: MoveTargetType::AllAdjacent,
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 30,
            status: Some("par".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("dive"), MoveDef {
        id: ID::new("dive"),
        name: "Dive".to_string(),
        base_power: 80,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Water".to_string(),
        flags: MoveFlags { contact: true, charge: true, protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("dizzypunch"), MoveDef {
        id: ID::new("dizzypunch"),
        name: "Dizzy Punch".to_string(),
        base_power: 70,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Normal".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, punch: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 20,
            volatile_status: Some("confusion".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("doodle"), MoveDef {
        id: ID::new("doodle"),
        name: "Doodle".to_string(),
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Status,
        move_type: "Normal".to_string(),
        ..Default::default()
    });

    map.insert(ID::new("doomdesire"), MoveDef {
        id: ID::new("doomdesire"),
        name: "Doom Desire".to_string(),
        base_power: 140,
        accuracy: 100,
        pp: 5,
        category: MoveCategory::Special,
        move_type: "Steel".to_string(),
        ..Default::default()
    });

    map.insert(ID::new("doublehit"), MoveDef {
        id: ID::new("doublehit"),
        name: "Double Hit".to_string(),
        base_power: 35,
        accuracy: 90,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Normal".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        multi_hit: Some((2, 2)),
        ..Default::default()
    });

    map.insert(ID::new("doubleironbash"), MoveDef {
        id: ID::new("doubleironbash"),
        name: "Double Iron Bash".to_string(),
        base_power: 60,
        accuracy: 100,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Steel".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, punch: true, ..Default::default() },
        multi_hit: Some((2, 2)),
        secondaries: vec![SecondaryEffect {
            chance: 30,
            volatile_status: Some("flinch".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("doublekick"), MoveDef {
        id: ID::new("doublekick"),
        name: "Double Kick".to_string(),
        base_power: 30,
        accuracy: 100,
        pp: 30,
        category: MoveCategory::Physical,
        move_type: "Fighting".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        multi_hit: Some((2, 2)),
        ..Default::default()
    });

    map.insert(ID::new("doubleshock"), MoveDef {
        id: ID::new("doubleshock"),
        name: "Double Shock".to_string(),
        base_power: 120,
        accuracy: 100,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Electric".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("doubleslap"), MoveDef {
        id: ID::new("doubleslap"),
        name: "Double Slap".to_string(),
        base_power: 15,
        accuracy: 85,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Normal".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        multi_hit: Some((2, 5)),
        ..Default::default()
    });

    map.insert(ID::new("doubleteam"), MoveDef {
        id: ID::new("doubleteam"),
        name: "Double Team".to_string(),
        accuracy: 0,
        pp: 15,
        category: MoveCategory::Status,
        move_type: "Normal".to_string(),
        target: MoveTargetType::Self_,
        ..Default::default()
    });

    map.insert(ID::new("dracometeor"), MoveDef {
        id: ID::new("dracometeor"),
        name: "Draco Meteor".to_string(),
        base_power: 130,
        accuracy: 90,
        pp: 5,
        category: MoveCategory::Special,
        move_type: "Dragon".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        self_drops: Some(vec![("spa".to_string(), -2)]),
        ..Default::default()
    });

    map.insert(ID::new("dragonascent"), MoveDef {
        id: ID::new("dragonascent"),
        name: "Dragon Ascent".to_string(),
        base_power: 120,
        accuracy: 100,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Flying".to_string(),
        target: MoveTargetType::Any,
        flags: MoveFlags { contact: true, protect: true, mirror: true, distance: true, ..Default::default() },
        self_drops: Some(vec![("def".to_string(), -1), ("spd".to_string(), -1)]),
        ..Default::default()
    });

    map.insert(ID::new("dragonbreath"), MoveDef {
        id: ID::new("dragonbreath"),
        name: "Dragon Breath".to_string(),
        base_power: 60,
        accuracy: 100,
        pp: 20,
        category: MoveCategory::Special,
        move_type: "Dragon".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 30,
            status: Some("par".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("dragoncheer"), MoveDef {
        id: ID::new("dragoncheer"),
        name: "Dragon Cheer".to_string(),
        accuracy: 0,
        pp: 15,
        category: MoveCategory::Status,
        move_type: "Dragon".to_string(),
        target: MoveTargetType::Ally,
        flags: MoveFlags { bypasssub: true, ..Default::default() },
        volatile_status: Some("dragoncheer".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("dragonclaw"), MoveDef {
        id: ID::new("dragonclaw"),
        name: "Dragon Claw".to_string(),
        base_power: 80,
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Physical,
        move_type: "Dragon".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("dragondance"), MoveDef {
        id: ID::new("dragondance"),
        name: "Dragon Dance".to_string(),
        accuracy: 0,
        pp: 20,
        category: MoveCategory::Status,
        move_type: "Dragon".to_string(),
        target: MoveTargetType::Self_,
        flags: MoveFlags { dance: true, ..Default::default() },
        self_boosts: Some(vec![("atk".to_string(), 1), ("spe".to_string(), 1)]),
        ..Default::default()
    });

    map.insert(ID::new("dragondarts"), MoveDef {
        id: ID::new("dragondarts"),
        name: "Dragon Darts".to_string(),
        base_power: 50,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Dragon".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        multi_hit: Some((2, 2)),
        ..Default::default()
    });

    map.insert(ID::new("dragonenergy"), MoveDef {
        id: ID::new("dragonenergy"),
        name: "Dragon Energy".to_string(),
        base_power: 150,
        accuracy: 100,
        pp: 5,
        category: MoveCategory::Special,
        move_type: "Dragon".to_string(),
        target: MoveTargetType::AllAdjacentFoes,
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("dragonhammer"), MoveDef {
        id: ID::new("dragonhammer"),
        name: "Dragon Hammer".to_string(),
        base_power: 90,
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Physical,
        move_type: "Dragon".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("dragonpulse"), MoveDef {
        id: ID::new("dragonpulse"),
        name: "Dragon Pulse".to_string(),
        base_power: 85,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Special,
        move_type: "Dragon".to_string(),
        target: MoveTargetType::Any,
        flags: MoveFlags { protect: true, mirror: true, distance: true, pulse: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("dragonrage"), MoveDef {
        id: ID::new("dragonrage"),
        name: "Dragon Rage".to_string(),
        base_power: 0,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Special,
        move_type: "Dragon".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("dragonrush"), MoveDef {
        id: ID::new("dragonrush"),
        name: "Dragon Rush".to_string(),
        base_power: 100,
        accuracy: 75,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Dragon".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 20,
            volatile_status: Some("flinch".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("dragontail"), MoveDef {
        id: ID::new("dragontail"),
        name: "Dragon Tail".to_string(),
        base_power: 60,
        accuracy: 90,
        pp: 10,
        priority: -6,
        category: MoveCategory::Physical,
        move_type: "Dragon".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        force_switch: true,
        ..Default::default()
    });

    map.insert(ID::new("drainingkiss"), MoveDef {
        id: ID::new("drainingkiss"),
        name: "Draining Kiss".to_string(),
        base_power: 50,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Special,
        move_type: "Fairy".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, heal: true, ..Default::default() },
        drain: Some(0.75),
        ..Default::default()
    });

    map.insert(ID::new("drainpunch"), MoveDef {
        id: ID::new("drainpunch"),
        name: "Drain Punch".to_string(),
        base_power: 75,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Fighting".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, punch: true, heal: true, ..Default::default() },
        drain: Some(0.5),
        ..Default::default()
    });

    map.insert(ID::new("dreameater"), MoveDef {
        id: ID::new("dreameater"),
        name: "Dream Eater".to_string(),
        base_power: 100,
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Special,
        move_type: "Psychic".to_string(),
        flags: MoveFlags { protect: true, mirror: true, heal: true, ..Default::default() },
        drain: Some(0.5),
        ..Default::default()
    });

    map.insert(ID::new("drillpeck"), MoveDef {
        id: ID::new("drillpeck"),
        name: "Drill Peck".to_string(),
        base_power: 80,
        accuracy: 100,
        pp: 20,
        category: MoveCategory::Physical,
        move_type: "Flying".to_string(),
        target: MoveTargetType::Any,
        flags: MoveFlags { contact: true, protect: true, mirror: true, distance: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("drillrun"), MoveDef {
        id: ID::new("drillrun"),
        name: "Drill Run".to_string(),
        base_power: 80,
        accuracy: 95,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Ground".to_string(),
        crit_ratio: 1,
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("drumbeating"), MoveDef {
        id: ID::new("drumbeating"),
        name: "Drum Beating".to_string(),
        base_power: 80,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Grass".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 100,
            boosts: Some(vec![("spe".to_string(), -1)]),
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("dualchop"), MoveDef {
        id: ID::new("dualchop"),
        name: "Dual Chop".to_string(),
        base_power: 40,
        accuracy: 90,
        pp: 15,
        category: MoveCategory::Physical,
        move_type: "Dragon".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        multi_hit: Some((2, 2)),
        ..Default::default()
    });

    map.insert(ID::new("dualwingbeat"), MoveDef {
        id: ID::new("dualwingbeat"),
        name: "Dual Wingbeat".to_string(),
        base_power: 40,
        accuracy: 90,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Flying".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        multi_hit: Some((2, 2)),
        ..Default::default()
    });

    map.insert(ID::new("dynamaxcannon"), MoveDef {
        id: ID::new("dynamaxcannon"),
        name: "Dynamax Cannon".to_string(),
        base_power: 100,
        accuracy: 100,
        pp: 5,
        category: MoveCategory::Special,
        move_type: "Dragon".to_string(),
        flags: MoveFlags { protect: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("dynamicpunch"), MoveDef {
        id: ID::new("dynamicpunch"),
        name: "Dynamic Punch".to_string(),
        base_power: 100,
        accuracy: 50,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Fighting".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, punch: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 100,
            volatile_status: Some("confusion".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    });

    // === E moves ===
    map.insert(ID::new("earthpower"), MoveDef {
        id: ID::new("earthpower"),
        name: "Earth Power".to_string(),
        base_power: 90,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Special,
        move_type: "Ground".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 10,
            boosts: Some(vec![("spd".to_string(), -1)]),
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("echoedvoice"), MoveDef {
        id: ID::new("echoedvoice"),
        name: "Echoed Voice".to_string(),
        base_power: 40,
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Special,
        move_type: "Normal".to_string(),
        flags: MoveFlags { protect: true, mirror: true, sound: true, bypasssub: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("eerieimpulse"), MoveDef {
        id: ID::new("eerieimpulse"),
        name: "Eerie Impulse".to_string(),
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Status,
        move_type: "Electric".to_string(),
        flags: MoveFlags { protect: true, reflectable: true, mirror: true, ..Default::default() },
        boosts: Some(vec![("spa".to_string(), -2)]),
        ..Default::default()
    });

    map.insert(ID::new("eeriespell"), MoveDef {
        id: ID::new("eeriespell"),
        name: "Eerie Spell".to_string(),
        base_power: 80,
        accuracy: 100,
        pp: 5,
        category: MoveCategory::Special,
        move_type: "Psychic".to_string(),
        flags: MoveFlags { protect: true, mirror: true, sound: true, bypasssub: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("eggbomb"), MoveDef {
        id: ID::new("eggbomb"),
        name: "Egg Bomb".to_string(),
        base_power: 100,
        accuracy: 75,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Normal".to_string(),
        flags: MoveFlags { protect: true, mirror: true, bullet: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("electricterrain"), MoveDef {
        id: ID::new("electricterrain"),
        name: "Electric Terrain".to_string(),
        accuracy: 0,
        pp: 10,
        category: MoveCategory::Status,
        move_type: "Electric".to_string(),
        target: MoveTargetType::All,
        terrain: Some("electricterrain".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("electrify"), MoveDef {
        id: ID::new("electrify"),
        name: "Electrify".to_string(),
        accuracy: 0,
        pp: 20,
        category: MoveCategory::Status,
        move_type: "Electric".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        volatile_status: Some("electrify".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("electroball"), MoveDef {
        id: ID::new("electroball"),
        name: "Electro Ball".to_string(),
        base_power: 0,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Special,
        move_type: "Electric".to_string(),
        flags: MoveFlags { protect: true, mirror: true, bullet: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("electrodrift"), MoveDef {
        id: ID::new("electrodrift"),
        name: "Electro Drift".to_string(),
        base_power: 100,
        accuracy: 100,
        pp: 5,
        category: MoveCategory::Special,
        move_type: "Electric".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("electroshot"), MoveDef {
        id: ID::new("electroshot"),
        name: "Electro Shot".to_string(),
        base_power: 130,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Special,
        move_type: "Electric".to_string(),
        flags: MoveFlags { charge: true, protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("electroweb"), MoveDef {
        id: ID::new("electroweb"),
        name: "Electroweb".to_string(),
        base_power: 55,
        accuracy: 95,
        pp: 15,
        category: MoveCategory::Special,
        move_type: "Electric".to_string(),
        target: MoveTargetType::AllAdjacentFoes,
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 100,
            boosts: Some(vec![("spe".to_string(), -1)]),
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("embargo"), MoveDef {
        id: ID::new("embargo"),
        name: "Embargo".to_string(),
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Status,
        move_type: "Dark".to_string(),
        flags: MoveFlags { protect: true, reflectable: true, mirror: true, ..Default::default() },
        volatile_status: Some("embargo".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("ember"), MoveDef {
        id: ID::new("ember"),
        name: "Ember".to_string(),
        base_power: 40,
        accuracy: 100,
        pp: 25,
        category: MoveCategory::Special,
        move_type: "Fire".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 10,
            status: Some("brn".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("encore"), MoveDef {
        id: ID::new("encore"),
        name: "Encore".to_string(),
        accuracy: 100,
        pp: 5,
        category: MoveCategory::Status,
        move_type: "Normal".to_string(),
        flags: MoveFlags { protect: true, reflectable: true, mirror: true, bypasssub: true, failencore: true, ..Default::default() },
        volatile_status: Some("encore".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("endeavor"), MoveDef {
        id: ID::new("endeavor"),
        name: "Endeavor".to_string(),
        base_power: 0,
        accuracy: 100,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Normal".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("endure"), MoveDef {
        id: ID::new("endure"),
        name: "Endure".to_string(),
        accuracy: 0,
        pp: 10,
        priority: 4,
        category: MoveCategory::Status,
        move_type: "Normal".to_string(),
        target: MoveTargetType::Self_,
        self_volatile: Some("endure".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("energyball"), MoveDef {
        id: ID::new("energyball"),
        name: "Energy Ball".to_string(),
        base_power: 90,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Special,
        move_type: "Grass".to_string(),
        flags: MoveFlags { protect: true, mirror: true, bullet: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 10,
            boosts: Some(vec![("spd".to_string(), -1)]),
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("entrainment"), MoveDef {
        id: ID::new("entrainment"),
        name: "Entrainment".to_string(),
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Status,
        move_type: "Normal".to_string(),
        flags: MoveFlags { protect: true, reflectable: true, mirror: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("eruption"), MoveDef {
        id: ID::new("eruption"),
        name: "Eruption".to_string(),
        base_power: 150,
        accuracy: 100,
        pp: 5,
        category: MoveCategory::Special,
        move_type: "Fire".to_string(),
        target: MoveTargetType::AllAdjacentFoes,
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("esperwing"), MoveDef {
        id: ID::new("esperwing"),
        name: "Esper Wing".to_string(),
        base_power: 80,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Special,
        move_type: "Psychic".to_string(),
        crit_ratio: 1,
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 100,
            boosts: Some(vec![("spe".to_string(), 1)]),
            self_boost: true,
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("eternabeam"), MoveDef {
        id: ID::new("eternabeam"),
        name: "Eternabeam".to_string(),
        base_power: 160,
        accuracy: 90,
        pp: 5,
        category: MoveCategory::Special,
        move_type: "Dragon".to_string(),
        flags: MoveFlags { recharge: true, protect: true, mirror: true, ..Default::default() },
        self_volatile: Some("mustrecharge".to_string()),
        ..Default::default()
    });

    map.insert(ID::new("expandingforce"), MoveDef {
        id: ID::new("expandingforce"),
        name: "Expanding Force".to_string(),
        base_power: 80,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Special,
        move_type: "Psychic".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("explosion"), MoveDef {
        id: ID::new("explosion"),
        name: "Explosion".to_string(),
        base_power: 250,
        accuracy: 100,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Normal".to_string(),
        target: MoveTargetType::AllAdjacent,
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });

    map.insert(ID::new("extrasensory"), MoveDef {
        id: ID::new("extrasensory"),
        name: "Extrasensory".to_string(),
        base_power: 80,
        accuracy: 100,
        pp: 20,
        category: MoveCategory::Special,
        move_type: "Psychic".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect {
            chance: 10,
            volatile_status: Some("flinch".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    });

    map.insert(ID::new("extremeevoboost"), MoveDef {
        id: ID::new("extremeevoboost"),
        name: "Extreme Evoboost".to_string(),
        accuracy: 0,
        pp: 1,
        category: MoveCategory::Status,
        move_type: "Normal".to_string(),
        target: MoveTargetType::Self_,
        is_z: true,
        self_boosts: Some(vec![("atk".to_string(), 2), ("def".to_string(), 2), ("spa".to_string(), 2), ("spd".to_string(), 2), ("spe".to_string(), 2)]),
        ..Default::default()
    });

    map.insert(ID::new("extremespeed"), MoveDef {
        id: ID::new("extremespeed"),
        name: "Extreme Speed".to_string(),
        base_power: 80,
        accuracy: 100,
        pp: 5,
        priority: 2,
        category: MoveCategory::Physical,
        move_type: "Normal".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });

    // F moves
    map.insert(ID::new("facade"), MoveDef {
        id: ID::new("facade"),
        name: "Facade".to_string(),
        base_power: 70,
        accuracy: 100,
        pp: 20,
        category: MoveCategory::Physical,
        move_type: "Normal".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("fairylock"), MoveDef {
        id: ID::new("fairylock"),
        name: "Fairy Lock".to_string(),
        base_power: 0,
        accuracy: 0,
        pp: 10,
        category: MoveCategory::Status,
        move_type: "Fairy".to_string(),
        flags: MoveFlags { mirror: true, bypasssub: true, ..Default::default() },
        target: MoveTargetType::All,
        pseudo_weather: Some("fairylock".to_string()),
        ..Default::default()
    });
    map.insert(ID::new("fairywind"), MoveDef {
        id: ID::new("fairywind"),
        name: "Fairy Wind".to_string(),
        base_power: 40,
        accuracy: 100,
        pp: 30,
        category: MoveCategory::Special,
        move_type: "Fairy".to_string(),
        flags: MoveFlags { protect: true, mirror: true, wind: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("fakeout"), MoveDef {
        id: ID::new("fakeout"),
        name: "Fake Out".to_string(),
        base_power: 40,
        accuracy: 100,
        pp: 10,
        priority: 3,
        category: MoveCategory::Physical,
        move_type: "Normal".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect { chance: 100, volatile_status: Some("flinch".to_string()), ..Default::default() }],
        ..Default::default()
    });
    map.insert(ID::new("faketears"), MoveDef {
        id: ID::new("faketears"),
        name: "Fake Tears".to_string(),
        base_power: 0,
        accuracy: 100,
        pp: 20,
        category: MoveCategory::Status,
        move_type: "Dark".to_string(),
        flags: MoveFlags { protect: true, reflectable: true, mirror: true, ..Default::default() },
        boosts: Some(vec![("spd".to_string(), -2)]),
        ..Default::default()
    });
    map.insert(ID::new("falsesurrender"), MoveDef {
        id: ID::new("falsesurrender"),
        name: "False Surrender".to_string(),
        base_power: 80,
        accuracy: 0,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Dark".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("falseswipe"), MoveDef {
        id: ID::new("falseswipe"),
        name: "False Swipe".to_string(),
        base_power: 40,
        accuracy: 100,
        pp: 40,
        category: MoveCategory::Physical,
        move_type: "Normal".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("featherdance"), MoveDef {
        id: ID::new("featherdance"),
        name: "Feather Dance".to_string(),
        base_power: 0,
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Status,
        move_type: "Flying".to_string(),
        flags: MoveFlags { protect: true, reflectable: true, mirror: true, dance: true, ..Default::default() },
        boosts: Some(vec![("atk".to_string(), -2)]),
        ..Default::default()
    });
    map.insert(ID::new("feint"), MoveDef {
        id: ID::new("feint"),
        name: "Feint".to_string(),
        base_power: 30,
        accuracy: 100,
        pp: 10,
        priority: 2,
        category: MoveCategory::Physical,
        move_type: "Normal".to_string(),
        flags: MoveFlags { mirror: true, ..Default::default() },
        breaks_protect: true,
        ..Default::default()
    });
    map.insert(ID::new("feintattack"), MoveDef {
        id: ID::new("feintattack"),
        name: "Feint Attack".to_string(),
        base_power: 60,
        accuracy: 0,
        pp: 20,
        category: MoveCategory::Physical,
        move_type: "Dark".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("fellstinger"), MoveDef {
        id: ID::new("fellstinger"),
        name: "Fell Stinger".to_string(),
        base_power: 50,
        accuracy: 100,
        pp: 25,
        category: MoveCategory::Physical,
        move_type: "Bug".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("ficklebeam"), MoveDef {
        id: ID::new("ficklebeam"),
        name: "Fickle Beam".to_string(),
        base_power: 80,
        accuracy: 100,
        pp: 5,
        category: MoveCategory::Special,
        move_type: "Dragon".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("fierydance"), MoveDef {
        id: ID::new("fierydance"),
        name: "Fiery Dance".to_string(),
        base_power: 80,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Special,
        move_type: "Fire".to_string(),
        flags: MoveFlags { protect: true, mirror: true, dance: true, ..Default::default() },
        secondaries: vec![SecondaryEffect { chance: 50, boosts: Some(vec![("spa".to_string(), 1)]), self_boost: true, ..Default::default() }],
        ..Default::default()
    });
    map.insert(ID::new("fierywrath"), MoveDef {
        id: ID::new("fierywrath"),
        name: "Fiery Wrath".to_string(),
        base_power: 90,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Special,
        move_type: "Dark".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        target: MoveTargetType::AllAdjacentFoes,
        secondaries: vec![SecondaryEffect { chance: 20, volatile_status: Some("flinch".to_string()), ..Default::default() }],
        ..Default::default()
    });
    map.insert(ID::new("filletaway"), MoveDef {
        id: ID::new("filletaway"),
        name: "Fillet Away".to_string(),
        base_power: 0,
        accuracy: 0,
        pp: 10,
        category: MoveCategory::Status,
        move_type: "Normal".to_string(),
        flags: MoveFlags { snatch: true, ..Default::default() },
        target: MoveTargetType::Self_,
        self_boosts: Some(vec![("atk".to_string(), 2), ("spa".to_string(), 2), ("spe".to_string(), 2)]),
        ..Default::default()
    });
    map.insert(ID::new("finalgambit"), MoveDef {
        id: ID::new("finalgambit"),
        name: "Final Gambit".to_string(),
        base_power: 0,
        accuracy: 100,
        pp: 5,
        category: MoveCategory::Special,
        move_type: "Fighting".to_string(),
        flags: MoveFlags { protect: true, ..Default::default() },
        self_destruct: Some("ifHit".to_string()),
        ..Default::default()
    });
    map.insert(ID::new("fireblast"), MoveDef {
        id: ID::new("fireblast"),
        name: "Fire Blast".to_string(),
        base_power: 110,
        accuracy: 85,
        pp: 5,
        category: MoveCategory::Special,
        move_type: "Fire".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect { chance: 10, status: Some("brn".to_string()), ..Default::default() }],
        ..Default::default()
    });
    map.insert(ID::new("firefang"), MoveDef {
        id: ID::new("firefang"),
        name: "Fire Fang".to_string(),
        base_power: 65,
        accuracy: 95,
        pp: 15,
        category: MoveCategory::Physical,
        move_type: "Fire".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, bite: true, ..Default::default() },
        secondaries: vec![
            SecondaryEffect { chance: 10, status: Some("brn".to_string()), ..Default::default() },
            SecondaryEffect { chance: 10, volatile_status: Some("flinch".to_string()), ..Default::default() },
        ],
        ..Default::default()
    });
    map.insert(ID::new("firelash"), MoveDef {
        id: ID::new("firelash"),
        name: "Fire Lash".to_string(),
        base_power: 80,
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Physical,
        move_type: "Fire".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect { chance: 100, boosts: Some(vec![("def".to_string(), -1)]), ..Default::default() }],
        ..Default::default()
    });
    map.insert(ID::new("firepledge"), MoveDef {
        id: ID::new("firepledge"),
        name: "Fire Pledge".to_string(),
        base_power: 80,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Special,
        move_type: "Fire".to_string(),
        flags: MoveFlags { protect: true, mirror: true, nonsky: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("firepunch"), MoveDef {
        id: ID::new("firepunch"),
        name: "Fire Punch".to_string(),
        base_power: 75,
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Physical,
        move_type: "Fire".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, punch: true, ..Default::default() },
        secondaries: vec![SecondaryEffect { chance: 10, status: Some("brn".to_string()), ..Default::default() }],
        ..Default::default()
    });
    map.insert(ID::new("firespin"), MoveDef {
        id: ID::new("firespin"),
        name: "Fire Spin".to_string(),
        base_power: 35,
        accuracy: 85,
        pp: 15,
        category: MoveCategory::Special,
        move_type: "Fire".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        volatile_status: Some("partiallytrapped".to_string()),
        ..Default::default()
    });
    map.insert(ID::new("firstimpression"), MoveDef {
        id: ID::new("firstimpression"),
        name: "First Impression".to_string(),
        base_power: 90,
        accuracy: 100,
        pp: 10,
        priority: 2,
        category: MoveCategory::Physical,
        move_type: "Bug".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("fishiousrend"), MoveDef {
        id: ID::new("fishiousrend"),
        name: "Fishious Rend".to_string(),
        base_power: 85,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Water".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, bite: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("fissure"), MoveDef {
        id: ID::new("fissure"),
        name: "Fissure".to_string(),
        base_power: 0,
        accuracy: 30,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Ground".to_string(),
        flags: MoveFlags { protect: true, mirror: true, nonsky: true, ..Default::default() },
        ohko: true,
        ..Default::default()
    });
    map.insert(ID::new("flail"), MoveDef {
        id: ID::new("flail"),
        name: "Flail".to_string(),
        base_power: 0,
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Physical,
        move_type: "Normal".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("flameburst"), MoveDef {
        id: ID::new("flameburst"),
        name: "Flame Burst".to_string(),
        base_power: 70,
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Special,
        move_type: "Fire".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("flamecharge"), MoveDef {
        id: ID::new("flamecharge"),
        name: "Flame Charge".to_string(),
        base_power: 50,
        accuracy: 100,
        pp: 20,
        category: MoveCategory::Physical,
        move_type: "Fire".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect { chance: 100, boosts: Some(vec![("spe".to_string(), 1)]), self_boost: true, ..Default::default() }],
        ..Default::default()
    });
    map.insert(ID::new("flamethrower"), MoveDef {
        id: ID::new("flamethrower"),
        name: "Flamethrower".to_string(),
        base_power: 90,
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Special,
        move_type: "Fire".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect { chance: 10, status: Some("brn".to_string()), ..Default::default() }],
        ..Default::default()
    });
    map.insert(ID::new("flamewheel"), MoveDef {
        id: ID::new("flamewheel"),
        name: "Flame Wheel".to_string(),
        base_power: 60,
        accuracy: 100,
        pp: 25,
        category: MoveCategory::Physical,
        move_type: "Fire".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, defrost: true, ..Default::default() },
        secondaries: vec![SecondaryEffect { chance: 10, status: Some("brn".to_string()), ..Default::default() }],
        ..Default::default()
    });
    map.insert(ID::new("flareblitz"), MoveDef {
        id: ID::new("flareblitz"),
        name: "Flare Blitz".to_string(),
        base_power: 120,
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Physical,
        move_type: "Fire".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, defrost: true, ..Default::default() },
        recoil: Some(0.33),
        secondaries: vec![SecondaryEffect { chance: 10, status: Some("brn".to_string()), ..Default::default() }],
        ..Default::default()
    });
    map.insert(ID::new("flash"), MoveDef {
        id: ID::new("flash"),
        name: "Flash".to_string(),
        base_power: 0,
        accuracy: 100,
        pp: 20,
        category: MoveCategory::Status,
        move_type: "Normal".to_string(),
        flags: MoveFlags { protect: true, reflectable: true, mirror: true, ..Default::default() },
        boosts: Some(vec![("accuracy".to_string(), -1)]),
        ..Default::default()
    });
    map.insert(ID::new("flashcannon"), MoveDef {
        id: ID::new("flashcannon"),
        name: "Flash Cannon".to_string(),
        base_power: 80,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Special,
        move_type: "Steel".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect { chance: 10, boosts: Some(vec![("spd".to_string(), -1)]), ..Default::default() }],
        ..Default::default()
    });
    map.insert(ID::new("flatter"), MoveDef {
        id: ID::new("flatter"),
        name: "Flatter".to_string(),
        base_power: 0,
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Status,
        move_type: "Dark".to_string(),
        flags: MoveFlags { protect: true, reflectable: true, mirror: true, ..Default::default() },
        volatile_status: Some("confusion".to_string()),
        boosts: Some(vec![("spa".to_string(), 1)]),
        ..Default::default()
    });
    map.insert(ID::new("fleurcannon"), MoveDef {
        id: ID::new("fleurcannon"),
        name: "Fleur Cannon".to_string(),
        base_power: 130,
        accuracy: 90,
        pp: 5,
        category: MoveCategory::Special,
        move_type: "Fairy".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        self_drops: Some(vec![("spa".to_string(), -2)]),
        ..Default::default()
    });
    map.insert(ID::new("fling"), MoveDef {
        id: ID::new("fling"),
        name: "Fling".to_string(),
        base_power: 0,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Dark".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("flipturn"), MoveDef {
        id: ID::new("flipturn"),
        name: "Flip Turn".to_string(),
        base_power: 60,
        accuracy: 100,
        pp: 20,
        category: MoveCategory::Physical,
        move_type: "Water".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        is_pivot: true,
        ..Default::default()
    });
    map.insert(ID::new("floatyfall"), MoveDef {
        id: ID::new("floatyfall"),
        name: "Floaty Fall".to_string(),
        base_power: 90,
        accuracy: 95,
        pp: 15,
        category: MoveCategory::Physical,
        move_type: "Flying".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, gravity: true, ..Default::default() },
        secondaries: vec![SecondaryEffect { chance: 30, volatile_status: Some("flinch".to_string()), ..Default::default() }],
        ..Default::default()
    });
    map.insert(ID::new("floralhealing"), MoveDef {
        id: ID::new("floralhealing"),
        name: "Floral Healing".to_string(),
        base_power: 0,
        accuracy: 0,
        pp: 10,
        category: MoveCategory::Status,
        move_type: "Fairy".to_string(),
        flags: MoveFlags { protect: true, reflectable: true, heal: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("flowershield"), MoveDef {
        id: ID::new("flowershield"),
        name: "Flower Shield".to_string(),
        base_power: 0,
        accuracy: 0,
        pp: 10,
        category: MoveCategory::Status,
        move_type: "Fairy".to_string(),
        flags: MoveFlags { distance: true, ..Default::default() },
        target: MoveTargetType::All,
        ..Default::default()
    });
    map.insert(ID::new("flowertrick"), MoveDef {
        id: ID::new("flowertrick"),
        name: "Flower Trick".to_string(),
        base_power: 70,
        accuracy: 0,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Grass".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        will_crit: true,
        ..Default::default()
    });
    map.insert(ID::new("fly"), MoveDef {
        id: ID::new("fly"),
        name: "Fly".to_string(),
        base_power: 90,
        accuracy: 95,
        pp: 15,
        category: MoveCategory::Physical,
        move_type: "Flying".to_string(),
        flags: MoveFlags { contact: true, charge: true, protect: true, mirror: true, gravity: true, distance: true, ..Default::default() },
        target: MoveTargetType::Any,
        ..Default::default()
    });
    map.insert(ID::new("flyingpress"), MoveDef {
        id: ID::new("flyingpress"),
        name: "Flying Press".to_string(),
        base_power: 100,
        accuracy: 95,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Fighting".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, gravity: true, distance: true, nonsky: true, ..Default::default() },
        target: MoveTargetType::Any,
        ..Default::default()
    });
    map.insert(ID::new("focusblast"), MoveDef {
        id: ID::new("focusblast"),
        name: "Focus Blast".to_string(),
        base_power: 120,
        accuracy: 70,
        pp: 5,
        category: MoveCategory::Special,
        move_type: "Fighting".to_string(),
        flags: MoveFlags { protect: true, mirror: true, bullet: true, ..Default::default() },
        secondaries: vec![SecondaryEffect { chance: 10, boosts: Some(vec![("spd".to_string(), -1)]), ..Default::default() }],
        ..Default::default()
    });
    map.insert(ID::new("focusenergy"), MoveDef {
        id: ID::new("focusenergy"),
        name: "Focus Energy".to_string(),
        base_power: 0,
        accuracy: 0,
        pp: 30,
        category: MoveCategory::Status,
        move_type: "Normal".to_string(),
        flags: MoveFlags { snatch: true, ..Default::default() },
        self_volatile: Some("focusenergy".to_string()),
        target: MoveTargetType::Self_,
        ..Default::default()
    });
    map.insert(ID::new("focuspunch"), MoveDef {
        id: ID::new("focuspunch"),
        name: "Focus Punch".to_string(),
        base_power: 150,
        accuracy: 100,
        pp: 20,
        priority: -3,
        category: MoveCategory::Physical,
        move_type: "Fighting".to_string(),
        flags: MoveFlags { contact: true, protect: true, punch: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("followme"), MoveDef {
        id: ID::new("followme"),
        name: "Follow Me".to_string(),
        base_power: 0,
        accuracy: 0,
        pp: 20,
        priority: 2,
        category: MoveCategory::Status,
        move_type: "Normal".to_string(),
        flags: MoveFlags { ..Default::default() },
        self_volatile: Some("followme".to_string()),
        target: MoveTargetType::Self_,
        ..Default::default()
    });
    map.insert(ID::new("forcepalm"), MoveDef {
        id: ID::new("forcepalm"),
        name: "Force Palm".to_string(),
        base_power: 60,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Fighting".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect { chance: 30, status: Some("par".to_string()), ..Default::default() }],
        ..Default::default()
    });
    map.insert(ID::new("foresight"), MoveDef {
        id: ID::new("foresight"),
        name: "Foresight".to_string(),
        base_power: 0,
        accuracy: 0,
        pp: 40,
        category: MoveCategory::Status,
        move_type: "Normal".to_string(),
        flags: MoveFlags { protect: true, reflectable: true, mirror: true, bypasssub: true, ..Default::default() },
        volatile_status: Some("foresight".to_string()),
        ..Default::default()
    });
    map.insert(ID::new("forestscurse"), MoveDef {
        id: ID::new("forestscurse"),
        name: "Forest's Curse".to_string(),
        base_power: 0,
        accuracy: 100,
        pp: 20,
        category: MoveCategory::Status,
        move_type: "Grass".to_string(),
        flags: MoveFlags { protect: true, reflectable: true, mirror: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("foulplay"), MoveDef {
        id: ID::new("foulplay"),
        name: "Foul Play".to_string(),
        base_power: 95,
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Physical,
        move_type: "Dark".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("freezedry"), MoveDef {
        id: ID::new("freezedry"),
        name: "Freeze-Dry".to_string(),
        base_power: 70,
        accuracy: 100,
        pp: 20,
        category: MoveCategory::Special,
        move_type: "Ice".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect { chance: 10, status: Some("frz".to_string()), ..Default::default() }],
        ..Default::default()
    });
    map.insert(ID::new("freezeshock"), MoveDef {
        id: ID::new("freezeshock"),
        name: "Freeze Shock".to_string(),
        base_power: 140,
        accuracy: 90,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Ice".to_string(),
        flags: MoveFlags { charge: true, protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect { chance: 30, status: Some("par".to_string()), ..Default::default() }],
        ..Default::default()
    });
    map.insert(ID::new("freezingglare"), MoveDef {
        id: ID::new("freezingglare"),
        name: "Freezing Glare".to_string(),
        base_power: 90,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Special,
        move_type: "Psychic".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect { chance: 10, status: Some("frz".to_string()), ..Default::default() }],
        ..Default::default()
    });
    map.insert(ID::new("freezyfrost"), MoveDef {
        id: ID::new("freezyfrost"),
        name: "Freezy Frost".to_string(),
        base_power: 100,
        accuracy: 90,
        pp: 10,
        category: MoveCategory::Special,
        move_type: "Ice".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("frenzyplant"), MoveDef {
        id: ID::new("frenzyplant"),
        name: "Frenzy Plant".to_string(),
        base_power: 150,
        accuracy: 90,
        pp: 5,
        category: MoveCategory::Special,
        move_type: "Grass".to_string(),
        flags: MoveFlags { recharge: true, protect: true, mirror: true, nonsky: true, ..Default::default() },
        self_volatile: Some("mustrecharge".to_string()),
        ..Default::default()
    });
    map.insert(ID::new("frostbreath"), MoveDef {
        id: ID::new("frostbreath"),
        name: "Frost Breath".to_string(),
        base_power: 60,
        accuracy: 90,
        pp: 10,
        category: MoveCategory::Special,
        move_type: "Ice".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        will_crit: true,
        ..Default::default()
    });
    map.insert(ID::new("frustration"), MoveDef {
        id: ID::new("frustration"),
        name: "Frustration".to_string(),
        base_power: 0,
        accuracy: 100,
        pp: 20,
        category: MoveCategory::Physical,
        move_type: "Normal".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("furyattack"), MoveDef {
        id: ID::new("furyattack"),
        name: "Fury Attack".to_string(),
        base_power: 15,
        accuracy: 85,
        pp: 20,
        category: MoveCategory::Physical,
        move_type: "Normal".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        multi_hit: Some((2, 5)),
        ..Default::default()
    });
    map.insert(ID::new("furycutter"), MoveDef {
        id: ID::new("furycutter"),
        name: "Fury Cutter".to_string(),
        base_power: 40,
        accuracy: 95,
        pp: 20,
        category: MoveCategory::Physical,
        move_type: "Bug".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, slicing: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("furyswipes"), MoveDef {
        id: ID::new("furyswipes"),
        name: "Fury Swipes".to_string(),
        base_power: 18,
        accuracy: 80,
        pp: 15,
        category: MoveCategory::Physical,
        move_type: "Normal".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        multi_hit: Some((2, 5)),
        ..Default::default()
    });
    map.insert(ID::new("fusionbolt"), MoveDef {
        id: ID::new("fusionbolt"),
        name: "Fusion Bolt".to_string(),
        base_power: 100,
        accuracy: 100,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Electric".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("fusionflare"), MoveDef {
        id: ID::new("fusionflare"),
        name: "Fusion Flare".to_string(),
        base_power: 100,
        accuracy: 100,
        pp: 5,
        category: MoveCategory::Special,
        move_type: "Fire".to_string(),
        flags: MoveFlags { protect: true, mirror: true, defrost: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("futuresight"), MoveDef {
        id: ID::new("futuresight"),
        name: "Future Sight".to_string(),
        base_power: 120,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Special,
        move_type: "Psychic".to_string(),
        flags: MoveFlags { ..Default::default() },
        ignore_immunity: true,
        ..Default::default()
    });
    // G moves
    map.insert(ID::new("gastroacid"), MoveDef {
        id: ID::new("gastroacid"),
        name: "Gastro Acid".to_string(),
        base_power: 0,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Status,
        move_type: "Poison".to_string(),
        flags: MoveFlags { protect: true, reflectable: true, mirror: true, ..Default::default() },
        volatile_status: Some("gastroacid".to_string()),
        ..Default::default()
    });
    map.insert(ID::new("geargrind"), MoveDef {
        id: ID::new("geargrind"),
        name: "Gear Grind".to_string(),
        base_power: 50,
        accuracy: 85,
        pp: 15,
        category: MoveCategory::Physical,
        move_type: "Steel".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        multi_hit: Some((2, 2)),
        ..Default::default()
    });
    map.insert(ID::new("gearup"), MoveDef {
        id: ID::new("gearup"),
        name: "Gear Up".to_string(),
        base_power: 0,
        accuracy: 0,
        pp: 20,
        category: MoveCategory::Status,
        move_type: "Steel".to_string(),
        flags: MoveFlags { snatch: true, bypasssub: true, ..Default::default() },
        target: MoveTargetType::AllySide,
        ..Default::default()
    });
    map.insert(ID::new("genesissupernova"), MoveDef {
        id: ID::new("genesissupernova"),
        name: "Genesis Supernova".to_string(),
        base_power: 185,
        accuracy: 0,
        pp: 1,
        category: MoveCategory::Special,
        move_type: "Psychic".to_string(),
        flags: MoveFlags { ..Default::default() },
        is_z: true,
        ..Default::default()
    });
    map.insert(ID::new("geomancy"), MoveDef {
        id: ID::new("geomancy"),
        name: "Geomancy".to_string(),
        base_power: 0,
        accuracy: 0,
        pp: 10,
        category: MoveCategory::Status,
        move_type: "Fairy".to_string(),
        flags: MoveFlags { charge: true, nonsky: true, ..Default::default() },
        target: MoveTargetType::Self_,
        self_boosts: Some(vec![("spa".to_string(), 2), ("spd".to_string(), 2), ("spe".to_string(), 2)]),
        ..Default::default()
    });
    map.insert(ID::new("gigadrain"), MoveDef {
        id: ID::new("gigadrain"),
        name: "Giga Drain".to_string(),
        base_power: 75,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Special,
        move_type: "Grass".to_string(),
        flags: MoveFlags { protect: true, mirror: true, heal: true, ..Default::default() },
        drain: Some(0.5),
        ..Default::default()
    });
    map.insert(ID::new("gigaimpact"), MoveDef {
        id: ID::new("gigaimpact"),
        name: "Giga Impact".to_string(),
        base_power: 150,
        accuracy: 90,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Normal".to_string(),
        flags: MoveFlags { contact: true, recharge: true, protect: true, mirror: true, ..Default::default() },
        self_volatile: Some("mustrecharge".to_string()),
        ..Default::default()
    });
    map.insert(ID::new("gigatonhammer"), MoveDef {
        id: ID::new("gigatonhammer"),
        name: "Gigaton Hammer".to_string(),
        base_power: 160,
        accuracy: 100,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Steel".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("gigavolthavoc"), MoveDef {
        id: ID::new("gigavolthavoc"),
        name: "Gigavolt Havoc".to_string(),
        base_power: 1,
        accuracy: 0,
        pp: 1,
        category: MoveCategory::Physical,
        move_type: "Electric".to_string(),
        flags: MoveFlags { ..Default::default() },
        is_z: true,
        ..Default::default()
    });
    map.insert(ID::new("glaciallance"), MoveDef {
        id: ID::new("glaciallance"),
        name: "Glacial Lance".to_string(),
        base_power: 120,
        accuracy: 100,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Ice".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        target: MoveTargetType::AllAdjacentFoes,
        ..Default::default()
    });
    map.insert(ID::new("glaciate"), MoveDef {
        id: ID::new("glaciate"),
        name: "Glaciate".to_string(),
        base_power: 65,
        accuracy: 95,
        pp: 10,
        category: MoveCategory::Special,
        move_type: "Ice".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        target: MoveTargetType::AllAdjacentFoes,
        secondaries: vec![SecondaryEffect { chance: 100, boosts: Some(vec![("spe".to_string(), -1)]), ..Default::default() }],
        ..Default::default()
    });
    map.insert(ID::new("glaiverush"), MoveDef {
        id: ID::new("glaiverush"),
        name: "Glaive Rush".to_string(),
        base_power: 120,
        accuracy: 100,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Dragon".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        self_volatile: Some("glaiverush".to_string()),
        ..Default::default()
    });
    map.insert(ID::new("glare"), MoveDef {
        id: ID::new("glare"),
        name: "Glare".to_string(),
        base_power: 0,
        accuracy: 100,
        pp: 30,
        category: MoveCategory::Status,
        move_type: "Normal".to_string(),
        flags: MoveFlags { protect: true, reflectable: true, mirror: true, ..Default::default() },
        status: Some("par".to_string()),
        ..Default::default()
    });
    map.insert(ID::new("glitzyglow"), MoveDef {
        id: ID::new("glitzyglow"),
        name: "Glitzy Glow".to_string(),
        base_power: 80,
        accuracy: 95,
        pp: 15,
        category: MoveCategory::Special,
        move_type: "Psychic".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        side_condition: Some("lightscreen".to_string()),
        ..Default::default()
    });
    map.insert(ID::new("gmaxbefuddle"), MoveDef {
        id: ID::new("gmaxbefuddle"),
        name: "G-Max Befuddle".to_string(),
        base_power: 10,
        accuracy: 0,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Bug".to_string(),
        flags: MoveFlags { ..Default::default() },
        is_max: true,
        ..Default::default()
    });
    map.insert(ID::new("gmaxcannonade"), MoveDef {
        id: ID::new("gmaxcannonade"),
        name: "G-Max Cannonade".to_string(),
        base_power: 10,
        accuracy: 0,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Water".to_string(),
        flags: MoveFlags { ..Default::default() },
        is_max: true,
        ..Default::default()
    });
    map.insert(ID::new("gmaxcentiferno"), MoveDef {
        id: ID::new("gmaxcentiferno"),
        name: "G-Max Centiferno".to_string(),
        base_power: 10,
        accuracy: 0,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Fire".to_string(),
        flags: MoveFlags { ..Default::default() },
        is_max: true,
        ..Default::default()
    });
    map.insert(ID::new("gmaxchistrike"), MoveDef {
        id: ID::new("gmaxchistrike"),
        name: "G-Max Chi Strike".to_string(),
        base_power: 10,
        accuracy: 0,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Fighting".to_string(),
        flags: MoveFlags { ..Default::default() },
        is_max: true,
        ..Default::default()
    });
    map.insert(ID::new("gmaxcuddle"), MoveDef {
        id: ID::new("gmaxcuddle"),
        name: "G-Max Cuddle".to_string(),
        base_power: 10,
        accuracy: 0,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Normal".to_string(),
        flags: MoveFlags { ..Default::default() },
        is_max: true,
        ..Default::default()
    });
    map.insert(ID::new("gmaxdepletion"), MoveDef {
        id: ID::new("gmaxdepletion"),
        name: "G-Max Depletion".to_string(),
        base_power: 10,
        accuracy: 0,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Dragon".to_string(),
        flags: MoveFlags { ..Default::default() },
        is_max: true,
        ..Default::default()
    });
    map.insert(ID::new("gmaxdrumsolo"), MoveDef {
        id: ID::new("gmaxdrumsolo"),
        name: "G-Max Drum Solo".to_string(),
        base_power: 160,
        accuracy: 0,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Grass".to_string(),
        flags: MoveFlags { ..Default::default() },
        is_max: true,
        ignores_ability: true,
        ..Default::default()
    });
    map.insert(ID::new("gmaxfinale"), MoveDef {
        id: ID::new("gmaxfinale"),
        name: "G-Max Finale".to_string(),
        base_power: 10,
        accuracy: 0,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Fairy".to_string(),
        flags: MoveFlags { ..Default::default() },
        is_max: true,
        ..Default::default()
    });
    map.insert(ID::new("gmaxfireball"), MoveDef {
        id: ID::new("gmaxfireball"),
        name: "G-Max Fireball".to_string(),
        base_power: 160,
        accuracy: 0,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Fire".to_string(),
        flags: MoveFlags { ..Default::default() },
        is_max: true,
        ignores_ability: true,
        ..Default::default()
    });
    map.insert(ID::new("gmaxfoamburst"), MoveDef {
        id: ID::new("gmaxfoamburst"),
        name: "G-Max Foam Burst".to_string(),
        base_power: 10,
        accuracy: 0,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Water".to_string(),
        flags: MoveFlags { ..Default::default() },
        is_max: true,
        ..Default::default()
    });
    map.insert(ID::new("gmaxgoldrush"), MoveDef {
        id: ID::new("gmaxgoldrush"),
        name: "G-Max Gold Rush".to_string(),
        base_power: 10,
        accuracy: 0,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Normal".to_string(),
        flags: MoveFlags { ..Default::default() },
        is_max: true,
        ..Default::default()
    });
    map.insert(ID::new("gmaxgravitas"), MoveDef {
        id: ID::new("gmaxgravitas"),
        name: "G-Max Gravitas".to_string(),
        base_power: 10,
        accuracy: 0,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Psychic".to_string(),
        flags: MoveFlags { ..Default::default() },
        is_max: true,
        pseudo_weather: Some("gravity".to_string()),
        ..Default::default()
    });
    map.insert(ID::new("gmaxhydrosnipe"), MoveDef {
        id: ID::new("gmaxhydrosnipe"),
        name: "G-Max Hydrosnipe".to_string(),
        base_power: 160,
        accuracy: 0,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Water".to_string(),
        flags: MoveFlags { ..Default::default() },
        is_max: true,
        ignores_ability: true,
        ..Default::default()
    });
    map.insert(ID::new("gmaxmalodor"), MoveDef {
        id: ID::new("gmaxmalodor"),
        name: "G-Max Malodor".to_string(),
        base_power: 10,
        accuracy: 0,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Poison".to_string(),
        flags: MoveFlags { ..Default::default() },
        is_max: true,
        ..Default::default()
    });
    map.insert(ID::new("gmaxmeltdown"), MoveDef {
        id: ID::new("gmaxmeltdown"),
        name: "G-Max Meltdown".to_string(),
        base_power: 10,
        accuracy: 0,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Steel".to_string(),
        flags: MoveFlags { ..Default::default() },
        is_max: true,
        ..Default::default()
    });
    map.insert(ID::new("gmaxoneblow"), MoveDef {
        id: ID::new("gmaxoneblow"),
        name: "G-Max One Blow".to_string(),
        base_power: 10,
        accuracy: 0,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Dark".to_string(),
        flags: MoveFlags { ..Default::default() },
        is_max: true,
        breaks_protect: true,
        ..Default::default()
    });
    map.insert(ID::new("gmaxrapidflow"), MoveDef {
        id: ID::new("gmaxrapidflow"),
        name: "G-Max Rapid Flow".to_string(),
        base_power: 10,
        accuracy: 0,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Water".to_string(),
        flags: MoveFlags { ..Default::default() },
        is_max: true,
        breaks_protect: true,
        ..Default::default()
    });
    map.insert(ID::new("gmaxreplenish"), MoveDef {
        id: ID::new("gmaxreplenish"),
        name: "G-Max Replenish".to_string(),
        base_power: 10,
        accuracy: 0,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Normal".to_string(),
        flags: MoveFlags { ..Default::default() },
        is_max: true,
        ..Default::default()
    });
    map.insert(ID::new("gmaxresonance"), MoveDef {
        id: ID::new("gmaxresonance"),
        name: "G-Max Resonance".to_string(),
        base_power: 10,
        accuracy: 0,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Ice".to_string(),
        flags: MoveFlags { ..Default::default() },
        is_max: true,
        side_condition: Some("auroraveil".to_string()),
        ..Default::default()
    });
    map.insert(ID::new("gmaxsandblast"), MoveDef {
        id: ID::new("gmaxsandblast"),
        name: "G-Max Sandblast".to_string(),
        base_power: 10,
        accuracy: 0,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Ground".to_string(),
        flags: MoveFlags { ..Default::default() },
        is_max: true,
        ..Default::default()
    });
    map.insert(ID::new("gmaxsmite"), MoveDef {
        id: ID::new("gmaxsmite"),
        name: "G-Max Smite".to_string(),
        base_power: 10,
        accuracy: 0,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Fairy".to_string(),
        flags: MoveFlags { ..Default::default() },
        is_max: true,
        ..Default::default()
    });
    map.insert(ID::new("gmaxsnooze"), MoveDef {
        id: ID::new("gmaxsnooze"),
        name: "G-Max Snooze".to_string(),
        base_power: 10,
        accuracy: 0,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Dark".to_string(),
        flags: MoveFlags { ..Default::default() },
        is_max: true,
        ..Default::default()
    });
    map.insert(ID::new("gmaxsteelsurge"), MoveDef {
        id: ID::new("gmaxsteelsurge"),
        name: "G-Max Steelsurge".to_string(),
        base_power: 10,
        accuracy: 0,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Steel".to_string(),
        flags: MoveFlags { ..Default::default() },
        is_max: true,
        ..Default::default()
    });
    map.insert(ID::new("gmaxstonesurge"), MoveDef {
        id: ID::new("gmaxstonesurge"),
        name: "G-Max Stonesurge".to_string(),
        base_power: 10,
        accuracy: 0,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Water".to_string(),
        flags: MoveFlags { ..Default::default() },
        is_max: true,
        ..Default::default()
    });
    map.insert(ID::new("gmaxstunshock"), MoveDef {
        id: ID::new("gmaxstunshock"),
        name: "G-Max Stun Shock".to_string(),
        base_power: 10,
        accuracy: 0,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Electric".to_string(),
        flags: MoveFlags { ..Default::default() },
        is_max: true,
        ..Default::default()
    });
    map.insert(ID::new("gmaxsweetness"), MoveDef {
        id: ID::new("gmaxsweetness"),
        name: "G-Max Sweetness".to_string(),
        base_power: 10,
        accuracy: 0,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Grass".to_string(),
        flags: MoveFlags { ..Default::default() },
        is_max: true,
        ..Default::default()
    });
    map.insert(ID::new("gmaxtartness"), MoveDef {
        id: ID::new("gmaxtartness"),
        name: "G-Max Tartness".to_string(),
        base_power: 10,
        accuracy: 0,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Grass".to_string(),
        flags: MoveFlags { ..Default::default() },
        is_max: true,
        ..Default::default()
    });
    map.insert(ID::new("gmaxterror"), MoveDef {
        id: ID::new("gmaxterror"),
        name: "G-Max Terror".to_string(),
        base_power: 10,
        accuracy: 0,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Ghost".to_string(),
        flags: MoveFlags { ..Default::default() },
        is_max: true,
        ..Default::default()
    });
    map.insert(ID::new("gmaxvinelash"), MoveDef {
        id: ID::new("gmaxvinelash"),
        name: "G-Max Vine Lash".to_string(),
        base_power: 10,
        accuracy: 0,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Grass".to_string(),
        flags: MoveFlags { ..Default::default() },
        is_max: true,
        ..Default::default()
    });
    map.insert(ID::new("gmaxvolcalith"), MoveDef {
        id: ID::new("gmaxvolcalith"),
        name: "G-Max Volcalith".to_string(),
        base_power: 10,
        accuracy: 0,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Rock".to_string(),
        flags: MoveFlags { ..Default::default() },
        is_max: true,
        ..Default::default()
    });
    map.insert(ID::new("gmaxvoltcrash"), MoveDef {
        id: ID::new("gmaxvoltcrash"),
        name: "G-Max Volt Crash".to_string(),
        base_power: 10,
        accuracy: 0,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Electric".to_string(),
        flags: MoveFlags { ..Default::default() },
        is_max: true,
        ..Default::default()
    });
    map.insert(ID::new("gmaxwildfire"), MoveDef {
        id: ID::new("gmaxwildfire"),
        name: "G-Max Wildfire".to_string(),
        base_power: 10,
        accuracy: 0,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Fire".to_string(),
        flags: MoveFlags { ..Default::default() },
        is_max: true,
        ..Default::default()
    });
    map.insert(ID::new("gmaxwindrage"), MoveDef {
        id: ID::new("gmaxwindrage"),
        name: "G-Max Wind Rage".to_string(),
        base_power: 10,
        accuracy: 0,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Flying".to_string(),
        flags: MoveFlags { ..Default::default() },
        is_max: true,
        ..Default::default()
    });
    map.insert(ID::new("grassknot"), MoveDef {
        id: ID::new("grassknot"),
        name: "Grass Knot".to_string(),
        base_power: 0,
        accuracy: 100,
        pp: 20,
        category: MoveCategory::Special,
        move_type: "Grass".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, nonsky: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("grasspledge"), MoveDef {
        id: ID::new("grasspledge"),
        name: "Grass Pledge".to_string(),
        base_power: 80,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Special,
        move_type: "Grass".to_string(),
        flags: MoveFlags { protect: true, mirror: true, nonsky: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("grasswhistle"), MoveDef {
        id: ID::new("grasswhistle"),
        name: "Grass Whistle".to_string(),
        base_power: 0,
        accuracy: 55,
        pp: 15,
        category: MoveCategory::Status,
        move_type: "Grass".to_string(),
        flags: MoveFlags { protect: true, reflectable: true, mirror: true, sound: true, bypasssub: true, ..Default::default() },
        status: Some("slp".to_string()),
        ..Default::default()
    });
    map.insert(ID::new("grassyglide"), MoveDef {
        id: ID::new("grassyglide"),
        name: "Grassy Glide".to_string(),
        base_power: 55,
        accuracy: 100,
        pp: 20,
        category: MoveCategory::Physical,
        move_type: "Grass".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("grassyterrain"), MoveDef {
        id: ID::new("grassyterrain"),
        name: "Grassy Terrain".to_string(),
        base_power: 0,
        accuracy: 0,
        pp: 10,
        category: MoveCategory::Status,
        move_type: "Grass".to_string(),
        flags: MoveFlags { nonsky: true, ..Default::default() },
        terrain: Some("grassyterrain".to_string()),
        target: MoveTargetType::All,
        ..Default::default()
    });
    map.insert(ID::new("gravapple"), MoveDef {
        id: ID::new("gravapple"),
        name: "Grav Apple".to_string(),
        base_power: 80,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Grass".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect { chance: 100, boosts: Some(vec![("def".to_string(), -1)]), ..Default::default() }],
        ..Default::default()
    });
    map.insert(ID::new("gravity"), MoveDef {
        id: ID::new("gravity"),
        name: "Gravity".to_string(),
        base_power: 0,
        accuracy: 0,
        pp: 5,
        category: MoveCategory::Status,
        move_type: "Psychic".to_string(),
        flags: MoveFlags { nonsky: true, ..Default::default() },
        pseudo_weather: Some("gravity".to_string()),
        target: MoveTargetType::All,
        ..Default::default()
    });
    map.insert(ID::new("growl"), MoveDef {
        id: ID::new("growl"),
        name: "Growl".to_string(),
        base_power: 0,
        accuracy: 100,
        pp: 40,
        category: MoveCategory::Status,
        move_type: "Normal".to_string(),
        flags: MoveFlags { protect: true, reflectable: true, mirror: true, sound: true, bypasssub: true, ..Default::default() },
        target: MoveTargetType::AllAdjacentFoes,
        boosts: Some(vec![("atk".to_string(), -1)]),
        ..Default::default()
    });
    map.insert(ID::new("growth"), MoveDef {
        id: ID::new("growth"),
        name: "Growth".to_string(),
        base_power: 0,
        accuracy: 0,
        pp: 20,
        category: MoveCategory::Status,
        move_type: "Normal".to_string(),
        flags: MoveFlags { snatch: true, ..Default::default() },
        target: MoveTargetType::Self_,
        self_boosts: Some(vec![("atk".to_string(), 1), ("spa".to_string(), 1)]),
        ..Default::default()
    });
    map.insert(ID::new("grudge"), MoveDef {
        id: ID::new("grudge"),
        name: "Grudge".to_string(),
        base_power: 0,
        accuracy: 0,
        pp: 5,
        category: MoveCategory::Status,
        move_type: "Ghost".to_string(),
        flags: MoveFlags { bypasssub: true, ..Default::default() },
        target: MoveTargetType::Self_,
        self_volatile: Some("grudge".to_string()),
        ..Default::default()
    });
    map.insert(ID::new("guardianofalola"), MoveDef {
        id: ID::new("guardianofalola"),
        name: "Guardian of Alola".to_string(),
        base_power: 0,
        accuracy: 0,
        pp: 1,
        category: MoveCategory::Special,
        move_type: "Fairy".to_string(),
        flags: MoveFlags { ..Default::default() },
        is_z: true,
        ..Default::default()
    });
    map.insert(ID::new("guardsplit"), MoveDef {
        id: ID::new("guardsplit"),
        name: "Guard Split".to_string(),
        base_power: 0,
        accuracy: 0,
        pp: 10,
        category: MoveCategory::Status,
        move_type: "Psychic".to_string(),
        flags: MoveFlags { protect: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("guardswap"), MoveDef {
        id: ID::new("guardswap"),
        name: "Guard Swap".to_string(),
        base_power: 0,
        accuracy: 0,
        pp: 10,
        category: MoveCategory::Status,
        move_type: "Psychic".to_string(),
        flags: MoveFlags { protect: true, mirror: true, bypasssub: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("guillotine"), MoveDef {
        id: ID::new("guillotine"),
        name: "Guillotine".to_string(),
        base_power: 0,
        accuracy: 30,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Normal".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        ohko: true,
        ..Default::default()
    });
    map.insert(ID::new("gunkshot"), MoveDef {
        id: ID::new("gunkshot"),
        name: "Gunk Shot".to_string(),
        base_power: 120,
        accuracy: 80,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Poison".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect { chance: 30, status: Some("psn".to_string()), ..Default::default() }],
        ..Default::default()
    });
    map.insert(ID::new("gust"), MoveDef {
        id: ID::new("gust"),
        name: "Gust".to_string(),
        base_power: 40,
        accuracy: 100,
        pp: 35,
        category: MoveCategory::Special,
        move_type: "Flying".to_string(),
        flags: MoveFlags { protect: true, mirror: true, distance: true, wind: true, ..Default::default() },
        target: MoveTargetType::Any,
        ..Default::default()
    });
    map.insert(ID::new("gyroball"), MoveDef {
        id: ID::new("gyroball"),
        name: "Gyro Ball".to_string(),
        base_power: 0,
        accuracy: 100,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Steel".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, bullet: true, ..Default::default() },
        ..Default::default()
    });
    // H moves
    map.insert(ID::new("hail"), MoveDef {
        id: ID::new("hail"),
        name: "Hail".to_string(),
        base_power: 0,
        accuracy: 0,
        pp: 10,
        category: MoveCategory::Status,
        move_type: "Ice".to_string(),
        flags: MoveFlags { ..Default::default() },
        weather: Some("hail".to_string()),
        target: MoveTargetType::All,
        ..Default::default()
    });
    map.insert(ID::new("hammerarm"), MoveDef {
        id: ID::new("hammerarm"),
        name: "Hammer Arm".to_string(),
        base_power: 100,
        accuracy: 90,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Fighting".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, punch: true, ..Default::default() },
        self_drops: Some(vec![("spe".to_string(), -1)]),
        ..Default::default()
    });
    map.insert(ID::new("happyhour"), MoveDef {
        id: ID::new("happyhour"),
        name: "Happy Hour".to_string(),
        base_power: 0,
        accuracy: 0,
        pp: 30,
        category: MoveCategory::Status,
        move_type: "Normal".to_string(),
        flags: MoveFlags { ..Default::default() },
        target: MoveTargetType::AllySide,
        ..Default::default()
    });
    map.insert(ID::new("harden"), MoveDef {
        id: ID::new("harden"),
        name: "Harden".to_string(),
        base_power: 0,
        accuracy: 0,
        pp: 30,
        category: MoveCategory::Status,
        move_type: "Normal".to_string(),
        flags: MoveFlags { snatch: true, ..Default::default() },
        target: MoveTargetType::Self_,
        self_boosts: Some(vec![("def".to_string(), 1)]),
        ..Default::default()
    });
    map.insert(ID::new("hardpress"), MoveDef {
        id: ID::new("hardpress"),
        name: "Hard Press".to_string(),
        base_power: 0,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Steel".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("haze"), MoveDef {
        id: ID::new("haze"),
        name: "Haze".to_string(),
        base_power: 0,
        accuracy: 0,
        pp: 30,
        category: MoveCategory::Status,
        move_type: "Ice".to_string(),
        flags: MoveFlags { bypasssub: true, ..Default::default() },
        target: MoveTargetType::All,
        ..Default::default()
    });
    map.insert(ID::new("headbutt"), MoveDef {
        id: ID::new("headbutt"),
        name: "Headbutt".to_string(),
        base_power: 70,
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Physical,
        move_type: "Normal".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect { chance: 30, volatile_status: Some("flinch".to_string()), ..Default::default() }],
        ..Default::default()
    });
    map.insert(ID::new("headcharge"), MoveDef {
        id: ID::new("headcharge"),
        name: "Head Charge".to_string(),
        base_power: 120,
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Physical,
        move_type: "Normal".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        recoil: Some(0.25),
        ..Default::default()
    });
    map.insert(ID::new("headlongrush"), MoveDef {
        id: ID::new("headlongrush"),
        name: "Headlong Rush".to_string(),
        base_power: 120,
        accuracy: 100,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Ground".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, punch: true, ..Default::default() },
        self_drops: Some(vec![("def".to_string(), -1), ("spd".to_string(), -1)]),
        ..Default::default()
    });
    map.insert(ID::new("headsmash"), MoveDef {
        id: ID::new("headsmash"),
        name: "Head Smash".to_string(),
        base_power: 150,
        accuracy: 80,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Rock".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        recoil: Some(0.5),
        ..Default::default()
    });
    map.insert(ID::new("healbell"), MoveDef {
        id: ID::new("healbell"),
        name: "Heal Bell".to_string(),
        base_power: 0,
        accuracy: 0,
        pp: 5,
        category: MoveCategory::Status,
        move_type: "Normal".to_string(),
        flags: MoveFlags { snatch: true, sound: true, distance: true, bypasssub: true, ..Default::default() },
        target: MoveTargetType::AllySide,
        ..Default::default()
    });
    map.insert(ID::new("healblock"), MoveDef {
        id: ID::new("healblock"),
        name: "Heal Block".to_string(),
        base_power: 0,
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Status,
        move_type: "Psychic".to_string(),
        flags: MoveFlags { protect: true, reflectable: true, mirror: true, ..Default::default() },
        volatile_status: Some("healblock".to_string()),
        target: MoveTargetType::AllAdjacentFoes,
        ..Default::default()
    });
    map.insert(ID::new("healingwish"), MoveDef {
        id: ID::new("healingwish"),
        name: "Healing Wish".to_string(),
        base_power: 0,
        accuracy: 0,
        pp: 10,
        category: MoveCategory::Status,
        move_type: "Psychic".to_string(),
        flags: MoveFlags { snatch: true, heal: true, ..Default::default() },
        target: MoveTargetType::Self_,
        self_destruct: Some("ifHit".to_string()),
        ..Default::default()
    });
    map.insert(ID::new("healorder"), MoveDef {
        id: ID::new("healorder"),
        name: "Heal Order".to_string(),
        base_power: 0,
        accuracy: 0,
        pp: 10,
        category: MoveCategory::Status,
        move_type: "Bug".to_string(),
        flags: MoveFlags { snatch: true, heal: true, ..Default::default() },
        target: MoveTargetType::Self_,
        heal: Some(0.5),
        ..Default::default()
    });
    map.insert(ID::new("healpulse"), MoveDef {
        id: ID::new("healpulse"),
        name: "Heal Pulse".to_string(),
        base_power: 0,
        accuracy: 0,
        pp: 10,
        category: MoveCategory::Status,
        move_type: "Psychic".to_string(),
        flags: MoveFlags { protect: true, reflectable: true, distance: true, heal: true, pulse: true, ..Default::default() },
        target: MoveTargetType::Any,
        ..Default::default()
    });
    map.insert(ID::new("heartstamp"), MoveDef {
        id: ID::new("heartstamp"),
        name: "Heart Stamp".to_string(),
        base_power: 60,
        accuracy: 100,
        pp: 25,
        category: MoveCategory::Physical,
        move_type: "Psychic".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        secondaries: vec![SecondaryEffect { chance: 30, volatile_status: Some("flinch".to_string()), ..Default::default() }],
        ..Default::default()
    });
    map.insert(ID::new("heartswap"), MoveDef {
        id: ID::new("heartswap"),
        name: "Heart Swap".to_string(),
        base_power: 0,
        accuracy: 0,
        pp: 10,
        category: MoveCategory::Status,
        move_type: "Psychic".to_string(),
        flags: MoveFlags { protect: true, mirror: true, bypasssub: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("heatcrash"), MoveDef {
        id: ID::new("heatcrash"),
        name: "Heat Crash".to_string(),
        base_power: 0,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Fire".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, nonsky: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("heatwave"), MoveDef {
        id: ID::new("heatwave"),
        name: "Heat Wave".to_string(),
        base_power: 95,
        accuracy: 90,
        pp: 10,
        category: MoveCategory::Special,
        move_type: "Fire".to_string(),
        flags: MoveFlags { protect: true, mirror: true, wind: true, ..Default::default() },
        target: MoveTargetType::AllAdjacentFoes,
        secondaries: vec![SecondaryEffect { chance: 10, status: Some("brn".to_string()), ..Default::default() }],
        ..Default::default()
    });
    map.insert(ID::new("heavyslam"), MoveDef {
        id: ID::new("heavyslam"),
        name: "Heavy Slam".to_string(),
        base_power: 0,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Steel".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, nonsky: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("helpinghand"), MoveDef {
        id: ID::new("helpinghand"),
        name: "Helping Hand".to_string(),
        base_power: 0,
        accuracy: 0,
        pp: 20,
        priority: 5,
        category: MoveCategory::Status,
        move_type: "Normal".to_string(),
        flags: MoveFlags { bypasssub: true, ..Default::default() },
        target: MoveTargetType::Ally,
        volatile_status: Some("helpinghand".to_string()),
        ..Default::default()
    });
    map.insert(ID::new("hex"), MoveDef {
        id: ID::new("hex"),
        name: "Hex".to_string(),
        base_power: 65,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Special,
        move_type: "Ghost".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("hiddenpower"), MoveDef {
        id: ID::new("hiddenpower"),
        name: "Hidden Power".to_string(),
        base_power: 60,
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Special,
        move_type: "Normal".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("hiddenpowerbug"), MoveDef {
        id: ID::new("hiddenpowerbug"),
        name: "Hidden Power Bug".to_string(),
        base_power: 60,
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Special,
        move_type: "Bug".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("hiddenpowerdark"), MoveDef {
        id: ID::new("hiddenpowerdark"),
        name: "Hidden Power Dark".to_string(),
        base_power: 60,
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Special,
        move_type: "Dark".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("hiddenpowerdragon"), MoveDef {
        id: ID::new("hiddenpowerdragon"),
        name: "Hidden Power Dragon".to_string(),
        base_power: 60,
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Special,
        move_type: "Dragon".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("hiddenpowerelectric"), MoveDef {
        id: ID::new("hiddenpowerelectric"),
        name: "Hidden Power Electric".to_string(),
        base_power: 60,
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Special,
        move_type: "Electric".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("hiddenpowerfighting"), MoveDef {
        id: ID::new("hiddenpowerfighting"),
        name: "Hidden Power Fighting".to_string(),
        base_power: 60,
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Special,
        move_type: "Fighting".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("hiddenpowerfire"), MoveDef {
        id: ID::new("hiddenpowerfire"),
        name: "Hidden Power Fire".to_string(),
        base_power: 60,
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Special,
        move_type: "Fire".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("hiddenpowerflying"), MoveDef {
        id: ID::new("hiddenpowerflying"),
        name: "Hidden Power Flying".to_string(),
        base_power: 60,
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Special,
        move_type: "Flying".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("hiddenpowerghost"), MoveDef {
        id: ID::new("hiddenpowerghost"),
        name: "Hidden Power Ghost".to_string(),
        base_power: 60,
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Special,
        move_type: "Ghost".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("hiddenpowergrass"), MoveDef {
        id: ID::new("hiddenpowergrass"),
        name: "Hidden Power Grass".to_string(),
        base_power: 60,
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Special,
        move_type: "Grass".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("hiddenpowerground"), MoveDef {
        id: ID::new("hiddenpowerground"),
        name: "Hidden Power Ground".to_string(),
        base_power: 60,
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Special,
        move_type: "Ground".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("hiddenpowerice"), MoveDef {
        id: ID::new("hiddenpowerice"),
        name: "Hidden Power Ice".to_string(),
        base_power: 60,
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Special,
        move_type: "Ice".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("hiddenpowerpoison"), MoveDef {
        id: ID::new("hiddenpowerpoison"),
        name: "Hidden Power Poison".to_string(),
        base_power: 60,
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Special,
        move_type: "Poison".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("hiddenpowerpsychic"), MoveDef {
        id: ID::new("hiddenpowerpsychic"),
        name: "Hidden Power Psychic".to_string(),
        base_power: 60,
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Special,
        move_type: "Psychic".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("hiddenpowerrock"), MoveDef {
        id: ID::new("hiddenpowerrock"),
        name: "Hidden Power Rock".to_string(),
        base_power: 60,
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Special,
        move_type: "Rock".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("hiddenpowersteel"), MoveDef {
        id: ID::new("hiddenpowersteel"),
        name: "Hidden Power Steel".to_string(),
        base_power: 60,
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Special,
        move_type: "Steel".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("hiddenpowerwater"), MoveDef {
        id: ID::new("hiddenpowerwater"),
        name: "Hidden Power Water".to_string(),
        base_power: 60,
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Special,
        move_type: "Water".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("highhorsepower"), MoveDef {
        id: ID::new("highhorsepower"),
        name: "High Horsepower".to_string(),
        base_power: 95,
        accuracy: 95,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Ground".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("highjumpkick"), MoveDef {
        id: ID::new("highjumpkick"),
        name: "High Jump Kick".to_string(),
        base_power: 130,
        accuracy: 90,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Fighting".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, gravity: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("holdback"), MoveDef {
        id: ID::new("holdback"),
        name: "Hold Back".to_string(),
        base_power: 40,
        accuracy: 100,
        pp: 40,
        category: MoveCategory::Physical,
        move_type: "Normal".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("holdhands"), MoveDef {
        id: ID::new("holdhands"),
        name: "Hold Hands".to_string(),
        base_power: 0,
        accuracy: 0,
        pp: 40,
        category: MoveCategory::Status,
        move_type: "Normal".to_string(),
        flags: MoveFlags { bypasssub: true, ..Default::default() },
        target: MoveTargetType::Ally,
        ..Default::default()
    });
    map.insert(ID::new("honeclaws"), MoveDef {
        id: ID::new("honeclaws"),
        name: "Hone Claws".to_string(),
        base_power: 0,
        accuracy: 0,
        pp: 15,
        category: MoveCategory::Status,
        move_type: "Dark".to_string(),
        flags: MoveFlags { snatch: true, ..Default::default() },
        target: MoveTargetType::Self_,
        self_boosts: Some(vec![("atk".to_string(), 1), ("accuracy".to_string(), 1)]),
        ..Default::default()
    });
    map.insert(ID::new("hornattack"), MoveDef {
        id: ID::new("hornattack"),
        name: "Horn Attack".to_string(),
        base_power: 65,
        accuracy: 100,
        pp: 25,
        category: MoveCategory::Physical,
        move_type: "Normal".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("horndrill"), MoveDef {
        id: ID::new("horndrill"),
        name: "Horn Drill".to_string(),
        base_power: 0,
        accuracy: 30,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Normal".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
        ohko: true,
        ..Default::default()
    });
    map.insert(ID::new("hornleech"), MoveDef {
        id: ID::new("hornleech"),
        name: "Horn Leech".to_string(),
        base_power: 75,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Physical,
        move_type: "Grass".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, heal: true, ..Default::default() },
        drain: Some(0.5),
        ..Default::default()
    });
    map.insert(ID::new("howl"), MoveDef {
        id: ID::new("howl"),
        name: "Howl".to_string(),
        base_power: 0,
        accuracy: 0,
        pp: 40,
        category: MoveCategory::Status,
        move_type: "Normal".to_string(),
        flags: MoveFlags { snatch: true, sound: true, ..Default::default() },
        target: MoveTargetType::AllySide,
        boosts: Some(vec![("atk".to_string(), 1)]),
        ..Default::default()
    });
    map.insert(ID::new("hurricane"), MoveDef {
        id: ID::new("hurricane"),
        name: "Hurricane".to_string(),
        base_power: 110,
        accuracy: 70,
        pp: 10,
        category: MoveCategory::Special,
        move_type: "Flying".to_string(),
        flags: MoveFlags { protect: true, mirror: true, distance: true, wind: true, ..Default::default() },
        target: MoveTargetType::Any,
        secondaries: vec![SecondaryEffect { chance: 30, volatile_status: Some("confusion".to_string()), ..Default::default() }],
        ..Default::default()
    });
    map.insert(ID::new("hydrocannon"), MoveDef {
        id: ID::new("hydrocannon"),
        name: "Hydro Cannon".to_string(),
        base_power: 150,
        accuracy: 90,
        pp: 5,
        category: MoveCategory::Special,
        move_type: "Water".to_string(),
        flags: MoveFlags { recharge: true, protect: true, mirror: true, ..Default::default() },
        self_volatile: Some("mustrecharge".to_string()),
        ..Default::default()
    });
    map.insert(ID::new("hydropump"), MoveDef {
        id: ID::new("hydropump"),
        name: "Hydro Pump".to_string(),
        base_power: 110,
        accuracy: 80,
        pp: 5,
        category: MoveCategory::Special,
        move_type: "Water".to_string(),
        flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
        ..Default::default()
    });
    map.insert(ID::new("hydrosteam"), MoveDef {
        id: ID::new("hydrosteam"),
        name: "Hydro Steam".to_string(),
        base_power: 80,
        accuracy: 100,
        pp: 15,
        category: MoveCategory::Special,
        move_type: "Water".to_string(),
        flags: MoveFlags { protect: true, mirror: true, defrost: true, ..Default::default() },
        thaws_user: true,
        ..Default::default()
    });
    map.insert(ID::new("hydrovortex"), MoveDef {
        id: ID::new("hydrovortex"),
        name: "Hydro Vortex".to_string(),
        base_power: 1,
        accuracy: 0,
        pp: 1,
        category: MoveCategory::Physical,
        move_type: "Water".to_string(),
        flags: MoveFlags { ..Default::default() },
        is_z: true,
        ..Default::default()
    });
    map.insert(ID::new("hyperbeam"), MoveDef {
        id: ID::new("hyperbeam"),
        name: "Hyper Beam".to_string(),
        base_power: 150,
        accuracy: 90,
        pp: 5,
        category: MoveCategory::Special,
        move_type: "Normal".to_string(),
        flags: MoveFlags { recharge: true, protect: true, mirror: true, ..Default::default() },
        self_volatile: Some("mustrecharge".to_string()),
        ..Default::default()
    });
    map.insert(ID::new("hyperdrill"), MoveDef {
        id: ID::new("hyperdrill"),
        name: "Hyper Drill".to_string(),
        base_power: 100,
        accuracy: 100,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Normal".to_string(),
        flags: MoveFlags { contact: true, mirror: true, ..Default::default() },
        breaks_protect: true,
        ..Default::default()
    });
    map.insert(ID::new("hyperfang"), MoveDef {
        id: ID::new("hyperfang"),
        name: "Hyper Fang".to_string(),
        base_power: 80,
        accuracy: 90,
        pp: 15,
        category: MoveCategory::Physical,
        move_type: "Normal".to_string(),
        flags: MoveFlags { contact: true, protect: true, mirror: true, bite: true, ..Default::default() },
        secondaries: vec![SecondaryEffect { chance: 10, volatile_status: Some("flinch".to_string()), ..Default::default() }],
        ..Default::default()
    });
    map.insert(ID::new("hyperspacefury"), MoveDef {
        id: ID::new("hyperspacefury"),
        name: "Hyperspace Fury".to_string(),
        base_power: 100,
        accuracy: 0,
        pp: 5,
        category: MoveCategory::Physical,
        move_type: "Dark".to_string(),
        flags: MoveFlags { mirror: true, bypasssub: true, ..Default::default() },
        breaks_protect: true,
        self_drops: Some(vec![("def".to_string(), -1)]),
        ..Default::default()
    });
    map.insert(ID::new("hyperspacehole"), MoveDef {
        id: ID::new("hyperspacehole"),
        name: "Hyperspace Hole".to_string(),
        base_power: 80,
        accuracy: 0,
        pp: 5,
        category: MoveCategory::Special,
        move_type: "Psychic".to_string(),
        flags: MoveFlags { mirror: true, bypasssub: true, ..Default::default() },
        breaks_protect: true,
        ..Default::default()
    });
    map.insert(ID::new("hypervoice"), MoveDef {
        id: ID::new("hypervoice"),
        name: "Hyper Voice".to_string(),
        base_power: 90,
        accuracy: 100,
        pp: 10,
        category: MoveCategory::Special,
        move_type: "Normal".to_string(),
        flags: MoveFlags { protect: true, mirror: true, sound: true, bypasssub: true, ..Default::default() },
        target: MoveTargetType::AllAdjacentFoes,
        ..Default::default()
    });
    map.insert(ID::new("hypnosis"), MoveDef {
        id: ID::new("hypnosis"),
        name: "Hypnosis".to_string(),
        base_power: 0,
        accuracy: 60,
        pp: 20,
        category: MoveCategory::Status,
        move_type: "Psychic".to_string(),
        flags: MoveFlags { protect: true, reflectable: true, mirror: true, ..Default::default() },
        status: Some("slp".to_string()),
        ..Default::default()
    });
        // I moves
        map.insert(ID::new("iceball"), MoveDef {
            id: ID::new("iceball"),
            name: "Ice Ball".to_string(),
            base_power: 30,
            accuracy: 90,
            pp: 20,
            category: MoveCategory::Physical,
            move_type: "Ice".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, bullet: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("icebeam"), MoveDef {
            id: ID::new("icebeam"),
            name: "Ice Beam".to_string(),
            base_power: 90,
            accuracy: 100,
            pp: 10,
            category: MoveCategory::Special,
            move_type: "Ice".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 10,
                status: Some("frz".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("iceburn"), MoveDef {
            id: ID::new("iceburn"),
            name: "Ice Burn".to_string(),
            base_power: 140,
            accuracy: 90,
            pp: 5,
            category: MoveCategory::Special,
            move_type: "Ice".to_string(),
            flags: MoveFlags { charge: true, protect: true, mirror: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 30,
                status: Some("brn".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("icefang"), MoveDef {
            id: ID::new("icefang"),
            name: "Ice Fang".to_string(),
            base_power: 65,
            accuracy: 95,
            pp: 15,
            category: MoveCategory::Physical,
            move_type: "Ice".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, bite: true, ..Default::default() },
            secondaries: vec![
                SecondaryEffect {
                    chance: 10,
                    status: Some("frz".to_string()),
                    ..Default::default()
                },
                SecondaryEffect {
                    chance: 10,
                    volatile_status: Some("flinch".to_string()),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });
        map.insert(ID::new("icehammer"), MoveDef {
            id: ID::new("icehammer"),
            name: "Ice Hammer".to_string(),
            base_power: 100,
            accuracy: 90,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Ice".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, punch: true, ..Default::default() },
            self_boosts: Some(vec![("spe".to_string(), -1)]),
            ..Default::default()
        });
        map.insert(ID::new("icepunch"), MoveDef {
            id: ID::new("icepunch"),
            name: "Ice Punch".to_string(),
            base_power: 75,
            accuracy: 100,
            pp: 15,
            category: MoveCategory::Physical,
            move_type: "Ice".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, punch: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 10,
                status: Some("frz".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("iceshard"), MoveDef {
            id: ID::new("iceshard"),
            name: "Ice Shard".to_string(),
            base_power: 40,
            accuracy: 100,
            pp: 30,
            priority: 1,
            category: MoveCategory::Physical,
            move_type: "Ice".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("icespinner"), MoveDef {
            id: ID::new("icespinner"),
            name: "Ice Spinner".to_string(),
            base_power: 80,
            accuracy: 100,
            pp: 15,
            category: MoveCategory::Physical,
            move_type: "Ice".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("iciclecrash"), MoveDef {
            id: ID::new("iciclecrash"),
            name: "Icicle Crash".to_string(),
            base_power: 85,
            accuracy: 90,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Ice".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 30,
                volatile_status: Some("flinch".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("iciclespear"), MoveDef {
            id: ID::new("iciclespear"),
            name: "Icicle Spear".to_string(),
            base_power: 25,
            accuracy: 100,
            pp: 30,
            category: MoveCategory::Physical,
            move_type: "Ice".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            multi_hit: Some((2, 5)),
            ..Default::default()
        });
        map.insert(ID::new("icywind"), MoveDef {
            id: ID::new("icywind"),
            name: "Icy Wind".to_string(),
            base_power: 55,
            accuracy: 95,
            pp: 15,
            category: MoveCategory::Special,
            move_type: "Ice".to_string(),
            target: MoveTargetType::AllAdjacentFoes,
            flags: MoveFlags { protect: true, mirror: true, wind: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 100,
                boosts: Some(vec![("spe".to_string(), -1)]),
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("imprison"), MoveDef {
            id: ID::new("imprison"),
            name: "Imprison".to_string(),
            base_power: 0,
            pp: 10,
            category: MoveCategory::Status,
            move_type: "Psychic".to_string(),
            target: MoveTargetType::Self_,
            flags: MoveFlags { snatch: true, ..Default::default() },
            volatile_status: Some("imprison".to_string()),
            ..Default::default()
        });
        map.insert(ID::new("incinerate"), MoveDef {
            id: ID::new("incinerate"),
            name: "Incinerate".to_string(),
            base_power: 60,
            accuracy: 100,
            pp: 15,
            category: MoveCategory::Special,
            move_type: "Fire".to_string(),
            target: MoveTargetType::AllAdjacentFoes,
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("infernalparade"), MoveDef {
            id: ID::new("infernalparade"),
            name: "Infernal Parade".to_string(),
            base_power: 60,
            accuracy: 100,
            pp: 15,
            category: MoveCategory::Special,
            move_type: "Ghost".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 30,
                status: Some("brn".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("inferno"), MoveDef {
            id: ID::new("inferno"),
            name: "Inferno".to_string(),
            base_power: 100,
            accuracy: 50,
            pp: 5,
            category: MoveCategory::Special,
            move_type: "Fire".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 100,
                status: Some("brn".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("infernooverdrive"), MoveDef {
            id: ID::new("infernooverdrive"),
            name: "Inferno Overdrive".to_string(),
            base_power: 1,
            pp: 1,
            category: MoveCategory::Physical,
            move_type: "Fire".to_string(),
            flags: MoveFlags { ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("infestation"), MoveDef {
            id: ID::new("infestation"),
            name: "Infestation".to_string(),
            base_power: 20,
            accuracy: 100,
            pp: 20,
            category: MoveCategory::Special,
            move_type: "Bug".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
            volatile_status: Some("partiallytrapped".to_string()),
            ..Default::default()
        });
        map.insert(ID::new("ingrain"), MoveDef {
            id: ID::new("ingrain"),
            name: "Ingrain".to_string(),
            base_power: 0,
            pp: 20,
            category: MoveCategory::Status,
            move_type: "Grass".to_string(),
            target: MoveTargetType::Self_,
            flags: MoveFlags { snatch: true, nonsky: true, ..Default::default() },
            volatile_status: Some("ingrain".to_string()),
            ..Default::default()
        });
        map.insert(ID::new("instruct"), MoveDef {
            id: ID::new("instruct"),
            name: "Instruct".to_string(),
            base_power: 0,
            pp: 15,
            category: MoveCategory::Status,
            move_type: "Psychic".to_string(),
            flags: MoveFlags { protect: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("iondeluge"), MoveDef {
            id: ID::new("iondeluge"),
            name: "Ion Deluge".to_string(),
            base_power: 0,
            pp: 25,
            priority: 1,
            category: MoveCategory::Status,
            move_type: "Electric".to_string(),
            target: MoveTargetType::All,
            flags: MoveFlags { ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("irondefense"), MoveDef {
            id: ID::new("irondefense"),
            name: "Iron Defense".to_string(),
            base_power: 0,
            pp: 15,
            category: MoveCategory::Status,
            move_type: "Steel".to_string(),
            target: MoveTargetType::Self_,
            flags: MoveFlags { snatch: true, ..Default::default() },
            boosts: Some(vec![("def".to_string(), 2)]),
            ..Default::default()
        });
        map.insert(ID::new("ironhead"), MoveDef {
            id: ID::new("ironhead"),
            name: "Iron Head".to_string(),
            base_power: 80,
            accuracy: 100,
            pp: 15,
            category: MoveCategory::Physical,
            move_type: "Steel".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 30,
                volatile_status: Some("flinch".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("irontail"), MoveDef {
            id: ID::new("irontail"),
            name: "Iron Tail".to_string(),
            base_power: 100,
            accuracy: 75,
            pp: 15,
            category: MoveCategory::Physical,
            move_type: "Steel".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 30,
                boosts: Some(vec![("def".to_string(), -1)]),
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("ivycudgel"), MoveDef {
            id: ID::new("ivycudgel"),
            name: "Ivy Cudgel".to_string(),
            base_power: 100,
            accuracy: 100,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Grass".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            crit_ratio: 2,
            ..Default::default()
        });
        // J moves
        map.insert(ID::new("jawlock"), MoveDef {
            id: ID::new("jawlock"),
            name: "Jaw Lock".to_string(),
            base_power: 80,
            accuracy: 100,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Dark".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, bite: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("jetpunch"), MoveDef {
            id: ID::new("jetpunch"),
            name: "Jet Punch".to_string(),
            base_power: 60,
            accuracy: 100,
            pp: 15,
            priority: 1,
            category: MoveCategory::Physical,
            move_type: "Water".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, punch: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("judgment"), MoveDef {
            id: ID::new("judgment"),
            name: "Judgment".to_string(),
            base_power: 100,
            accuracy: 100,
            pp: 10,
            category: MoveCategory::Special,
            move_type: "Normal".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("jumpkick"), MoveDef {
            id: ID::new("jumpkick"),
            name: "Jump Kick".to_string(),
            base_power: 100,
            accuracy: 95,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Fighting".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, gravity: true, ..Default::default() },
            has_crash_damage: true,
            ..Default::default()
        });
        map.insert(ID::new("junglehealing"), MoveDef {
            id: ID::new("junglehealing"),
            name: "Jungle Healing".to_string(),
            base_power: 0,
            pp: 10,
            category: MoveCategory::Status,
            move_type: "Grass".to_string(),
            target: MoveTargetType::AllAllies,
            flags: MoveFlags { heal: true, ..Default::default() },
            ..Default::default()
        });
        // K moves
        map.insert(ID::new("karatechop"), MoveDef {
            id: ID::new("karatechop"),
            name: "Karate Chop".to_string(),
            base_power: 50,
            accuracy: 100,
            pp: 25,
            category: MoveCategory::Physical,
            move_type: "Fighting".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
            crit_ratio: 2,
            ..Default::default()
        });
        map.insert(ID::new("kinesis"), MoveDef {
            id: ID::new("kinesis"),
            name: "Kinesis".to_string(),
            base_power: 0,
            accuracy: 80,
            pp: 15,
            category: MoveCategory::Status,
            move_type: "Psychic".to_string(),
            flags: MoveFlags { protect: true, reflectable: true, mirror: true, ..Default::default() },
            boosts: Some(vec![("accuracy".to_string(), -1)]),
            ..Default::default()
        });
        map.insert(ID::new("kingsshield"), MoveDef {
            id: ID::new("kingsshield"),
            name: "King's Shield".to_string(),
            base_power: 0,
            pp: 10,
            priority: 4,
            category: MoveCategory::Status,
            move_type: "Steel".to_string(),
            target: MoveTargetType::Self_,
            flags: MoveFlags { ..Default::default() },
            volatile_status: Some("kingsshield".to_string()),
            ..Default::default()
        });
        map.insert(ID::new("knockoff"), MoveDef {
            id: ID::new("knockoff"),
            name: "Knock Off".to_string(),
            base_power: 65,
            accuracy: 100,
            pp: 20,
            category: MoveCategory::Physical,
            move_type: "Dark".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("kowtowcleave"), MoveDef {
            id: ID::new("kowtowcleave"),
            name: "Kowtow Cleave".to_string(),
            base_power: 85,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Dark".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, slicing: true, ..Default::default() },
            ..Default::default()
        });
        // L moves
        map.insert(ID::new("landswrath"), MoveDef {
            id: ID::new("landswrath"),
            name: "Land's Wrath".to_string(),
            base_power: 90,
            accuracy: 100,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Ground".to_string(),
            target: MoveTargetType::AllAdjacentFoes,
            flags: MoveFlags { protect: true, mirror: true, nonsky: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("laserfocus"), MoveDef {
            id: ID::new("laserfocus"),
            name: "Laser Focus".to_string(),
            base_power: 0,
            pp: 30,
            category: MoveCategory::Status,
            move_type: "Normal".to_string(),
            target: MoveTargetType::Self_,
            flags: MoveFlags { snatch: true, ..Default::default() },
            volatile_status: Some("laserfocus".to_string()),
            ..Default::default()
        });
        map.insert(ID::new("lashout"), MoveDef {
            id: ID::new("lashout"),
            name: "Lash Out".to_string(),
            base_power: 75,
            accuracy: 100,
            pp: 5,
            category: MoveCategory::Physical,
            move_type: "Dark".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("lastresort"), MoveDef {
            id: ID::new("lastresort"),
            name: "Last Resort".to_string(),
            base_power: 140,
            accuracy: 100,
            pp: 5,
            category: MoveCategory::Physical,
            move_type: "Normal".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("lastrespects"), MoveDef {
            id: ID::new("lastrespects"),
            name: "Last Respects".to_string(),
            base_power: 50,
            accuracy: 100,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Ghost".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("lavaplume"), MoveDef {
            id: ID::new("lavaplume"),
            name: "Lava Plume".to_string(),
            base_power: 80,
            accuracy: 100,
            pp: 15,
            category: MoveCategory::Special,
            move_type: "Fire".to_string(),
            target: MoveTargetType::AllAdjacent,
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 30,
                status: Some("brn".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("leafage"), MoveDef {
            id: ID::new("leafage"),
            name: "Leafage".to_string(),
            base_power: 40,
            accuracy: 100,
            pp: 40,
            category: MoveCategory::Physical,
            move_type: "Grass".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("leafblade"), MoveDef {
            id: ID::new("leafblade"),
            name: "Leaf Blade".to_string(),
            base_power: 90,
            accuracy: 100,
            pp: 15,
            category: MoveCategory::Physical,
            move_type: "Grass".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, slicing: true, ..Default::default() },
            crit_ratio: 2,
            ..Default::default()
        });
        map.insert(ID::new("leafstorm"), MoveDef {
            id: ID::new("leafstorm"),
            name: "Leaf Storm".to_string(),
            base_power: 130,
            accuracy: 90,
            pp: 5,
            category: MoveCategory::Special,
            move_type: "Grass".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            self_boosts: Some(vec![("spa".to_string(), -2)]),
            ..Default::default()
        });
        map.insert(ID::new("leaftornado"), MoveDef {
            id: ID::new("leaftornado"),
            name: "Leaf Tornado".to_string(),
            base_power: 65,
            accuracy: 90,
            pp: 10,
            category: MoveCategory::Special,
            move_type: "Grass".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 50,
                boosts: Some(vec![("accuracy".to_string(), -1)]),
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("leechlife"), MoveDef {
            id: ID::new("leechlife"),
            name: "Leech Life".to_string(),
            base_power: 80,
            accuracy: 100,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Bug".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, heal: true, ..Default::default() },
            drain: Some(0.5),
            ..Default::default()
        });
        map.insert(ID::new("leechseed"), MoveDef {
            id: ID::new("leechseed"),
            name: "Leech Seed".to_string(),
            base_power: 0,
            accuracy: 90,
            pp: 10,
            category: MoveCategory::Status,
            move_type: "Grass".to_string(),
            flags: MoveFlags { protect: true, reflectable: true, mirror: true, ..Default::default() },
            volatile_status: Some("leechseed".to_string()),
            ..Default::default()
        });
        map.insert(ID::new("leer"), MoveDef {
            id: ID::new("leer"),
            name: "Leer".to_string(),
            base_power: 0,
            accuracy: 100,
            pp: 30,
            category: MoveCategory::Status,
            move_type: "Normal".to_string(),
            target: MoveTargetType::AllAdjacentFoes,
            flags: MoveFlags { protect: true, reflectable: true, mirror: true, ..Default::default() },
            boosts: Some(vec![("def".to_string(), -1)]),
            ..Default::default()
        });
        map.insert(ID::new("letssnuggleforever"), MoveDef {
            id: ID::new("letssnuggleforever"),
            name: "Let's Snuggle Forever".to_string(),
            base_power: 190,
            pp: 1,
            category: MoveCategory::Physical,
            move_type: "Fairy".to_string(),
            flags: MoveFlags { contact: true, ..Default::default() },
            is_z: true,
            ..Default::default()
        });
        map.insert(ID::new("lick"), MoveDef {
            id: ID::new("lick"),
            name: "Lick".to_string(),
            base_power: 30,
            accuracy: 100,
            pp: 30,
            category: MoveCategory::Physical,
            move_type: "Ghost".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 30,
                status: Some("par".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("lifedew"), MoveDef {
            id: ID::new("lifedew"),
            name: "Life Dew".to_string(),
            base_power: 0,
            pp: 10,
            category: MoveCategory::Status,
            move_type: "Water".to_string(),
            target: MoveTargetType::AllAllies,
            flags: MoveFlags { snatch: true, heal: true, ..Default::default() },
            heal: Some(0.25),
            ..Default::default()
        });
        map.insert(ID::new("lightofruin"), MoveDef {
            id: ID::new("lightofruin"),
            name: "Light of Ruin".to_string(),
            base_power: 140,
            accuracy: 90,
            pp: 5,
            category: MoveCategory::Special,
            move_type: "Fairy".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            recoil: Some(0.5),
            ..Default::default()
        });
        map.insert(ID::new("lightscreen"), MoveDef {
            id: ID::new("lightscreen"),
            name: "Light Screen".to_string(),
            base_power: 0,
            pp: 30,
            category: MoveCategory::Status,
            move_type: "Psychic".to_string(),
            target: MoveTargetType::AllySide,
            flags: MoveFlags { snatch: true, ..Default::default() },
            side_condition: Some("lightscreen".to_string()),
            ..Default::default()
        });
        map.insert(ID::new("lightthatburnsthesky"), MoveDef {
            id: ID::new("lightthatburnsthesky"),
            name: "Light That Burns the Sky".to_string(),
            base_power: 200,
            pp: 1,
            category: MoveCategory::Special,
            move_type: "Psychic".to_string(),
            flags: MoveFlags { ..Default::default() },
            is_z: true,
            ignores_ability: true,
            ..Default::default()
        });
        map.insert(ID::new("liquidation"), MoveDef {
            id: ID::new("liquidation"),
            name: "Liquidation".to_string(),
            base_power: 85,
            accuracy: 100,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Water".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 20,
                boosts: Some(vec![("def".to_string(), -1)]),
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("lockon"), MoveDef {
            id: ID::new("lockon"),
            name: "Lock-On".to_string(),
            base_power: 0,
            pp: 5,
            category: MoveCategory::Status,
            move_type: "Normal".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("lovelykiss"), MoveDef {
            id: ID::new("lovelykiss"),
            name: "Lovely Kiss".to_string(),
            base_power: 0,
            accuracy: 75,
            pp: 10,
            category: MoveCategory::Status,
            move_type: "Normal".to_string(),
            flags: MoveFlags { protect: true, reflectable: true, mirror: true, ..Default::default() },
            status: Some("slp".to_string()),
            ..Default::default()
        });
        map.insert(ID::new("lowkick"), MoveDef {
            id: ID::new("lowkick"),
            name: "Low Kick".to_string(),
            base_power: 0,
            accuracy: 100,
            pp: 20,
            category: MoveCategory::Physical,
            move_type: "Fighting".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("lowsweep"), MoveDef {
            id: ID::new("lowsweep"),
            name: "Low Sweep".to_string(),
            base_power: 65,
            accuracy: 100,
            pp: 20,
            category: MoveCategory::Physical,
            move_type: "Fighting".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 100,
                boosts: Some(vec![("spe".to_string(), -1)]),
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("luckychant"), MoveDef {
            id: ID::new("luckychant"),
            name: "Lucky Chant".to_string(),
            base_power: 0,
            pp: 30,
            category: MoveCategory::Status,
            move_type: "Normal".to_string(),
            target: MoveTargetType::AllySide,
            flags: MoveFlags { snatch: true, ..Default::default() },
            side_condition: Some("luckychant".to_string()),
            ..Default::default()
        });
        map.insert(ID::new("luminacrash"), MoveDef {
            id: ID::new("luminacrash"),
            name: "Lumina Crash".to_string(),
            base_power: 80,
            accuracy: 100,
            pp: 10,
            category: MoveCategory::Special,
            move_type: "Psychic".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 100,
                boosts: Some(vec![("spd".to_string(), -2)]),
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("lunarblessing"), MoveDef {
            id: ID::new("lunarblessing"),
            name: "Lunar Blessing".to_string(),
            base_power: 0,
            pp: 5,
            category: MoveCategory::Status,
            move_type: "Psychic".to_string(),
            target: MoveTargetType::AllAllies,
            flags: MoveFlags { snatch: true, heal: true, ..Default::default() },
            heal: Some(0.25),
            ..Default::default()
        });
        map.insert(ID::new("lunardance"), MoveDef {
            id: ID::new("lunardance"),
            name: "Lunar Dance".to_string(),
            base_power: 0,
            pp: 10,
            category: MoveCategory::Status,
            move_type: "Psychic".to_string(),
            target: MoveTargetType::Self_,
            flags: MoveFlags { snatch: true, dance: true, heal: true, ..Default::default() },
            self_destruct: Some("ifHit".to_string()),
            slot_condition: Some("lunardance".to_string()),
            ..Default::default()
        });
        map.insert(ID::new("lunge"), MoveDef {
            id: ID::new("lunge"),
            name: "Lunge".to_string(),
            base_power: 80,
            accuracy: 100,
            pp: 15,
            category: MoveCategory::Physical,
            move_type: "Bug".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 100,
                boosts: Some(vec![("atk".to_string(), -1)]),
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("lusterpurge"), MoveDef {
            id: ID::new("lusterpurge"),
            name: "Luster Purge".to_string(),
            base_power: 95,
            accuracy: 100,
            pp: 5,
            category: MoveCategory::Special,
            move_type: "Psychic".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 50,
                boosts: Some(vec![("spd".to_string(), -1)]),
                ..Default::default()
            }],
            ..Default::default()
        });
        // M moves
        map.insert(ID::new("machpunch"), MoveDef {
            id: ID::new("machpunch"),
            name: "Mach Punch".to_string(),
            base_power: 40,
            accuracy: 100,
            pp: 30,
            priority: 1,
            category: MoveCategory::Physical,
            move_type: "Fighting".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, punch: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("magicalleaf"), MoveDef {
            id: ID::new("magicalleaf"),
            name: "Magical Leaf".to_string(),
            base_power: 60,
            pp: 20,
            category: MoveCategory::Special,
            move_type: "Grass".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("magicaltorque"), MoveDef {
            id: ID::new("magicaltorque"),
            name: "Magical Torque".to_string(),
            base_power: 100,
            accuracy: 100,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Fairy".to_string(),
            flags: MoveFlags { protect: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 30,
                volatile_status: Some("confusion".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("magiccoat"), MoveDef {
            id: ID::new("magiccoat"),
            name: "Magic Coat".to_string(),
            base_power: 0,
            pp: 15,
            priority: 4,
            category: MoveCategory::Status,
            move_type: "Psychic".to_string(),
            target: MoveTargetType::Self_,
            flags: MoveFlags { ..Default::default() },
            volatile_status: Some("magiccoat".to_string()),
            ..Default::default()
        });
        map.insert(ID::new("magicpowder"), MoveDef {
            id: ID::new("magicpowder"),
            name: "Magic Powder".to_string(),
            base_power: 0,
            accuracy: 100,
            pp: 20,
            category: MoveCategory::Status,
            move_type: "Psychic".to_string(),
            flags: MoveFlags { protect: true, reflectable: true, mirror: true, powder: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("magicroom"), MoveDef {
            id: ID::new("magicroom"),
            name: "Magic Room".to_string(),
            base_power: 0,
            pp: 10,
            category: MoveCategory::Status,
            move_type: "Psychic".to_string(),
            target: MoveTargetType::All,
            flags: MoveFlags { mirror: true, ..Default::default() },
            pseudo_weather: Some("magicroom".to_string()),
            ..Default::default()
        });
        map.insert(ID::new("magmastorm"), MoveDef {
            id: ID::new("magmastorm"),
            name: "Magma Storm".to_string(),
            base_power: 100,
            accuracy: 75,
            pp: 5,
            category: MoveCategory::Special,
            move_type: "Fire".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            volatile_status: Some("partiallytrapped".to_string()),
            ..Default::default()
        });
        map.insert(ID::new("magnetbomb"), MoveDef {
            id: ID::new("magnetbomb"),
            name: "Magnet Bomb".to_string(),
            base_power: 60,
            pp: 20,
            category: MoveCategory::Physical,
            move_type: "Steel".to_string(),
            flags: MoveFlags { protect: true, mirror: true, bullet: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("magneticflux"), MoveDef {
            id: ID::new("magneticflux"),
            name: "Magnetic Flux".to_string(),
            base_power: 0,
            pp: 20,
            category: MoveCategory::Status,
            move_type: "Electric".to_string(),
            target: MoveTargetType::AllySide,
            flags: MoveFlags { snatch: true, distance: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("magnetrise"), MoveDef {
            id: ID::new("magnetrise"),
            name: "Magnet Rise".to_string(),
            base_power: 0,
            pp: 10,
            category: MoveCategory::Status,
            move_type: "Electric".to_string(),
            target: MoveTargetType::Self_,
            flags: MoveFlags { snatch: true, gravity: true, ..Default::default() },
            volatile_status: Some("magnetrise".to_string()),
            ..Default::default()
        });
        map.insert(ID::new("magnitude"), MoveDef {
            id: ID::new("magnitude"),
            name: "Magnitude".to_string(),
            base_power: 0,
            accuracy: 100,
            pp: 30,
            category: MoveCategory::Physical,
            move_type: "Ground".to_string(),
            target: MoveTargetType::AllAdjacent,
            flags: MoveFlags { protect: true, mirror: true, nonsky: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("makeitrain"), MoveDef {
            id: ID::new("makeitrain"),
            name: "Make It Rain".to_string(),
            base_power: 120,
            accuracy: 100,
            pp: 5,
            category: MoveCategory::Special,
            move_type: "Steel".to_string(),
            target: MoveTargetType::AllAdjacentFoes,
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            self_boosts: Some(vec![("spa".to_string(), -1)]),
            ..Default::default()
        });
        map.insert(ID::new("maliciousmoonsault"), MoveDef {
            id: ID::new("maliciousmoonsault"),
            name: "Malicious Moonsault".to_string(),
            base_power: 180,
            pp: 1,
            category: MoveCategory::Physical,
            move_type: "Dark".to_string(),
            flags: MoveFlags { contact: true, ..Default::default() },
            is_z: true,
            ..Default::default()
        });
        map.insert(ID::new("malignantchain"), MoveDef {
            id: ID::new("malignantchain"),
            name: "Malignant Chain".to_string(),
            base_power: 100,
            accuracy: 100,
            pp: 5,
            category: MoveCategory::Special,
            move_type: "Poison".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 50,
                status: Some("tox".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("matblock"), MoveDef {
            id: ID::new("matblock"),
            name: "Mat Block".to_string(),
            base_power: 0,
            pp: 10,
            category: MoveCategory::Status,
            move_type: "Fighting".to_string(),
            target: MoveTargetType::AllySide,
            flags: MoveFlags { snatch: true, nonsky: true, ..Default::default() },
            side_condition: Some("matblock".to_string()),
            ..Default::default()
        });
        map.insert(ID::new("matchagotcha"), MoveDef {
            id: ID::new("matchagotcha"),
            name: "Matcha Gotcha".to_string(),
            base_power: 80,
            accuracy: 90,
            pp: 15,
            category: MoveCategory::Special,
            move_type: "Grass".to_string(),
            target: MoveTargetType::AllAdjacentFoes,
            flags: MoveFlags { protect: true, mirror: true, defrost: true, heal: true, ..Default::default() },
            drain: Some(0.5),
            thaws_user: true,
            secondaries: vec![SecondaryEffect {
                chance: 20,
                status: Some("brn".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("maxairstream"), MoveDef {
            id: ID::new("maxairstream"),
            name: "Max Airstream".to_string(),
            base_power: 10,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Flying".to_string(),
            flags: MoveFlags { ..Default::default() },
            is_max: true,
            ..Default::default()
        });
        map.insert(ID::new("maxdarkness"), MoveDef {
            id: ID::new("maxdarkness"),
            name: "Max Darkness".to_string(),
            base_power: 10,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Dark".to_string(),
            flags: MoveFlags { ..Default::default() },
            is_max: true,
            ..Default::default()
        });
        map.insert(ID::new("maxflare"), MoveDef {
            id: ID::new("maxflare"),
            name: "Max Flare".to_string(),
            base_power: 100,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Fire".to_string(),
            flags: MoveFlags { ..Default::default() },
            is_max: true,
            ..Default::default()
        });
        map.insert(ID::new("maxflutterby"), MoveDef {
            id: ID::new("maxflutterby"),
            name: "Max Flutterby".to_string(),
            base_power: 10,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Bug".to_string(),
            flags: MoveFlags { ..Default::default() },
            is_max: true,
            ..Default::default()
        });
        map.insert(ID::new("maxgeyser"), MoveDef {
            id: ID::new("maxgeyser"),
            name: "Max Geyser".to_string(),
            base_power: 10,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Water".to_string(),
            flags: MoveFlags { ..Default::default() },
            is_max: true,
            ..Default::default()
        });
        map.insert(ID::new("maxguard"), MoveDef {
            id: ID::new("maxguard"),
            name: "Max Guard".to_string(),
            base_power: 0,
            pp: 10,
            priority: 4,
            category: MoveCategory::Status,
            move_type: "Normal".to_string(),
            target: MoveTargetType::Self_,
            flags: MoveFlags { ..Default::default() },
            is_max: true,
            volatile_status: Some("maxguard".to_string()),
            ..Default::default()
        });
        map.insert(ID::new("maxhailstorm"), MoveDef {
            id: ID::new("maxhailstorm"),
            name: "Max Hailstorm".to_string(),
            base_power: 10,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Ice".to_string(),
            flags: MoveFlags { ..Default::default() },
            is_max: true,
            ..Default::default()
        });
        map.insert(ID::new("maxknuckle"), MoveDef {
            id: ID::new("maxknuckle"),
            name: "Max Knuckle".to_string(),
            base_power: 10,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Fighting".to_string(),
            flags: MoveFlags { ..Default::default() },
            is_max: true,
            ..Default::default()
        });
        map.insert(ID::new("maxlightning"), MoveDef {
            id: ID::new("maxlightning"),
            name: "Max Lightning".to_string(),
            base_power: 10,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Electric".to_string(),
            flags: MoveFlags { ..Default::default() },
            is_max: true,
            ..Default::default()
        });
        map.insert(ID::new("maxmindstorm"), MoveDef {
            id: ID::new("maxmindstorm"),
            name: "Max Mindstorm".to_string(),
            base_power: 10,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Psychic".to_string(),
            flags: MoveFlags { ..Default::default() },
            is_max: true,
            ..Default::default()
        });
        map.insert(ID::new("maxooze"), MoveDef {
            id: ID::new("maxooze"),
            name: "Max Ooze".to_string(),
            base_power: 10,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Poison".to_string(),
            flags: MoveFlags { ..Default::default() },
            is_max: true,
            ..Default::default()
        });
        map.insert(ID::new("maxovergrowth"), MoveDef {
            id: ID::new("maxovergrowth"),
            name: "Max Overgrowth".to_string(),
            base_power: 10,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Grass".to_string(),
            flags: MoveFlags { ..Default::default() },
            is_max: true,
            ..Default::default()
        });
        map.insert(ID::new("maxphantasm"), MoveDef {
            id: ID::new("maxphantasm"),
            name: "Max Phantasm".to_string(),
            base_power: 10,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Ghost".to_string(),
            flags: MoveFlags { ..Default::default() },
            is_max: true,
            ..Default::default()
        });
        map.insert(ID::new("maxquake"), MoveDef {
            id: ID::new("maxquake"),
            name: "Max Quake".to_string(),
            base_power: 10,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Ground".to_string(),
            flags: MoveFlags { ..Default::default() },
            is_max: true,
            ..Default::default()
        });
        map.insert(ID::new("maxrockfall"), MoveDef {
            id: ID::new("maxrockfall"),
            name: "Max Rockfall".to_string(),
            base_power: 10,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Rock".to_string(),
            flags: MoveFlags { ..Default::default() },
            is_max: true,
            ..Default::default()
        });
        map.insert(ID::new("maxstarfall"), MoveDef {
            id: ID::new("maxstarfall"),
            name: "Max Starfall".to_string(),
            base_power: 10,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Fairy".to_string(),
            flags: MoveFlags { ..Default::default() },
            is_max: true,
            ..Default::default()
        });
        map.insert(ID::new("maxsteelspike"), MoveDef {
            id: ID::new("maxsteelspike"),
            name: "Max Steelspike".to_string(),
            base_power: 10,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Steel".to_string(),
            flags: MoveFlags { ..Default::default() },
            is_max: true,
            ..Default::default()
        });
        map.insert(ID::new("maxstrike"), MoveDef {
            id: ID::new("maxstrike"),
            name: "Max Strike".to_string(),
            base_power: 10,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Normal".to_string(),
            flags: MoveFlags { ..Default::default() },
            is_max: true,
            ..Default::default()
        });
        map.insert(ID::new("maxwyrmwind"), MoveDef {
            id: ID::new("maxwyrmwind"),
            name: "Max Wyrmwind".to_string(),
            base_power: 10,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Dragon".to_string(),
            flags: MoveFlags { ..Default::default() },
            is_max: true,
            ..Default::default()
        });
        map.insert(ID::new("meanlook"), MoveDef {
            id: ID::new("meanlook"),
            name: "Mean Look".to_string(),
            base_power: 0,
            pp: 5,
            category: MoveCategory::Status,
            move_type: "Normal".to_string(),
            flags: MoveFlags { reflectable: true, mirror: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("meditate"), MoveDef {
            id: ID::new("meditate"),
            name: "Meditate".to_string(),
            base_power: 0,
            pp: 40,
            category: MoveCategory::Status,
            move_type: "Psychic".to_string(),
            target: MoveTargetType::Self_,
            flags: MoveFlags { snatch: true, ..Default::default() },
            boosts: Some(vec![("atk".to_string(), 1)]),
            ..Default::default()
        });
        map.insert(ID::new("mefirst"), MoveDef {
            id: ID::new("mefirst"),
            name: "Me First".to_string(),
            base_power: 0,
            pp: 20,
            category: MoveCategory::Status,
            move_type: "Normal".to_string(),
            flags: MoveFlags { protect: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("megadrain"), MoveDef {
            id: ID::new("megadrain"),
            name: "Mega Drain".to_string(),
            base_power: 40,
            accuracy: 100,
            pp: 15,
            category: MoveCategory::Special,
            move_type: "Grass".to_string(),
            flags: MoveFlags { protect: true, mirror: true, heal: true, ..Default::default() },
            drain: Some(0.5),
            ..Default::default()
        });
        map.insert(ID::new("megahorn"), MoveDef {
            id: ID::new("megahorn"),
            name: "Megahorn".to_string(),
            base_power: 120,
            accuracy: 85,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Bug".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("megakick"), MoveDef {
            id: ID::new("megakick"),
            name: "Mega Kick".to_string(),
            base_power: 120,
            accuracy: 75,
            pp: 5,
            category: MoveCategory::Physical,
            move_type: "Normal".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("megapunch"), MoveDef {
            id: ID::new("megapunch"),
            name: "Mega Punch".to_string(),
            base_power: 80,
            accuracy: 85,
            pp: 20,
            category: MoveCategory::Physical,
            move_type: "Normal".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, punch: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("memento"), MoveDef {
            id: ID::new("memento"),
            name: "Memento".to_string(),
            base_power: 0,
            accuracy: 100,
            pp: 10,
            category: MoveCategory::Status,
            move_type: "Dark".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            boosts: Some(vec![("atk".to_string(), -2), ("spa".to_string(), -2)]),
            self_destruct: Some("ifHit".to_string()),
            ..Default::default()
        });
        map.insert(ID::new("menacingmoonrazemaelstrom"), MoveDef {
            id: ID::new("menacingmoonrazemaelstrom"),
            name: "Menacing Moonraze Maelstrom".to_string(),
            base_power: 200,
            pp: 1,
            category: MoveCategory::Special,
            move_type: "Ghost".to_string(),
            flags: MoveFlags { ..Default::default() },
            is_z: true,
            ignores_ability: true,
            ..Default::default()
        });
        map.insert(ID::new("metalburst"), MoveDef {
            id: ID::new("metalburst"),
            name: "Metal Burst".to_string(),
            base_power: 0,
            accuracy: 100,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Steel".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("metalclaw"), MoveDef {
            id: ID::new("metalclaw"),
            name: "Metal Claw".to_string(),
            base_power: 50,
            accuracy: 95,
            pp: 35,
            category: MoveCategory::Physical,
            move_type: "Steel".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 10,
                boosts: Some(vec![("atk".to_string(), 1)]),
                self_boost: true, ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("metalsound"), MoveDef {
            id: ID::new("metalsound"),
            name: "Metal Sound".to_string(),
            base_power: 0,
            accuracy: 85,
            pp: 40,
            category: MoveCategory::Status,
            move_type: "Steel".to_string(),
            flags: MoveFlags { protect: true, reflectable: true, mirror: true, sound: true, ..Default::default() },
            boosts: Some(vec![("spd".to_string(), -2)]),
            ..Default::default()
        });
        map.insert(ID::new("meteorassault"), MoveDef {
            id: ID::new("meteorassault"),
            name: "Meteor Assault".to_string(),
            base_power: 150,
            accuracy: 100,
            pp: 5,
            category: MoveCategory::Physical,
            move_type: "Fighting".to_string(),
            flags: MoveFlags { recharge: true, protect: true, mirror: true, ..Default::default() },
            self_volatile: Some("mustrecharge".to_string()),
            ..Default::default()
        });
        map.insert(ID::new("meteorbeam"), MoveDef {
            id: ID::new("meteorbeam"),
            name: "Meteor Beam".to_string(),
            base_power: 120,
            accuracy: 90,
            pp: 10,
            category: MoveCategory::Special,
            move_type: "Rock".to_string(),
            flags: MoveFlags { charge: true, protect: true, mirror: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("meteormash"), MoveDef {
            id: ID::new("meteormash"),
            name: "Meteor Mash".to_string(),
            base_power: 90,
            accuracy: 90,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Steel".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, punch: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 20,
                boosts: Some(vec![("atk".to_string(), 1)]),
                self_boost: true, ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("metronome"), MoveDef {
            id: ID::new("metronome"),
            name: "Metronome".to_string(),
            base_power: 0,
            pp: 10,
            category: MoveCategory::Status,
            move_type: "Normal".to_string(),
            target: MoveTargetType::Self_,
            flags: MoveFlags { ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("mightycleave"), MoveDef {
            id: ID::new("mightycleave"),
            name: "Mighty Cleave".to_string(),
            base_power: 95,
            accuracy: 100,
            pp: 5,
            category: MoveCategory::Physical,
            move_type: "Rock".to_string(),
            flags: MoveFlags { contact: true, mirror: true, slicing: true, ..Default::default() },
            breaks_protect: true,
            ..Default::default()
        });
        map.insert(ID::new("milkdrink"), MoveDef {
            id: ID::new("milkdrink"),
            name: "Milk Drink".to_string(),
            base_power: 0,
            pp: 5,
            category: MoveCategory::Status,
            move_type: "Normal".to_string(),
            target: MoveTargetType::Self_,
            flags: MoveFlags { snatch: true, heal: true, ..Default::default() },
            heal: Some(0.5),
            ..Default::default()
        });
        map.insert(ID::new("mimic"), MoveDef {
            id: ID::new("mimic"),
            name: "Mimic".to_string(),
            base_power: 0,
            pp: 10,
            category: MoveCategory::Status,
            move_type: "Normal".to_string(),
            flags: MoveFlags { protect: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("mindblown"), MoveDef {
            id: ID::new("mindblown"),
            name: "Mind Blown".to_string(),
            base_power: 150,
            accuracy: 100,
            pp: 5,
            category: MoveCategory::Special,
            move_type: "Fire".to_string(),
            target: MoveTargetType::AllAdjacent,
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            recoil_hp: Some(0.5),
            ..Default::default()
        });
        map.insert(ID::new("mindreader"), MoveDef {
            id: ID::new("mindreader"),
            name: "Mind Reader".to_string(),
            base_power: 0,
            pp: 5,
            category: MoveCategory::Status,
            move_type: "Normal".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("minimize"), MoveDef {
            id: ID::new("minimize"),
            name: "Minimize".to_string(),
            base_power: 0,
            pp: 10,
            category: MoveCategory::Status,
            move_type: "Normal".to_string(),
            target: MoveTargetType::Self_,
            flags: MoveFlags { snatch: true, ..Default::default() },
            volatile_status: Some("minimize".to_string()),
            boosts: Some(vec![("evasion".to_string(), 2)]),
            ..Default::default()
        });
        map.insert(ID::new("miracleeye"), MoveDef {
            id: ID::new("miracleeye"),
            name: "Miracle Eye".to_string(),
            base_power: 0,
            pp: 40,
            category: MoveCategory::Status,
            move_type: "Psychic".to_string(),
            flags: MoveFlags { protect: true, reflectable: true, mirror: true, ..Default::default() },
            volatile_status: Some("miracleeye".to_string()),
            ..Default::default()
        });
        map.insert(ID::new("mirrorcoat"), MoveDef {
            id: ID::new("mirrorcoat"),
            name: "Mirror Coat".to_string(),
            base_power: 0,
            accuracy: 100,
            pp: 20,
            priority: -5,
            category: MoveCategory::Special,
            move_type: "Psychic".to_string(),
            flags: MoveFlags { protect: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("mirrormove"), MoveDef {
            id: ID::new("mirrormove"),
            name: "Mirror Move".to_string(),
            base_power: 0,
            pp: 20,
            category: MoveCategory::Status,
            move_type: "Flying".to_string(),
            flags: MoveFlags { ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("mirrorshot"), MoveDef {
            id: ID::new("mirrorshot"),
            name: "Mirror Shot".to_string(),
            base_power: 65,
            accuracy: 85,
            pp: 10,
            category: MoveCategory::Special,
            move_type: "Steel".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 30,
                boosts: Some(vec![("accuracy".to_string(), -1)]),
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("mist"), MoveDef {
            id: ID::new("mist"),
            name: "Mist".to_string(),
            base_power: 0,
            pp: 30,
            category: MoveCategory::Status,
            move_type: "Ice".to_string(),
            target: MoveTargetType::AllySide,
            flags: MoveFlags { snatch: true, ..Default::default() },
            side_condition: Some("mist".to_string()),
            ..Default::default()
        });
        map.insert(ID::new("mistball"), MoveDef {
            id: ID::new("mistball"),
            name: "Mist Ball".to_string(),
            base_power: 95,
            accuracy: 100,
            pp: 5,
            category: MoveCategory::Special,
            move_type: "Psychic".to_string(),
            flags: MoveFlags { protect: true, mirror: true, bullet: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 50,
                boosts: Some(vec![("spa".to_string(), -1)]),
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("mistyexplosion"), MoveDef {
            id: ID::new("mistyexplosion"),
            name: "Misty Explosion".to_string(),
            base_power: 100,
            accuracy: 100,
            pp: 5,
            category: MoveCategory::Special,
            move_type: "Fairy".to_string(),
            target: MoveTargetType::AllAdjacent,
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            self_destruct: Some("always".to_string()),
            ..Default::default()
        });
        map.insert(ID::new("mistyterrain"), MoveDef {
            id: ID::new("mistyterrain"),
            name: "Misty Terrain".to_string(),
            base_power: 0,
            pp: 10,
            category: MoveCategory::Status,
            move_type: "Fairy".to_string(),
            target: MoveTargetType::All,
            flags: MoveFlags { nonsky: true, ..Default::default() },
            terrain: Some("mistyterrain".to_string()),
            ..Default::default()
        });
        map.insert(ID::new("moonblast"), MoveDef {
            id: ID::new("moonblast"),
            name: "Moonblast".to_string(),
            base_power: 95,
            accuracy: 100,
            pp: 15,
            category: MoveCategory::Special,
            move_type: "Fairy".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 30,
                boosts: Some(vec![("spa".to_string(), -1)]),
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("moongeistbeam"), MoveDef {
            id: ID::new("moongeistbeam"),
            name: "Moongeist Beam".to_string(),
            base_power: 100,
            accuracy: 100,
            pp: 5,
            category: MoveCategory::Special,
            move_type: "Ghost".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            ignores_ability: true,
            ..Default::default()
        });
        map.insert(ID::new("moonlight"), MoveDef {
            id: ID::new("moonlight"),
            name: "Moonlight".to_string(),
            base_power: 0,
            pp: 5,
            category: MoveCategory::Status,
            move_type: "Fairy".to_string(),
            target: MoveTargetType::Self_,
            flags: MoveFlags { snatch: true, heal: true, ..Default::default() },
            heal: Some(0.5),
            ..Default::default()
        });
        map.insert(ID::new("morningsun"), MoveDef {
            id: ID::new("morningsun"),
            name: "Morning Sun".to_string(),
            base_power: 0,
            pp: 5,
            category: MoveCategory::Status,
            move_type: "Normal".to_string(),
            target: MoveTargetType::Self_,
            flags: MoveFlags { snatch: true, heal: true, ..Default::default() },
            heal: Some(0.5),
            ..Default::default()
        });
        map.insert(ID::new("mortalspin"), MoveDef {
            id: ID::new("mortalspin"),
            name: "Mortal Spin".to_string(),
            base_power: 30,
            accuracy: 100,
            pp: 15,
            category: MoveCategory::Physical,
            move_type: "Poison".to_string(),
            target: MoveTargetType::AllAdjacentFoes,
            flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 100,
                status: Some("psn".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("mountaingale"), MoveDef {
            id: ID::new("mountaingale"),
            name: "Mountain Gale".to_string(),
            base_power: 100,
            accuracy: 85,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Ice".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 30,
                volatile_status: Some("flinch".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("mudbomb"), MoveDef {
            id: ID::new("mudbomb"),
            name: "Mud Bomb".to_string(),
            base_power: 65,
            accuracy: 85,
            pp: 10,
            category: MoveCategory::Special,
            move_type: "Ground".to_string(),
            flags: MoveFlags { protect: true, mirror: true, bullet: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 30,
                boosts: Some(vec![("accuracy".to_string(), -1)]),
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("muddywater"), MoveDef {
            id: ID::new("muddywater"),
            name: "Muddy Water".to_string(),
            base_power: 90,
            accuracy: 85,
            pp: 10,
            category: MoveCategory::Special,
            move_type: "Water".to_string(),
            target: MoveTargetType::AllAdjacentFoes,
            flags: MoveFlags { protect: true, mirror: true, nonsky: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 30,
                boosts: Some(vec![("accuracy".to_string(), -1)]),
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("mudshot"), MoveDef {
            id: ID::new("mudshot"),
            name: "Mud Shot".to_string(),
            base_power: 55,
            accuracy: 95,
            pp: 15,
            category: MoveCategory::Special,
            move_type: "Ground".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 100,
                boosts: Some(vec![("spe".to_string(), -1)]),
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("mudslap"), MoveDef {
            id: ID::new("mudslap"),
            name: "Mud-Slap".to_string(),
            base_power: 20,
            accuracy: 100,
            pp: 10,
            category: MoveCategory::Special,
            move_type: "Ground".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 100,
                boosts: Some(vec![("accuracy".to_string(), -1)]),
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("mudsport"), MoveDef {
            id: ID::new("mudsport"),
            name: "Mud Sport".to_string(),
            base_power: 0,
            pp: 15,
            category: MoveCategory::Status,
            move_type: "Ground".to_string(),
            target: MoveTargetType::All,
            flags: MoveFlags { nonsky: true, ..Default::default() },
            pseudo_weather: Some("mudsport".to_string()),
            ..Default::default()
        });
        map.insert(ID::new("multiattack"), MoveDef {
            id: ID::new("multiattack"),
            name: "Multi-Attack".to_string(),
            base_power: 120,
            accuracy: 100,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Normal".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("mysticalfire"), MoveDef {
            id: ID::new("mysticalfire"),
            name: "Mystical Fire".to_string(),
            base_power: 75,
            accuracy: 100,
            pp: 10,
            category: MoveCategory::Special,
            move_type: "Fire".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 100,
                boosts: Some(vec![("spa".to_string(), -1)]),
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("mysticalpower"), MoveDef {
            id: ID::new("mysticalpower"),
            name: "Mystical Power".to_string(),
            base_power: 70,
            accuracy: 90,
            pp: 10,
            category: MoveCategory::Special,
            move_type: "Psychic".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 100,
                boosts: Some(vec![("spa".to_string(), 1)]),
                self_boost: true, ..Default::default()
            }],
            ..Default::default()
        });
        // N moves
        map.insert(ID::new("nastyplot"), MoveDef {
            id: ID::new("nastyplot"),
            name: "Nasty Plot".to_string(),
            base_power: 0,
            pp: 20,
            category: MoveCategory::Status,
            move_type: "Dark".to_string(),
            target: MoveTargetType::Self_,
            flags: MoveFlags { snatch: true, ..Default::default() },
            boosts: Some(vec![("spa".to_string(), 2)]),
            ..Default::default()
        });
        map.insert(ID::new("naturalgift"), MoveDef {
            id: ID::new("naturalgift"),
            name: "Natural Gift".to_string(),
            base_power: 0,
            accuracy: 100,
            pp: 15,
            category: MoveCategory::Physical,
            move_type: "Normal".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("naturepower"), MoveDef {
            id: ID::new("naturepower"),
            name: "Nature Power".to_string(),
            base_power: 0,
            pp: 20,
            category: MoveCategory::Status,
            move_type: "Normal".to_string(),
            flags: MoveFlags { ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("naturesmadness"), MoveDef {
            id: ID::new("naturesmadness"),
            name: "Nature's Madness".to_string(),
            base_power: 0,
            accuracy: 90,
            pp: 10,
            category: MoveCategory::Special,
            move_type: "Fairy".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("needlearm"), MoveDef {
            id: ID::new("needlearm"),
            name: "Needle Arm".to_string(),
            base_power: 60,
            accuracy: 100,
            pp: 15,
            category: MoveCategory::Physical,
            move_type: "Grass".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 30,
                volatile_status: Some("flinch".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("neverendingnightmare"), MoveDef {
            id: ID::new("neverendingnightmare"),
            name: "Never-Ending Nightmare".to_string(),
            base_power: 1,
            pp: 1,
            category: MoveCategory::Physical,
            move_type: "Ghost".to_string(),
            flags: MoveFlags { ..Default::default() },
            is_z: true,
            ..Default::default()
        });
        map.insert(ID::new("nightdaze"), MoveDef {
            id: ID::new("nightdaze"),
            name: "Night Daze".to_string(),
            base_power: 85,
            accuracy: 95,
            pp: 10,
            category: MoveCategory::Special,
            move_type: "Dark".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 40,
                boosts: Some(vec![("accuracy".to_string(), -1)]),
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("nightmare"), MoveDef {
            id: ID::new("nightmare"),
            name: "Nightmare".to_string(),
            base_power: 0,
            accuracy: 100,
            pp: 15,
            category: MoveCategory::Status,
            move_type: "Ghost".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            volatile_status: Some("nightmare".to_string()),
            ..Default::default()
        });
        map.insert(ID::new("nightshade"), MoveDef {
            id: ID::new("nightshade"),
            name: "Night Shade".to_string(),
            base_power: 0,
            accuracy: 100,
            pp: 15,
            category: MoveCategory::Special,
            move_type: "Ghost".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("nightslash"), MoveDef {
            id: ID::new("nightslash"),
            name: "Night Slash".to_string(),
            base_power: 70,
            accuracy: 100,
            pp: 15,
            category: MoveCategory::Physical,
            move_type: "Dark".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, slicing: true, ..Default::default() },
            crit_ratio: 2,
            ..Default::default()
        });
        map.insert(ID::new("nobleroar"), MoveDef {
            id: ID::new("nobleroar"),
            name: "Noble Roar".to_string(),
            base_power: 0,
            accuracy: 100,
            pp: 30,
            category: MoveCategory::Status,
            move_type: "Normal".to_string(),
            flags: MoveFlags { protect: true, reflectable: true, mirror: true, sound: true, ..Default::default() },
            boosts: Some(vec![("atk".to_string(), -1), ("spa".to_string(), -1)]),
            ..Default::default()
        });
        map.insert(ID::new("noretreat"), MoveDef {
            id: ID::new("noretreat"),
            name: "No Retreat".to_string(),
            base_power: 0,
            pp: 5,
            category: MoveCategory::Status,
            move_type: "Fighting".to_string(),
            target: MoveTargetType::Self_,
            flags: MoveFlags { snatch: true, ..Default::default() },
            volatile_status: Some("noretreat".to_string()),
            boosts: Some(vec![("atk".to_string(), 1), ("def".to_string(), 1), ("spa".to_string(), 1), ("spd".to_string(), 1), ("spe".to_string(), 1)]),
            ..Default::default()
        });
        map.insert(ID::new("noxioustorque"), MoveDef {
            id: ID::new("noxioustorque"),
            name: "Noxious Torque".to_string(),
            base_power: 100,
            accuracy: 100,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Poison".to_string(),
            flags: MoveFlags { protect: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 30,
                status: Some("psn".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("nuzzle"), MoveDef {
            id: ID::new("nuzzle"),
            name: "Nuzzle".to_string(),
            base_power: 20,
            accuracy: 100,
            pp: 20,
            category: MoveCategory::Physical,
            move_type: "Electric".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 100,
                status: Some("par".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        });
        // O moves
        map.insert(ID::new("oblivionwing"), MoveDef {
            id: ID::new("oblivionwing"),
            name: "Oblivion Wing".to_string(),
            base_power: 80,
            accuracy: 100,
            pp: 10,
            category: MoveCategory::Special,
            move_type: "Flying".to_string(),
            target: MoveTargetType::Any,
            flags: MoveFlags { protect: true, mirror: true, distance: true, heal: true, ..Default::default() },
            drain: Some(0.75),
            ..Default::default()
        });
        map.insert(ID::new("obstruct"), MoveDef {
            id: ID::new("obstruct"),
            name: "Obstruct".to_string(),
            base_power: 0,
            accuracy: 100,
            pp: 10,
            priority: 4,
            category: MoveCategory::Status,
            move_type: "Dark".to_string(),
            target: MoveTargetType::Self_,
            flags: MoveFlags { ..Default::default() },
            volatile_status: Some("obstruct".to_string()),
            ..Default::default()
        });
        map.insert(ID::new("oceanicoperetta"), MoveDef {
            id: ID::new("oceanicoperetta"),
            name: "Oceanic Operetta".to_string(),
            base_power: 195,
            pp: 1,
            category: MoveCategory::Special,
            move_type: "Water".to_string(),
            flags: MoveFlags { ..Default::default() },
            is_z: true,
            ..Default::default()
        });
        map.insert(ID::new("octazooka"), MoveDef {
            id: ID::new("octazooka"),
            name: "Octazooka".to_string(),
            base_power: 65,
            accuracy: 85,
            pp: 10,
            category: MoveCategory::Special,
            move_type: "Water".to_string(),
            flags: MoveFlags { protect: true, mirror: true, bullet: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 50,
                boosts: Some(vec![("accuracy".to_string(), -1)]),
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("octolock"), MoveDef {
            id: ID::new("octolock"),
            name: "Octolock".to_string(),
            base_power: 0,
            accuracy: 100,
            pp: 15,
            category: MoveCategory::Status,
            move_type: "Fighting".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            volatile_status: Some("octolock".to_string()),
            ..Default::default()
        });
        map.insert(ID::new("odorsleuth"), MoveDef {
            id: ID::new("odorsleuth"),
            name: "Odor Sleuth".to_string(),
            base_power: 0,
            pp: 40,
            category: MoveCategory::Status,
            move_type: "Normal".to_string(),
            flags: MoveFlags { protect: true, reflectable: true, mirror: true, ..Default::default() },
            volatile_status: Some("foresight".to_string()),
            ..Default::default()
        });
        map.insert(ID::new("ominouswind"), MoveDef {
            id: ID::new("ominouswind"),
            name: "Ominous Wind".to_string(),
            base_power: 60,
            accuracy: 100,
            pp: 5,
            category: MoveCategory::Special,
            move_type: "Ghost".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 10,
                boosts: Some(vec![("atk".to_string(), 1), ("def".to_string(), 1), ("spa".to_string(), 1), ("spd".to_string(), 1), ("spe".to_string(), 1)]),
                self_boost: true,
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("orderup"), MoveDef {
            id: ID::new("orderup"),
            name: "Order Up".to_string(),
            base_power: 80,
            accuracy: 100,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Dragon".to_string(),
            flags: MoveFlags { protect: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("originpulse"), MoveDef {
            id: ID::new("originpulse"),
            name: "Origin Pulse".to_string(),
            base_power: 110,
            accuracy: 85,
            pp: 10,
            category: MoveCategory::Special,
            move_type: "Water".to_string(),
            target: MoveTargetType::AllAdjacentFoes,
            flags: MoveFlags { protect: true, mirror: true, pulse: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("outrage"), MoveDef {
            id: ID::new("outrage"),
            name: "Outrage".to_string(),
            base_power: 120,
            accuracy: 100,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Dragon".to_string(),
            target: MoveTargetType::RandomNormal,
            flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
            self_volatile: Some("lockedmove".to_string()),
            ..Default::default()
        });
        map.insert(ID::new("overdrive"), MoveDef {
            id: ID::new("overdrive"),
            name: "Overdrive".to_string(),
            base_power: 80,
            accuracy: 100,
            pp: 10,
            category: MoveCategory::Special,
            move_type: "Electric".to_string(),
            target: MoveTargetType::AllAdjacentFoes,
            flags: MoveFlags { protect: true, mirror: true, sound: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("overheat"), MoveDef {
            id: ID::new("overheat"),
            name: "Overheat".to_string(),
            base_power: 130,
            accuracy: 90,
            pp: 5,
            category: MoveCategory::Special,
            move_type: "Fire".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            self_boosts: Some(vec![("spa".to_string(), -2)]),
            ..Default::default()
        });
        // P moves
        map.insert(ID::new("painsplit"), MoveDef {
            id: ID::new("painsplit"),
            name: "Pain Split".to_string(),
            base_power: 0,
            pp: 20,
            category: MoveCategory::Status,
            move_type: "Normal".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("paraboliccharge"), MoveDef {
            id: ID::new("paraboliccharge"),
            name: "Parabolic Charge".to_string(),
            base_power: 65,
            accuracy: 100,
            pp: 20,
            category: MoveCategory::Special,
            move_type: "Electric".to_string(),
            target: MoveTargetType::AllAdjacent,
            flags: MoveFlags { protect: true, mirror: true, heal: true, ..Default::default() },
            drain: Some(0.5),
            ..Default::default()
        });
        map.insert(ID::new("partingshot"), MoveDef {
            id: ID::new("partingshot"),
            name: "Parting Shot".to_string(),
            base_power: 0,
            accuracy: 100,
            pp: 20,
            category: MoveCategory::Status,
            move_type: "Dark".to_string(),
            flags: MoveFlags { protect: true, reflectable: true, mirror: true, sound: true, ..Default::default() },
            boosts: Some(vec![("atk".to_string(), -1), ("spa".to_string(), -1)]),
            is_pivot: true,
            ..Default::default()
        });
        map.insert(ID::new("payback"), MoveDef {
            id: ID::new("payback"),
            name: "Payback".to_string(),
            base_power: 50,
            accuracy: 100,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Dark".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("payday"), MoveDef {
            id: ID::new("payday"),
            name: "Pay Day".to_string(),
            base_power: 40,
            accuracy: 100,
            pp: 20,
            category: MoveCategory::Physical,
            move_type: "Normal".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("peck"), MoveDef {
            id: ID::new("peck"),
            name: "Peck".to_string(),
            base_power: 35,
            accuracy: 100,
            pp: 35,
            category: MoveCategory::Physical,
            move_type: "Flying".to_string(),
            target: MoveTargetType::Any,
            flags: MoveFlags { contact: true, protect: true, mirror: true, distance: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("perishsong"), MoveDef {
            id: ID::new("perishsong"),
            name: "Perish Song".to_string(),
            base_power: 0,
            pp: 5,
            category: MoveCategory::Status,
            move_type: "Normal".to_string(),
            target: MoveTargetType::All,
            flags: MoveFlags { sound: true, distance: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("petalblizzard"), MoveDef {
            id: ID::new("petalblizzard"),
            name: "Petal Blizzard".to_string(),
            base_power: 90,
            accuracy: 100,
            pp: 15,
            category: MoveCategory::Physical,
            move_type: "Grass".to_string(),
            target: MoveTargetType::AllAdjacent,
            flags: MoveFlags { protect: true, mirror: true, wind: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("petaldance"), MoveDef {
            id: ID::new("petaldance"),
            name: "Petal Dance".to_string(),
            base_power: 120,
            accuracy: 100,
            pp: 10,
            category: MoveCategory::Special,
            move_type: "Grass".to_string(),
            target: MoveTargetType::RandomNormal,
            flags: MoveFlags { contact: true, protect: true, mirror: true, dance: true, ..Default::default() },
            self_volatile: Some("lockedmove".to_string()),
            ..Default::default()
        });
        map.insert(ID::new("phantomforce"), MoveDef {
            id: ID::new("phantomforce"),
            name: "Phantom Force".to_string(),
            base_power: 90,
            accuracy: 100,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Ghost".to_string(),
            flags: MoveFlags { contact: true, charge: true, mirror: true, ..Default::default() },
            breaks_protect: true,
            ..Default::default()
        });
        map.insert(ID::new("photongeyser"), MoveDef {
            id: ID::new("photongeyser"),
            name: "Photon Geyser".to_string(),
            base_power: 100,
            accuracy: 100,
            pp: 5,
            category: MoveCategory::Special,
            move_type: "Psychic".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            ignores_ability: true,
            ..Default::default()
        });
        map.insert(ID::new("pinmissile"), MoveDef {
            id: ID::new("pinmissile"),
            name: "Pin Missile".to_string(),
            base_power: 25,
            accuracy: 95,
            pp: 20,
            category: MoveCategory::Physical,
            move_type: "Bug".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            multi_hit: Some((2, 5)),
            ..Default::default()
        });
        map.insert(ID::new("plasmafists"), MoveDef {
            id: ID::new("plasmafists"),
            name: "Plasma Fists".to_string(),
            base_power: 100,
            accuracy: 100,
            pp: 15,
            category: MoveCategory::Physical,
            move_type: "Electric".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, punch: true, ..Default::default() },
            pseudo_weather: Some("iondeluge".to_string()),
            ..Default::default()
        });
        map.insert(ID::new("playnice"), MoveDef {
            id: ID::new("playnice"),
            name: "Play Nice".to_string(),
            base_power: 0,
            pp: 20,
            category: MoveCategory::Status,
            move_type: "Normal".to_string(),
            flags: MoveFlags { reflectable: true, mirror: true, ..Default::default() },
            boosts: Some(vec![("atk".to_string(), -1)]),
            ..Default::default()
        });
        map.insert(ID::new("playrough"), MoveDef {
            id: ID::new("playrough"),
            name: "Play Rough".to_string(),
            base_power: 90,
            accuracy: 90,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Fairy".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 10,
                boosts: Some(vec![("atk".to_string(), -1)]),
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("pluck"), MoveDef {
            id: ID::new("pluck"),
            name: "Pluck".to_string(),
            base_power: 60,
            accuracy: 100,
            pp: 20,
            category: MoveCategory::Physical,
            move_type: "Flying".to_string(),
            target: MoveTargetType::Any,
            flags: MoveFlags { contact: true, protect: true, mirror: true, distance: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("poisonfang"), MoveDef {
            id: ID::new("poisonfang"),
            name: "Poison Fang".to_string(),
            base_power: 50,
            accuracy: 100,
            pp: 15,
            category: MoveCategory::Physical,
            move_type: "Poison".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, bite: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 50,
                status: Some("tox".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("poisongas"), MoveDef {
            id: ID::new("poisongas"),
            name: "Poison Gas".to_string(),
            base_power: 0,
            accuracy: 90,
            pp: 40,
            category: MoveCategory::Status,
            move_type: "Poison".to_string(),
            target: MoveTargetType::AllAdjacentFoes,
            flags: MoveFlags { protect: true, reflectable: true, mirror: true, ..Default::default() },
            status: Some("psn".to_string()),
            ..Default::default()
        });
        map.insert(ID::new("poisonjab"), MoveDef {
            id: ID::new("poisonjab"),
            name: "Poison Jab".to_string(),
            base_power: 80,
            accuracy: 100,
            pp: 20,
            category: MoveCategory::Physical,
            move_type: "Poison".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 30,
                status: Some("psn".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("poisonpowder"), MoveDef {
            id: ID::new("poisonpowder"),
            name: "Poison Powder".to_string(),
            base_power: 0,
            accuracy: 75,
            pp: 35,
            category: MoveCategory::Status,
            move_type: "Poison".to_string(),
            flags: MoveFlags { protect: true, reflectable: true, mirror: true, powder: true, ..Default::default() },
            status: Some("psn".to_string()),
            ..Default::default()
        });
        map.insert(ID::new("poisonsting"), MoveDef {
            id: ID::new("poisonsting"),
            name: "Poison Sting".to_string(),
            base_power: 15,
            accuracy: 100,
            pp: 35,
            category: MoveCategory::Physical,
            move_type: "Poison".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 30,
                status: Some("psn".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("poisontail"), MoveDef {
            id: ID::new("poisontail"),
            name: "Poison Tail".to_string(),
            base_power: 50,
            accuracy: 100,
            pp: 25,
            category: MoveCategory::Physical,
            move_type: "Poison".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
            crit_ratio: 2,
            secondaries: vec![SecondaryEffect {
                chance: 10,
                status: Some("psn".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("pollenpuff"), MoveDef {
            id: ID::new("pollenpuff"),
            name: "Pollen Puff".to_string(),
            base_power: 90,
            accuracy: 100,
            pp: 15,
            category: MoveCategory::Special,
            move_type: "Bug".to_string(),
            flags: MoveFlags { protect: true, mirror: true, bullet: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("poltergeist"), MoveDef {
            id: ID::new("poltergeist"),
            name: "Poltergeist".to_string(),
            base_power: 110,
            accuracy: 90,
            pp: 5,
            category: MoveCategory::Physical,
            move_type: "Ghost".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("populationbomb"), MoveDef {
            id: ID::new("populationbomb"),
            name: "Population Bomb".to_string(),
            base_power: 20,
            accuracy: 90,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Normal".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, slicing: true, ..Default::default() },
            multi_hit: Some((10, 10)),
            ..Default::default()
        });
        map.insert(ID::new("pounce"), MoveDef {
            id: ID::new("pounce"),
            name: "Pounce".to_string(),
            base_power: 50,
            accuracy: 100,
            pp: 20,
            category: MoveCategory::Physical,
            move_type: "Bug".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 100,
                boosts: Some(vec![("spe".to_string(), -1)]),
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("pound"), MoveDef {
            id: ID::new("pound"),
            name: "Pound".to_string(),
            base_power: 40,
            accuracy: 100,
            pp: 35,
            category: MoveCategory::Physical,
            move_type: "Normal".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("powder"), MoveDef {
            id: ID::new("powder"),
            name: "Powder".to_string(),
            base_power: 0,
            accuracy: 100,
            pp: 20,
            priority: 1,
            category: MoveCategory::Status,
            move_type: "Bug".to_string(),
            flags: MoveFlags { protect: true, reflectable: true, mirror: true, powder: true, ..Default::default() },
            volatile_status: Some("powder".to_string()),
            ..Default::default()
        });
        map.insert(ID::new("powdersnow"), MoveDef {
            id: ID::new("powdersnow"),
            name: "Powder Snow".to_string(),
            base_power: 40,
            accuracy: 100,
            pp: 25,
            category: MoveCategory::Special,
            move_type: "Ice".to_string(),
            target: MoveTargetType::AllAdjacentFoes,
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 10,
                status: Some("frz".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("powergem"), MoveDef {
            id: ID::new("powergem"),
            name: "Power Gem".to_string(),
            base_power: 80,
            accuracy: 100,
            pp: 20,
            category: MoveCategory::Special,
            move_type: "Rock".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("powersplit"), MoveDef {
            id: ID::new("powersplit"),
            name: "Power Split".to_string(),
            base_power: 0,
            pp: 10,
            category: MoveCategory::Status,
            move_type: "Psychic".to_string(),
            flags: MoveFlags { protect: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("powerswap"), MoveDef {
            id: ID::new("powerswap"),
            name: "Power Swap".to_string(),
            base_power: 0,
            pp: 10,
            category: MoveCategory::Status,
            move_type: "Psychic".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("powertrick"), MoveDef {
            id: ID::new("powertrick"),
            name: "Power Trick".to_string(),
            base_power: 0,
            pp: 10,
            category: MoveCategory::Status,
            move_type: "Psychic".to_string(),
            target: MoveTargetType::Self_,
            flags: MoveFlags { snatch: true, ..Default::default() },
            volatile_status: Some("powertrick".to_string()),
            ..Default::default()
        });
        map.insert(ID::new("powertrip"), MoveDef {
            id: ID::new("powertrip"),
            name: "Power Trip".to_string(),
            base_power: 20,
            accuracy: 100,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Dark".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("poweruppunch"), MoveDef {
            id: ID::new("poweruppunch"),
            name: "Power-Up Punch".to_string(),
            base_power: 40,
            accuracy: 100,
            pp: 20,
            category: MoveCategory::Physical,
            move_type: "Fighting".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, punch: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 100,
                boosts: Some(vec![("atk".to_string(), 1)]),
                self_boost: true,
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("powerwhip"), MoveDef {
            id: ID::new("powerwhip"),
            name: "Power Whip".to_string(),
            base_power: 120,
            accuracy: 85,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Grass".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("precipiceblades"), MoveDef {
            id: ID::new("precipiceblades"),
            name: "Precipice Blades".to_string(),
            base_power: 120,
            accuracy: 85,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Ground".to_string(),
            target: MoveTargetType::AllAdjacentFoes,
            flags: MoveFlags { protect: true, mirror: true, nonsky: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("present"), MoveDef {
            id: ID::new("present"),
            name: "Present".to_string(),
            base_power: 0,
            accuracy: 90,
            pp: 15,
            category: MoveCategory::Physical,
            move_type: "Normal".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("prismaticlaser"), MoveDef {
            id: ID::new("prismaticlaser"),
            name: "Prismatic Laser".to_string(),
            base_power: 160,
            accuracy: 100,
            pp: 10,
            category: MoveCategory::Special,
            move_type: "Psychic".to_string(),
            flags: MoveFlags { recharge: true, protect: true, mirror: true, ..Default::default() },
            self_volatile: Some("mustrecharge".to_string()),
            ..Default::default()
        });
        map.insert(ID::new("protect"), MoveDef {
            id: ID::new("protect"),
            name: "Protect".to_string(),
            base_power: 0,
            pp: 10,
            priority: 4,
            category: MoveCategory::Status,
            move_type: "Normal".to_string(),
            target: MoveTargetType::Self_,
            flags: MoveFlags { ..Default::default() },
            volatile_status: Some("protect".to_string()),
            ..Default::default()
        });
        map.insert(ID::new("psybeam"), MoveDef {
            id: ID::new("psybeam"),
            name: "Psybeam".to_string(),
            base_power: 65,
            accuracy: 100,
            pp: 20,
            category: MoveCategory::Special,
            move_type: "Psychic".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 10,
                volatile_status: Some("confusion".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("psyblade"), MoveDef {
            id: ID::new("psyblade"),
            name: "Psyblade".to_string(),
            base_power: 80,
            accuracy: 100,
            pp: 15,
            category: MoveCategory::Physical,
            move_type: "Psychic".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, slicing: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("psychic"), MoveDef {
            id: ID::new("psychic"),
            name: "Psychic".to_string(),
            base_power: 90,
            accuracy: 100,
            pp: 10,
            category: MoveCategory::Special,
            move_type: "Psychic".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 10,
                boosts: Some(vec![("spd".to_string(), -1)]),
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("psychicfangs"), MoveDef {
            id: ID::new("psychicfangs"),
            name: "Psychic Fangs".to_string(),
            base_power: 85,
            accuracy: 100,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Psychic".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, bite: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("psychicnoise"), MoveDef {
            id: ID::new("psychicnoise"),
            name: "Psychic Noise".to_string(),
            base_power: 75,
            accuracy: 100,
            pp: 10,
            category: MoveCategory::Special,
            move_type: "Psychic".to_string(),
            flags: MoveFlags { protect: true, mirror: true, sound: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 100,
                volatile_status: Some("healblock".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("psychicterrain"), MoveDef {
            id: ID::new("psychicterrain"),
            name: "Psychic Terrain".to_string(),
            base_power: 0,
            pp: 10,
            category: MoveCategory::Status,
            move_type: "Psychic".to_string(),
            target: MoveTargetType::All,
            flags: MoveFlags { nonsky: true, ..Default::default() },
            terrain: Some("psychicterrain".to_string()),
            ..Default::default()
        });
        map.insert(ID::new("psychoboost"), MoveDef {
            id: ID::new("psychoboost"),
            name: "Psycho Boost".to_string(),
            base_power: 140,
            accuracy: 90,
            pp: 5,
            category: MoveCategory::Special,
            move_type: "Psychic".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            self_boosts: Some(vec![("spa".to_string(), -2)]),
            ..Default::default()
        });
        map.insert(ID::new("psychocut"), MoveDef {
            id: ID::new("psychocut"),
            name: "Psycho Cut".to_string(),
            base_power: 70,
            accuracy: 100,
            pp: 20,
            category: MoveCategory::Physical,
            move_type: "Psychic".to_string(),
            flags: MoveFlags { protect: true, mirror: true, slicing: true, ..Default::default() },
            crit_ratio: 2,
            ..Default::default()
        });
        map.insert(ID::new("psychoshift"), MoveDef {
            id: ID::new("psychoshift"),
            name: "Psycho Shift".to_string(),
            base_power: 0,
            accuracy: 100,
            pp: 10,
            category: MoveCategory::Status,
            move_type: "Psychic".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("psychup"), MoveDef {
            id: ID::new("psychup"),
            name: "Psych Up".to_string(),
            base_power: 0,
            pp: 10,
            category: MoveCategory::Status,
            move_type: "Normal".to_string(),
            flags: MoveFlags { ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("psyshieldbash"), MoveDef {
            id: ID::new("psyshieldbash"),
            name: "Psyshield Bash".to_string(),
            base_power: 70,
            accuracy: 90,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Psychic".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 100,
                boosts: Some(vec![("def".to_string(), 1)]),
                self_boost: true,
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("psyshock"), MoveDef {
            id: ID::new("psyshock"),
            name: "Psyshock".to_string(),
            base_power: 80,
            accuracy: 100,
            pp: 10,
            category: MoveCategory::Special,
            move_type: "Psychic".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("psystrike"), MoveDef {
            id: ID::new("psystrike"),
            name: "Psystrike".to_string(),
            base_power: 100,
            accuracy: 100,
            pp: 10,
            category: MoveCategory::Special,
            move_type: "Psychic".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("psywave"), MoveDef {
            id: ID::new("psywave"),
            name: "Psywave".to_string(),
            base_power: 0,
            accuracy: 100,
            pp: 15,
            category: MoveCategory::Special,
            move_type: "Psychic".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("pulverizingpancake"), MoveDef {
            id: ID::new("pulverizingpancake"),
            name: "Pulverizing Pancake".to_string(),
            base_power: 210,
            pp: 1,
            category: MoveCategory::Physical,
            move_type: "Normal".to_string(),
            flags: MoveFlags { contact: true, ..Default::default() },
            is_z: true,
            ..Default::default()
        });
        map.insert(ID::new("punishment"), MoveDef {
            id: ID::new("punishment"),
            name: "Punishment".to_string(),
            base_power: 0,
            accuracy: 100,
            pp: 5,
            category: MoveCategory::Physical,
            move_type: "Dark".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("purify"), MoveDef {
            id: ID::new("purify"),
            name: "Purify".to_string(),
            base_power: 0,
            pp: 20,
            category: MoveCategory::Status,
            move_type: "Poison".to_string(),
            flags: MoveFlags { protect: true, reflectable: true, heal: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("pursuit"), MoveDef {
            id: ID::new("pursuit"),
            name: "Pursuit".to_string(),
            base_power: 40,
            accuracy: 100,
            pp: 20,
            category: MoveCategory::Physical,
            move_type: "Dark".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("pyroball"), MoveDef {
            id: ID::new("pyroball"),
            name: "Pyro Ball".to_string(),
            base_power: 120,
            accuracy: 90,
            pp: 5,
            category: MoveCategory::Physical,
            move_type: "Fire".to_string(),
            flags: MoveFlags { protect: true, mirror: true, defrost: true, bullet: true, ..Default::default() },
            thaws_user: true,
            secondaries: vec![SecondaryEffect {
                chance: 10,
                status: Some("brn".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        });
        // Q moves
        map.insert(ID::new("quash"), MoveDef {
            id: ID::new("quash"),
            name: "Quash".to_string(),
            base_power: 0,
            accuracy: 100,
            pp: 15,
            category: MoveCategory::Status,
            move_type: "Dark".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("quickattack"), MoveDef {
            id: ID::new("quickattack"),
            name: "Quick Attack".to_string(),
            base_power: 40,
            accuracy: 100,
            pp: 30,
            priority: 1,
            category: MoveCategory::Physical,
            move_type: "Normal".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("quickguard"), MoveDef {
            id: ID::new("quickguard"),
            name: "Quick Guard".to_string(),
            base_power: 0,
            pp: 15,
            priority: 3,
            category: MoveCategory::Status,
            move_type: "Fighting".to_string(),
            target: MoveTargetType::AllySide,
            flags: MoveFlags { snatch: true, ..Default::default() },
            side_condition: Some("quickguard".to_string()),
            ..Default::default()
        });
        map.insert(ID::new("quiverdance"), MoveDef {
            id: ID::new("quiverdance"),
            name: "Quiver Dance".to_string(),
            base_power: 0,
            pp: 20,
            category: MoveCategory::Status,
            move_type: "Bug".to_string(),
            target: MoveTargetType::Self_,
            flags: MoveFlags { snatch: true, dance: true, ..Default::default() },
            boosts: Some(vec![("spa".to_string(), 1), ("spd".to_string(), 1), ("spe".to_string(), 1)]),
            ..Default::default()
        });
        // R moves
        map.insert(ID::new("rage"), MoveDef {
            id: ID::new("rage"),
            name: "Rage".to_string(),
            base_power: 20,
            accuracy: 100,
            pp: 20,
            category: MoveCategory::Physical,
            move_type: "Normal".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
            self_volatile: Some("rage".to_string()),
            ..Default::default()
        });
        map.insert(ID::new("ragefist"), MoveDef {
            id: ID::new("ragefist"),
            name: "Rage Fist".to_string(),
            base_power: 50,
            accuracy: 100,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Ghost".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, punch: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("ragepowder"), MoveDef {
            id: ID::new("ragepowder"),
            name: "Rage Powder".to_string(),
            base_power: 0,
            pp: 20,
            priority: 2,
            category: MoveCategory::Status,
            move_type: "Bug".to_string(),
            target: MoveTargetType::Self_,
            flags: MoveFlags { powder: true, ..Default::default() },
            volatile_status: Some("ragepowder".to_string()),
            ..Default::default()
        });
        map.insert(ID::new("ragingbull"), MoveDef {
            id: ID::new("ragingbull"),
            name: "Raging Bull".to_string(),
            base_power: 90,
            accuracy: 100,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Normal".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("ragingfury"), MoveDef {
            id: ID::new("ragingfury"),
            name: "Raging Fury".to_string(),
            base_power: 120,
            accuracy: 100,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Fire".to_string(),
            target: MoveTargetType::RandomNormal,
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            self_volatile: Some("lockedmove".to_string()),
            ..Default::default()
        });
        map.insert(ID::new("raindance"), MoveDef {
            id: ID::new("raindance"),
            name: "Rain Dance".to_string(),
            base_power: 0,
            pp: 5,
            category: MoveCategory::Status,
            move_type: "Water".to_string(),
            target: MoveTargetType::All,
            flags: MoveFlags { ..Default::default() },
            weather: Some("RainDance".to_string()),
            ..Default::default()
        });
        map.insert(ID::new("rapidspin"), MoveDef {
            id: ID::new("rapidspin"),
            name: "Rapid Spin".to_string(),
            base_power: 50,
            accuracy: 100,
            pp: 40,
            category: MoveCategory::Physical,
            move_type: "Normal".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 100,
                boosts: Some(vec![("spe".to_string(), 1)]),
                self_boost: true,
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("razorleaf"), MoveDef {
            id: ID::new("razorleaf"),
            name: "Razor Leaf".to_string(),
            base_power: 55,
            accuracy: 95,
            pp: 25,
            category: MoveCategory::Physical,
            move_type: "Grass".to_string(),
            target: MoveTargetType::AllAdjacentFoes,
            flags: MoveFlags { protect: true, mirror: true, slicing: true, ..Default::default() },
            crit_ratio: 2,
            ..Default::default()
        });
        map.insert(ID::new("razorshell"), MoveDef {
            id: ID::new("razorshell"),
            name: "Razor Shell".to_string(),
            base_power: 75,
            accuracy: 95,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Water".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, slicing: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 50,
                boosts: Some(vec![("def".to_string(), -1)]),
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("razorwind"), MoveDef {
            id: ID::new("razorwind"),
            name: "Razor Wind".to_string(),
            base_power: 80,
            accuracy: 100,
            pp: 10,
            category: MoveCategory::Special,
            move_type: "Normal".to_string(),
            target: MoveTargetType::AllAdjacentFoes,
            flags: MoveFlags { charge: true, protect: true, mirror: true, ..Default::default() },
            crit_ratio: 2,
            ..Default::default()
        });
        map.insert(ID::new("recover"), MoveDef {
            id: ID::new("recover"),
            name: "Recover".to_string(),
            base_power: 0,
            pp: 5,
            category: MoveCategory::Status,
            move_type: "Normal".to_string(),
            target: MoveTargetType::Self_,
            flags: MoveFlags { snatch: true, heal: true, ..Default::default() },
            heal: Some(0.5),
            ..Default::default()
        });
        map.insert(ID::new("recycle"), MoveDef {
            id: ID::new("recycle"),
            name: "Recycle".to_string(),
            base_power: 0,
            pp: 10,
            category: MoveCategory::Status,
            move_type: "Normal".to_string(),
            target: MoveTargetType::Self_,
            flags: MoveFlags { snatch: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("reflect"), MoveDef {
            id: ID::new("reflect"),
            name: "Reflect".to_string(),
            base_power: 0,
            pp: 20,
            category: MoveCategory::Status,
            move_type: "Psychic".to_string(),
            target: MoveTargetType::AllySide,
            flags: MoveFlags { snatch: true, ..Default::default() },
            side_condition: Some("reflect".to_string()),
            ..Default::default()
        });
        map.insert(ID::new("reflecttype"), MoveDef {
            id: ID::new("reflecttype"),
            name: "Reflect Type".to_string(),
            base_power: 0,
            pp: 15,
            category: MoveCategory::Status,
            move_type: "Normal".to_string(),
            flags: MoveFlags { protect: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("refresh"), MoveDef {
            id: ID::new("refresh"),
            name: "Refresh".to_string(),
            base_power: 0,
            pp: 20,
            category: MoveCategory::Status,
            move_type: "Normal".to_string(),
            target: MoveTargetType::Self_,
            flags: MoveFlags { snatch: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("relicsong"), MoveDef {
            id: ID::new("relicsong"),
            name: "Relic Song".to_string(),
            base_power: 75,
            accuracy: 100,
            pp: 10,
            category: MoveCategory::Special,
            move_type: "Normal".to_string(),
            target: MoveTargetType::AllAdjacentFoes,
            flags: MoveFlags { protect: true, mirror: true, sound: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 10,
                status: Some("slp".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("rest"), MoveDef {
            id: ID::new("rest"),
            name: "Rest".to_string(),
            base_power: 0,
            pp: 5,
            category: MoveCategory::Status,
            move_type: "Psychic".to_string(),
            target: MoveTargetType::Self_,
            flags: MoveFlags { snatch: true, heal: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("retaliate"), MoveDef {
            id: ID::new("retaliate"),
            name: "Retaliate".to_string(),
            base_power: 70,
            accuracy: 100,
            pp: 5,
            category: MoveCategory::Physical,
            move_type: "Normal".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("return"), MoveDef {
            id: ID::new("return"),
            name: "Return".to_string(),
            base_power: 0,
            accuracy: 100,
            pp: 20,
            category: MoveCategory::Physical,
            move_type: "Normal".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("revelationdance"), MoveDef {
            id: ID::new("revelationdance"),
            name: "Revelation Dance".to_string(),
            base_power: 90,
            accuracy: 100,
            pp: 15,
            category: MoveCategory::Special,
            move_type: "Normal".to_string(),
            flags: MoveFlags { protect: true, mirror: true, dance: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("revenge"), MoveDef {
            id: ID::new("revenge"),
            name: "Revenge".to_string(),
            base_power: 60,
            accuracy: 100,
            pp: 10,
            priority: -4,
            category: MoveCategory::Physical,
            move_type: "Fighting".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("reversal"), MoveDef {
            id: ID::new("reversal"),
            name: "Reversal".to_string(),
            base_power: 0,
            accuracy: 100,
            pp: 15,
            category: MoveCategory::Physical,
            move_type: "Fighting".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("revivalblessing"), MoveDef {
            id: ID::new("revivalblessing"),
            name: "Revival Blessing".to_string(),
            base_power: 0,
            pp: 1,
            category: MoveCategory::Status,
            move_type: "Normal".to_string(),
            target: MoveTargetType::Self_,
            flags: MoveFlags { heal: true, ..Default::default() },
            slot_condition: Some("revivalblessing".to_string()),
            ..Default::default()
        });
        map.insert(ID::new("risingvoltage"), MoveDef {
            id: ID::new("risingvoltage"),
            name: "Rising Voltage".to_string(),
            base_power: 70,
            accuracy: 100,
            pp: 20,
            category: MoveCategory::Special,
            move_type: "Electric".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("roar"), MoveDef {
            id: ID::new("roar"),
            name: "Roar".to_string(),
            base_power: 0,
            pp: 20,
            priority: -6,
            category: MoveCategory::Status,
            move_type: "Normal".to_string(),
            flags: MoveFlags { reflectable: true, mirror: true, sound: true, ..Default::default() },
            force_switch: true,
            ..Default::default()
        });
        map.insert(ID::new("roaroftime"), MoveDef {
            id: ID::new("roaroftime"),
            name: "Roar of Time".to_string(),
            base_power: 150,
            accuracy: 90,
            pp: 5,
            category: MoveCategory::Special,
            move_type: "Dragon".to_string(),
            flags: MoveFlags { recharge: true, protect: true, mirror: true, ..Default::default() },
            self_volatile: Some("mustrecharge".to_string()),
            ..Default::default()
        });
        map.insert(ID::new("rockblast"), MoveDef {
            id: ID::new("rockblast"),
            name: "Rock Blast".to_string(),
            base_power: 25,
            accuracy: 90,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Rock".to_string(),
            flags: MoveFlags { protect: true, mirror: true, bullet: true, ..Default::default() },
            multi_hit: Some((2, 5)),
            ..Default::default()
        });
        map.insert(ID::new("rockclimb"), MoveDef {
            id: ID::new("rockclimb"),
            name: "Rock Climb".to_string(),
            base_power: 90,
            accuracy: 85,
            pp: 20,
            category: MoveCategory::Physical,
            move_type: "Normal".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 20,
                volatile_status: Some("confusion".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("rockpolish"), MoveDef {
            id: ID::new("rockpolish"),
            name: "Rock Polish".to_string(),
            base_power: 0,
            pp: 20,
            category: MoveCategory::Status,
            move_type: "Rock".to_string(),
            target: MoveTargetType::Self_,
            flags: MoveFlags { snatch: true, ..Default::default() },
            boosts: Some(vec![("spe".to_string(), 2)]),
            ..Default::default()
        });
        map.insert(ID::new("rockslide"), MoveDef {
            id: ID::new("rockslide"),
            name: "Rock Slide".to_string(),
            base_power: 75,
            accuracy: 90,
            pp: 10,
            category: MoveCategory::Physical,
            move_type: "Rock".to_string(),
            target: MoveTargetType::AllAdjacentFoes,
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 30,
                volatile_status: Some("flinch".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("rocksmash"), MoveDef {
            id: ID::new("rocksmash"),
            name: "Rock Smash".to_string(),
            base_power: 40,
            accuracy: 100,
            pp: 15,
            category: MoveCategory::Physical,
            move_type: "Fighting".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 50,
                boosts: Some(vec![("def".to_string(), -1)]),
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("rockthrow"), MoveDef {
            id: ID::new("rockthrow"),
            name: "Rock Throw".to_string(),
            base_power: 50,
            accuracy: 90,
            pp: 15,
            category: MoveCategory::Physical,
            move_type: "Rock".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("rocktomb"), MoveDef {
            id: ID::new("rocktomb"),
            name: "Rock Tomb".to_string(),
            base_power: 60,
            accuracy: 95,
            pp: 15,
            category: MoveCategory::Physical,
            move_type: "Rock".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 100,
                boosts: Some(vec![("spe".to_string(), -1)]),
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("rockwrecker"), MoveDef {
            id: ID::new("rockwrecker"),
            name: "Rock Wrecker".to_string(),
            base_power: 150,
            accuracy: 90,
            pp: 5,
            category: MoveCategory::Physical,
            move_type: "Rock".to_string(),
            flags: MoveFlags { recharge: true, protect: true, mirror: true, bullet: true, ..Default::default() },
            self_volatile: Some("mustrecharge".to_string()),
            ..Default::default()
        });
        map.insert(ID::new("roleplay"), MoveDef {
            id: ID::new("roleplay"),
            name: "Role Play".to_string(),
            base_power: 0,
            pp: 10,
            category: MoveCategory::Status,
            move_type: "Psychic".to_string(),
            flags: MoveFlags { ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("rollingkick"), MoveDef {
            id: ID::new("rollingkick"),
            name: "Rolling Kick".to_string(),
            base_power: 60,
            accuracy: 85,
            pp: 15,
            category: MoveCategory::Physical,
            move_type: "Fighting".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
            secondaries: vec![SecondaryEffect {
                chance: 30,
                volatile_status: Some("flinch".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        });
        map.insert(ID::new("rollout"), MoveDef {
            id: ID::new("rollout"),
            name: "Rollout".to_string(),
            base_power: 30,
            accuracy: 90,
            pp: 20,
            category: MoveCategory::Physical,
            move_type: "Rock".to_string(),
            flags: MoveFlags { contact: true, protect: true, mirror: true, ..Default::default() },
            self_volatile: Some("rollout".to_string()),
            ..Default::default()
        });
        map.insert(ID::new("roost"), MoveDef {
            id: ID::new("roost"),
            name: "Roost".to_string(),
            base_power: 0,
            pp: 5,
            category: MoveCategory::Status,
            move_type: "Flying".to_string(),
            target: MoveTargetType::Self_,
            flags: MoveFlags { snatch: true, heal: true, ..Default::default() },
            heal: Some(0.5),
            self_volatile: Some("roost".to_string()),
            ..Default::default()
        });
        map.insert(ID::new("rototiller"), MoveDef {
            id: ID::new("rototiller"),
            name: "Rototiller".to_string(),
            base_power: 0,
            pp: 10,
            category: MoveCategory::Status,
            move_type: "Ground".to_string(),
            target: MoveTargetType::All,
            flags: MoveFlags { distance: true, nonsky: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("round"), MoveDef {
            id: ID::new("round"),
            name: "Round".to_string(),
            base_power: 60,
            accuracy: 100,
            pp: 15,
            category: MoveCategory::Special,
            move_type: "Normal".to_string(),
            flags: MoveFlags { protect: true, mirror: true, sound: true, ..Default::default() },
            ..Default::default()
        });
        map.insert(ID::new("ruination"), MoveDef {
            id: ID::new("ruination"),
            name: "Ruination".to_string(),
            base_power: 0,
            accuracy: 90,
            pp: 10,
            category: MoveCategory::Special,
            move_type: "Dark".to_string(),
            flags: MoveFlags { protect: true, mirror: true, ..Default::default() },
            ..Default::default()
        });
    map
});

/// Get move definition by ID
pub fn get_move(id: &ID) -> Option<&'static MoveDef> {
    MOVES.get(id)
}

/// Check if a move is a pivot move
pub fn is_pivot_move(move_id: &ID) -> bool {
    get_move(move_id).map_or(false, |m| m.is_pivot)
}

/// Check if a move is a status move
pub fn is_status_move(move_id: &ID) -> bool {
    get_move(move_id).map_or(false, |m| m.is_status())
}

/// Get move base power
pub fn get_base_power(move_id: &ID) -> u32 {
    get_move(move_id).map_or(0, |m| m.base_power)
}

/// Get move accuracy
pub fn get_accuracy(move_id: &ID) -> u8 {
    get_move(move_id).map_or(100, |m| m.accuracy)
}

/// Get move category
pub fn get_category(move_id: &ID) -> MoveCategory {
    get_move(move_id).map_or(MoveCategory::Physical, |m| m.category)
}

/// Get move type
pub fn get_move_type(move_id: &ID) -> String {
    get_move(move_id).map_or("Normal".to_string(), |m| m.move_type.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_earthquake() {
        let eq = ID::new("earthquake");
        assert_eq!(get_base_power(&eq), 100);
        assert_eq!(get_accuracy(&eq), 100);
        assert!(matches!(get_category(&eq), MoveCategory::Physical));
        assert_eq!(get_move_type(&eq), "Ground");
    }

    #[test]
    fn test_pivot_moves() {
        assert!(is_pivot_move(&ID::new("uturn")));
        assert!(is_pivot_move(&ID::new("voltswitch")));
        assert!(is_pivot_move(&ID::new("flipturn")));
        assert!(!is_pivot_move(&ID::new("earthquake")));
    }

    #[test]
    fn test_status_moves() {
        assert!(is_status_move(&ID::new("thunderwave")));
        assert!(is_status_move(&ID::new("swordsdance")));
        assert!(is_status_move(&ID::new("protect")));
        assert!(!is_status_move(&ID::new("earthquake")));
    }

    #[test]
    fn test_move_properties() {
        let sd = get_move(&ID::new("swordsdance")).unwrap();
        assert_eq!(sd.self_boosts, Some(vec![("atk".to_string(), 2)]));

        let cc = get_move(&ID::new("closecombat")).unwrap();
        assert_eq!(cc.self_drops, Some(vec![("def".to_string(), -1), ("spd".to_string(), -1)]));

        let protect = get_move(&ID::new("protect")).unwrap();
        assert_eq!(protect.priority, 4);
    }

    #[test]
    fn test_recoil_moves() {
        let de = get_move(&ID::new("doubleedge")).unwrap();
        assert!(de.recoil.is_some());
        assert!((de.recoil.unwrap() - 1.0/3.0).abs() < 0.001);
    }

    #[test]
    fn test_multi_hit() {
        let bs = get_move(&ID::new("bulletseed")).unwrap();
        assert_eq!(bs.multi_hit, Some((2, 5)));
    }
}
