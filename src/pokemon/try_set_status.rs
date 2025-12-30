use crate::*;

impl Pokemon {

    /// Try to set status with immunity checks
    /// Equivalent to trySetStatus in pokemon.ts
    //
    // 	trySetStatus(status: string | Condition, source: Pokemon | null = null, sourceEffect: Effect | null = null) {
    // 		return this.setStatus(this.status || status, source, sourceEffect);
    // 	}
    //
    pub fn try_set_status(&mut self, status_id: ID, _source_effect: Option<&str>) -> bool {
        // Check if already has status
        if !self.status.is_empty() {
            return false;
        }

        // Check for type-based immunities
        let status_str = status_id.as_str();
        match status_str {
            "brn" => {
                // Fire types immune to burn
                if self.has_type("fire") {
                    return false;
                }
            }
            "par" => {
                // Electric types immune to paralysis (Gen 6+)
                if self.has_type("electric") {
                    return false;
                }
            }
            "psn" | "tox" => {
                // Poison and Steel types immune to poison
                if self.has_type("poison") || self.has_type("steel") {
                    return false;
                }
            }
            "frz" => {
                // Ice types immune to freeze
                if self.has_type("ice") {
                    return false;
                }
            }
            _ => {}
        }

        self.set_status(status_id)
    }
}
