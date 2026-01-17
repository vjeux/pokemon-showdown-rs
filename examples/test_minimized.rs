/// Battle Test Runner for Minimized Seeds (Rust)
///
/// Runs all minimized seeds in parallel using Rayon.
///
/// Usage: cargo run --release --example test_minimized <minimized_dir>
/// Output format: SEED <n>: turns=<t>, prng=<p>, winner=<w>

use pokemon_showdown::{Battle, BattleOptions, PlayerOptions, PokemonSet, PRNGSeed, ID};
use pokemon_showdown::battle::TeamFormat;
use pokemon_showdown::dex_data::{StatsTable, Gender};
use rayon::prelude::*;
use std::fs;
use std::env;
use std::path::Path;

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
    evs: TestStats,
    ivs: TestStats,
}

#[derive(serde::Deserialize)]
struct TestStats {
    hp: i32,
    atk: i32,
    def: i32,
    spa: i32,
    spd: i32,
    spe: i32,
}

fn run_battle(seed_num: u32, minimized_dir: &str) -> (u32, String) {
    let result = run_battle_inner(seed_num, minimized_dir);
    (seed_num, result)
}

fn run_battle_inner(seed_num: u32, minimized_dir: &str) -> String {
    let teams_file = format!("{}/seed{}.json", minimized_dir, seed_num);

    if !Path::new(&teams_file).exists() {
        return format!("ERROR - Team file not found: {}", teams_file);
    }

    let teams_json = match fs::read_to_string(&teams_file) {
        Ok(s) => s,
        Err(e) => return format!("ERROR - Failed to read team file: {}", e),
    };

    let teams: Teams = match serde_json::from_str(&teams_json) {
        Ok(t) => t,
        Err(e) => return format!("ERROR - Failed to parse teams: {}", e),
    };

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
        evs: StatsTable::new(set.evs.hp, set.evs.atk, set.evs.def, set.evs.spa, set.evs.spd, set.evs.spe),
        ivs: StatsTable::new(set.ivs.hp, set.ivs.atk, set.ivs.def, set.ivs.spa, set.ivs.spd, set.ivs.spe),
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
        evs: StatsTable::new(set.evs.hp, set.evs.atk, set.evs.def, set.evs.spa, set.evs.spd, set.evs.spe),
        ivs: StatsTable::new(set.ivs.hp, set.ivs.atk, set.ivs.def, set.ivs.spa, set.ivs.spd, set.ivs.spe),
        ..Default::default()
    }).collect();

    // Split u32 seed into two u16 values for PRNGSeed::Gen5
    // This matches the JavaScript behavior where large seed numbers are stored properly
    let seed_lo = (seed_num & 0xFFFF) as u16;
    let seed_hi = ((seed_num >> 16) & 0xFFFF) as u16;

    let mut battle = Battle::new(BattleOptions {
        format_id: ID::new("gen9randombattle"),
        seed: Some(PRNGSeed::Gen5([0, 0, seed_hi, seed_lo])),
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

    // Run battle for up to 100 turns
    for i in 1..=100 {
        battle.make_choices(&["default", "default"]);

        // Reset log position to prevent "LINE LIMIT EXCEEDED" check from failing
        battle.sent_log_pos = battle.log.len();

        if battle.ended || i >= 100 {
            break;
        }
    }

    // Determine winner
    let winner = match &battle.winner {
        Some(w) if w == "Player 1" => "p1",
        Some(w) if w == "Player 2" => "p2",
        Some(w) if w.is_empty() => "tie",
        Some(w) => {
            eprintln!("[DEBUG seed {}] Unexpected winner value: {:?}", seed_num, w);
            "none"
        }
        None => {
            eprintln!("[DEBUG seed {}] Winner is None, ended={}", seed_num, battle.ended);
            "none"
        }
    };

    format!(
        "turns={}, prng={}, winner={}",
        battle.turn, battle.prng.call_count, winner
    )
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let minimized_dir = args.get(1)
        .map(|s| s.as_str())
        .unwrap_or("tests/minimized");

    // Configure thread pool with larger stack size
    rayon::ThreadPoolBuilder::new()
        .num_threads(10)
        .stack_size(1280 * 1024 * 1024) // 1280MB stack per thread
        .build_global()
        .ok();

    // Read all seed files from the minimized directory
    let seed_files: Vec<u32> = match fs::read_dir(minimized_dir) {
        Ok(entries) => {
            let mut seeds: Vec<u32> = entries
                .filter_map(|entry| {
                    let entry = entry.ok()?;
                    let name = entry.file_name().to_string_lossy().to_string();
                    if name.starts_with("seed") && name.ends_with(".json") {
                        name.strip_prefix("seed")?
                            .strip_suffix(".json")?
                            .parse()
                            .ok()
                    } else {
                        None
                    }
                })
                .collect();
            seeds.sort();
            seeds
        }
        Err(e) => {
            eprintln!("Failed to read minimized directory: {}", e);
            std::process::exit(1);
        }
    };

    // Run battles in parallel, collect results
    let mut results: Vec<(u32, String)> = seed_files
        .par_iter()
        .map(|&seed_num| run_battle(seed_num, minimized_dir))
        .collect();

    // Sort by seed number to ensure consistent output order
    results.sort_by_key(|(seed, _)| *seed);

    // Print results in order
    for (seed_num, summary) in results {
        println!("SEED {}: {}", seed_num, summary);
    }
}
