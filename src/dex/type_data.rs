//! Type effectiveness data

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Type effectiveness data
#[derive(Debug, Clone, Serialize, Deserialize)]
/// JavaScript equivalent: TypeData (sim/dex-data.ts)
/// ~4 fields in JavaScript
pub struct TypeData {
    /// Type matchup chart (opponent type -> effectiveness)
    /// JavaScript: damageTaken: { [attackingType: string]: number }
    #[serde(rename = "damageTaken")]
    pub damage_taken: HashMap<String, u8>,
}
