//! DexConditions - Conditions lookup helper
//!
//! Equivalent to DexConditions class in dex-conditions.ts

use crate::dex::{Dex, ConditionData};
use crate::dex_data::ID;

/// Helper struct for conditions lookups
/// JavaScript equivalent: DexConditions (sim/dex-conditions.ts)
/// 1 field in JavaScript (dex)
pub struct DexConditions<'a> {
    /// Dex reference
    /// JavaScript: readonly dex: ModdedDex
    pub(crate) dex: &'a Dex,
}

impl<'a> DexConditions<'a> {
    /// Get condition data by name or ID
    /// Equivalent to DexConditions.get() in dex-conditions.ts
    pub fn get(&self, name: &str) -> Option<&'a ConditionData> {
        let id = ID::new(name);
        // Try direct lookup first
        if let Some(condition) = self.dex.conditions.get(&id) {
            return Some(condition);
        }
        // Try alias lookup
        if let Some(canonical_name) = self.dex.aliases.get(&id) {
            let canonical_id = ID::new(canonical_name);
            return self.dex.conditions.get(&canonical_id);
        }
        None
    }

    /// Get condition by ID
    /// Equivalent to DexConditions.getByID() in dex-conditions.ts
    pub fn get_by_id(&self, id: &ID) -> Option<&'a ConditionData> {
        self.dex.conditions.get(id)
    }

    /// Get all conditions data
    /// Equivalent to DexConditions.all() in dex-conditions.ts
    pub fn all(&self) -> Vec<&'a ConditionData> {
        self.dex.conditions.values().collect()
    }
}
