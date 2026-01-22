//! Data-driven Format/Ruleset Definitions
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This module contains data-driven definitions for battle formats and rulesets,
//! following the Pokemon Showdown JS architecture.

use crate::dex_data::ID;
use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Game type (singles, doubles, etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// JavaScript equivalent: GameType (sim/global-types.ts)
pub enum GameType {
    #[default]
    Singles,
    Doubles,
    Triples,
    Multi,
    FreeForAll,
}

impl GameType {
    pub fn active_per_half(&self) -> usize {
        match self {
            GameType::Singles => 1,
            GameType::Doubles => 2,
            GameType::Triples => 3,
            GameType::Multi => 2,
            GameType::FreeForAll => 1,
        }
    }
}

/// Format mod (generation)
/// JavaScript equivalent: ModID (sim/dex-formats.ts)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FormatMod {
    Gen1,
    Gen2,
    Gen3,
    Gen4,
    Gen5,
    Gen6,
    Gen7,
    Gen8,
    #[default]
    Gen9,
}

impl FormatMod {
    pub fn number(&self) -> u8 {
        match self {
            FormatMod::Gen1 => 1,
            FormatMod::Gen2 => 2,
            FormatMod::Gen3 => 3,
            FormatMod::Gen4 => 4,
            FormatMod::Gen5 => 5,
            FormatMod::Gen6 => 6,
            FormatMod::Gen7 => 7,
            FormatMod::Gen8 => 8,
            FormatMod::Gen9 => 9,
        }
    }

    pub fn from_number(n: u8) -> Option<Self> {
        match n {
            1 => Some(FormatMod::Gen1),
            2 => Some(FormatMod::Gen2),
            3 => Some(FormatMod::Gen3),
            4 => Some(FormatMod::Gen4),
            5 => Some(FormatMod::Gen5),
            6 => Some(FormatMod::Gen6),
            7 => Some(FormatMod::Gen7),
            8 => Some(FormatMod::Gen8),
            9 => Some(FormatMod::Gen9),
            _ => None,
        }
    }
}

/// Rule definition
#[derive(Debug, Clone)]
/// JavaScript equivalent: RuleData (sim/dex-formats.ts)
/// 5 fields in JavaScript
pub struct RuleDef {
    /// Rule name
    /// JavaScript: name: string
    pub name: &'static str,
    /// Rule description
    /// JavaScript: desc: string
    pub desc: &'static str,
    /// Is this a ban rule?
    /// JavaScript: isBan?: boolean
    pub is_ban: bool,
    /// Bans specific things (pokemon, moves, abilities, items)
    /// JavaScript: bans?: string[]
    pub bans: &'static [&'static str],
    /// Unbans specific things
    /// JavaScript: unbans?: string[]
    pub unbans: &'static [&'static str],
}

/// Format definition
#[derive(Debug, Clone)]
/// JavaScript equivalent: FormatData (sim/dex-formats.ts)
/// ~20 fields in JavaScript
pub struct FormatDef {
    /// Format ID (e.g., "gen9ou")
    /// JavaScript: id: ID
    pub id: &'static str,
    /// Format name (e.g., "Gen 9 OU")
    /// JavaScript: name: string
    pub name: &'static str,
    /// Game type
    /// JavaScript: gameType: 'singles' | 'doubles' | 'triples' | 'rotation' | 'multi' | 'freeforall'
    pub game_type: GameType,
    /// Generation mod
    /// JavaScript: mod: string
    pub mod_: FormatMod,
    /// Team size
    /// JavaScript: teamSize?: { battle?: number, validate?: [number, number] }
    /// TODO: Rust uses simple usize, JavaScript uses complex object
    pub team_size: usize,
    /// Minimum team size
    /// JavaScript: (part of teamSize.validate)
    pub min_team_size: usize,
    /// Level for Pokemon (usually 100)
    /// JavaScript: maxLevel?: number
    pub max_level: u8,
    /// Default level (usually 100)
    /// JavaScript: defaultLevel?: number
    pub default_level: u8,
    /// Rulesets this format inherits from
    /// JavaScript: ruleset?: string[]
    pub rulesets: &'static [&'static str],
    /// Banned Pokemon, moves, abilities, items
    /// JavaScript: banlist?: string[]
    pub bans: &'static [&'static str],
    /// Unbanned items (exceptions to ruleset bans)
    /// JavaScript: unbanlist?: string[]
    pub unbans: &'static [&'static str],
    /// Is this format rated?
    /// JavaScript: rated?: boolean
    pub rated: bool,
    /// Minimum source generation for Pokemon (e.g., Gen 8+ only)
    /// JavaScript: minSourceGen?: number
    pub min_source_gen: Option<i32>,
}

/// Standard ruleset definitions
pub static RULESETS: Lazy<HashMap<ID, RuleDef>> = Lazy::new(|| {
    let mut rulesets = HashMap::new();

    rulesets.insert(
        ID::new("Standard"),
        RuleDef {
            name: "Standard",
            desc: "Standard rules for competitive play",
            is_ban: false,
            bans: &[],
            unbans: &[],
        },
    );

    rulesets.insert(
        ID::new("Team Preview"),
        RuleDef {
            name: "Team Preview",
            desc: "Pokemon are revealed before battle begins",
            is_ban: false,
            bans: &[],
            unbans: &[],
        },
    );

    rulesets.insert(
        ID::new("Sleep Clause Mod"),
        RuleDef {
            name: "Sleep Clause Mod",
            desc: "Only one Pokemon on a team can be asleep at a time",
            is_ban: false,
            bans: &[],
            unbans: &[],
        },
    );

    rulesets.insert(
        ID::new("Species Clause"),
        RuleDef {
            name: "Species Clause",
            desc: "No two Pokemon on a team can have the same National Dex number",
            is_ban: false,
            bans: &[],
            unbans: &[],
        },
    );

    rulesets.insert(
        ID::new("Nickname Clause"),
        RuleDef {
            name: "Nickname Clause",
            desc: "No two Pokemon on a team can have the same nickname",
            is_ban: false,
            bans: &[],
            unbans: &[],
        },
    );

    rulesets.insert(
        ID::new("OHKO Clause"),
        RuleDef {
            name: "OHKO Clause",
            desc: "OHKO moves are banned",
            is_ban: true,
            bans: &["Fissure", "Guillotine", "Horn Drill", "Sheer Cold"],
            unbans: &[],
        },
    );

    rulesets.insert(
        ID::new("Evasion Clause"),
        RuleDef {
            name: "Evasion Clause",
            desc: "Evasion-boosting moves and abilities are banned",
            is_ban: true,
            bans: &["Double Team", "Minimize"],
            unbans: &[],
        },
    );

    rulesets.insert(
        ID::new("Moody Clause"),
        RuleDef {
            name: "Moody Clause",
            desc: "Moody ability is banned",
            is_ban: true,
            bans: &["Moody"],
            unbans: &[],
        },
    );

    rulesets.insert(
        ID::new("Endless Battle Clause"),
        RuleDef {
            name: "Endless Battle Clause",
            desc: "Combinations that can cause an endless battle are banned",
            is_ban: false,
            bans: &[],
            unbans: &[],
        },
    );

    rulesets.insert(
        ID::new("HP Percentage Mod"),
        RuleDef {
            name: "HP Percentage Mod",
            desc: "HP is shown as a percentage rather than raw HP",
            is_ban: false,
            bans: &[],
            unbans: &[],
        },
    );

    rulesets.insert(
        ID::new("Cancel Mod"),
        RuleDef {
            name: "Cancel Mod",
            desc: "Players can cancel their moves before both have chosen",
            is_ban: false,
            bans: &[],
            unbans: &[],
        },
    );

    rulesets.insert(
        ID::new("Dynamax Clause"),
        RuleDef {
            name: "Dynamax Clause",
            desc: "Dynamaxing is banned",
            is_ban: true,
            bans: &[],
            unbans: &[],
        },
    );

    rulesets.insert(
        ID::new("Terastal Clause"),
        RuleDef {
            name: "Terastal Clause",
            desc: "Terastallization is banned",
            is_ban: true,
            bans: &[],
            unbans: &[],
        },
    );

    rulesets
});

