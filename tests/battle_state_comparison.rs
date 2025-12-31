//! Battle State Comparison Test
//!
//! This test loads pre-generated teams (from test-teams.json) and runs a battle
//! with random moves derived from a specific seed. It records the state at every
//! step to enable comparison with the JavaScript implementation.

use pokemon_showdown::{
    Battle, BattleOptions, PRNGSeed, PlayerOptions, PRNG,
};
use pokemon_showdown::dex_data::{StatsTable, Gender};
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
fn make_random_choice(battle: &Battle, player_id: &str, prng: &mut PRNG) -> String {
    // Find the side for this player
    let side = battle
        .sides
        .iter()
        .find(|s| s.id.to_str() == player_id);

    let Some(side) = side else {
        return "pass".to_string();
    };

    // Get the active Pokemon
    if side.active.is_empty() {
        return "pass".to_string();
    }

    let active_idx = side.active[0];
    let Some(pokemon_idx) = active_idx else {
        return "pass".to_string();
    };

    let pokemon = &side.pokemon[pokemon_idx];

    // If fainted, we need to switch
    if pokemon.fainted {
        // Find a Pokemon to switch to
        for (idx, p) in side.pokemon.iter().enumerate() {
            if !p.fainted && !side.active.contains(&Some(idx)) {
                return format!("switch {}", idx + 1);
            }
        }
        return "pass".to_string();
    }

    // Otherwise, pick a random move (1-4)
    if pokemon.move_slots.is_empty() {
        return "pass".to_string();
    }

    let num_moves = pokemon.move_slots.len();
    let move_idx = prng.random(Some(0), Some(num_moves as i32)) as usize;
    format!("move {}", move_idx + 1)
}

/// Run a deterministic battle and record states
fn run_battle_with_states(seed: PRNGSeed, max_turns: i32) -> BattleLog {
    // Load pre-generated teams from JSON
    let test_teams_path = "test-teams.json";
    let test_teams_json = fs::read_to_string(test_teams_path)
        .expect("Failed to read test-teams.json");

    #[derive(Debug, Deserialize)]
    struct TestTeams {
        p1: Vec<TestPokemon>,
        p2: Vec<TestPokemon>,
    }

    #[derive(Debug, Deserialize)]
    struct TestPokemon {
        species: String,
        level: u8,
        ability: String,
        item: String,
        nature: String,
        #[serde(default)]
        gender: String,  // Optional field, defaults to empty string
        moves: Vec<String>,
        evs: TestStats,
        ivs: TestStats,
    }

    #[derive(Debug, Deserialize)]
    struct TestStats {
        hp: i32,
        atk: i32,
        def: i32,
        spa: i32,
        spd: i32,
        spe: i32,
    }

    let test_teams: TestTeams = serde_json::from_str(&test_teams_json)
        .expect("Failed to parse test-teams.json");

    // Convert to PokemonSets for battle
    let team1: Vec<_> = test_teams.p1
        .iter()
        .map(|set| pokemon_showdown::PokemonSet {
            name: set.species.clone(),
            species: set.species.clone(),
            level: set.level,
            ability: set.ability.clone(),
            item: set.item.clone(),
            nature: set.nature.clone(),
            gender: Gender::parse(&set.gender),  // Parse gender from string
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

    let team2: Vec<_> = test_teams.p2
        .iter()
        .map(|set| pokemon_showdown::PokemonSet {
            name: set.species.clone(),
            species: set.species.clone(),
            level: set.level,
            ability: set.ability.clone(),
            item: set.item.clone(),
            nature: set.nature.clone(),
            gender: Gender::parse(&set.gender),  // Parse gender from string
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
    let mut battle = Battle::new(BattleOptions {
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

    // Make initial team choices to skip team preview
    battle.make_choices(&["default", "default"]);

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

        // Execute choices
        battle.make_choices(&[&p1_choice, &p2_choice]);

        // Record state after turn
        log.states.push(StateRecord {
            turn,
            state: extract_battle_state(&battle),
        });

        // Check if battle ended
        if turn >= max_turns {
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
    let seed = PRNGSeed::Gen5([0, 0, 0, 21]);
    println!("Running deterministic battle with seed: {:?}", seed);

    let log = run_battle_with_states(seed, 100);

    let output_path = "battle-state-rust.json";
    let json = serde_json::to_string_pretty(&log).expect("Failed to serialize log");
    fs::write(output_path, json).expect("Failed to write log file");

    println!("Battle completed in {} turns", log.summary.turns);
    println!("Winner: {:?}", log.summary.winner);
    println!("Output saved to: {}", output_path);

    // Print turn 1 results for comparison
    if log.states.len() > 1 {
        let turn1 = &log.states[1];
        let p1_hp = turn1.state.sides[0].pokemon[0].hp;
        let p2_hp = turn1.state.sides[1].pokemon[0].hp;
        println!("After turn 1: P1 HP={}, P2 HP={}", p1_hp, p2_hp);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_comparison() {
        test_battle_state_comparison();
    }
}
