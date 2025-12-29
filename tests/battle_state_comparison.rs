//! Battle State Comparison Test
//!
//! This test generates a random team in gen9 and runs a battle with random moves
//! derived from a specific seed. It records the state at every step to enable
//! comparison with the JavaScript implementation.

use pokemon_showdown::{
    Battle, BattleOptions, PRNGSeed, PlayerOptions, PRNG,
};
use pokemon_showdown::dex_data::StatsTable;
use pokemon_showdown::random_teams::RandomTeamGenerator;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct PokemonState {
    name: String,
    species: String,
    hp: i32,
    maxhp: i32,
    status: String,
    fainted: bool,
    level: u8,
    ability: String,
    item: String,
    moves: Vec<String>,
    boosts: std::collections::HashMap<String, i32>,
    types: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SideState {
    name: String,
    id: String,
    pokemon: Vec<PokemonState>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BattleState {
    turn: i32,
    ended: bool,
    winner: Option<String>,
    sides: Vec<SideState>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TurnLog {
    turn: i32,
    p1: String,
    p2: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct StateRecord {
    turn: i32,
    state: BattleState,
}

#[derive(Debug, Serialize, Deserialize)]
struct TeamInfo {
    species: String,
    moves: Vec<String>,
    ability: String,
    item: Option<String>,
    level: u8,
}

#[derive(Debug, Serialize, Deserialize)]
struct BattleLog {
    seed: Vec<i32>,
    teams: std::collections::HashMap<String, Vec<TeamInfo>>,
    states: Vec<StateRecord>,
    battle_log: Vec<TurnLog>,
    summary: BattleSummary,
}

#[derive(Debug, Serialize, Deserialize)]
struct BattleSummary {
    turns: i32,
    winner: Option<String>,
    ended: bool,
}

/// Extract battle state at a given point
fn extract_battle_state(battle: &Battle) -> BattleState {
    let mut sides = Vec::new();

    for side in &battle.sides {
        let mut pokemon_states = Vec::new();

        for pokemon in &side.pokemon {
            pokemon_states.push(PokemonState {
                name: pokemon.name.clone(),
                species: pokemon.species_id.to_string(),
                hp: pokemon.hp,
                maxhp: pokemon.maxhp,
                status: pokemon.status.to_string(),
                fainted: pokemon.fainted,
                level: pokemon.level,
                ability: pokemon.ability.to_string(),
                item: pokemon.item.to_string(),
                moves: pokemon.move_slots.iter().map(|m| m.id.to_string()).collect(),
                boosts: std::collections::HashMap::new(), // TODO: Add boosts when implemented
                types: pokemon.types.clone(),
            });
        }

        sides.push(SideState {
            name: side.name.clone(),
            id: side.id.to_str().to_string(),
            pokemon: pokemon_states,
        });
    }

    BattleState {
        turn: battle.turn,
        ended: battle.ended,
        winner: battle.winner.clone(),
        sides,
    }
}

/// Make a random choice for a player
fn make_random_choice(_battle: &Battle, _player_id: &str, _prng: &mut PRNG) -> String {
    // For now, just return a simple move 1 choice
    // TODO: Implement proper random choice logic based on request
    "move 1".to_string()
}

/// Run a deterministic battle and record states
fn run_battle_with_states(seed: PRNGSeed, max_turns: i32) -> BattleLog {
    // Generate teams using the same seed
    let mut team_generator = RandomTeamGenerator::with_seed("gen9randombattle", seed.clone());
    let team1_sets = team_generator.generate_team();
    let team2_sets = team_generator.generate_team();

    // Convert to PokemonSets for battle
    let team1: Vec<_> = team1_sets
        .iter()
        .map(|set| pokemon_showdown::PokemonSet {
            name: set.species.clone(),
            species: set.species.clone(),
            level: set.level,
            ability: set.ability.clone(),
            item: set.item.clone().unwrap_or_default(),
            nature: set.nature.clone().unwrap_or_else(|| "Hardy".to_string()),
            moves: set.moves.clone(),
            evs: StatsTable::new(
                set.evs.hp,
                set.evs.atk,
                set.evs.def,
                set.evs.spa,
                set.evs.spd,
                set.evs.spe,
            ),
            ivs: StatsTable::new(
                set.ivs.hp,
                set.ivs.atk,
                set.ivs.def,
                set.ivs.spa,
                set.ivs.spd,
                set.ivs.spe,
            ),
            ..Default::default()
        })
        .collect();

    let team2: Vec<_> = team2_sets
        .iter()
        .map(|set| pokemon_showdown::PokemonSet {
            name: set.species.clone(),
            species: set.species.clone(),
            level: set.level,
            ability: set.ability.clone(),
            item: set.item.clone().unwrap_or_default(),
            nature: set.nature.clone().unwrap_or_else(|| "Hardy".to_string()),
            moves: set.moves.clone(),
            evs: StatsTable::new(
                set.evs.hp,
                set.evs.atk,
                set.evs.def,
                set.evs.spa,
                set.evs.spd,
                set.evs.spe,
            ),
            ivs: StatsTable::new(
                set.ivs.hp,
                set.ivs.atk,
                set.ivs.def,
                set.ivs.spa,
                set.ivs.spd,
                set.ivs.spe,
            ),
            ..Default::default()
        })
        .collect();

    // Create battle with the seed
    let battle = Battle::new(BattleOptions {
        format_id: pokemon_showdown::ID::new("gen9randombattle"),
        seed: Some(seed.clone()),
        p1: Some(PlayerOptions {
            name: "Player 1".to_string(),
            team: team1.clone(),
            avatar: None,
            rating: None,
        }),
        p2: Some(PlayerOptions {
            name: "Player 2".to_string(),
            team: team2.clone(),
            avatar: None,
            rating: None,
        }),
        ..Default::default()
    });

    // PRNG for making random choices
    let mut choice_prng = PRNG::new(Some(seed.clone()));

    // Prepare log
    let seed_vec: Vec<i32> = match seed {
        PRNGSeed::Gen5(ref s) => s.iter().map(|&x| x as i32).collect(),
        PRNGSeed::Sodium(_) => vec![0, 0, 0, 0],
    };

    let mut log = BattleLog {
        seed: seed_vec,
        teams: std::collections::HashMap::new(),
        states: Vec::new(),
        battle_log: Vec::new(),
        summary: BattleSummary {
            turns: 0,
            winner: None,
            ended: false,
        },
    };

    // Add team info
    log.teams.insert(
        "p1".to_string(),
        team1
            .iter()
            .map(|p| TeamInfo {
                species: p.species.clone(),
                moves: p.moves.clone(),
                ability: p.ability.clone(),
                item: Some(p.item.clone()),
                level: p.level,
            })
            .collect(),
    );

    log.teams.insert(
        "p2".to_string(),
        team2
            .iter()
            .map(|p| TeamInfo {
                species: p.species.clone(),
                moves: p.moves.clone(),
                ability: p.ability.clone(),
                item: Some(p.item.clone()),
                level: p.level,
            })
            .collect(),
    );

    // Record initial state
    log.states.push(StateRecord {
        turn: 0,
        state: extract_battle_state(&battle),
    });

    // Run the battle
    let mut turn = 0;
    while !battle.ended && turn < max_turns {
        turn += 1;

        // Make random choices for both players
        let p1_choice = make_random_choice(&battle, "p1", &mut choice_prng);
        let p2_choice = make_random_choice(&battle, "p2", &mut choice_prng);

        // Record choices
        log.battle_log.push(TurnLog {
            turn,
            p1: p1_choice.clone(),
            p2: p2_choice.clone(),
        });

        // Execute choices (TODO: implement battle.make_choices)
        // For now, we'll just increment turn to prevent infinite loop
        // battle.make_choices(&p1_choice, &p2_choice);

        // Record state after turn
        log.states.push(StateRecord {
            turn,
            state: extract_battle_state(&battle),
        });

        // Safety check
        if turn >= max_turns {
            eprintln!("Battle reached max turns");
            break;
        }
    }

    // Add final summary
    log.summary = BattleSummary {
        turns: turn,
        winner: battle.winner.clone(),
        ended: battle.ended,
    };

    log
}

#[test]
fn test_battle_state_comparison() {
    let seed = PRNGSeed::Gen5([12345, 23456, 11111, 22222]);
    println!("Running deterministic battle with seed: {:?}", seed);

    let log = run_battle_with_states(seed, 100);

    let output_path = "battle-state-rust.json";
    let json = serde_json::to_string_pretty(&log).expect("Failed to serialize log");
    fs::write(output_path, json).expect("Failed to write log file");

    println!("Battle completed in {} turns", log.summary.turns);
    println!("Winner: {:?}", log.summary.winner);
    println!("Output saved to: {}", output_path);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_comparison() {
        test_battle_state_comparison();
    }
}