/// Standard format definitions
pub static FORMATS: Lazy<HashMap<ID, FormatDef>> = Lazy::new(|| {
    let mut formats = HashMap::new();

    // Gen 9 formats
    formats.insert(
        ID::new("gen9ou"),
        FormatDef {
            id: "gen9ou",
            name: "Gen 9 OU",
            game_type: GameType::Singles,
            mod_: FormatMod::Gen9,
            team_size: 6,
            min_team_size: 1,
            max_level: 100,
            default_level: 100,
            rulesets: &[
                "Standard",
                "Team Preview",
                "Sleep Clause Mod",
                "Species Clause",
                "Nickname Clause",
                "OHKO Clause",
                "Evasion Clause",
                "Moody Clause",
                "Endless Battle Clause",
                "HP Percentage Mod",
                "Cancel Mod",
            ],
            bans: &[
                // Uber Pokemon
                "Koraidon",
                "Miraidon",
                "Mewtwo",
                "Ho-Oh",
                "Lugia",
                "Kyogre",
                "Groudon",
                "Rayquaza",
                "Dialga",
                "Palkia",
                "Giratina",
                "Arceus",
                "Reshiram",
                "Zekrom",
                "Kyurem-White",
                "Kyurem-Black",
                "Xerneas",
                "Yveltal",
                "Zygarde-Complete",
                "Solgaleo",
                "Lunala",
                "Necrozma-Dusk-Mane",
                "Necrozma-Dawn-Wings",
                "Zacian",
                "Zamazenta",
                "Eternatus",
                "Calyrex-Ice",
                "Calyrex-Shadow",
                // Banned abilities
                "Arena Trap",
                "Shadow Tag",
                "Moody",
                // Banned items
                "King's Rock",
                "Razor Fang",
                // Banned moves
                "Baton Pass",
                "Last Respects",
                "Shed Tail",
            ],
            unbans: &[],
            rated: true,
            min_source_gen: None,
        },
    );

    formats.insert(
        ID::new("gen9ubers"),
        FormatDef {
            id: "gen9ubers",
            name: "Gen 9 Ubers",
            game_type: GameType::Singles,
            mod_: FormatMod::Gen9,
            team_size: 6,
            min_team_size: 1,
            max_level: 100,
            default_level: 100,
            rulesets: &[
                "Standard",
                "Team Preview",
                "Sleep Clause Mod",
                "Species Clause",
                "Nickname Clause",
                "OHKO Clause",
                "Evasion Clause",
                "Endless Battle Clause",
                "HP Percentage Mod",
                "Cancel Mod",
            ],
            bans: &["Moody", "Baton Pass"],
            unbans: &[],
            rated: true,
            min_source_gen: None,
        },
    );

    formats.insert(
        ID::new("gen9uu"),
        FormatDef {
            id: "gen9uu",
            name: "Gen 9 UU",
            game_type: GameType::Singles,
            mod_: FormatMod::Gen9,
            team_size: 6,
            min_team_size: 1,
            max_level: 100,
            default_level: 100,
            rulesets: &[
                "Standard",
                "Team Preview",
                "Sleep Clause Mod",
                "Species Clause",
                "Nickname Clause",
                "OHKO Clause",
                "Evasion Clause",
                "Moody Clause",
                "Endless Battle Clause",
                "HP Percentage Mod",
                "Cancel Mod",
            ],
            bans: &[
                // Everything in OU
                "OU", "UUBL", // Specific bans
                "Drizzle", "Drought",
            ],
            unbans: &[],
            rated: true,
            min_source_gen: None,
        },
    );

    formats.insert(
        ID::new("gen9doublesou"),
        FormatDef {
            id: "gen9doublesou",
            name: "Gen 9 Doubles OU",
            game_type: GameType::Doubles,
            mod_: FormatMod::Gen9,
            team_size: 6,
            min_team_size: 1,
            max_level: 100,
            default_level: 100,
            rulesets: &[
                "Standard",
                "Team Preview",
                "Species Clause",
                "Nickname Clause",
                "OHKO Clause",
                "Evasion Clause",
                "Moody Clause",
                "Endless Battle Clause",
                "HP Percentage Mod",
                "Cancel Mod",
            ],
            bans: &[
                "Koraidon",
                "Miraidon",
                "Mewtwo",
                "Kyogre",
                "Groudon",
                "Rayquaza",
                "Arceus",
                "Calyrex-Shadow",
            ],
            unbans: &[],
            rated: true,
            min_source_gen: None,
        },
    );

    formats.insert(
        ID::new("gen9vgc2024"),
        FormatDef {
            id: "gen9vgc2024",
            name: "Gen 9 VGC 2024",
            game_type: GameType::Doubles,
            mod_: FormatMod::Gen9,
            team_size: 6,
            min_team_size: 4,
            max_level: 50,
            default_level: 50,
            rulesets: &[
                "Standard",
                "Team Preview",
                "Species Clause",
                "Nickname Clause",
                "HP Percentage Mod",
            ],
            bans: &[
                "Mewtwo", "Mew", "Lugia", "Ho-Oh", "Celebi", "Kyogre", "Groudon", "Rayquaza",
                "Jirachi", "Deoxys",
            ],
            unbans: &[],
            rated: true,
            min_source_gen: None,
        },
    );

    formats.insert(
        ID::new("gen9randombattle"),
        FormatDef {
            id: "gen9randombattle",
            name: "Gen 9 Random Battle",
            game_type: GameType::Singles,
            mod_: FormatMod::Gen9,
            team_size: 6,
            min_team_size: 6,
            max_level: 100,
            default_level: 100,
            // JavaScript: ['PotD', 'Obtainable', 'Species Clause', 'HP Percentage Mod', 'Cancel Mod', 'Sleep Clause Mod', 'Illusion Level Mod']
            rulesets: &["HP Percentage Mod", "Cancel Mod", "Sleep Clause Mod"],
            bans: &[],
            unbans: &[],
            rated: true,
            min_source_gen: None,
        },
    );

    // Gen 8 formats
    formats.insert(
        ID::new("gen8ou"),
        FormatDef {
            id: "gen8ou",
            name: "Gen 8 OU",
            game_type: GameType::Singles,
            mod_: FormatMod::Gen8,
            team_size: 6,
            min_team_size: 1,
            max_level: 100,
            default_level: 100,
            rulesets: &[
                "Standard",
                "Team Preview",
                "Sleep Clause Mod",
                "Species Clause",
                "Nickname Clause",
                "OHKO Clause",
                "Evasion Clause",
                "Moody Clause",
                "Dynamax Clause",
                "Endless Battle Clause",
                "HP Percentage Mod",
                "Cancel Mod",
            ],
            bans: &[
                "Uber",
                "Arena Trap",
                "Power Construct",
                "Shadow Tag",
                "Baton Pass",
            ],
            unbans: &[],
            rated: true,
            min_source_gen: None,
        },
    );

    // Gen 7 formats
    formats.insert(
        ID::new("gen7ou"),
        FormatDef {
            id: "gen7ou",
            name: "Gen 7 OU",
            game_type: GameType::Singles,
            mod_: FormatMod::Gen7,
            team_size: 6,
            min_team_size: 1,
            max_level: 100,
            default_level: 100,
            rulesets: &[
                "Standard",
                "Team Preview",
                "Sleep Clause Mod",
                "Species Clause",
                "Nickname Clause",
                "OHKO Clause",
                "Evasion Clause",
                "Moody Clause",
                "Endless Battle Clause",
                "HP Percentage Mod",
                "Cancel Mod",
            ],
            bans: &[
                "Uber",
                "Arena Trap",
                "Power Construct",
                "Shadow Tag",
                "Baton Pass",
            ],
            unbans: &[],
            rated: true,
            min_source_gen: None,
        },
    );

    // Gen 1 formats
    formats.insert(
        ID::new("gen1ou"),
        FormatDef {
            id: "gen1ou",
            name: "Gen 1 OU",
            game_type: GameType::Singles,
            mod_: FormatMod::Gen1,
            team_size: 6,
            min_team_size: 1,
            max_level: 100,
            default_level: 100,
            rulesets: &[
                "Standard",
                "Sleep Clause Mod",
                "Species Clause",
                "Nickname Clause",
                "OHKO Clause",
                "Evasion Clause",
                "Endless Battle Clause",
            ],
            bans: &["Uber", "Baton Pass"],
            unbans: &[],
            rated: true,
            min_source_gen: None,
        },
    );

    formats
});

/// Get a format definition by ID
pub fn get_format(id: &ID) -> Option<&'static FormatDef> {
    FORMATS.get(id)
}

/// Get a format by name (case-insensitive, strips spaces)
pub fn get_format_by_name(name: &str) -> Option<&'static FormatDef> {
    get_format(&ID::new(name))
}

/// Check if a format exists
pub fn format_exists(name: &str) -> bool {
    get_format(&ID::new(name)).is_some()
}

/// Get a ruleset definition by ID
pub fn get_ruleset(id: &ID) -> Option<&'static RuleDef> {
    RULESETS.get(id)
}

/// Get all formats for a generation
pub fn get_formats_by_gen(gen: FormatMod) -> Vec<&'static FormatDef> {
    FORMATS.values().filter(|f| f.mod_ == gen).collect()
}

/// Get all singles formats
pub fn get_singles_formats() -> Vec<&'static FormatDef> {
    FORMATS
        .values()
        .filter(|f| f.game_type == GameType::Singles)
        .collect()
}

/// Get all doubles formats
pub fn get_doubles_formats() -> Vec<&'static FormatDef> {
    FORMATS
        .values()
        .filter(|f| f.game_type == GameType::Doubles)
        .collect()
}

/// Check if something is banned in a format
pub fn is_banned_in_format(format_id: &ID, thing: &str) -> bool {
    if let Some(format) = get_format(format_id) {
        let thing_lower = thing.to_lowercase();
        format.bans.iter().any(|b| b.to_lowercase() == thing_lower)
    } else {
        false
    }
}

// =========================================================================
// RULETABLE - Equivalent to dex-formats.ts RuleTable class
// =========================================================================

/// Complex ban entry: (rule, source, limit, bans)
/// JavaScript equivalent: ComplexBan (sim/dex-formats.ts)
pub type ComplexBan = (String, String, i32, Vec<String>);

/// Timer settings for battle
#[derive(Debug, Clone, Default)]
/// JavaScript equivalent: GameTimerSettings (sim/dex-formats.ts)
/// 9 fields in JavaScript
pub struct GameTimerSettings {
    /// Enable disconnect timer
    /// JavaScript: dcTimer: boolean
    pub dc_timer: bool,
    /// Enable disconnect timer bank
    /// JavaScript: dcTimerBank: boolean
    pub dc_timer_bank: bool,
    /// Starting time in seconds
    /// JavaScript: starting: number
    pub starting: i32,
    /// Grace period in seconds
    /// JavaScript: grace: number
    pub grace: i32,
    /// Time added per turn in seconds
    /// JavaScript: addPerTurn: number
    pub add_per_turn: i32,
    /// Maximum time per turn in seconds
    /// JavaScript: maxPerTurn: number
    pub max_per_turn: i32,
    /// Maximum time for first turn in seconds
    /// JavaScript: maxFirstTurn: number
    pub max_first_turn: i32,
    /// Auto-choose move on timeout
    /// JavaScript: timeoutAutoChoose: boolean
    pub timeout_auto_choose: bool,
    /// Accelerate timer
    /// JavaScript: accelerate: boolean
    pub accelerate: bool,
}

/// RuleTable - tracks rules in effect for a format
/// The key can be:
/// - '[ruleid]' the ID of a rule in effect
/// - '-[thing]' or '-[category]:[thing]' ban a thing
/// - '+[thing]' or '+[category]:[thing]' allow a thing (override a ban)
#[derive(Debug, Clone, Default)]
/// JavaScript equivalent: RuleTable (sim/dex.ts)
/// ~20 fields in JavaScript
pub struct RuleTable {
    /// Map of rule ID to source format name
    /// JavaScript: has(ruleID: string): boolean
    /// TODO: JavaScript uses Map methods, Rust uses HashMap
    rules: HashMap<String, String>,
    /// Complex bans (rule, source, limit, bans)
    /// JavaScript: complexBans: [string, string, number, string[]][]
    pub complex_bans: Vec<ComplexBan>,
    /// Complex team bans
    /// JavaScript: complexTeamBans: [string, string, number, string[]][]
    pub complex_team_bans: Vec<ComplexBan>,
    /// Tag rules (pokemontag rules)
    /// JavaScript: tagRules: string[]
    pub tag_rules: Vec<String>,
    /// Value rules (rules with = values)
    /// JavaScript: valueRules: Map<string, string>
    pub value_rules: HashMap<String, String>,
    /// Timer settings
    /// JavaScript: timer?: GameTimerSettings
    pub timer: Option<GameTimerSettings>,

    // Resolved numeric properties
    /// Minimum team size
    /// JavaScript: minTeamSize: number
    pub min_team_size: usize,
    /// Maximum team size
    /// JavaScript: maxTeamSize: number
    pub max_team_size: usize,
    /// Picked team size
    /// JavaScript: pickedTeamSize?: number
    pub picked_team_size: Option<usize>,
    /// Maximum total level
    /// JavaScript: maxTotalLevel?: number
    pub max_total_level: Option<i32>,
    /// Maximum move count
    /// JavaScript: maxMoveCount: number
    pub max_move_count: usize,
    /// Minimum source generation
    /// JavaScript: minSourceGen: number
    pub min_source_gen: u8,
    /// Minimum level
    /// JavaScript: minLevel: number
    pub min_level: u8,
    /// Maximum level
    /// JavaScript: maxLevel: number
    pub max_level: u8,
    /// Default level
    /// JavaScript: defaultLevel: number
    pub default_level: u8,
    /// Adjust level
    /// JavaScript: adjustLevel?: number
    pub adjust_level: Option<u8>,
    /// Adjust level down
    /// JavaScript: adjustLevelDown?: number
    pub adjust_level_down: Option<u8>,
    /// EV limit
    /// JavaScript: evLimit?: number
    pub ev_limit: Option<i32>,
}

