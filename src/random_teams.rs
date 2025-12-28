//! Random Team Generator
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This module generates random teams for Random Battle format.

use crate::dex_data::ID;
use crate::prng::PRNG;
use crate::data::species::{SpeciesDef, SPECIES};
use crate::team_validator::{ValidatorSet, EVSpread, IVSpread};

/// Random team generator
pub struct RandomTeamGenerator {
    prng: PRNG,
    /// Format ID (not yet implemented)
    #[allow(dead_code)]
    format_id: ID,
}

impl RandomTeamGenerator {
    /// Create a new random team generator
    pub fn new(format: &str) -> Self {
        Self {
            prng: PRNG::new(None),
            format_id: ID::new(format),
        }
    }

    /// Create with a specific seed for reproducibility
    pub fn with_seed(format: &str, seed: crate::prng::PRNGSeed) -> Self {
        Self {
            prng: PRNG::new(Some(seed)),
            format_id: ID::new(format),
        }
    }

    /// Generate a random team of 6 Pokemon
    pub fn generate_team(&mut self) -> Vec<ValidatorSet> {
        let mut team = Vec::new();
        let mut used_species: Vec<String> = Vec::new();

        for _ in 0..6 {
            if let Some(pokemon) = self.generate_pokemon(&used_species) {
                used_species.push(pokemon.species.clone());
                team.push(pokemon);
            }
        }

        team
    }

    /// Generate a single random Pokemon
    fn generate_pokemon(&mut self, excluded: &[String]) -> Option<ValidatorSet> {
        // Get all available species
        let available: Vec<_> = SPECIES.iter()
            .filter(|(id, spec)| {
                // Filter out already used species
                !excluded.iter().any(|e| ID::new(e) == **id)
                // Filter out NFE (Not Fully Evolved) for random battles
                && spec.evos.is_empty()
                // Filter out Uber tier for standard random battles
                && spec.tier != "Uber" && spec.tier != "AG"
            })
            .collect();

        if available.is_empty() {
            return None;
        }

        // Pick a random species
        let idx = self.prng.random_int(available.len() as i32) as usize;
        let (_, species) = available[idx];

        Some(self.build_set(species))
    }

    /// Build a random set for a species
    fn build_set(&mut self, species: &SpeciesDef) -> ValidatorSet {
        let mut set = ValidatorSet::default();

        set.species = species.name.to_string();
        set.level = self.get_level(species);
        set.ability = self.pick_ability(species);
        set.item = Some(self.pick_item(species));
        set.nature = Some(self.pick_nature(species));
        set.moves = self.pick_moves(species);
        set.evs = self.generate_evs(species);
        set.ivs = IVSpread::default(); // Perfect IVs

        set
    }

    /// Get level for random battles (based on BST)
    fn get_level(&self, species: &SpeciesDef) -> u8 {
        let bst = species.base_stats.total();

        // Higher BST Pokemon get lower levels
        if bst >= 600 {
            75
        } else if bst >= 550 {
            80
        } else if bst >= 500 {
            85
        } else if bst >= 450 {
            90
        } else if bst >= 400 {
            95
        } else {
            100
        }
    }

    /// Pick an ability for a species
    fn pick_ability(&mut self, species: &SpeciesDef) -> String {
        let mut abilities = vec![species.abilities.0.to_string()];

        if let Some(ability1) = species.abilities.1 {
            abilities.push(ability1.to_string());
        }

        // 30% chance for hidden ability if available
        if let Some(hidden) = species.abilities.2 {
            if self.prng.random_int(100) < 30 {
                return hidden.to_string();
            }
        }

        let idx = self.prng.random_int(abilities.len() as i32) as usize;
        abilities[idx].clone()
    }

