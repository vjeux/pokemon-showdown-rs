//! Format data

use serde::{Deserialize, Serialize};

/// Format data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// JavaScript equivalent: Format (sim/dex-formats.ts)
/// ~20 fields in JavaScript
pub struct FormatData {
    /// Format name
    /// JavaScript: name: string
    pub name: String,
    /// Mod ID
    /// JavaScript: mod?: string
    #[serde(default, rename = "mod")]
    pub mod_id: Option<String>,
    /// Team setting
    /// JavaScript: team?: string
    #[serde(default)]
    pub team: Option<String>,
    /// Game type
    /// JavaScript: gameType?: string
    #[serde(default)]
    pub game_type: Option<String>,
    /// Description
    /// JavaScript: desc?: string
    #[serde(default)]
    pub desc: Option<String>,
    /// Debug mode
    /// JavaScript: debug: boolean
    #[serde(default)]
    pub debug: bool,
    /// Rated format
    /// JavaScript: rated?: boolean | string
    /// TODO: Rust uses Option<serde_json::Value>, JavaScript uses boolean | string union
    #[serde(default)]
    pub rated: Option<serde_json::Value>, // can be bool or string
    /// Search show
    /// JavaScript: searchShow?: boolean
    #[serde(default)]
    pub search_show: Option<bool>,
    /// Challenge show
    /// JavaScript: challengeShow?: boolean
    #[serde(default)]
    pub challenge_show: Option<bool>,
    /// Tournament show
    /// JavaScript: tournamentShow?: boolean
    #[serde(default)]
    pub tournament_show: Option<bool>,
    /// Best of default
    /// JavaScript: bestOfDefault?: boolean
    #[serde(default)]
    pub best_of_default: Option<bool>,
    /// Ruleset
    /// JavaScript: ruleset: string[]
    #[serde(default)]
    pub ruleset: Vec<String>,
    /// Banlist
    /// JavaScript: banlist: string[]
    #[serde(default)]
    pub banlist: Vec<String>,
    /// Restricted list
    /// JavaScript: restricted: string[]
    #[serde(default)]
    pub restricted: Vec<String>,
    /// Unban list
    /// JavaScript: unbanlist: string[]
    #[serde(default)]
    pub unbanlist: Vec<String>,
    /// Custom rules
    /// JavaScript: customRules?: string[]
    #[serde(default)]
    pub custom_rules: Option<Vec<String>>,
}