impl RuleTable {
    /// Create a new empty RuleTable
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
            complex_bans: Vec::new(),
            complex_team_bans: Vec::new(),
            tag_rules: Vec::new(),
            value_rules: HashMap::new(),
            timer: None,
            min_team_size: 1,
            max_team_size: 6,
            picked_team_size: None,
            max_total_level: None,
            max_move_count: 4,
            min_source_gen: 1,
            min_level: 1,
            max_level: 100,
            default_level: 100,
            adjust_level: None,
            adjust_level_down: None,
            ev_limit: None,
        }
    }

    /// Check if a rule exists
    pub fn has(&self, key: &str) -> bool {
        self.rules.contains_key(key)
    }

    /// Get a rule's source
    pub fn get(&self, key: &str) -> Option<&String> {
        self.rules.get(key)
    }

    /// Set a rule
    pub fn set(&mut self, key: &str, source: &str) {
        self.rules.insert(key.to_string(), source.to_string());
    }

    /// Delete a rule
    pub fn delete(&mut self, key: &str) {
        self.rules.remove(key);
    }

    /// Get all rule keys
    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.rules.keys()
    }

    /// Check if something is banned
    /// Equivalent to isBanned()
    // TypeScript source:
    //
    //
    // 	isBanned(thing: string) {
    // 		if (this.has(`+${thing}`)) return false;
    // 		return this.has(`-${thing}`);
    // 	}
    //
    pub fn is_banned(&self, thing: &str) -> bool {
        if self.has(&format!("+{}", thing)) {
            return false;
        }
        self.has(&format!("-{}", thing))
    }

    /// Check if a species is banned
    /// Equivalent to isBannedSpecies()
    //
    // 	isBannedSpecies(species: Species) {
    // 		if (this.has(`+pokemon:${species.id}`)) return false;
    // 		if (this.has(`-pokemon:${species.id}`)) return true;
    // 		if (this.has(`+basepokemon:${toID(species.baseSpecies)}`)) return false;
    // 		if (this.has(`-basepokemon:${toID(species.baseSpecies)}`)) return true;
    // 		for (const tagid in Tags) {
    // 			const tag = Tags[tagid as ID];
    // 			if (this.has(`-pokemontag:${tagid}`)) {
    // 				if ((tag.speciesFilter || tag.genericFilter)!(species)) return true;
    // 			}
    // 		}
    // 		for (const tagid in Tags) {
    // 			const tag = Tags[tagid as ID];
    // 			if (this.has(`+pokemontag:${tagid}`)) {
    // 				if ((tag.speciesFilter || tag.genericFilter)!(species)) return false;
    // 			}
    // 		}
    // 		return this.has(`-pokemontag:allpokemon`);
    // 	}
    //
    pub fn is_banned_species(&self, species_id: &str, base_species_id: &str) -> bool {
        if self.has(&format!("+pokemon:{}", species_id)) {
            return false;
        }
        if self.has(&format!("-pokemon:{}", species_id)) {
            return true;
        }
        if self.has(&format!("+basepokemon:{}", base_species_id)) {
            return false;
        }
        if self.has(&format!("-basepokemon:{}", base_species_id)) {
            return true;
        }
        self.has("-pokemontag:allpokemon")
    }

    /// Check if something is restricted
    /// Equivalent to isRestricted()
    //
    // 	isRestricted(thing: string) {
    // 		if (this.has(`+${thing}`)) return false;
    // 		return this.has(`*${thing}`);
    // 	}
    //
    pub fn is_restricted(&self, thing: &str) -> bool {
        if self.has(&format!("+{}", thing)) {
            return false;
        }
        self.has(&format!("*{}", thing))
    }

    /// Check if a species is restricted
    /// Equivalent to isRestrictedSpecies()
    //
    // 	isRestrictedSpecies(species: Species) {
    // 		if (this.has(`+pokemon:${species.id}`)) return false;
    // 		if (this.has(`*pokemon:${species.id}`)) return true;
    // 		if (this.has(`+basepokemon:${toID(species.baseSpecies)}`)) return false;
    // 		if (this.has(`*basepokemon:${toID(species.baseSpecies)}`)) return true;
    // 		for (const tagid in Tags) {
    // 			const tag = Tags[tagid as ID];
    // 			if (this.has(`*pokemontag:${tagid}`)) {
    // 				if ((tag.speciesFilter || tag.genericFilter)!(species)) return true;
    // 			}
    // 		}
    // 		for (const tagid in Tags) {
    // 			const tag = Tags[tagid as ID];
    // 			if (this.has(`+pokemontag:${tagid}`)) {
    // 				if ((tag.speciesFilter || tag.genericFilter)!(species)) return false;
    // 			}
    // 		}
    // 		return this.has(`*pokemontag:allpokemon`);
    // 	}
    //
    pub fn is_restricted_species(&self, species_id: &str, base_species_id: &str) -> bool {
        if self.has(&format!("+pokemon:{}", species_id)) {
            return false;
        }
        if self.has(&format!("*pokemon:{}", species_id)) {
            return true;
        }
        if self.has(&format!("+basepokemon:{}", base_species_id)) {
            return false;
        }
        if self.has(&format!("*basepokemon:{}", base_species_id)) {
            return true;
        }
        self.has("*pokemontag:allpokemon")
    }

    /// Get tag rules
    /// Equivalent to getTagRules()
    //
    // 	getTagRules() {
    // 		const tagRules = [];
    // 		for (const ruleid of this.keys()) {
    // 			if (/^[+*-]pokemontag:/.test(ruleid)) {
    // 				const banid = ruleid.slice(12);
    // 				if (
    // 					banid === 'allpokemon' || banid === 'allitems' || banid === 'allmoves' ||
    // 					banid === 'allabilities' || banid === 'allnatures'
    // 				) {
    // 					// hardcoded and not a part of the ban rule system
    // 				} else {
    // 					tagRules.push(ruleid);
    // 				}
    // 			} else if ('+*-'.includes(ruleid.charAt(0)) && ruleid.slice(1) === 'nonexistent') {
    // 				tagRules.push(ruleid.charAt(0) + 'pokemontag:nonexistent');
    // 			}
    // 		}
    // 		this.tagRules = tagRules.reverse();
    // 		return this.tagRules;
    // 	}
    //
    pub fn get_tag_rules(&mut self) -> &Vec<String> {
        let mut tag_rules = Vec::new();
        for ruleid in self.rules.keys() {
            if ruleid.starts_with("+pokemontag:")
                || ruleid.starts_with("*pokemontag:")
                || ruleid.starts_with("-pokemontag:")
            {
                let banid = &ruleid[12..];
                if banid != "allpokemon"
                    && banid != "allitems"
                    && banid != "allmoves"
                    && banid != "allabilities"
                    && banid != "allnatures"
                {
                    tag_rules.push(ruleid.clone());
                }
            } else if ruleid.len() > 1
                && "+*-".contains(&ruleid[..1])
                && &ruleid[1..] == "nonexistent"
            {
                tag_rules.push(format!("{}pokemontag:nonexistent", &ruleid[..1]));
            }
        }
        tag_rules.reverse();
        self.tag_rules = tag_rules;
        &self.tag_rules
    }

    /// Check a thing for bans
    /// Returns: empty string = whitelisted, Some(reason) = banned, None = neither
    /// Equivalent to check()
    // TypeScript source:
    // /**
    // 	 * - non-empty string: banned, string is the reason
    // 	 * - '': whitelisted
    // 	 * - null: neither whitelisted nor banned
    // 	 */
    // 	check(thing: string, setHas: { [id: string]: true } | null = null) {
    // 		if (this.has(`+${thing}`)) return '';
    // 		if (setHas) setHas[thing] = true;
    // 		return this.getReason(`-${thing}`);
    // 	}
    //
    pub fn check(&self, thing: &str) -> Option<String> {
        if self.has(&format!("+{}", thing)) {
            return Some(String::new()); // whitelisted
        }
        self.get_reason(&format!("-{}", thing))
    }

    /// Get reason for a rule
    /// Equivalent to getReason()
    //
    // 	getReason(key: string): string | null {
    // 		const source = this.get(key);
    // 		if (source === undefined) return null;
    // 		if (key === '-nonexistent' || key.startsWith('obtainable')) {
    // 			return 'not obtainable';
    // 		}
    // 		return source ? `banned by ${source}` : `banned`;
    // 	}
    //
    pub fn get_reason(&self, key: &str) -> Option<String> {
        match self.rules.get(key) {
            None => None,
            Some(source) => {
                if key == "-nonexistent" || key.starts_with("obtainable") {
                    Some("not obtainable".to_string())
                } else if source.is_empty() {
                    Some("banned".to_string())
                } else {
                    Some(format!("banned by {}", source))
                }
            }
        }
    }

    /// Get blame string for a rule
    /// Equivalent to blame()
    //
    // 	blame(key: string): string {
    // 		const source = this.get(key);
    // 		return source ? ` from ${source}` : ``;
    // 	}
    //
    pub fn blame(&self, key: &str) -> String {
        match self.rules.get(key) {
            Some(source) if !source.is_empty() => format!(" from {}", source),
            _ => String::new(),
        }
    }

    /// Get index of a complex ban
    /// Equivalent to getComplexBanIndex()
    //
    // 	getComplexBanIndex(complexBans: ComplexBan[], rule: string): number {
    // 		const ruleId = toID(rule);
    // 		let complexBanIndex = -1;
    // 		for (let i = 0; i < complexBans.length; i++) {
    // 			if (toID(complexBans[i][0]) === ruleId) {
    // 				complexBanIndex = i;
    // 				break;
    // 			}
    // 		}
    // 		return complexBanIndex;
    // 	}
    //
    pub fn get_complex_ban_index(&self, complex_bans: &[ComplexBan], rule: &str) -> Option<usize> {
        let rule_id = ID::new(rule);
        complex_bans
            .iter()
            .position(|(r, _, _, _)| ID::new(r) == rule_id)
    }

    /// Add a complex ban
    /// Equivalent to addComplexBan()
    //
    // 	addComplexBan(rule: string, source: string, limit: number, bans: string[]) {
    // 		const complexBanIndex = this.getComplexBanIndex(this.complexBans, rule);
    // 		if (complexBanIndex !== -1) {
    // 			if (this.complexBans[complexBanIndex][2] === Infinity) return;
    // 			this.complexBans[complexBanIndex] = [rule, source, limit, bans];
    // 		} else {
    // 			this.complexBans.push([rule, source, limit, bans]);
    // 		}
    // 	}
    //
    pub fn add_complex_ban(&mut self, rule: &str, source: &str, limit: i32, bans: Vec<String>) {
        if let Some(idx) = self.get_complex_ban_index(&self.complex_bans, rule) {
            if self.complex_bans[idx].2 == i32::MAX {
                return; // Infinity
            }
            self.complex_bans[idx] = (rule.to_string(), source.to_string(), limit, bans);
        } else {
            self.complex_bans
                .push((rule.to_string(), source.to_string(), limit, bans));
        }
    }

    /// Add a complex team ban
    /// Equivalent to addComplexTeamBan()
    //
    // 	addComplexTeamBan(rule: string, source: string, limit: number, bans: string[]) {
    // 		const complexBanTeamIndex = this.getComplexBanIndex(this.complexTeamBans, rule);
    // 		if (complexBanTeamIndex !== -1) {
    // 			if (this.complexTeamBans[complexBanTeamIndex][2] === Infinity) return;
    // 			this.complexTeamBans[complexBanTeamIndex] = [rule, source, limit, bans];
    // 		} else {
    // 			this.complexTeamBans.push([rule, source, limit, bans]);
    // 		}
    // 	}
    //
    pub fn add_complex_team_ban(
        &mut self,
        rule: &str,
        source: &str,
        limit: i32,
        bans: Vec<String>,
    ) {
        if let Some(idx) = self.get_complex_ban_index(&self.complex_team_bans, rule) {
            if self.complex_team_bans[idx].2 == i32::MAX {
                return;
            }
            self.complex_team_bans[idx] = (rule.to_string(), source.to_string(), limit, bans);
        } else {
            self.complex_team_bans
                .push((rule.to_string(), source.to_string(), limit, bans));
        }
    }

    /// Resolve numeric properties
    /// Equivalent to resolveNumbers()
    // TypeScript source:
    // /** After a RuleTable has been filled out, resolve its hardcoded numeric properties */
    // 	resolveNumbers(format: Format, dex: ModdedDex) {
    // 		const gameTypeMinTeamSize = ['triples', 'rotation'].includes(format.gameType as 'triples') ? 3 :
    // 			format.gameType === 'doubles' ? 2 :
    // 			1;
    //
    // 		// NOTE: These numbers are pre-calculated here because they're hardcoded
    // 		// into the team validator and battle engine, and can affect validation
    // 		// in complicated ways.
    //
    // 		// If you're making your own rule, it nearly definitely does not not
    // 		// belong here: `onValidateRule`, `onValidateSet`, and `onValidateTeam`
    // 		// should be enough for a validator rule, and the battle event system
    // 		// should be enough for a battle rule.
    //
    // 		this.minTeamSize = Number(this.valueRules.get('minteamsize')) || 0;
    // 		this.maxTeamSize = Number(this.valueRules.get('maxteamsize')) || 6;
    // 		this.pickedTeamSize = Number(this.valueRules.get('pickedteamsize')) || null;
    // 		this.maxTotalLevel = Number(this.valueRules.get('maxtotallevel')) || null;
    // 		this.maxMoveCount = Number(this.valueRules.get('maxmovecount')) || 4;
    // 		this.minSourceGen = Number(this.valueRules.get('minsourcegen'));
    // 		this.minLevel = Number(this.valueRules.get('minlevel')) || 1;
    // 		this.maxLevel = Number(this.valueRules.get('maxlevel')) || 100;
    // 		this.defaultLevel = Number(this.valueRules.get('defaultlevel')) || 0;
    // 		this.adjustLevel = Number(this.valueRules.get('adjustlevel')) || null;
    // 		this.adjustLevelDown = Number(this.valueRules.get('adjustleveldown')) || null;
    // 		this.evLimit = Number(this.valueRules.get('evlimit'));
    // 		if (isNaN(this.evLimit)) this.evLimit = null;
    // 		if (!this.minSourceGen) {
    // 			if (dex.gen >= 9 && this.has('obtainable') && !this.has('natdexmod')) {
    // 				this.minSourceGen = dex.gen;
    // 			} else {
    // 				this.minSourceGen = 1;
    // 			}
    // 		}
    //
    // 		const timer: Partial<GameTimerSettings> = {};
    // 		if (this.valueRules.has('timerstarting')) {
    // 			timer.starting = Number(this.valueRules.get('timerstarting'));
    // 		}
    // 		if (this.has('dctimer')) {
    // 			timer.dcTimer = true;
    // 		}
    // 		if (this.has('dctimerbank')) {
    // 			timer.dcTimer = true;
    // 		}
    // 		if (this.valueRules.has('timergrace')) {
    // 			timer.grace = Number(this.valueRules.get('timergrace'));
    // 		}
    // 		if (this.valueRules.has('timeraddperturn')) {
    // 			timer.addPerTurn = Number(this.valueRules.get('timeraddperturn'));
    // 		}
    // 		if (this.valueRules.has('timermaxperturn')) {
    // 			timer.maxPerTurn = Number(this.valueRules.get('timermaxperturn'));
    // 		}
    // 		if (this.valueRules.has('timermaxfirstturn')) {
    // 			timer.maxFirstTurn = Number(this.valueRules.get('timermaxfirstturn'));
    // 		}
    // 		if (this.has('timeoutautochoose')) {
    // 			timer.timeoutAutoChoose = true;
    // 		}
    // 		if (this.has('timeraccelerate')) {
    // 			timer.accelerate = true;
    // 		}
    // 		if (Object.keys(timer).length) this.timer = [timer, format.name];
    //
    // 		if (this.valueRules.get('pickedteamsize') === 'Auto') {
    // 			this.pickedTeamSize = (
    // 				['doubles', 'rotation'].includes(format.gameType) ? 4 :
    // 				format.gameType === 'triples' ? 6 :
    // 				3
    // 			);
    // 		}
    // 		if (this.valueRules.get('evlimit') === 'Auto') {
    // 			this.evLimit = dex.gen > 2 ? 510 : null;
    // 			if (format.mod === 'gen7letsgo') {
    // 				this.evLimit = this.has('lgpenormalrules') ? 0 : null;
    // 			}
    // 			// Gen 6 hackmons also has a limit, which is currently implemented
    // 			// at the appropriate format.
    // 		}
    //
    // 		// sanity checks; these _could_ be inside `onValidateRule` but this way
    // 		// involves less string conversion.
    //
    // 		// engine hard limits
    // 		if (this.maxTeamSize > 24) {
    // 			throw new Error(`Max team size ${this.maxTeamSize}${this.blame('maxteamsize')} is unsupported (we only support up to 24).`);
    // 		}
    // 		if (this.maxLevel > 99999) {
    // 			throw new Error(`Max level ${this.maxLevel}${this.blame('maxlevel')} is unsupported (we only support up to 99999)`);
    // 		}
    // 		if (this.maxMoveCount > 24) {
    // 			// A limit is imposed here to prevent too much engine strain or
    // 			// too much layout deformation - to be exact, this is the limit
    // 			// allowed in Custom Game.
    // 			throw new Error(`Max move count ${this.maxMoveCount}${this.blame('maxmovecount')} is unsupported (we only support up to 24)`);
    // 		}
    //
    // 		if (!this.defaultLevel) {
    // 			// defaultLevel will set level 100 pokemon to the default level, which can break
    // 			// Max Total Level if Max Level is above 100.
    // 			const maxTeamSize = this.pickedTeamSize || this.maxTeamSize;
    // 			if (this.maxTotalLevel && this.maxLevel > 100 && this.maxLevel * maxTeamSize > this.maxTotalLevel) {
    // 				this.defaultLevel = 100;
    // 			} else {
    // 				this.defaultLevel = this.maxLevel;
    // 			}
    // 		}
    // 		if (this.minTeamSize && this.minTeamSize < gameTypeMinTeamSize) {
    // 			throw new Error(`Min team size ${this.minTeamSize}${this.blame('minteamsize')} must be at least ${gameTypeMinTeamSize} for a ${format.gameType} game.`);
    // 		}
    // 		if (this.pickedTeamSize && this.pickedTeamSize < gameTypeMinTeamSize) {
    // 			throw new Error(`Chosen team size ${this.pickedTeamSize}${this.blame('pickedteamsize')} must be at least ${gameTypeMinTeamSize} for a ${format.gameType} game.`);
    // 		}
    // 		if (this.minTeamSize && this.pickedTeamSize && this.minTeamSize < this.pickedTeamSize) {
    // 			throw new Error(`Min team size ${this.minTeamSize}${this.blame('minteamsize')} is lower than chosen team size ${this.pickedTeamSize}${this.blame('pickedteamsize')}.`);
    // 		}
    // 		if (!this.minTeamSize) this.minTeamSize = Math.max(gameTypeMinTeamSize, this.pickedTeamSize || 0);
    // 		if (this.maxTeamSize < gameTypeMinTeamSize) {
    // 			throw new Error(`Max team size ${this.maxTeamSize}${this.blame('maxteamsize')} must be at least ${gameTypeMinTeamSize} for a ${format.gameType} game.`);
    // 		}
    // 		if (this.maxTeamSize < this.minTeamSize) {
    // 			throw new Error(`Max team size ${this.maxTeamSize}${this.blame('maxteamsize')} must be at least min team size ${this.minTeamSize}${this.blame('minteamsize')}.`);
    // 		}
    // 		if (this.minLevel > this.maxLevel) {
    // 			throw new Error(`Min level ${this.minLevel}${this.blame('minlevel')} should not be above max level ${this.maxLevel}${this.blame('maxlevel')}.`);
    // 		}
    // 		if (this.defaultLevel > this.maxLevel) {
    // 			throw new Error(`Default level ${this.defaultLevel}${this.blame('defaultlevel')} should not be above max level ${this.maxLevel}${this.blame('maxlevel')}.`);
    // 		}
    // 		if (this.defaultLevel < this.minLevel) {
    // 			throw new Error(`Default level ${this.defaultLevel}${this.blame('defaultlevel')} should not be below min level ${this.minLevel}${this.blame('minlevel')}.`);
    // 		}
    // 		if (this.adjustLevelDown && this.adjustLevelDown >= this.maxLevel) {
    // 			throw new Error(`Adjust Level Down ${this.adjustLevelDown}${this.blame('adjustleveldown')} will have no effect because it's not below max level ${this.maxLevel}${this.blame('maxlevel')}.`);
    // 		}
    // 		if (this.adjustLevel && this.valueRules.has('minlevel')) {
    // 			throw new Error(`Min Level ${this.minLevel}${this.blame('minlevel')} will have no effect because you're using Adjust Level ${this.adjustLevel}${this.blame('adjustlevel')}.`);
    // 		}
    // 		if (this.evLimit && this.evLimit >= 1512) {
    // 			throw new Error(`EV Limit ${this.evLimit}${this.blame('evlimit')} will have no effect because it's not lower than 1512, the maximum possible combination of 252 EVs in every stat (if you currently have an EV limit, use "! EV Limit" to remove the limit).`);
    // 		}
    // 		if (this.evLimit && this.evLimit < 0) {
    // 			throw new Error(`EV Limit ${this.evLimit}${this.blame('evlimit')} can't be less than 0 (you might have meant: "! EV Limit" to remove the limit, or "EV Limit = 0" to ban EVs).`);
    // 		}
    //
    // 		if (timer.starting !== undefined && (timer.starting < 10 || timer.starting > 1200)) {
    // 			throw new Error(`Timer starting value ${timer.starting}${this.blame('timerstarting')} must be between 10 and 1200 seconds.`);
    // 		}
    // 		if (timer.grace && timer.grace > 300) {
    // 			throw new Error(`Timer grace value ${timer.grace}${this.blame('timergrace')} must be at most 300 seconds.`);
    // 		}
    // 		if (timer.addPerTurn && timer.addPerTurn > 30) {
    // 			throw new Error(`Timer add per turn value ${timer.addPerTurn}${this.blame('timeraddperturn')} must be at most 30 seconds.`);
    // 		}
    // 		if (timer.maxPerTurn !== undefined && (timer.maxPerTurn < 10 || timer.maxPerTurn > 1200)) {
    // 			throw new Error(`Timer max per turn value ${timer.maxPerTurn}${this.blame('timermaxperturn')} must be between 10 and 1200 seconds.`);
    // 		}
    // 		if (timer.maxFirstTurn !== undefined && (timer.maxFirstTurn < 10 || timer.maxFirstTurn > 1200)) {
    // 			throw new Error(`Timer max first turn value ${timer.maxFirstTurn}${this.blame('timermaxfirstturn')} must be between 10 and 1200 seconds.`);
    // 		}
    //
    // 		if ((format as any).cupLevelLimit) {
    // 			throw new Error(`cupLevelLimit.range[0], cupLevelLimit.range[1], cupLevelLimit.total are now rules, respectively: "Min Level = NUMBER", "Max Level = NUMBER", and "Max Total Level = NUMBER"`);
    // 		}
    // 		if ((format as any).teamLength) {
    // 			throw new Error(`teamLength.validate[0], teamLength.validate[1], teamLength.battle are now rules, respectively: "Min Team Size = NUMBER", "Max Team Size = NUMBER", and "Picked Team Size = NUMBER"`);
    // 		}
    // 		if ((format as any).minSourceGen) {
    // 			throw new Error(`minSourceGen is now a rule: "Min Source Gen = NUMBER"`);
    // 		}
    // 		if ((format as any).maxLevel) {
    // 			throw new Error(`maxLevel is now a rule: "Max Level = NUMBER"`);
    // 		}
    // 		if ((format as any).defaultLevel) {
    // 			throw new Error(`defaultLevel is now a rule: "Default Level = NUMBER"`);
    // 		}
    // 		if ((format as any).forcedLevel) {
    // 			throw new Error(`forcedLevel is now a rule: "Adjust Level = NUMBER"`);
    // 		}
    // 		if ((format as any).maxForcedLevel) {
    // 			throw new Error(`maxForcedLevel is now a rule: "Adjust Level Down = NUMBER"`);
    // 		}
    // 	}
    //
    pub fn resolve_numbers(&mut self, format: &FormatDef) {
        let game_type_min_team_size = match format.game_type {
            GameType::Triples => 3,
            GameType::Doubles => 2,
            _ => 1,
        };

        self.min_team_size = self
            .value_rules
            .get("minteamsize")
            .and_then(|v| v.parse().ok())
            .unwrap_or(0);
        self.max_team_size = self
            .value_rules
            .get("maxteamsize")
            .and_then(|v| v.parse().ok())
            .unwrap_or(6);
        self.picked_team_size = self
            .value_rules
            .get("pickedteamsize")
            .and_then(|v| v.parse().ok());
        self.max_total_level = self
            .value_rules
            .get("maxtotallevel")
            .and_then(|v| v.parse().ok());
        self.max_move_count = self
            .value_rules
            .get("maxmovecount")
            .and_then(|v| v.parse().ok())
            .unwrap_or(4);
        self.min_source_gen = self
            .value_rules
            .get("minsourcegen")
            .and_then(|v| v.parse().ok())
            .unwrap_or(1);
        self.min_level = self
            .value_rules
            .get("minlevel")
            .and_then(|v| v.parse().ok())
            .unwrap_or(1);
        self.max_level = self
            .value_rules
            .get("maxlevel")
            .and_then(|v| v.parse().ok())
            .unwrap_or(100);
        self.default_level = self
            .value_rules
            .get("defaultlevel")
            .and_then(|v| v.parse().ok())
            .unwrap_or(0);
        self.adjust_level = self
            .value_rules
            .get("adjustlevel")
            .and_then(|v| v.parse().ok());
        self.adjust_level_down = self
            .value_rules
            .get("adjustleveldown")
            .and_then(|v| v.parse().ok());
        self.ev_limit = self.value_rules.get("evlimit").and_then(|v| v.parse().ok());

        // Handle auto values for picked team size
        if self.value_rules.get("pickedteamsize").map(|s| s.as_str()) == Some("Auto") {
            self.picked_team_size = Some(match format.game_type {
                GameType::Doubles => 4,
                GameType::Triples => 6,
                _ => 3,
            });
        }

        // Set default level if not specified
        if self.default_level == 0 {
            self.default_level = self.max_level;
        }

        // Set min team size if not specified
        if self.min_team_size == 0 {
            self.min_team_size = game_type_min_team_size.max(self.picked_team_size.unwrap_or(0));
        }
    }

    /// Check if there are complex bans
    /// Equivalent to hasComplexBans()
    //
    // 	hasComplexBans() {
    // 		return (this.complexBans?.length > 0) || (this.complexTeamBans?.length > 0);
    // 	}
    //
    pub fn has_complex_bans(&self) -> bool {
        !self.complex_bans.is_empty() || !self.complex_team_bans.is_empty()
    }
}

