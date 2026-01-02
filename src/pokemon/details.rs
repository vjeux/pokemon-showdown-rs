// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Pokemon {

    /// Get details string for protocol
    /// Format: "species, L{level}, {gender}, {shiny}"
    pub fn details(&self) -> String {
        let mut details = self.species_id.as_str().to_string();

        // Add level if not 100
        if self.level != 100 {
            details.push_str(&format!(", L{}", self.level));
        }

        // Add gender if not genderless
        if self.gender != Gender::None {
            details.push_str(&format!(", {}", self.gender.to_str()));
        }

        // ✅ NOW IMPLEMENTED: Add shiny flag if Pokemon is shiny
        if self.shiny {
            details.push_str(", shiny");
        }

        // Note: Terastallization is typically shown separately in protocol, not in details string
        // The details string is for permanent characteristics, while tera is a temporary battle state

        details
    }
}