    /// Pick an item for a species
    fn pick_item(&mut self, species: &SpeciesDef) -> String {
        // Common competitive items
        let items = [
            "Leftovers",
            "Life Orb",
            "Choice Band",
            "Choice Specs",
            "Choice Scarf",
            "Focus Sash",
            "Rocky Helmet",
            "Assault Vest",
            "Heavy-Duty Boots",
            "Black Sludge",
            "Sitrus Berry",
            "Lum Berry",
        ];

        // Pick based on Pokemon's stats
        let _bst = species.base_stats.total();
        let atk = species.base_stats.atk;
        let spa = species.base_stats.spa;
        let spe = species.base_stats.spe;

        // Poison types can use Black Sludge
        if species.has_type("Poison") && self.prng.random_int(100) < 50 {
            return "Black Sludge".to_string();
        }

        // Fast frail Pokemon often use Focus Sash
        if spe > 100 && species.base_stats.hp < 70 && self.prng.random_int(100) < 30 {
            return "Focus Sash".to_string();
        }

        // Physical attackers
        if atk > spa {
            if spe < 80 && self.prng.random_int(100) < 40 {
                return "Choice Band".to_string();
            }
            if self.prng.random_int(100) < 30 {
                return "Life Orb".to_string();
            }
        }

        // Special attackers
        if spa > atk {
            if spe < 80 && self.prng.random_int(100) < 40 {
                return "Choice Specs".to_string();
            }
            if self.prng.random_int(100) < 30 {
                return "Life Orb".to_string();
            }
        }

        // Bulky Pokemon
        if (species.base_stats.hp > 80 || species.base_stats.def > 100 || species.base_stats.spd > 100)
            && self.prng.random_int(100) < 50 {
                return "Leftovers".to_string();
            }

        // Default: random from common items
        let idx = self.prng.random_int(items.len() as i32) as usize;
        items[idx].to_string()
    }

    /// Pick a nature for a species
    fn pick_nature(&mut self, species: &SpeciesDef) -> String {
        let atk = species.base_stats.atk;
        let spa = species.base_stats.spa;
        let spe = species.base_stats.spe;

        // Physical attackers
        if atk > spa {
            if spe > 90 {
                return "Jolly".to_string(); // +Spe -SpA
            } else {
                return "Adamant".to_string(); // +Atk -SpA
            }
        }

        // Special attackers
        if spa > atk {
            if spe > 90 {
                return "Timid".to_string(); // +Spe -Atk
            } else {
                return "Modest".to_string(); // +SpA -Atk
            }
        }

        // Balanced/Defensive
        if species.base_stats.def > species.base_stats.spd {
            "Bold".to_string()// +Def -Atk
        } else {
            "Calm".to_string()// +SpD -Atk
        }
    }