// =========================================================================
// DEXFORMATS - Equivalent to dex-formats.ts DexFormats class
// =========================================================================

/// Format class - runtime format object
#[derive(Debug, Clone)]
/// JavaScript equivalent: Format (sim/global-types.ts)
/// 16 fields in JavaScript
pub struct Format {
    /// JavaScript: id: ID
    pub id: String,
    /// JavaScript: name: string
    pub name: String,
    /// JavaScript: mod: string
    pub mod_: String,
    /// JavaScript: effectType: string
    pub effect_type: String,
    /// JavaScript: debug: boolean
    pub debug: bool,
    /// JavaScript: rated: boolean
    pub rated: bool,
    /// JavaScript: gameType: GameType
    pub game_type: GameType,
    /// JavaScript: playerCount: number
    pub player_count: u8,
    /// JavaScript: ruleset: string[]
    pub ruleset: Vec<String>,
    /// JavaScript: baseRuleset: string[]
    pub base_ruleset: Vec<String>,
    /// JavaScript: banlist: string[]
    pub banlist: Vec<String>,
    /// JavaScript: restricted: string[]
    pub restricted: Vec<String>,
    /// JavaScript: unbanlist: string[]
    pub unbanlist: Vec<String>,
    /// JavaScript: customRules?: string[]
    pub custom_rules: Option<Vec<String>>,
    /// JavaScript: ruleTable?: RuleTable
    pub rule_table: Option<RuleTable>,
    /// JavaScript: team?: string
    pub team: Option<String>,
    /// JavaScript: exists: boolean
    pub exists: bool,
}

