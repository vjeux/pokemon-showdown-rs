use crate::*;

impl Battle {
    #[allow(dead_code)]
    /// Apply a status condition
    pub fn apply_status(&mut self, side_idx: usize, poke_idx: usize, status: &str) {
        if side_idx >= self.sides.len() || poke_idx >= self.sides[side_idx].pokemon.len() {
            return;
        }

        // First check if status can be applied
        {
            let pokemon = &self.sides[side_idx].pokemon[poke_idx];

            // Check if already has status
            if !pokemon.status.is_empty() {
                return;
            }

            // Check type immunities
            let has_type = |t: &str| {
                pokemon
                    .types
                    .iter()
                    .any(|pt| pt.to_lowercase() == t.to_lowercase())
            };

            match status {
                "brn" if has_type("fire") => return,
                "par" if has_type("electric") && self.gen >= 6 => return,
                "psn" | "tox" if has_type("poison") || has_type("steel") => return,
                "frz" if has_type("ice") => return,
                _ => {}
            }
        }

        // Get random duration for sleep before mutating
        let sleep_duration = if status == "slp" {
            Some(1 + self.random(3))
        } else {
            None
        };

        // Now apply the status
        let pokemon = &mut self.sides[side_idx].pokemon[poke_idx];
        pokemon.set_status(ID::new(status));

        // Set duration for sleep/toxic
        match status {
            "slp" => {
                pokemon.status_state.duration = sleep_duration;
            }
            "tox" => {
                pokemon.status_state.duration = Some(1); // Toxic counter starts at 1
            }
            _ => {}
        }

        // Get name for logging
        let name = pokemon.name.clone();
        let side_id = self.sides[side_idx].id_str();
        let full_name = format!("{}: {}", side_id, name);
        self.add_log("-status", &[&full_name, status]);
    }
}
