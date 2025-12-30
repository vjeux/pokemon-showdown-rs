use crate::*;

impl Dex {

    /// Check if a species is banned in a format
    /// Simplified version - full implementation would need RuleTable
    pub fn is_species_banned(&self, _species_name: &str, _format: &str) -> bool {
        // Stub - would need format rules to implement
        false
    }
}