impl Format {
    /// Create a new Format from a FormatDef
    pub fn from_def(def: &FormatDef) -> Self {
        Self {
            id: def.id.to_string(),
            name: def.name.to_string(),
            mod_: format!("gen{}", def.mod_.number()),
            effect_type: "Format".to_string(),
            debug: false,
            rated: def.rated,
            game_type: def.game_type,
            player_count: if matches!(def.game_type, GameType::Multi | GameType::FreeForAll) {
                4
            } else {
                2
            },
            ruleset: def.rulesets.iter().map(|s| s.to_string()).collect(),
            base_ruleset: def.rulesets.iter().map(|s| s.to_string()).collect(),
            banlist: def.bans.iter().map(|s| s.to_string()).collect(),
            restricted: Vec::new(),
            unbanlist: def.unbans.iter().map(|s| s.to_string()).collect(),
            custom_rules: None,
            rule_table: None,
            team: None, // Set when format uses team generation (e.g., random battles)
            exists: true,
        }
    }

    /// Create a non-existent format
    pub fn non_existent(id: &str, name: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            mod_: "gen9".to_string(),
            effect_type: "Format".to_string(),
            debug: false,
            rated: false,
            game_type: GameType::Singles,
            player_count: 2,
            ruleset: Vec::new(),
            base_ruleset: Vec::new(),
            banlist: Vec::new(),
            restricted: Vec::new(),
            unbanlist: Vec::new(),
            custom_rules: None,
            rule_table: None,
            team: None,
            exists: false,
        }
    }
}

/// DexFormats - manages format loading and validation
/// JavaScript equivalent: DexFormats (sim/dex-formats.ts)
/// 4 fields in JavaScript (dex, formatsListCache, searchShow, default)
pub struct DexFormats {
    /// Cache of loaded formats
    pub formats_cache: HashMap<ID, Format>,
    /// List of all formats
    pub formats_list: Vec<Format>,
    /// Whether formats have been loaded
    loaded: bool,
}

impl DexFormats {
    /// Create a new DexFormats
    pub fn new() -> Self {
        Self {
            formats_cache: HashMap::new(),
            formats_list: Vec::new(),
            loaded: false,
        }
    }

