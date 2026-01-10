//! Battle Options

use crate::dex_data::{GameType, ID};
use crate::prng::PRNGSeed;
use super::PlayerOptions;

/// Battle creation options
#[derive(Debug, Clone, Default)]
/// JavaScript equivalent: BattleOptions (sim/battle.ts)
/// 14 fields in JavaScript
pub struct BattleOptions {
    /// Format ID
    /// JavaScript: formatid: ID
    pub format_id: ID,
    /// Format name
    /// JavaScript: format?: string
    pub format_name: Option<String>,
    /// Game type (singles, doubles, etc.)
    /// JavaScript: gameType?: GameType
    pub game_type: Option<GameType>,
    /// PRNG seed
    /// JavaScript: seed?: PRNGSeed
    pub seed: Option<PRNGSeed>,
    /// Rated match
    /// JavaScript: rated?: boolean | string
    pub rated: Option<String>,
    /// Debug mode
    /// JavaScript: debug?: boolean
    pub debug: bool,
    /// Strict choice validation
    /// JavaScript: strictChoices?: boolean
    pub strict_choices: bool,
    /// Force random chance outcome
    /// JavaScript: forceRandomChance?: boolean
    pub force_random_chance: Option<bool>,
    /// Player 1 options
    /// JavaScript: p1?: PlayerOptions
    pub p1: Option<PlayerOptions>,
    /// Player 2 options
    /// JavaScript: p2?: PlayerOptions
    pub p2: Option<PlayerOptions>,
    /// Player 3 options (multi battles)
    /// JavaScript: p3?: PlayerOptions
    pub p3: Option<PlayerOptions>,
    /// Player 4 options (multi battles)
    /// JavaScript: p4?: PlayerOptions
    pub p4: Option<PlayerOptions>,
}
