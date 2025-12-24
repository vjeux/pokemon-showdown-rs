//! Data-driven Format/Ruleset Definitions
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This module contains data-driven definitions for battle formats and rulesets,
//! following the Pokemon Showdown JS architecture.

use std::collections::HashMap;
use once_cell::sync::Lazy;
use crate::dex_data::ID;

/// Game type (singles, doubles, etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormatMod {
    Gen1,
    Gen2,
    Gen3,
    Gen4,
    Gen5,
    Gen6,
    Gen7,
    Gen8,
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

impl Default for FormatMod {
    fn default() -> Self {
        FormatMod::Gen9
    }
}

/// Rule definition
#[derive(Debug, Clone)]
pub struct RuleDef {
    /// Rule name
    pub name: &'static str,
    /// Rule description
    pub desc: &'static str,
    /// Is this a ban rule?
    pub is_ban: bool,
    /// Bans specific things (pokemon, moves, abilities, items)
    pub bans: &'static [&'static str],
    /// Unbans specific things
    pub unbans: &'static [&'static str],
}

/// Format definition
#[derive(Debug, Clone)]
pub struct FormatDef {
    /// Format ID (e.g., "gen9ou")
    pub id: &'static str,
    /// Format name (e.g., "Gen 9 OU")
    pub name: &'static str,
    /// Game type
    pub game_type: GameType,
    /// Generation mod
    pub mod_: FormatMod,
    /// Team size
    pub team_size: usize,
    /// Minimum team size
    pub min_team_size: usize,
    /// Level for Pokemon (usually 100)
    pub max_level: u8,
    /// Default level (usually 100)
    pub default_level: u8,
    /// Rulesets this format inherits from
    pub rulesets: &'static [&'static str],
    /// Banned Pokemon, moves, abilities, items
    pub bans: &'static [&'static str],
    /// Unbanned items (exceptions to ruleset bans)
    pub unbans: &'static [&'static str],
    /// Is this format rated?
    pub rated: bool,
    /// Minimum ELO for this format
    pub min_elo: Option<u32>,
}

/// Standard ruleset definitions
pub static RULESETS: Lazy<HashMap<ID, RuleDef>> = Lazy::new(|| {
    let mut rulesets = HashMap::new();

    rulesets.insert(ID::new("Standard"), RuleDef {
        name: "Standard",
        desc: "Standard rules for competitive play",
        is_ban: false,
        bans: &[],
        unbans: &[],
    });

    rulesets.insert(ID::new("Team Preview"), RuleDef {
        name: "Team Preview",
        desc: "Pokemon are revealed before battle begins",
        is_ban: false,
        bans: &[],
        unbans: &[],
    });

    rulesets.insert(ID::new("Sleep Clause Mod"), RuleDef {
        name: "Sleep Clause Mod",
        desc: "Only one Pokemon on a team can be asleep at a time",
        is_ban: false,
        bans: &[],
        unbans: &[],
    });

    rulesets.insert(ID::new("Species Clause"), RuleDef {
        name: "Species Clause",
        desc: "No two Pokemon on a team can have the same National Dex number",
        is_ban: false,
        bans: &[],
        unbans: &[],
    });

    rulesets.insert(ID::new("Nickname Clause"), RuleDef {
        name: "Nickname Clause",
        desc: "No two Pokemon on a team can have the same nickname",
        is_ban: false,
        bans: &[],
        unbans: &[],
    });

    rulesets.insert(ID::new("OHKO Clause"), RuleDef {
        name: "OHKO Clause",
        desc: "OHKO moves are banned",
        is_ban: true,
        bans: &["Fissure", "Guillotine", "Horn Drill", "Sheer Cold"],
        unbans: &[],
    });

    rulesets.insert(ID::new("Evasion Clause"), RuleDef {
        name: "Evasion Clause",
        desc: "Evasion-boosting moves and abilities are banned",
        is_ban: true,
        bans: &["Double Team", "Minimize"],
        unbans: &[],
    });

    rulesets.insert(ID::new("Moody Clause"), RuleDef {
        name: "Moody Clause",
        desc: "Moody ability is banned",
        is_ban: true,
        bans: &["Moody"],
        unbans: &[],
    });

    rulesets.insert(ID::new("Endless Battle Clause"), RuleDef {
        name: "Endless Battle Clause",
        desc: "Combinations that can cause an endless battle are banned",
        is_ban: false,
        bans: &[],
        unbans: &[],
    });

    rulesets.insert(ID::new("HP Percentage Mod"), RuleDef {
        name: "HP Percentage Mod",
        desc: "HP is shown as a percentage rather than raw HP",
        is_ban: false,
        bans: &[],
        unbans: &[],
    });

    rulesets.insert(ID::new("Cancel Mod"), RuleDef {
        name: "Cancel Mod",
        desc: "Players can cancel their moves before both have chosen",
        is_ban: false,
        bans: &[],
        unbans: &[],
    });

    rulesets.insert(ID::new("Dynamax Clause"), RuleDef {
        name: "Dynamax Clause",
        desc: "Dynamaxing is banned",
        is_ban: true,
        bans: &[],
        unbans: &[],
    });

    rulesets.insert(ID::new("Terastal Clause"), RuleDef {
        name: "Terastal Clause",
        desc: "Terastallization is banned",
        is_ban: true,
        bans: &[],
        unbans: &[],
    });

    rulesets
});