    /// Load formats from static definitions
    /// Equivalent to load()
    //
    // 	load(): this {
    // 		if (!this.dex.isBase) throw new Error(`This should only be run on the base mod`);
    // 		this.dex.includeMods();
    // 		if (this.formatsListCache) return this;
    //
    // 		const formatsList = [];
    //
    // 		// Load formats
    // 		let customFormats;
    // 		try {
    // 			customFormats = require(`${__dirname}/../config/custom-formats`).Formats;
    // 			if (!Array.isArray(customFormats)) {
    // 				throw new TypeError(`Exported property 'Formats' from "./config/custom-formats.ts" must be an array`);
    // 			}
    // 		} catch (e: any) {
    // 			if (e.code !== 'MODULE_NOT_FOUND' && e.code !== 'ENOENT') {
    // 				throw e;
    // 			}
    // 		}
    // 		let Formats: AnyObject[] = require(`${__dirname}/../config/formats`).Formats;
    // 		if (!Array.isArray(Formats)) {
    // 			throw new TypeError(`Exported property 'Formats' from "./config/formats.ts" must be an array`);
    // 		}
    // 		if (customFormats) Formats = mergeFormatLists(Formats as any, customFormats);
    //
    // 		let section = '';
    // 		let column = 1;
    // 		for (const [i, format] of Formats.entries()) {
    // 			const id = toID(format.name);
    // 			if (format.section) section = format.section;
    // 			if (format.column) column = format.column;
    // 			if (!format.name && format.section) continue;
    // 			if (!id) {
    // 				throw new RangeError(`Format #${i + 1} must have a name with alphanumeric characters, not '${format.name}'`);
    // 			}
    // 			if (!format.section) format.section = section;
    // 			if (!format.column) format.column = column;
    // 			if (this.rulesetCache.has(id)) throw new Error(`Format #${i + 1} has a duplicate ID: '${id}'`);
    // 			format.effectType = 'Format';
    // 			format.baseRuleset = format.ruleset ? format.ruleset.slice() : [];
    // 			if (format.challengeShow === undefined) format.challengeShow = true;
    // 			if (format.searchShow === undefined) format.searchShow = true;
    // 			if (format.tournamentShow === undefined) format.tournamentShow = true;
    // 			if (format.bestOfDefault === undefined) format.bestOfDefault = false;
    // 			if (format.teraPreviewDefault === undefined) format.teraPreviewDefault = false;
    // 			if (format.mod === undefined) format.mod = 'gen9';
    // 			if (!this.dex.dexes[format.mod]) throw new Error(`Format "${format.name}" requires nonexistent mod: '${format.mod}'`);
    //
    // 			const ruleset = new Format(format);
    // 			this.rulesetCache.set(id, ruleset);
    // 			formatsList.push(ruleset);
    // 		}
    //
    // 		this.formatsListCache = formatsList;
    // 		return this;
    // 	}
    //
    pub fn load(&mut self) {
        if self.loaded {
            return;
        }

        for (id, def) in FORMATS.iter() {
            let format = Format::from_def(def);
            self.formats_cache.insert(id.clone(), format.clone());
            self.formats_list.push(format);
        }

        self.loaded = true;
    }

    /// Validate a format name and return sanitized ID
    /// Equivalent to validate()
    // TypeScript source:
    // /**
    // 	 * Returns a sanitized format ID if valid, or throws if invalid.
    // 	 */
    // 	validate(name: string) {
    // 		const [formatName, customRulesString] = name.split('@@@', 2);
    // 		const format = this.get(formatName);
    // 		if (format.effectType !== 'Format') throw new Error(`Unrecognized format "${formatName}"`);
    // 		if (!customRulesString) return format.id;
    // 		const ruleTable = this.getRuleTable(format);
    // 		let hasCustomRules = false;
    // 		let hasPokemonRule = false;
    // 		const customRules = customRulesString.split(',').map(rule => {
    // 			rule = rule.replace(/[\r\n|]*/g, '').trim();
    // 			const ruleSpec = this.validateRule(rule);
    // 			if (typeof ruleSpec === 'string') {
    // 				if (ruleSpec === '-pokemontag:allpokemon' || ruleSpec === '+pokemontag:allpokemon') {
    // 					if (hasPokemonRule) throw new Error(`You can't ban/unban pokemon before banning/unbanning all Pokemon.`);
    // 				}
    // 				if (this.isPokemonRule(ruleSpec)) hasPokemonRule = true;
    // 			}
    // 			if (typeof ruleSpec !== 'string' || !ruleTable.has(ruleSpec)) hasCustomRules = true;
    // 			return rule;
    // 		});
    // 		if (!hasCustomRules) throw new Error(`None of your custom rules change anything`);
    // 		const validatedFormatid = format.id + '@@@' + customRules.join(',');
    // 		const moddedFormat = this.get(validatedFormatid, true);
    // 		this.getRuleTable(moddedFormat);
    // 		return validatedFormatid;
    // 	}
    //
    pub fn validate(&mut self, name: &str) -> Result<String, String> {
        let parts: Vec<&str> = name.split("@@@").collect();
        let format_name = parts[0];
        let format = self.get(format_name);

        if !format.exists {
            return Err(format!("Unrecognized format \"{}\"", format_name));
        }

        if parts.len() == 1 {
            return Ok(format.id.clone());
        }

        // Has custom rules
        let custom_rules: Vec<String> = parts[1].split(',').map(|r| r.trim().to_string()).collect();

        Ok(format!("{}@@@{}", format.id, custom_rules.join(",")))
    }

    /// Get a format by name
    /// Equivalent to get()
    // TypeScript source:
    // /**
    // 	 * The default mode is `isTrusted = false`, which is a bit of a
    // 	 * footgun. PS will never do anything unsafe, but `isTrusted = true`
    // 	 * will throw if the format string is invalid, while
    // 	 * `isTrusted = false` will silently fall back to the original format.
    // 	 */
    // 	get(name?: string | Format, isTrusted = false): Format {
    // 		if (name && typeof name !== 'string') return name;
    //
    // 		name = (name || '').trim();
    // 		let id = toID(name);
    //
    // 		if (!name.includes('@@@')) {
    // 			const ruleset = this.rulesetCache.get(id);
    // 			if (ruleset) return ruleset;
    // 		}
    //
    // 		if (this.dex.getAlias(id)) {
    // 			id = this.dex.getAlias(id)!;
    // 			name = id;
    // 		}
    // 		if (this.dex.data.Rulesets.hasOwnProperty(DEFAULT_MOD + id)) {
    // 			id = (DEFAULT_MOD + id) as ID;
    // 		}
    // 		let supplementaryAttributes: AnyObject | null = null;
    // 		if (name.includes('@@@')) {
    // 			if (!isTrusted) {
    // 				try {
    // 					name = this.validate(name);
    // 					isTrusted = true;
    // 				} catch {}
    // 			}
    // 			const [newName, customRulesString] = name.split('@@@', 2);
    // 			name = newName.trim();
    // 			id = toID(name);
    // 			if (isTrusted && customRulesString) {
    // 				supplementaryAttributes = {
    // 					customRules: customRulesString.split(','),
    // 					searchShow: false,
    // 				};
    // 			}
    // 		}
    // 		let effect;
    // 		if (this.dex.data.Rulesets.hasOwnProperty(id)) {
    // 			effect = new Format({ name, ...this.dex.data.Rulesets[id] as any, ...supplementaryAttributes });
    // 		} else {
    // 			effect = new Format({ id, name, exists: false });
    // 		}
    // 		return effect;
    // 	}
    //
    pub fn get(&mut self, name: &str) -> Format {
        self.load();

        let name = name.trim();
        let id = ID::new(name);

        // Check cache
        if let Some(format) = self.formats_cache.get(&id) {
            return format.clone();
        }

        // Check static formats
        if let Some(def) = FORMATS.get(&id) {
            let format = Format::from_def(def);
            self.formats_cache.insert(id.clone(), format.clone());
            return format;
        }

        // Return non-existent format
        Format::non_existent(id.as_str(), name)
    }

    /// Get all formats
    /// Equivalent to all()
    //
    // 	all() {
    // 		this.load();
    // 		return this.formatsListCache!;
    // 	}
    //
    pub fn all(&mut self) -> &Vec<Format> {
        self.load();
        &self.formats_list
    }

    /// Check if a rule spec is for Pokemon
    /// Equivalent to isPokemonRule()
    //
    // 	isPokemonRule(ruleSpec: string) {
    // 		return (
    // 			ruleSpec.slice(1).startsWith('pokemontag:') || ruleSpec.slice(1).startsWith('pokemon:') ||
    // 			ruleSpec.slice(1).startsWith('basepokemon:')
    // 		);
    // 	}
    //
    pub fn is_pokemon_rule(rule_spec: &str) -> bool {
        let rest = if rule_spec.len() > 1 {
            &rule_spec[1..]
        } else {
            ""
        };
        rest.starts_with("pokemontag:")
            || rest.starts_with("pokemon:")
            || rest.starts_with("basepokemon:")
    }

