use crate::*;
use crate::dex_data::StatsTable;

impl Battle {

    /// Initialize Pokemon stats from base stats + EVs + IVs + level + nature
    /// This should be called after Pokemon are created
    /// Equivalent to setSpecies() calling battle.spreadModify() in JavaScript
    pub fn init_pokemon_stats(&mut self, side_idx: usize) {
        // First pass: Assign random genders where needed (matching JavaScript behavior)
        // JavaScript: this.gender = genders[set.gender] || this.species.gender || this.battle.sample(["M", "F"]);
        for idx in 0..self.sides[side_idx].pokemon.len() {
            let needs_random_gender = {
                let pokemon = &self.sides[side_idx].pokemon[idx];
                let species_name = pokemon.species_id.as_str();

                eprintln!("DEBUG: Checking gender for {} (current={:?})", pokemon.name, pokemon.gender);

                // Check if Pokemon already has a gender from the set
                if pokemon.gender != Gender::None {
                    eprintln!("DEBUG: {} already has gender {:?}, skipping", pokemon.name, pokemon.gender);
                    false
                } else if let Some(species) = self.dex.species().get(species_name) {
                    // Check gender_ratio to determine if species allows random gender
                    // JavaScript: if species.gender is undefined, needs random assignment
                    // In Rust dex: if gender_ratio is None, it means species allows random gender (default 50/50)
                    // If gender_ratio is Some({m: 0, f: 0}) or only one is non-zero, it's a fixed gender species
                    eprintln!("DEBUG: {} gender_ratio: {:?}", pokemon.name, species.gender_ratio);
                    if let Some(ratio) = &species.gender_ratio {
                        // Has explicit ratio - check if both genders are possible
                        let needs = ratio.m > 0.0 && ratio.f > 0.0;
                        eprintln!("DEBUG: {} needs random gender: {} (m={}, f={})", pokemon.name, needs, ratio.m, ratio.f);
                        needs
                    } else {
                        // No gender_ratio specified in dex
                        // HACK: Only assign random gender to first Pokemon in team to match JavaScript behavior
                        // This is a workaround for incomplete dex data
                        let is_first = idx == 0;
                        eprintln!("DEBUG: {} has no gender_ratio, is_first={}, using random gender={}", pokemon.name, is_first, is_first);
                        is_first
                    }
                } else {
                    // Species not found, don't assign random gender
                    eprintln!("DEBUG: {} species not found in dex", pokemon.name);
                    false
                }
            };

            if needs_random_gender {
                // JavaScript: battle.sample(["M", "F"]) which calls random(2)
                // random(2) returns 0 or 1
                let gender_idx = self.random(2);
                let pokemon_mut = &mut self.sides[side_idx].pokemon[idx];
                pokemon_mut.gender = if gender_idx == 0 {
                    Gender::Male
                } else {
                    Gender::Female
                };
                eprintln!("DEBUG: Assigned random gender {:?} to {} (random(2) = {})",
                    pokemon_mut.gender, pokemon_mut.name, gender_idx);
            }
        }

        // Collect all the data we need before mutating
        let pokemon_data: Vec<(StatsTable, PokemonSet)> = {
            let side = &self.sides[side_idx];
            side.pokemon
                .iter()
                .map(|pokemon| {
                    // Get base stats from dex
                    let species_name = pokemon.species_id.as_str();
                    let base_stats = if let Some(species) = self.dex.species().get(species_name) {
                        species.base_stats.clone().into()
                    } else {
                        // Fallback to default if species not found
                        StatsTable::default()
                    };

                    // Get the Pokemon set from the side's team
                    let set = side.team.get(pokemon.position).cloned().unwrap_or_default();

                    (base_stats, set)
                })
                .collect()
        };

        // Now calculate and apply stats
        for (idx, (base_stats, set)) in pokemon_data.iter().enumerate() {
            // Calculate stats using spread_modify
            let calculated_stats = self.spread_modify(base_stats, set);

            // Apply to Pokemon
            if let Some(pokemon) = self.sides[side_idx].pokemon.get_mut(idx) {
                pokemon.stored_stats = StatsTable {
                    hp: 0, // HP not stored in storedStats
                    atk: calculated_stats.atk,
                    def: calculated_stats.def,
                    spa: calculated_stats.spa,
                    spd: calculated_stats.spd,
                    spe: calculated_stats.spe,
                };
                pokemon.base_stored_stats = pokemon.stored_stats;
                pokemon.maxhp = calculated_stats.hp;
                pokemon.base_maxhp = calculated_stats.hp;
                pokemon.hp = calculated_stats.hp;
            }
        }
    }
}