/// Standard format definitions
pub static FORMATS: Lazy<HashMap<ID, FormatDef>> = Lazy::new(|| {
    let mut formats = HashMap::new();

    // Gen 9 formats
    formats.insert(ID::new("gen9ou"), FormatDef {
        id: "gen9ou",
        name: "Gen 9 OU",
        game_type: GameType::Singles,
        mod_: FormatMod::Gen9,
        team_size: 6,
        min_team_size: 1,
        max_level: 100,
        default_level: 100,
        rulesets: &["Standard", "Team Preview", "Sleep Clause Mod", "Species Clause", "Nickname Clause", "OHKO Clause", "Evasion Clause", "Moody Clause", "Endless Battle Clause", "HP Percentage Mod", "Cancel Mod"],
        bans: &[
            // Uber Pokemon
            "Koraidon", "Miraidon", "Mewtwo", "Ho-Oh", "Lugia", "Kyogre", "Groudon", "Rayquaza",
            "Dialga", "Palkia", "Giratina", "Arceus", "Reshiram", "Zekrom", "Kyurem-White", "Kyurem-Black",
            "Xerneas", "Yveltal", "Zygarde-Complete", "Solgaleo", "Lunala", "Necrozma-Dusk-Mane",
            "Necrozma-Dawn-Wings", "Zacian", "Zamazenta", "Eternatus", "Calyrex-Ice", "Calyrex-Shadow",
            // Banned abilities
            "Arena Trap", "Shadow Tag", "Moody",
            // Banned items
            "King's Rock", "Razor Fang",
            // Banned moves
            "Baton Pass", "Last Respects", "Shed Tail",
        ],
        unbans: &[],
        rated: true,
        min_elo: None,
    });

    formats.insert(ID::new("gen9ubers"), FormatDef {
        id: "gen9ubers",
        name: "Gen 9 Ubers",
        game_type: GameType::Singles,
        mod_: FormatMod::Gen9,
        team_size: 6,
        min_team_size: 1,
        max_level: 100,
        default_level: 100,
        rulesets: &["Standard", "Team Preview", "Sleep Clause Mod", "Species Clause", "Nickname Clause", "OHKO Clause", "Evasion Clause", "Endless Battle Clause", "HP Percentage Mod", "Cancel Mod"],
        bans: &["Moody", "Baton Pass"],
        unbans: &[],
        rated: true,
        min_elo: None,
    });

    formats.insert(ID::new("gen9uu"), FormatDef {
        id: "gen9uu",
        name: "Gen 9 UU",
        game_type: GameType::Singles,
        mod_: FormatMod::Gen9,
        team_size: 6,
        min_team_size: 1,
        max_level: 100,
        default_level: 100,
        rulesets: &["Standard", "Team Preview", "Sleep Clause Mod", "Species Clause", "Nickname Clause", "OHKO Clause", "Evasion Clause", "Moody Clause", "Endless Battle Clause", "HP Percentage Mod", "Cancel Mod"],
        bans: &[
            // Everything in OU
            "OU", "UUBL",
            // Specific bans
            "Drizzle", "Drought",
        ],
        unbans: &[],
        rated: true,
        min_elo: None,
    });

    formats.insert(ID::new("gen9doublesou"), FormatDef {
        id: "gen9doublesou",
        name: "Gen 9 Doubles OU",
        game_type: GameType::Doubles,
        mod_: FormatMod::Gen9,
        team_size: 6,
        min_team_size: 1,
        max_level: 100,
        default_level: 100,
        rulesets: &["Standard", "Team Preview", "Species Clause", "Nickname Clause", "OHKO Clause", "Evasion Clause", "Moody Clause", "Endless Battle Clause", "HP Percentage Mod", "Cancel Mod"],
        bans: &["Koraidon", "Miraidon", "Mewtwo", "Kyogre", "Groudon", "Rayquaza", "Arceus", "Calyrex-Shadow"],
        unbans: &[],
        rated: true,
        min_elo: None,
    });

    formats.insert(ID::new("gen9vgc2024"), FormatDef {
        id: "gen9vgc2024",
        name: "Gen 9 VGC 2024",
        game_type: GameType::Doubles,
        mod_: FormatMod::Gen9,
        team_size: 6,
        min_team_size: 4,
        max_level: 50,
        default_level: 50,
        rulesets: &["Standard", "Team Preview", "Species Clause", "Nickname Clause", "HP Percentage Mod"],
        bans: &["Mewtwo", "Mew", "Lugia", "Ho-Oh", "Celebi", "Kyogre", "Groudon", "Rayquaza", "Jirachi", "Deoxys"],
        unbans: &[],
        rated: true,
        min_elo: None,
    });

    formats.insert(ID::new("gen9randombattle"), FormatDef {
        id: "gen9randombattle",
        name: "Gen 9 Random Battle",
        game_type: GameType::Singles,
        mod_: FormatMod::Gen9,
        team_size: 6,
        min_team_size: 6,
        max_level: 100,
        default_level: 100,
        rulesets: &["HP Percentage Mod", "Cancel Mod"],
        bans: &[],
        unbans: &[],
        rated: true,
        min_elo: None,
    });

    // Gen 8 formats
    formats.insert(ID::new("gen8ou"), FormatDef {
        id: "gen8ou",
        name: "Gen 8 OU",
        game_type: GameType::Singles,
        mod_: FormatMod::Gen8,
        team_size: 6,
        min_team_size: 1,
        max_level: 100,
        default_level: 100,
        rulesets: &["Standard", "Team Preview", "Sleep Clause Mod", "Species Clause", "Nickname Clause", "OHKO Clause", "Evasion Clause", "Moody Clause", "Dynamax Clause", "Endless Battle Clause", "HP Percentage Mod", "Cancel Mod"],
        bans: &[
            "Uber", "Arena Trap", "Power Construct", "Shadow Tag", "Baton Pass",
        ],
        unbans: &[],
        rated: true,
        min_elo: None,
    });

    // Gen 7 formats
    formats.insert(ID::new("gen7ou"), FormatDef {
        id: "gen7ou",
        name: "Gen 7 OU",
        game_type: GameType::Singles,
        mod_: FormatMod::Gen7,
        team_size: 6,
        min_team_size: 1,
        max_level: 100,
        default_level: 100,
        rulesets: &["Standard", "Team Preview", "Sleep Clause Mod", "Species Clause", "Nickname Clause", "OHKO Clause", "Evasion Clause", "Moody Clause", "Endless Battle Clause", "HP Percentage Mod", "Cancel Mod"],
        bans: &["Uber", "Arena Trap", "Power Construct", "Shadow Tag", "Baton Pass"],
        unbans: &[],
        rated: true,
        min_elo: None,
    });

    // Gen 1 formats
    formats.insert(ID::new("gen1ou"), FormatDef {
        id: "gen1ou",
        name: "Gen 1 OU",
        game_type: GameType::Singles,
        mod_: FormatMod::Gen1,
        team_size: 6,
        min_team_size: 1,
        max_level: 100,
        default_level: 100,
        rulesets: &["Standard", "Sleep Clause Mod", "Species Clause", "Nickname Clause", "OHKO Clause", "Evasion Clause", "Endless Battle Clause"],
        bans: &["Uber", "Baton Pass"],
        unbans: &[],
        rated: true,
        min_elo: None,
    });

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
    FORMATS.values()
        .filter(|f| f.mod_ == gen)
        .collect()
}

/// Get all singles formats
pub fn get_singles_formats() -> Vec<&'static FormatDef> {
    FORMATS.values()
        .filter(|f| f.game_type == GameType::Singles)
        .collect()
}