    /// Get rule table for a format
    /// Equivalent to getRuleTable()
    // 	getRuleTable(format: Format, depth = 1, repeals?: Map<string, number>): RuleTable {
    // 		if (format.ruleTable && !repeals) return format.ruleTable;
    // 		if (format.name.length > 50) {
    // 			throw new Error(`Format "${format.name}" has a name longer than 50 characters`);
    // 		}
    // 		if (depth === 1) {
    // 			const dex = this.dex.mod(format.mod);
    // 			if (dex !== this.dex) {
    // 				return dex.formats.getRuleTable(format, 2, repeals);
    // 			}
    // 		}
    // 		const ruleTable = new RuleTable();
    //
    // 		const ruleset = format.ruleset.slice();
    // 		for (const ban of format.banlist) {
    // 			ruleset.push('-' + ban);
    // 		}
    // 		for (const ban of format.restricted) {
    // 			ruleset.push('*' + ban);
    // 		}
    // 		for (const ban of format.unbanlist) {
    // 			ruleset.push('+' + ban);
    // 		}
    // 		if (format.customRules) {
    // 			ruleset.push(...format.customRules);
    // 		}
    // 		if (format.checkCanLearn) {
    // 			ruleTable.checkCanLearn = [format.checkCanLearn, format.name];
    // 		}
    // 		if (format.onChooseTeam) {
    // 			ruleTable.onChooseTeam = [format.onChooseTeam, format.name];
    // 		}
    //
    // 		// apply rule repeals before other rules
    // 		// repeals is a ruleid:depth map (positive: unused, negative: used)
    // 		const ruleSpecs = ruleset.map(rule => this.validateRule(rule, format));
    // 		for (let ruleSpec of ruleSpecs) {
    // 			if (typeof ruleSpec !== 'string') continue;
    // 			if (ruleSpec.startsWith('^')) ruleSpec = ruleSpec.slice(1);
    // 			if (ruleSpec.startsWith('!') && !ruleSpec.startsWith('!!')) {
    // 				repeals ||= new Map();
    // 				repeals.set(ruleSpec.slice(1), depth);
    // 			}
    // 		}
    //
    // 		let skipPokemonBans = ruleSpecs.filter(r => r === '+pokemontag:allpokemon').length;
    // 		let hasPokemonBans = false;
    // 		const warnForNoPokemonBans = !!skipPokemonBans && !format.customRules;
    // 		skipPokemonBans += ruleSpecs.filter(r => r === '-pokemontag:allpokemon').length;
    //
    // 		// if (format.customRules) console.log(`${format.id}: ${format.customRules.join(', ')}`);
    //
    // 		for (let ruleSpec of ruleSpecs) {
    // 			// complex ban/unban
    // 			if (typeof ruleSpec !== 'string') {
    // 				if (ruleSpec[0] === 'complexTeamBan') {
    // 					const complexTeamBan: ComplexTeamBan = ruleSpec.slice(1) as ComplexTeamBan;
    // 					ruleTable.addComplexTeamBan(complexTeamBan[0], complexTeamBan[1], complexTeamBan[2], complexTeamBan[3]);
    // 				} else if (ruleSpec[0] === 'complexBan') {
    // 					const complexBan: ComplexBan = ruleSpec.slice(1) as ComplexBan;
    // 					ruleTable.addComplexBan(complexBan[0], complexBan[1], complexBan[2], complexBan[3]);
    // 				} else {
    // 					throw new Error(`Unrecognized rule spec ${ruleSpec}`);
    // 				}
    // 				continue;
    // 			}
    //
    // 			// ^ is undocumented because I really don't want it used outside of tests
    // 			const noWarn = ruleSpec.startsWith('^');
    // 			if (noWarn) ruleSpec = ruleSpec.slice(1);
    //
    // 			// repeal rule
    // 			if (ruleSpec.startsWith('!') && !ruleSpec.startsWith('!!')) {
    // 				const repealDepth = repeals!.get(ruleSpec.slice(1));
    // 				if (repealDepth === undefined) throw new Error(`Multiple "${ruleSpec}" rules in ${format.name}`);
    // 				if (repealDepth === depth && !noWarn) {
    // 					throw new Error(`Rule "${ruleSpec}" did nothing because "${ruleSpec.slice(1)}" is not in effect`);
    // 				}
    // 				if (repealDepth === -depth) repeals!.delete(ruleSpec.slice(1));
    // 				continue;
    // 			}
    //
    // 			// individual ban/unban
    // 			if ('+*-'.includes(ruleSpec.charAt(0))) {
    // 				if (ruleTable.has(ruleSpec)) {
    // 					throw new Error(`Rule "${ruleSpec}" in "${format.name}" already exists in "${ruleTable.get(ruleSpec) || format.name}"`);
    // 				}
    // 				if (skipPokemonBans) {
    // 					if (ruleSpec === '-pokemontag:allpokemon' || ruleSpec === '+pokemontag:allpokemon') {
    // 						skipPokemonBans--;
    // 					} else if (this.isPokemonRule(ruleSpec)) {
    // 						if (!format.customRules) {
    // 							throw new Error(`Rule "${ruleSpec}" must go after any "All Pokemon" rule in ${format.name} ("+All Pokemon" should go in ruleset, not unbanlist)`);
    // 						}
    // 						continue;
    // 					}
    // 				}
    // 				for (const prefix of '+*-') ruleTable.delete(prefix + ruleSpec.slice(1));
    // 				ruleTable.set(ruleSpec, '');
    // 				continue;
    // 			}
    //
    // 			// rule
    // 			let [formatid, value] = ruleSpec.split('=');
    // 			const subformat = this.get(formatid);
    // 			const repealAndReplace = ruleSpec.startsWith('!!');
    // 			if (repeals?.has(subformat.id)) {
    // 				repeals.set(subformat.id, -Math.abs(repeals.get(subformat.id)!));
    // 				continue;
    // 			}
    // 			if (subformat.hasValue) {
    // 				if (value === undefined) throw new Error(`Rule "${ruleSpec}" should have a value (like "${ruleSpec} = something")`);
    // 				if (value === 'Current Gen') value = `${this.dex.gen}`;
    // 				if ((subformat.id === 'pickedteamsize' || subformat.id === 'evlimit') && value === 'Auto') {
    // 					// can't be resolved until later
    // 				} else if (subformat.hasValue === 'integer' || subformat.hasValue === 'positive-integer') {
    // 					const intValue = parseInt(value);
    // 					if (isNaN(intValue) || value !== `${intValue}`) {
    // 						throw new Error(`In rule "${ruleSpec}", "${value}" must be an integer number.`);
    // 					}
    // 				}
    // 				if (subformat.hasValue === 'positive-integer') {
    // 					if (parseInt(value) === 0) {
    // 						throw new Error(`In rule "${ruleSpec}", "${value}" must be positive (to remove it, use the rule "! ${subformat.name}").`);
    // 					}
    // 					if (parseInt(value) <= 0) {
    // 						throw new Error(`In rule "${ruleSpec}", "${value}" must be positive.`);
    // 					}
    // 				}
    //
    // 				const oldValue = ruleTable.valueRules.get(subformat.id);
    // 				if (oldValue === value) {
    // 					if (!noWarn) {
    // 						throw new Error(`Rule "${ruleSpec}" is redundant with existing rule "${subformat.id}=${value}"${ruleTable.blame(subformat.id)}.`);
    // 					}
    // 				} else if (repealAndReplace) {
    // 					if (oldValue === undefined) {
    // 						if (subformat.mutuallyExclusiveWith && ruleTable.valueRules.has(subformat.mutuallyExclusiveWith)) {
    // 							if (this.dex.formats.get(subformat.mutuallyExclusiveWith).ruleset.length) {
    // 								throw new Error(`This format does not support "!!"`);
    // 							}
    // 							ruleTable.valueRules.delete(subformat.mutuallyExclusiveWith);
    // 							ruleTable.delete(subformat.mutuallyExclusiveWith);
    // 						} else {
    // 							throw new Error(`Rule "${ruleSpec}" is not replacing anything (it should not have "!!")`);
    // 						}
    // 					}
    // 				} else {
    // 					if (oldValue !== undefined) {
    // 						throw new Error(`Rule "${ruleSpec}" conflicts with "${subformat.id}=${oldValue}"${ruleTable.blame(subformat.id)} (Use "!! ${ruleSpec}" to override "${subformat.id}=${oldValue}".)`);
    // 					}
    // 					if (subformat.mutuallyExclusiveWith && ruleTable.valueRules.has(subformat.mutuallyExclusiveWith)) {
    // 						const oldRule = `"${subformat.mutuallyExclusiveWith}=${ruleTable.valueRules.get(subformat.mutuallyExclusiveWith)}"`;
    // 						throw new Error(`Format can't simultaneously have "${ruleSpec}" and ${oldRule}${ruleTable.blame(subformat.mutuallyExclusiveWith)} (Use "!! ${ruleSpec}" to override ${oldRule}.)`);
    // 					}
    // 				}
    // 				ruleTable.valueRules.set(subformat.id, value);
    // 			} else {
    // 				if (value !== undefined) throw new Error(`Rule "${ruleSpec}" should not have a value (no equals sign)`);
    // 				if (repealAndReplace) throw new Error(`"!!" is not supported for this rule`);
    // 				if (ruleTable.has(subformat.id) && !repealAndReplace && !noWarn) {
    // 					throw new Error(`Rule "${ruleSpec}" in "${format.name}" already exists in "${ruleTable.get(subformat.id) || format.name}"`);
    // 				}
    // 			}
    // 			ruleTable.set(subformat.id, '');
    // 			if (depth > 16) {
    // 				throw new Error(`Excessive ruleTable recursion in ${format.name}: ${ruleSpec} of ${format.ruleset}`);
    // 			}
    // 			const subRuleTable = this.getRuleTable(subformat, depth + 1, repeals);
    // 			for (const [ruleid, sourceFormat] of subRuleTable) {
    // 				// don't check for "already exists" here; multiple inheritance is allowed
    // 				if (repeals?.has(ruleid)) continue;
    //
    // 				if (skipPokemonBans && '+*-'.includes(ruleid.charAt(0))) {
    // 					if (this.isPokemonRule(ruleid)) {
    // 						hasPokemonBans = true;
    // 						continue;
    // 					}
    // 				}
    //
    // 				const newValue = subRuleTable.valueRules.get(ruleid);
    // 				const oldValue = ruleTable.valueRules.get(ruleid);
    // 				if (newValue !== undefined) {
    // 					// set a value
    // 					const subSubFormat = this.get(ruleid);
    // 					if (subSubFormat.mutuallyExclusiveWith && ruleTable.valueRules.has(subSubFormat.mutuallyExclusiveWith)) {
    // 						// mutually exclusive conflict!
    // 						throw new Error(`Rule "${ruleid}=${newValue}" from ${subformat.name}${subRuleTable.blame(ruleid)} conflicts with "${subSubFormat.mutuallyExclusiveWith}=${ruleTable.valueRules.get(subSubFormat.mutuallyExclusiveWith)}"${ruleTable.blame(subSubFormat.mutuallyExclusiveWith)} (Repeal one with ! before adding another)`);
    // 					}
    // 					if (newValue !== oldValue) {
    // 						if (oldValue !== undefined) {
    // 							// conflict!
    // 							throw new Error(`Rule "${ruleid}=${newValue}" from ${subformat.name}${subRuleTable.blame(ruleid)} conflicts with "${ruleid}=${oldValue}"${ruleTable.blame(ruleid)} (Repeal one with ! before adding another)`);
    // 						}
    // 						ruleTable.valueRules.set(ruleid, newValue);
    // 					}
    // 				}
    // 				ruleTable.set(ruleid, sourceFormat || subformat.name);
    // 			}
    // 			for (const [subRule, source, limit, bans] of subRuleTable.complexBans) {
    // 				ruleTable.addComplexBan(subRule, source || subformat.name, limit, bans);
    // 			}
    // 			for (const [subRule, source, limit, bans] of subRuleTable.complexTeamBans) {
    // 				ruleTable.addComplexTeamBan(subRule, source || subformat.name, limit, bans);
    // 			}
    // 			if (subRuleTable.checkCanLearn) {
    // 				if (ruleTable.checkCanLearn) {
    // 					throw new Error(
    // 						`"${format.name}" has conflicting move validation rules from ` +
    // 						`"${ruleTable.checkCanLearn[1]}" and "${subRuleTable.checkCanLearn[1]}"`
    // 					);
    // 				}
    // 				ruleTable.checkCanLearn = subRuleTable.checkCanLearn;
    // 			}
    // 			if (subRuleTable.onChooseTeam) {
    // 				if (ruleTable.onChooseTeam) {
    // 					throw new Error(
    // 						`"${format.name}" has conflicting team selection rules from ` +
    // 						`"${ruleTable.onChooseTeam[1]}" and "${subRuleTable.onChooseTeam[1]}"`
    // 					);
    // 				}
    // 				ruleTable.onChooseTeam = subRuleTable.onChooseTeam;
    // 			}
    // 		}
    // 		if (!hasPokemonBans && warnForNoPokemonBans) {
    // 			throw new Error(`"+All Pokemon" rule has no effect (no species are banned by default, and it does not override obtainability rules)`);
    // 		}
    // 		ruleTable.getTagRules();
    //
    // 		ruleTable.resolveNumbers(format, this.dex);
    //
    // 		const canMegaEvo = this.dex.gen <= 7 || ruleTable.has('+pokemontag:past');
    // 		if (ruleTable.has('obtainableformes') && canMegaEvo &&
    // 			ruleTable.isBannedSpecies(this.dex.species.get('rayquazamega')) &&
    // 			!ruleTable.isBannedSpecies(this.dex.species.get('rayquaza'))
    // 		) {
    // 			// Banning Rayquaza-Mega implicitly adds Mega Rayquaza Clause
    // 			// note that already having it explicitly in the ruleset is ok
    // 			ruleTable.set('megarayquazaclause', '');
    // 		}
    //
    // 		for (const rule of ruleTable.keys()) {
    // 			if ("+*-!".includes(rule.charAt(0))) continue;
    // 			const subFormat = this.dex.formats.get(rule);
    // 			if (subFormat.exists) {
    // 				const value = subFormat.onValidateRule?.call(
    // 					{ format, ruleTable, dex: this.dex }, ruleTable.valueRules.get(rule as ID)!
    // 				);
    // 				if (typeof value === 'string') ruleTable.valueRules.set(subFormat.id, value);
    // 			}
    // 		}
    //
    // 		if (!repeals) format.ruleTable = ruleTable;
    // 		return ruleTable;
    // 	}
    //
    pub fn get_rule_table(&self, format: &Format) -> RuleTable {
        let mut rule_table = RuleTable::new();

        // Add rules from ruleset - convert to IDs (lowercase alphanumeric)
        for rule in &format.ruleset {
            // JavaScript: ruleTable.set(ruleid, sourceFormat);
            // where ruleid is the ID form (toID(ruleName))
            rule_table.set(&ID::new(rule).to_string(), "");
        }

        // Add bans
        for ban in &format.banlist {
            rule_table.set(&format!("-{}", ban), "");
        }

        // Add unbans
        for unban in &format.unbanlist {
            rule_table.set(&format!("+{}", unban), "");
        }

        // Add restricted
        for restricted in &format.restricted {
            rule_table.set(&format!("*{}", restricted), "");
        }

        // Add custom rules
        if let Some(custom_rules) = &format.custom_rules {
            for rule in custom_rules {
                rule_table.set(rule, "custom");
            }
        }

        // Resolve numbers from a FormatDef if we can find one
        if let Some(def) = get_format(&ID::new(&format.id)) {
            rule_table.resolve_numbers(def);
        }

        rule_table.get_tag_rules();

        rule_table
    }