    /// Pick moves for a species
    fn pick_moves(&mut self, species: &SpeciesDef) -> Vec<String> {
        let mut moves = Vec::new();

        // Get species' types for STAB
        let types = species.types;

        // Common attacking moves by type
        let type_moves: Vec<(&str, &[&str])> = vec![
            ("Normal", &["Body Slam", "Return", "Facade", "Hyper Voice"]),
            ("Fire", &["Flamethrower", "Fire Blast", "Flare Blitz", "Heat Wave"]),
            ("Water", &["Surf", "Hydro Pump", "Scald", "Waterfall"]),
            ("Electric", &["Thunderbolt", "Thunder", "Volt Switch", "Wild Charge"]),
            ("Grass", &["Energy Ball", "Leaf Storm", "Giga Drain", "Power Whip"]),
            ("Ice", &["Ice Beam", "Blizzard", "Ice Punch", "Icicle Crash"]),
            ("Fighting", &["Close Combat", "Aura Sphere", "Drain Punch", "High Jump Kick"]),
            ("Poison", &["Sludge Bomb", "Sludge Wave", "Gunk Shot", "Poison Jab"]),
            ("Ground", &["Earthquake", "Earth Power", "Drill Run", "Stomping Tantrum"]),
            ("Flying", &["Hurricane", "Brave Bird", "Air Slash", "Dual Wingbeat"]),
            ("Psychic", &["Psychic", "Psyshock", "Zen Headbutt", "Expanding Force"]),
            ("Bug", &["Bug Buzz", "X-Scissor", "U-turn", "Megahorn"]),
            ("Rock", &["Stone Edge", "Rock Slide", "Power Gem", "Head Smash"]),
            ("Ghost", &["Shadow Ball", "Shadow Claw", "Phantom Force", "Poltergeist"]),
            ("Dragon", &["Draco Meteor", "Dragon Pulse", "Outrage", "Dragon Claw"]),
            ("Dark", &["Dark Pulse", "Knock Off", "Crunch", "Sucker Punch"]),
            ("Steel", &["Flash Cannon", "Iron Head", "Steel Beam", "Heavy Slam"]),
            ("Fairy", &["Moonblast", "Dazzling Gleam", "Play Rough", "Draining Kiss"]),
        ];

        // Add STAB moves
        for type_name in types {
            if let Some((_, move_list)) = type_moves.iter().find(|(t, _)| t == type_name) {
                let idx = self.prng.random_int(move_list.len() as i32) as usize;
                let move_name = move_list[idx];
                if !moves.contains(&move_name.to_string()) {
                    moves.push(move_name.to_string());
                }
            }
        }

        // Add coverage moves
        let coverage = [
            "Earthquake", "Ice Beam", "Thunderbolt", "Flamethrower",
            "Shadow Ball", "Psychic", "Dark Pulse", "Focus Blast",
        ];

        while moves.len() < 3 {
            let idx = self.prng.random_int(coverage.len() as i32) as usize;
            let move_name = coverage[idx];
            if !moves.contains(&move_name.to_string()) {
                moves.push(move_name.to_string());
            }
        }

        // Add a utility/status move
        let utility = [
            "Protect", "Substitute", "Toxic", "Will-O-Wisp",
            "Thunder Wave", "Stealth Rock", "Roost", "Recover",
            "Swords Dance", "Nasty Plot", "Calm Mind", "Dragon Dance",
        ];

        if moves.len() < 4 {
            let idx = self.prng.random_int(utility.len() as i32) as usize;
            moves.push(utility[idx].to_string());
        }

        // Ensure we have exactly 4 moves
        while moves.len() < 4 {
            let idx = self.prng.random_int(coverage.len() as i32) as usize;
            let move_name = coverage[idx];
            if !moves.contains(&move_name.to_string()) {
                moves.push(move_name.to_string());
            }
        }

        moves.truncate(4);
        moves
    }

    /// Generate EVs for a species
    fn generate_evs(&mut self, species: &SpeciesDef) -> EVSpread {
        let atk = species.base_stats.atk;
        let spa = species.base_stats.spa;
        let _spe = species.base_stats.spe;

        // Physical attacker spread
        if atk > spa && atk > 80 {
            return EVSpread {
                hp: 4,
                atk: 252,
                def: 0,
                spa: 0,
                spd: 0,
                spe: 252,
            };
        }

        // Special attacker spread
        if spa > atk && spa > 80 {
            return EVSpread {
                hp: 4,
                atk: 0,
                def: 0,
                spa: 252,
                spd: 0,
                spe: 252,
            };
        }

        // Bulky spread
        if species.base_stats.hp > 80 {
            if species.base_stats.def > species.base_stats.spd {
                return EVSpread {
                    hp: 252,
                    atk: 0,
                    def: 252,
                    spa: 0,
                    spd: 4,
                    spe: 0,
                };
            } else {
                return EVSpread {
                    hp: 252,
                    atk: 0,
                    def: 4,
                    spa: 0,
                    spd: 252,
                    spe: 0,
                };
            }
        }

        // Default balanced spread
        EVSpread {
            hp: 84,
            atk: 84,
            def: 84,
            spa: 84,
            spd: 84,
            spe: 84,
        }
    }
}

