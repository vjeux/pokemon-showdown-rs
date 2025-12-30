use crate::*;

impl Dex {

    // =========================================================================
    // Ability-specific methods (from dex-abilities.ts)
    // =========================================================================

    /// Get ability rating
    pub fn get_ability_rating(&self, ability_name: &str) -> Option<f64> {
        self.get_ability(ability_name).and_then(|a| a.rating)
    }
}
