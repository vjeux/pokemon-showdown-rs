//! Team Generator - Generates random Pokemon teams from a seed
//!
//! This module provides functionality to generate random 6v6 Pokemon teams
//! deterministically from a PRNG seed.

use crate::dex::Dex;
use crate::dex_data::{StatsTable, Gender};
use crate::pokemon::PokemonSet;
use crate::prng::PRNG;

/// Generate a random Pokemon team (6 Pokemon) from a PRNG
pub fn generate_random_team(prng: &mut PRNG, dex: &Dex) -> Vec<PokemonSet> {
    let mut team = Vec::new();

    // Get all available species as a Vec for sampling (sorted by name for determinism across languages)
    let mut all_species: Vec<_> = dex.species.values().collect();
    all_species.sort_by_key(|s| &s.name);
    if all_species.is_empty() {
        return team;
    }

    // Get all available moves as a Vec for sampling (sorted for determinism across languages)
    let mut all_moves: Vec<_> = dex.moves.keys().map(|id| id.as_str()).collect();
    all_moves.sort();

    // Get all available items as a Vec for sampling (sorted for determinism across languages)
    let mut all_items: Vec<_> = dex.items.keys().map(|id| id.as_str()).collect();
    all_items.sort();

    // Get all available natures as a Vec for sampling (sorted for determinism across languages)
    let mut all_natures: Vec<_> = dex.natures.keys().map(|id| id.as_str()).collect();
    all_natures.sort();

    // Track used species and items to avoid duplicates
    let mut used_species = Vec::new();
    let mut used_items: Vec<String> = Vec::new();

    for _ in 0..6 {
        // Select random species (avoid duplicates)
        let mut available_species: Vec<_> = all_species.iter()
            .filter(|s| !used_species.contains(&s.name.as_str()))
            .copied()
            .collect();

        // If we've used all species, allow reuse (shouldn't happen with 1000+ species)
        if available_species.is_empty() {
            available_species = all_species.clone();
        }

        let species = prng.sample(&available_species).expect("No species available");
        used_species.push(species.name.as_str());

        // Select random level between 50-100
        let level = prng.random_range(50, 101) as u8;

        // Select random ability from the species' abilities
        // For cosmetic formes, inherit from baseSpecies
        let ability = if let Some(ref a) = species.abilities.slot0 {
            a.clone()
        } else if let Some(ref base_species_name) = species.base_species {
            // Cosmetic forme - get ability from base species
            if let Some(base_species) = dex.species().get(base_species_name) {
                base_species.abilities.slot0.clone().unwrap_or_else(|| "No Ability".to_string())
            } else {
                "No Ability".to_string()
            }
        } else {
            "No Ability".to_string()
        };

        // Select random item (avoid duplicates, allow empty string for multiple Pokemon)
        let item = if !all_items.is_empty() {
            let mut available_items: Vec<_> = all_items.iter()
                .filter(|i| !used_items.iter().any(|used| used == *i) || i.is_empty())
                .copied()
                .collect();

            // If we've used all items, allow reuse or use empty string
            if available_items.is_empty() {
                available_items.push("");
            }

            let selected_item = prng.sample(&available_items).unwrap_or(&"").to_string();
            if !selected_item.is_empty() {
                used_items.push(selected_item.clone());
            }
            selected_item
        } else {
            "".to_string()
        };

        // Select random nature
        let nature = if !all_natures.is_empty() {
            prng.sample(&all_natures).unwrap_or(&"hardy").to_string()
        } else {
            "hardy".to_string()
        };

        // Select random gender based on species gender ratio
        // Match JavaScript behavior: if no gender_ratio, check gender field first, then default to 50/50
        let gender = if let Some(ref ratio) = species.gender_ratio {
            if ratio.m > 0.0 && ratio.f > 0.0 {
                // Mixed gender
                if prng.random_chance(1, 2) {
                    Gender::Male
                } else {
                    Gender::Female
                }
            } else if ratio.m > 0.0 {
                Gender::Male
            } else if ratio.f > 0.0 {
                Gender::Female
            } else {
                Gender::None
            }
        } else if let Some(ref gender_str) = species.gender {
            // No gender_ratio, but gender field is set (M, F, or N)
            match gender_str.as_str() {
                "M" => Gender::Male,
                "F" => Gender::Female,
                "N" => Gender::None,
                _ => {
                    // Default to 50/50 like JavaScript
                    if prng.random_chance(1, 2) {
                        Gender::Male
                    } else {
                        Gender::Female
                    }
                }
            }
        } else {
            // No gender_ratio and no gender field - default to 50/50 like JavaScript
            if prng.random_chance(1, 2) {
                Gender::Male
            } else {
                Gender::Female
            }
        };

        // Select 4 random moves
        let mut moves = Vec::new();
        for _ in 0..4 {
            if !all_moves.is_empty() {
                let move_name = prng.sample(&all_moves).unwrap_or(&"tackle").to_string();
                if !moves.contains(&move_name) {
                    moves.push(move_name);
                }
            }
        }
        // Fill with "tackle" if we don't have enough unique moves
        while moves.len() < 4 {
            moves.push("tackle".to_string());
        }

        // Generate random EVs (total max 510, each stat max 252)
        let evs = generate_random_evs(prng);

        // Generate random IVs (0-31 for each stat)
        let ivs = StatsTable::new(
            prng.random_range(0, 32),
            prng.random_range(0, 32),
            prng.random_range(0, 32),
            prng.random_range(0, 32),
            prng.random_range(0, 32),
            prng.random_range(0, 32),
        );

        team.push(PokemonSet {
            name: species.name.clone(),
            species: species.name.clone(),
            level,
            ability,
            item,
            nature,
            gender,
            moves,
            evs,
            ivs,
            ..Default::default()
        });
    }

    team
}