/// Get all doubles formats
pub fn get_doubles_formats() -> Vec<&'static FormatDef> {
    FORMATS.values()
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
pub type ComplexBan = (String, String, u32, Vec<String>);

/// Timer settings for battle
#[derive(Debug, Clone, Default)]
pub struct GameTimerSettings {
    pub dc_timer: bool,
    pub dc_timer_bank: bool,
    pub starting: u32,
    pub grace: u32,
    pub add_per_turn: u32,
    pub max_per_turn: u32,
    pub max_first_turn: u32,
    pub timeout_auto_choose: bool,
    pub accelerate: bool,
}

/// RuleTable - tracks rules in effect for a format
/// The key can be:
/// - '[ruleid]' the ID of a rule in effect
/// - '-[thing]' or '-[category]:[thing]' ban a thing
/// - '+[thing]' or '+[category]:[thing]' allow a thing (override a ban)
#[derive(Debug, Clone, Default)]
pub struct RuleTable {
    /// Map of rule ID to source format name
    rules: HashMap<String, String>,
    /// Complex bans (rule, source, limit, bans)
    pub complex_bans: Vec<ComplexBan>,
    /// Complex team bans
    pub complex_team_bans: Vec<ComplexBan>,
    /// Tag rules (pokemontag rules)
    pub tag_rules: Vec<String>,
    /// Value rules (rules with = values)
    pub value_rules: HashMap<String, String>,
    /// Timer settings
    pub timer: Option<GameTimerSettings>,

