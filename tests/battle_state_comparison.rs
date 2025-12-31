//! Battle State Comparison Test
//!
//! This test generates random teams from a seed and runs a battle with random
//! moves. It records the state at every step to enable comparison with the
//! JavaScript implementation.

use pokemon_showdown::{
    Battle, BattleOptions, PRNGSeed, PlayerOptions, PRNG,
};
use serde::{Deserialize, Serialize};

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
    // Load dex for team generation
    let dex = pokemon_showdown::Dex::load_default().expect("Failed to load dex");

    // Generate random teams from seed
    let mut team_prng = PRNG::new(Some(seed.clone()));
    let (team1, team2) = pokemon_showdown::team_generator::generate_random_teams(&mut team_prng, &dex);

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

    // Record initial state (before turn 1)
    log.states.push(StateRecord {
        turn: 0,
        state: extract_battle_state(&battle),
    });

    // Run the battle
    let mut turn = 0;
    while !battle.ended && turn < max_turns {
        turn += 1;

        // Use "default" choices (move 1) to match JavaScript test behavior
        let p1_choice = "default";
        let p2_choice = "default";

        // Record choices
        log.battle_log.push(TurnLog {
            turn,
            p1: p1_choice.to_string(),
            p2: p2_choice.to_string(),
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
    let seed = PRNGSeed::Gen5([0, 0, 0, 1]);
    println!("Running deterministic battle with seed: {:?}", seed);

    let log = run_battle_with_states(seed, 1);  // Only run 1 turn for debugging

    println!("Battle completed in {} turns", log.summary.turns);
    println!("Winner: {:?}", log.summary.winner);

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