/// Generate random EVs with constraints:
/// - Total EVs <= 510
/// - Each stat <= 252
fn generate_random_evs(prng: &mut PRNG) -> StatsTable {
    let mut evs = [0i32; 6]; // hp, atk, def, spa, spd, spe
    let mut total_evs = 0;

    // Distribute 510 EVs one at a time to random stats
    while total_evs < 510 {
        // Find stats that haven't reached the 252 limit
        let mut available_stats = Vec::new();
        for (i, &ev) in evs.iter().enumerate() {
            if ev < 252 {
                available_stats.push(i);
            }
        }

        // If no stats available (shouldn't happen), break
        if available_stats.is_empty() {
            break;
        }

        // Pick a random stat from available ones
        let stat_idx = *prng.sample(&available_stats).expect("No available stats");

        // Add 1-4 EVs to this stat (for faster generation), but don't exceed limits
        let amount = prng.random_range(1, 5).min(252 - evs[stat_idx]).min(510 - total_evs);
        evs[stat_idx] += amount;
        total_evs += amount;
    }

    StatsTable::new(evs[0], evs[1], evs[2], evs[3], evs[4], evs[5])
}

/// Generate two random teams (P1 and P2) from a seed
pub fn generate_random_teams(prng: &mut PRNG, dex: &Dex) -> (Vec<PokemonSet>, Vec<PokemonSet>) {
    let team1 = generate_random_team(prng, dex);
    let team2 = generate_random_team(prng, dex);
    (team1, team2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prng::PRNGSeed;

    #[test]
    fn test_generate_random_team() {
        let dex = Dex::load_default().expect("Failed to load dex");
        let mut prng = PRNG::new(Some(PRNGSeed::Gen5([1, 2, 3, 4])));

        let team = generate_random_team(&mut prng, &dex);

        assert_eq!(team.len(), 6);

        for pokemon in &team {
            assert!(!pokemon.species.is_empty());
            assert!(pokemon.level >= 50 && pokemon.level <= 100);
            assert_eq!(pokemon.moves.len(), 4);
        }
    }

    #[test]
    fn test_deterministic_generation() {
        let dex = Dex::load_default().expect("Failed to load dex");

        let mut prng1 = PRNG::new(Some(PRNGSeed::Gen5([1, 2, 3, 4])));
        let team1 = generate_random_team(&mut prng1, &dex);

        let mut prng2 = PRNG::new(Some(PRNGSeed::Gen5([1, 2, 3, 4])));
        let team2 = generate_random_team(&mut prng2, &dex);

        // Teams should be identical
        for (p1, p2) in team1.iter().zip(team2.iter()) {
            assert_eq!(p1.species, p2.species);
            assert_eq!(p1.level, p2.level);
            assert_eq!(p1.ability, p2.ability);
            assert_eq!(p1.moves, p2.moves);
        }
    }

    #[test]
    fn test_ev_distribution() {
        let dex = Dex::load_default().expect("Failed to load dex");
        let mut prng = PRNG::new(Some(PRNGSeed::Gen5([5, 6, 7, 8])));

        let team = generate_random_team(&mut prng, &dex);

        for pokemon in &team {
            // Check total EVs <= 510
            let total = pokemon.set.evs.hp + pokemon.set.evs.atk + pokemon.set.evs.def
                      + pokemon.set.evs.spa + pokemon.set.evs.spd + pokemon.set.evs.spe;
            assert!(total <= 510, "Total EVs {} exceeds 510", total);

            // Check each stat <= 252
            assert!(pokemon.set.evs.hp <= 252, "HP EVs {} exceeds 252", pokemon.set.evs.hp);
            assert!(pokemon.set.evs.atk <= 252, "ATK EVs {} exceeds 252", pokemon.set.evs.atk);
            assert!(pokemon.set.evs.def <= 252, "DEF EVs {} exceeds 252", pokemon.set.evs.def);
            assert!(pokemon.set.evs.spa <= 252, "SPA EVs {} exceeds 252", pokemon.set.evs.spa);
            assert!(pokemon.set.evs.spd <= 252, "SPD EVs {} exceeds 252", pokemon.set.evs.spd);
            assert!(pokemon.set.evs.spe <= 252, "SPE EVs {} exceeds 252", pokemon.set.evs.spe);
        }

        // Print first pokemon's EVs to verify distribution looks reasonable
        println!("Sample EVs: HP={}, ATK={}, DEF={}, SPA={}, SPD={}, SPE={}",
            team[0].set.evs.hp, team[0].set.evs.atk, team[0].set.evs.def,
            team[0].set.evs.spa, team[0].set.evs.spd, team[0].set.evs.spe);
    }

    #[test]
    fn test_no_duplicate_species_or_items() {
        let dex = Dex::load_default().expect("Failed to load dex");
        let mut prng = PRNG::new(Some(PRNGSeed::Gen5([10, 11, 12, 13])));

        let team = generate_random_team(&mut prng, &dex);

        // Check no duplicate species
        let mut species_seen = std::collections::HashSet::new();
        for pokemon in &team {
            assert!(
                species_seen.insert(&pokemon.species),
                "Duplicate species found: {}",
                pokemon.species
            );
        }

        // Check no duplicate items (excluding empty items)
        let mut items_seen = std::collections::HashSet::new();
        for pokemon in &team {
            if !pokemon.item.is_empty() {
                assert!(
                    items_seen.insert(&pokemon.item),
                    "Duplicate item found: {}",
                    pokemon.item
                );
            }
        }

        println!("Team has {} unique species and {} unique items",
            species_seen.len(), items_seen.len());
    }

    #[test]
    fn test_multiple_teams_no_duplicates() {
        let dex = Dex::load_default().expect("Failed to load dex");

        // Test 10 different teams to ensure the duplicate prevention works consistently
        for seed in 1..=10 {
            let mut prng = PRNG::new(Some(PRNGSeed::Gen5([0, 0, 0, seed])));
            let team = generate_random_team(&mut prng, &dex);

            // Check no duplicate species
            let species: Vec<_> = team.iter().map(|p| &p.species).collect();
            let unique_species: std::collections::HashSet<_> = species.iter().collect();
            assert_eq!(
                species.len(),
                unique_species.len(),
                "Seed {} has duplicate species",
                seed
            );

            // Check no duplicate items (excluding empty)
            let items: Vec<_> = team.iter()
                .map(|p| &p.item)
                .filter(|i| !i.is_empty())
                .collect();
            let unique_items: std::collections::HashSet<_> = items.iter().collect();
            assert_eq!(
                items.len(),
                unique_items.len(),
                "Seed {} has duplicate items",
                seed
            );
        }

        println!("All 10 teams have unique species and items");
    }
}
