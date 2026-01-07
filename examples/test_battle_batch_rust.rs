/// Batch Rust Battle Test Runner
///
/// Runs multiple battles in sequence
///
/// Usage: cargo run --example test_battle_batch_rust [start_seed] [end_seed]
/// Example: cargo run --example test_battle_batch_rust 1 100

use pokemon_showdown::{Battle, BattleOptions, PlayerOptions, PokemonSet, PRNGSeed, ID};
use pokemon_showdown::battle::TeamFormat;
use pokemon_showdown::dex_data::{StatsTable, Gender};
use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let start_seed: u16 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(1);
    let end_seed: u16 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(100);

    eprintln!("Running Rust battles for seeds {}-{}", start_seed, end_seed);

    for seed_num in start_seed..=end_seed {
        let teams_file = format!("/tmp/teams-seed{}-rust.json", seed_num);

        if !std::path::Path::new(&teams_file).exists() {
            eprintln!("Seed {}: SKIP - no team file", seed_num);
            continue;
        }

        match run_battle(seed_num, &teams_file) {
            Ok(iterations) => {
                eprintln!("Seed {}: OK - {} iterations", seed_num, iterations);
            }
            Err(e) => {
                eprintln!("Seed {}: ERROR - {}", seed_num, e);
            }
        }
    }

    eprintln!("Rust batch complete");
}

fn run_battle(seed_num: u16, teams_file: &str) -> Result<usize, String> {
    // Load teams
    let teams_json = fs::read_to_string(teams_file)
        .map_err(|e| format!("Failed to read teams: {}", e))?;

    #[derive(serde::Deserialize)]
    struct Teams {
        p1: Vec<TestPokemon>,
        p2: Vec<TestPokemon>,
    }

    #[derive(serde::Deserialize)]
    struct TestPokemon {
        name: String,
        species: String,
        level: u8,
        ability: String,
        item: String,
        nature: String,
        gender: String,
        moves: Vec<String>,
        evs: StatsTable,
        ivs: StatsTable,
    }

    let teams: Teams = serde_json::from_str(&teams_json)
        .map_err(|e| format!("Failed to parse teams: {}", e))?;

    // Create battle
    let team1: Vec<_> = teams.p1.iter().map(|set| PokemonSet {
        name: set.name.clone(),
        species: set.species.clone(),
        level: set.level,
        ability: set.ability.clone(),
        item: set.item.clone(),
        nature: set.nature.clone(),
        gender: match set.gender.as_str() {
            "M" => Gender::Male,
            "F" => Gender::Female,
            _ => Gender::None,
        },
        moves: set.moves.clone(),
        evs: set.evs.clone(),
        ivs: set.ivs.clone(),
        ..Default::default()
    }).collect();

    let team2: Vec<_> = teams.p2.iter().map(|set| PokemonSet {
        name: set.name.clone(),
        species: set.species.clone(),
        level: set.level,
        ability: set.ability.clone(),
        item: set.item.clone(),
        nature: set.nature.clone(),
        gender: match set.gender.as_str() {
            "M" => Gender::Male,
            "F" => Gender::Female,
            _ => Gender::None,
        },
        moves: set.moves.clone(),
        evs: set.evs.clone(),
        ivs: set.ivs.clone(),
        ..Default::default()
    }).collect();

    let mut battle = Battle::new(BattleOptions {
        format_id: ID::new("gen9randombattle"),
        seed: Some(PRNGSeed::Gen5([0, 0, 0, seed_num])),
        p1: Some(PlayerOptions {
            name: "Player 1".to_string(),
            team: TeamFormat::Sets(team1),
            avatar: None,
            rating: None,
            seed: None,
        }),
        p2: Some(PlayerOptions {
            name: "Player 2".to_string(),
            team: TeamFormat::Sets(team2),
            avatar: None,
            rating: None,
            seed: None,
        }),
        ..Default::default()
    });

    let mut iteration = 0;
    const MAX_TURNS: i32 = 100;

    let mut output_lines = Vec::new();

    while !battle.ended && battle.turn < MAX_TURNS {
        iteration += 1;
        let prng_before = battle.prng.call_count;

        // Make default choices
        battle.make_choices(&["default", "default"]);

        let prng_after = battle.prng.call_count;

        // Format Pokemon state
        let format_team = |side_idx: usize| -> String {
            let side = &battle.sides[side_idx];
            let active_pokemon: Vec<String> = side.active.iter()
                .filter_map(|&idx| idx)
                .filter_map(|idx| side.pokemon.get(idx))
                .filter(|p| p.is_active)
                .map(|p| format!("{}({}/{})", p.name, p.hp, p.maxhp))
                .collect();
            active_pokemon.join(", ")
        };

        let p1_state = format_team(0);
        let p2_state = format_team(1);

        // Output line
        let line = format!(
            "#{}: turn={}, prng={}->{}, P1=[{}], P2=[{}]",
            iteration, battle.turn, prng_before, prng_after, p1_state, p2_state
        );
        output_lines.push(line);
    }

    // Write output file
    let output_file = format!("/tmp/rust-battle-seed{}.txt", seed_num);
    fs::write(&output_file, output_lines.join("\n") + "\n")
        .map_err(|e| format!("Failed to write output: {}", e))?;

    Ok(iteration)
}
