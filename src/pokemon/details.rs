use crate::*;

impl Pokemon {

    /// Get details string for protocol
    pub fn details(&self) -> String {
        let mut details = self.species_id.as_str().to_string();
        if self.level != 100 {
            details.push_str(&format!(", L{}", self.level));
        }
        if self.gender != Gender::None {
            details.push_str(&format!(", {}", self.gender.to_str()));
        }
        // Could add shiny, tera state, etc.
        details
    }
}
