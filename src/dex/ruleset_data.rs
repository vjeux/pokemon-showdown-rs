//! Ruleset data

use serde::{Deserialize, Serialize};

/// Ruleset data
#[derive(Debug, Clone, Serialize, Deserialize)]
/// JavaScript equivalent: RulesetData (sim/dex-formats.ts)
/// 2 fields in JavaScript
pub struct RulesetData {
    /// Ruleset name
    /// JavaScript: name: string
    pub name: String,
    /// Mod ID (optional)
    /// JavaScript: mod?: string
    #[serde(default, rename = "mod")]
    pub mod_id: Option<String>,
}
