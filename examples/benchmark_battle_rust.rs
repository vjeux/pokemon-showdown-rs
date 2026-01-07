/// Rust Battle Benchmark
/// Runs battles for multiple seeds and measures performance
///
/// Usage: cargo run --example benchmark_battle_rust --release [start_seed] [end_seed]

use pokemon_showdown::{Battle, BattleOptions, PlayerOptions, PokemonSet, PRNGSeed, ID};
use pokemon_showdown::battle::TeamFormat;
use pokemon_showdown::dex_data::{StatsTable, Gender};
use std::fs;
use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    let start_seed: u16 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(1);
    let end_seed: u16 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(100);

    let mut total_turns = 0;
    let mut total_battles = 0;

    let start_time = Instant::now();

    for seed_num in start_seed..=end_seed {
        let teams_file = format!("/tmp/teams-seed{}-rust.json", seed_num);

        if !std::path::Path::new(&teams_file).exists() {
            continue;
        }

        match run_battle(seed_num, &teams_file) {
            Ok(turns) => {
                total_turns += turns;
                total_battles += 1;
            }
            Err(_) => {
                // Skip failed battles
            }
        }
    }

    let elapsed = start_time.elapsed();
    let elapsed_sec = elapsed.as_secs_f64();

    println!("Rust Benchmark Results:");
    println!("  Seeds: {}-{}", start_seed, end_seed);
    println!("  Battles completed: {}", total_battles);
    println!("  Total turns: {}", total_turns);
    println!("  Time: {:.2}s", elapsed_sec);
    println!("  Battles/sec: {:.2}", total_battles as f64 / elapsed_sec);
    println!("  Turns/sec: {:.2}", total_turns as f64 / elapsed_sec);
    println!("  Avg ms/battle: {:.2}", elapsed.as_millis() as f64 / total_battles as f64);
}

fn run_battle(seed_num: u16, teams_file: &str) -> Result<i32, String> {
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

    const MAX_TURNS: i32 = 100;

    while !battle.ended && battle.turn < MAX_TURNS {
        battle.make_choices(&["default", "default"]);
    }

    Ok(battle.turn)
}