    /// Validate a rule string
    /// Equivalent to validateRule()
    //
    // 	validateRule(rule: string, format: Format | null = null) {
    // 		if (rule !== rule.trim()) throw new Error(`Rule "${rule}" should be trimmed`);
    // 		switch (rule.charAt(0)) {
    // 		case '-':
    // 		case '*':
    // 		case '+':
    // 			if (rule.slice(1).includes('>') || rule.slice(1).includes('+')) {
    // 				let buf = rule.slice(1);
    // 				const gtIndex = buf.lastIndexOf('>');
    // 				let limit = rule.startsWith('+') ? Infinity : 0;
    // 				if (gtIndex >= 0 && /^[0-9]+$/.test(buf.slice(gtIndex + 1).trim())) {
    // 					if (limit === 0) limit = parseInt(buf.slice(gtIndex + 1));
    // 					buf = buf.slice(0, gtIndex);
    // 				}
    // 				let checkTeam = buf.includes('++');
    // 				const banNames = buf.split(checkTeam ? '++' : '+').map(v => v.trim());
    // 				if (banNames.length === 1 && limit > 0) checkTeam = true;
    // 				const innerRule = banNames.join(checkTeam ? ' ++ ' : ' + ');
    // 				const bans = banNames.map(v => this.validateBanRule(v));
    //
    // 				if (checkTeam) {
    // 					return ['complexTeamBan', innerRule, '', limit, bans];
    // 				}
    // 				if (bans.length > 1 || limit > 0) {
    // 					return ['complexBan', innerRule, '', limit, bans];
    // 				}
    // 				throw new Error(`Confusing rule ${rule}`);
    // 			}
    // 			return rule.charAt(0) + this.validateBanRule(rule.slice(1));
    // 		default:
    // 			const [ruleName, value] = rule.split('=');
    // 			let id: string = toID(ruleName);
    // 			const ruleset = this.dex.formats.get(id);
    // 			if (!ruleset.exists) {
    // 				throw new Error(`Unrecognized rule "${rule}"`);
    // 			}
    // 			if (typeof value === 'string') id = `${id}=${value.trim()}`;
    // 			if (rule.startsWith('^!')) return `^!${id}`;
    // 			if (rule.startsWith('^')) return `^${id}`;
    // 			if (rule.startsWith('!!')) return `!!${id}`;
    // 			if (rule.startsWith('!')) return `!${id}`;
    // 			return id;
    // 		}
    // 	}
    //
    pub fn validate_rule(&self, rule: &str) -> Result<String, String> {
        let rule = rule.trim();

        if rule.is_empty() {
            return Err("Empty rule".to_string());
        }

        let first_char = rule.chars().next().unwrap();

        match first_char {
            '-' | '*' | '+' => {
                // Ban/unban/restrict rule
                let rest = &rule[1..];
                self.validate_ban_rule(rest)
                    .map(|r| format!("{}{}", first_char, r))
            }
            '!' => {
                // Repeal rule
                let rest = rule[1..].trim();
                Ok(format!("!{}", ID::new(rest).as_str()))
            }
            _ => {
                // Regular rule
                if let Some((name, value)) = rule.split_once('=') {
                    let id = ID::new(name.trim());
                    Ok(format!("{}={}", id.as_str(), value.trim()))
                } else {
                    Ok(ID::new(rule).to_string())
                }
            }
        }
    }

    /// Check if a tag is a valid Pokemon tag
    /// Equivalent to validPokemonTag()
    //
    // 	validPokemonTag(tagid: ID) {
    // 		const tag = Tags.hasOwnProperty(tagid) && Tags[tagid];
    // 		if (!tag) return false;
    // 		return !!(tag.speciesFilter || tag.genericFilter);
    // 	}
    //
    pub fn valid_pokemon_tag(&self, _tagid: &str) -> bool {
        // Simplified - would check against Tags data
        true
    }

    /// Validate a ban rule
    /// Equivalent to validateBanRule()
    //
    // 	validateBanRule(rule: string) {
    // 		let id = toID(rule);
    // 		if (id === 'unreleased') return 'unreleased';
    // 		if (id === 'nonexistent') return 'nonexistent';
    // 		const matches = [];
    // 		let matchTypes = ['pokemon', 'move', 'ability', 'item', 'nature', 'pokemontag'];
    // 		for (const matchType of matchTypes) {
    // 			if (rule.startsWith(`${matchType}:`)) {
    // 				matchTypes = [matchType];
    // 				id = id.slice(matchType.length) as ID;
    // 				break;
    // 			}
    // 		}
    // 		const ruleid = id;
    // 		id = this.dex.getAlias(id) || id;
    // 		for (const matchType of matchTypes) {
    // 			if (matchType === 'item' && ruleid === 'noitem') return 'item:noitem';
    // 			let table;
    // 			switch (matchType) {
    // 			case 'pokemon': table = this.dex.data.Pokedex; break;
    // 			case 'move': table = this.dex.data.Moves; break;
    // 			case 'item': table = this.dex.data.Items; break;
    // 			case 'ability': table = this.dex.data.Abilities; break;
    // 			case 'nature': table = this.dex.data.Natures; break;
    // 			case 'pokemontag':
    // 				// valid pokemontags
    // 				const validTags = [
    // 					// all
    // 					'allpokemon', 'allitems', 'allmoves', 'allabilities', 'allnatures',
    // 				];
    // 				if (validTags.includes(ruleid) || this.validPokemonTag(ruleid)) {
    // 					matches.push('pokemontag:' + ruleid);
    // 				}
    // 				continue;
    // 			default:
    // 				throw new Error(`Unrecognized match type.`);
    // 			}
    // 			if (table.hasOwnProperty(id)) {
    // 				if (matchType === 'pokemon') {
    // 					const species = table[id] as SpeciesData;
    // 					if ((species.otherFormes || species.cosmeticFormes) && ruleid === id) {
    // 						matches.push('basepokemon:' + id);
    // 						continue;
    // 					}
    // 				}
    // 				matches.push(matchType + ':' + id);
    // 			} else if (matchType === 'pokemon' && id.endsWith('base')) {
    // 				id = id.slice(0, -4) as ID;
    // 				if (table.hasOwnProperty(id)) {
    // 					matches.push('pokemon:' + id);
    // 				}
    // 			}
    // 		}
    // 		if (matches.length > 1) {
    // 			throw new Error(`More than one thing matches "${rule}"; please specify one of: ` + matches.join(', '));
    // 		}
    // 		if (matches.length < 1) {
    // 			throw new Error(`Nothing matches "${rule}"`);
    // 		}
    // 		return matches[0];
    // 	}
    //
    pub fn validate_ban_rule(&self, rule: &str) -> Result<String, String> {
        let id = ID::new(rule);
        let id_str = id.as_str();

        if id_str == "unreleased" || id_str == "nonexistent" {
            return Ok(id_str.to_string());
        }

        // Check for category prefix
        let categories = ["pokemon", "move", "ability", "item", "nature", "pokemontag"];
        for cat in &categories {
            if rule.starts_with(&format!("{}:", cat)) {
                let rest = &rule[cat.len() + 1..];
                return Ok(format!("{}:{}", cat, ID::new(rest).as_str()));
            }
        }

        // Try to match against all possible types
        // JavaScript: for (const matchType of ['pokemon', 'move', 'ability', 'item', 'nature'])
        let mut matches: Vec<String> = Vec::new();
        // Use Gen 9 dex for validation (latest generation has all Pokemon/moves/etc.)
        let dex = crate::dex::Dex::new(9);

        // Try pokemon
        if let Some(_species) = dex.species().get(id_str) {
            matches.push(format!("pokemon:{}", id_str));
        } else if id_str.ends_with("base") {
            // JavaScript: else if (matchType === 'pokemon' && id.endsWith('base'))
            let base_id = &id_str[..id_str.len() - 4];
            if dex.species().get(base_id).is_some() {
                matches.push(format!("pokemon:{}", base_id));
            }
        }

        // Try move
        if dex.moves().get(id_str).is_some() {
            matches.push(format!("move:{}", id_str));
        }

        // Try ability
        if dex.abilities().get(id_str).is_some() {
            matches.push(format!("ability:{}", id_str));
        }

        // Try item
        if dex.items().get(id_str).is_some() {
            matches.push(format!("item:{}", id_str));
        }

        // Try nature
        if dex.natures().get(id_str).is_some() {
            matches.push(format!("nature:{}", id_str));
        }

        // JavaScript: if (matches.length > 1) throw new Error(...)
        if matches.len() > 1 {
            return Err(format!(
                "More than one thing matches \"{}\"; please specify one of: {}",
                rule,
                matches.join(", ")
            ));
        }

        // JavaScript: if (matches.length < 1) throw new Error(...)
        if matches.is_empty() {
            return Err(format!("Nothing matches \"{}\"", rule));
        }

        // JavaScript: return matches[0];
        Ok(matches[0].clone())
    }
}

impl Default for DexFormats {
    fn default() -> Self {
        Self::new()
    }
}