    // Resolved numeric properties
    pub min_team_size: usize,
    pub max_team_size: usize,
    pub picked_team_size: Option<usize>,
    pub max_total_level: Option<u32>,
    pub max_move_count: usize,
    pub min_source_gen: u8,
    pub min_level: u8,
    pub max_level: u8,
    pub default_level: u8,
    pub adjust_level: Option<u8>,
    pub adjust_level_down: Option<u8>,
    pub ev_limit: Option<u32>,
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
    pub fn is_banned(&self, thing: &str) -> bool {
        if self.has(&format!("+{}", thing)) {
            return false;
        }
        self.has(&format!("-{}", thing))
    }

    /// Check if a species is banned
    /// Equivalent to isBannedSpecies()
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
    pub fn is_restricted(&self, thing: &str) -> bool {
        if self.has(&format!("+{}", thing)) {
            return false;
        }
        self.has(&format!("*{}", thing))
    }

    /// Check if a species is restricted
    /// Equivalent to isRestrictedSpecies()
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
    pub fn get_tag_rules(&mut self) -> &Vec<String> {
        let mut tag_rules = Vec::new();
        for ruleid in self.rules.keys() {
            if ruleid.starts_with("+pokemontag:") || ruleid.starts_with("*pokemontag:") || ruleid.starts_with("-pokemontag:") {
                let banid = &ruleid[12..];
                if banid != "allpokemon" && banid != "allitems" && banid != "allmoves"
                    && banid != "allabilities" && banid != "allnatures" {
                    tag_rules.push(ruleid.clone());
                }
            } else if ruleid.len() > 1 && "+*-".contains(&ruleid[..1]) && &ruleid[1..] == "nonexistent" {
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
    pub fn check(&self, thing: &str) -> Option<String> {
        if self.has(&format!("+{}", thing)) {
            return Some(String::new()); // whitelisted
        }
        self.get_reason(&format!("-{}", thing))
    }

    /// Get reason for a rule
    /// Equivalent to getReason()
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
    pub fn blame(&self, key: &str) -> String {
        match self.rules.get(key) {
            Some(source) if !source.is_empty() => format!(" from {}", source),
            _ => String::new(),
        }
    }

    /// Get index of a complex ban
    /// Equivalent to getComplexBanIndex()
    pub fn get_complex_ban_index(&self, complex_bans: &[ComplexBan], rule: &str) -> Option<usize> {
        let rule_id = ID::new(rule);
        complex_bans.iter().position(|(r, _, _, _)| ID::new(r) == rule_id)
    }

    /// Add a complex ban
    /// Equivalent to addComplexBan()
    pub fn add_complex_ban(&mut self, rule: &str, source: &str, limit: u32, bans: Vec<String>) {
        if let Some(idx) = self.get_complex_ban_index(&self.complex_bans, rule) {
            if self.complex_bans[idx].2 == u32::MAX {
                return; // Infinity
            }
            self.complex_bans[idx] = (rule.to_string(), source.to_string(), limit, bans);
        } else {
            self.complex_bans.push((rule.to_string(), source.to_string(), limit, bans));
        }
    }

    /// Add a complex team ban
    /// Equivalent to addComplexTeamBan()
    pub fn add_complex_team_ban(&mut self, rule: &str, source: &str, limit: u32, bans: Vec<String>) {
        if let Some(idx) = self.get_complex_ban_index(&self.complex_team_bans, rule) {
            if self.complex_team_bans[idx].2 == u32::MAX {
                return;
            }
            self.complex_team_bans[idx] = (rule.to_string(), source.to_string(), limit, bans);
        } else {
            self.complex_team_bans.push((rule.to_string(), source.to_string(), limit, bans));
        }
    }

    /// Resolve numeric properties
    /// Equivalent to resolveNumbers()
    pub fn resolve_numbers(&mut self, format: &FormatDef) {
        let game_type_min_team_size = match format.game_type {
            GameType::Triples => 3,
            GameType::Doubles => 2,
            _ => 1,
        };

        self.min_team_size = self.value_rules.get("minteamsize")
            .and_then(|v| v.parse().ok())
            .unwrap_or(0);
        self.max_team_size = self.value_rules.get("maxteamsize")
            .and_then(|v| v.parse().ok())
            .unwrap_or(6);
        self.picked_team_size = self.value_rules.get("pickedteamsize")
            .and_then(|v| v.parse().ok());
        self.max_total_level = self.value_rules.get("maxtotallevel")
            .and_then(|v| v.parse().ok());
        self.max_move_count = self.value_rules.get("maxmovecount")
            .and_then(|v| v.parse().ok())
            .unwrap_or(4);
        self.min_source_gen = self.value_rules.get("minsourcegen")
            .and_then(|v| v.parse().ok())
            .unwrap_or(1);
        self.min_level = self.value_rules.get("minlevel")
            .and_then(|v| v.parse().ok())
            .unwrap_or(1);
        self.max_level = self.value_rules.get("maxlevel")
            .and_then(|v| v.parse().ok())
            .unwrap_or(100);
        self.default_level = self.value_rules.get("defaultlevel")
            .and_then(|v| v.parse().ok())
            .unwrap_or(0);
        self.adjust_level = self.value_rules.get("adjustlevel")
            .and_then(|v| v.parse().ok());
        self.adjust_level_down = self.value_rules.get("adjustleveldown")
            .and_then(|v| v.parse().ok());
        self.ev_limit = self.value_rules.get("evlimit")
            .and_then(|v| v.parse().ok());

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
    pub fn has_complex_bans(&self) -> bool {
        !self.complex_bans.is_empty() || !self.complex_team_bans.is_empty()
    }
}

// =========================================================================
// DEXFORMATS - Equivalent to dex-formats.ts DexFormats class
// =========================================================================

/// Format class - runtime format object
#[derive(Debug, Clone)]
pub struct Format {
    pub id: String,
    pub name: String,
    pub mod_: String,
    pub effect_type: String,
    pub debug: bool,
    pub rated: bool,
    pub game_type: GameType,
    pub player_count: u8,
    pub ruleset: Vec<String>,
    pub base_ruleset: Vec<String>,
    pub banlist: Vec<String>,
    pub restricted: Vec<String>,
    pub unbanlist: Vec<String>,
    pub custom_rules: Option<Vec<String>>,
    pub rule_table: Option<RuleTable>,
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
            player_count: if matches!(def.game_type, GameType::Multi | GameType::FreeForAll) { 4 } else { 2 },
            ruleset: def.rulesets.iter().map(|s| s.to_string()).collect(),
            base_ruleset: def.rulesets.iter().map(|s| s.to_string()).collect(),
            banlist: def.bans.iter().map(|s| s.to_string()).collect(),
            restricted: Vec::new(),
            unbanlist: def.unbans.iter().map(|s| s.to_string()).collect(),
            custom_rules: None,
            rule_table: None,
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
            exists: false,
        }
    }
}

/// DexFormats - manages format loading and validation
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
        let custom_rules: Vec<String> = parts[1]
            .split(',')
            .map(|r| r.trim().to_string())
            .collect();

        Ok(format!("{}@@@{}", format.id, custom_rules.join(",")))
    }

