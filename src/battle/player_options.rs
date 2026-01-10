//! Player Options

use serde::{Deserialize, Serialize};
use super::TeamFormat;

/// Player/side creation options
#[derive(Debug, Clone, Serialize, Deserialize)]
/// JavaScript equivalent: PlayerOptions (sim/global-types.ts)
/// 5 fields in JavaScript
pub struct PlayerOptions {
    /// Player name
    /// JavaScript: name: string
    pub name: String,
    /// Player's team
    /// JavaScript: team: PokemonSet[] | string
    /// Rust: Uses TeamFormat enum to represent union type
    pub team: TeamFormat,
    /// Player avatar
    /// JavaScript: avatar?: string
    pub avatar: Option<String>,
    /// Player rating
    /// JavaScript: rating?: number | string
    pub rating: Option<String>,
    /// RNG seed for team generation
    /// JavaScript: seed?: PRNGSeed
    /// PRNGSeed is a string like "sodium,abc123" or "gen5,xyz" or "1234,abc"
    pub seed: Option<String>,
}