/// Generate a random team as a Showdown-compatible string
pub fn generate_random_team_string(format: &str) -> String {
    let mut generator = RandomTeamGenerator::new(format);
    let team = generator.generate_team();

    let mut output = String::new();

    for pokemon in team {
        output.push_str(&pokemon.species);
        if let Some(ref item) = pokemon.item {
            output.push_str(&format!(" @ {}", item));
        }
        output.push('\n');

        output.push_str(&format!("Ability: {}\n", pokemon.ability));
        output.push_str(&format!("Level: {}\n", pokemon.level));

        // EVs
        let mut ev_parts = Vec::new();
        if pokemon.evs.hp > 0 { ev_parts.push(format!("{} HP", pokemon.evs.hp)); }
        if pokemon.evs.atk > 0 { ev_parts.push(format!("{} Atk", pokemon.evs.atk)); }
        if pokemon.evs.def > 0 { ev_parts.push(format!("{} Def", pokemon.evs.def)); }
        if pokemon.evs.spa > 0 { ev_parts.push(format!("{} SpA", pokemon.evs.spa)); }
        if pokemon.evs.spd > 0 { ev_parts.push(format!("{} SpD", pokemon.evs.spd)); }
        if pokemon.evs.spe > 0 { ev_parts.push(format!("{} Spe", pokemon.evs.spe)); }
        if !ev_parts.is_empty() {
            output.push_str(&format!("EVs: {}\n", ev_parts.join(" / ")));
        }

        if let Some(ref nature) = pokemon.nature {
            output.push_str(&format!("{} Nature\n", nature));
        }

        for move_name in &pokemon.moves {
            output.push_str(&format!("- {}\n", move_name));
        }

        output.push('\n');
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_team() {
        let mut generator = RandomTeamGenerator::new("gen9randombattle");
        let team = generator.generate_team();

        assert_eq!(team.len(), 6);

        // Check all Pokemon are different
        let species: Vec<_> = team.iter().map(|p| &p.species).collect();
        let unique: std::collections::HashSet<_> = species.iter().collect();
        assert_eq!(species.len(), unique.len());

        // Check all have 4 moves
        for pokemon in &team {
            assert_eq!(pokemon.moves.len(), 4);
            assert!(!pokemon.ability.is_empty());
            assert!(pokemon.item.is_some());
            assert!(pokemon.nature.is_some());
        }
    }

    #[test]
    fn test_deterministic_with_seed() {
        use crate::prng::PRNGSeed;

        let seed = PRNGSeed::Gen5([1, 2, 3, 4]);

        let mut gen1 = RandomTeamGenerator::with_seed("gen9randombattle", seed.clone());
        let team1 = gen1.generate_team();

        let mut gen2 = RandomTeamGenerator::with_seed("gen9randombattle", seed.clone());
        let team2 = gen2.generate_team();

        // Same seed should produce same team
        for (p1, p2) in team1.iter().zip(team2.iter()) {
            assert_eq!(p1.species, p2.species);
            assert_eq!(p1.ability, p2.ability);
            assert_eq!(p1.item, p2.item);
        }
    }

    #[test]
    fn test_generate_team_string() {
        let team_str = generate_random_team_string("gen9randombattle");

        assert!(!team_str.is_empty());
        assert!(team_str.contains("Ability:"));
        assert!(team_str.contains("Level:"));
        assert!(team_str.contains("- ")); // Moves
    }

    #[test]
    fn test_ev_spreads_valid() {
        let mut generator = RandomTeamGenerator::new("gen9randombattle");
        let team = generator.generate_team();

        for pokemon in &team {
            let total = pokemon.evs.hp + pokemon.evs.atk + pokemon.evs.def +
                        pokemon.evs.spa + pokemon.evs.spd + pokemon.evs.spe;
            assert!(total <= 510, "EVs exceed 510 for {}", pokemon.species);
        }
    }
}