    /// Get a format by name
    /// Equivalent to get()
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
    pub fn all(&mut self) -> &Vec<Format> {
        self.load();
        &self.formats_list
    }

    /// Check if a rule spec is for Pokemon
    /// Equivalent to isPokemonRule()
    pub fn is_pokemon_rule(rule_spec: &str) -> bool {
        let rest = if rule_spec.len() > 1 { &rule_spec[1..] } else { "" };
        rest.starts_with("pokemontag:") || rest.starts_with("pokemon:") || rest.starts_with("basepokemon:")
    }

    /// Get rule table for a format
    /// Equivalent to getRuleTable()
    pub fn get_rule_table(&self, format: &Format) -> RuleTable {
        let mut rule_table = RuleTable::new();

        // Add rules from ruleset
        for rule in &format.ruleset {
            rule_table.set(rule, "");
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
                self.validate_ban_rule(rest).map(|r| format!("{}{}", first_char, r))
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
    pub fn valid_pokemon_tag(&self, _tagid: &str) -> bool {
        // Simplified - would check against Tags data
        true
    }

    /// Validate a ban rule
    /// Equivalent to validateBanRule()
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

        // Assume it's a pokemon for now (simplified)
        Ok(format!("pokemon:{}", id_str))
    }
}

impl Default for DexFormats {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen9ou() {
        let format = get_format_by_name("gen9ou").unwrap();
        assert_eq!(format.name, "Gen 9 OU");
        assert_eq!(format.game_type, GameType::Singles);
        assert_eq!(format.mod_, FormatMod::Gen9);
        assert_eq!(format.team_size, 6);
        assert!(format.bans.contains(&"Koraidon"));
        assert!(format.bans.contains(&"Baton Pass"));
    }

