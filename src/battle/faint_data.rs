//! Faint Data

use serde::{Deserialize, Serialize};
use super::Effect;

/// Faint queue entry data
#[derive(Debug, Clone, Serialize, Deserialize)]
/// JavaScript equivalent: FaintQueue entry (sim/battle.ts)
/// 3 fields in JavaScript
pub struct FaintData {
    /// Pokemon that fainted (side_idx, poke_idx)
    /// JavaScript: target (Pokemon reference)
    /// TODO: Rust uses indices instead of Pokemon reference due to ownership
    pub target: (usize, usize),
    /// Source of the faint (side_idx, poke_idx) or None
    /// JavaScript: source (Pokemon reference or null)
    /// TODO: Rust uses indices instead of Pokemon reference due to ownership
    pub source: Option<(usize, usize)>,
    /// Effect that caused the faint
    /// JavaScript: effect (Effect object or null)
    pub effect: Option<Effect>,
}
