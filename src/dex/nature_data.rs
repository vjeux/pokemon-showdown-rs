//! Nature data

use serde::{Deserialize, Serialize};

/// Nature data
#[derive(Debug, Clone, Serialize, Deserialize)]
/// JavaScript equivalent: NatureData (sim/dex-data.ts)
/// 3 fields in JavaScript
pub struct NatureData {
    /// Nature name
    /// JavaScript: name: string
    pub name: String,
    /// Stat that gets a 1.1x boost
    /// JavaScript: plus?: StatIDExceptHP
    #[serde(default)]
    pub plus: Option<String>,
    /// Stat that gets a 0.9x penalty
    /// JavaScript: minus?: StatIDExceptHP
    #[serde(default)]
    pub minus: Option<String>,
}