    #[test]
    fn test_vgc() {
        let format = get_format_by_name("gen9vgc2024").unwrap();
        assert_eq!(format.game_type, GameType::Doubles);
        assert_eq!(format.max_level, 50);
        assert_eq!(format.min_team_size, 4);
    }

    #[test]
    fn test_game_type_active() {
        assert_eq!(GameType::Singles.active_per_half(), 1);
        assert_eq!(GameType::Doubles.active_per_half(), 2);
        assert_eq!(GameType::Triples.active_per_half(), 3);
    }

    #[test]
    fn test_format_gen() {
        let gen9_formats = get_formats_by_gen(FormatMod::Gen9);
        assert!(gen9_formats.iter().any(|f| f.id == "gen9ou"));
        assert!(gen9_formats.iter().any(|f| f.id == "gen9doublesou"));
    }

    #[test]
    fn test_is_banned() {
        let format_id = ID::new("gen9ou");
        assert!(is_banned_in_format(&format_id, "Koraidon"));
        assert!(is_banned_in_format(&format_id, "Shadow Tag"));
        assert!(!is_banned_in_format(&format_id, "Pikachu"));
    }

    #[test]
    fn test_ruleset_ohko() {
        let ruleset = get_ruleset(&ID::new("ohkoclause")).unwrap();
        assert!(ruleset.is_ban);
        assert!(ruleset.bans.contains(&"Fissure"));
        assert!(ruleset.bans.contains(&"Sheer Cold"));
    }

    #[test]
    fn test_singles_doubles() {
        let singles = get_singles_formats();
        let doubles = get_doubles_formats();

        assert!(singles.iter().any(|f| f.id == "gen9ou"));
        assert!(doubles.iter().any(|f| f.id == "gen9doublesou"));
    }

    #[test]
    fn test_format_mod() {
        assert_eq!(FormatMod::Gen9.number(), 9);
        assert_eq!(FormatMod::Gen1.number(), 1);
        assert_eq!(FormatMod::from_number(9), Some(FormatMod::Gen9));
        assert_eq!(FormatMod::from_number(10), None);
    }
}
