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
    ///
    /// JavaScript source (dex-conditions.ts getByID):
    /// ```js
    /// } else if (this.dex.data.Moves.hasOwnProperty(id) && (found = this.dex.data.Moves[id]).condition ||
    ///            this.dex.data.Abilities.hasOwnProperty(id) && (found = this.dex.data.Abilities[id]).condition ||
    ///            this.dex.data.Items.hasOwnProperty(id) && (found = this.dex.data.Items[id]).condition) {
    ///     condition = new Condition({ name: found.name || id, ...found.condition });
    /// ```
    pub fn get_by_id(&self, id: &ID) -> Option<&'a ConditionData> {
        // First try direct lookup in conditions
        if let Some(condition) = self.dex.conditions.get(id) {
            return Some(condition);
        }

        // Check if it's a move with an embedded condition
        // JavaScript: this.dex.data.Moves.hasOwnProperty(id) && (found = this.dex.data.Moves[id]).condition
        if let Some(move_data) = self.dex.moves.get(id) {
            if let Some(ref condition) = move_data.condition {
                return Some(condition);
            }
        }

        // TODO: Add ability condition lookup once AbilityData has condition field
        // JavaScript: this.dex.data.Abilities.hasOwnProperty(id) && (found = this.dex.data.Abilities[id]).condition

        // TODO: Add item condition lookup once ItemData has condition field
        // JavaScript: this.dex.data.Items.hasOwnProperty(id) && (found = this.dex.data.Items[id]).condition

        None
    }

    /// Get all conditions data
    /// Equivalent to DexConditions.all() in dex-conditions.ts
    pub fn all(&self) -> Vec<&'a ConditionData> {
        self.dex.conditions.values().collect()
    }
}
