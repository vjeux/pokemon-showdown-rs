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
