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
