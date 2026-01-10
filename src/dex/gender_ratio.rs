//! Gender ratio structure

use serde::{Deserialize, Serialize};

/// Gender ratio structure
/// Gender ratio (chance of male vs female)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
/// JavaScript equivalent: GenderRatio (sim/dex-species.ts)
/// 2 fields in JavaScript
pub struct GenderRatio {
    /// Probability of male (0.0 to 1.0)
    /// JavaScript: M: number
    #[serde(default)]
    pub m: f64,
    /// Probability of female (0.0 to 1.0)
    /// JavaScript: F: number
    #[serde(default)]
    pub f: f64,
}
