/// Unified Battle Test Runner (Rust)
///
/// Generates teams and runs battles entirely in memory.
/// No file I/O - outputs one summary line per seed to stdout.
/// Uses Rayon for parallel execution.
///
/// Usage: cargo run --release --example test_unified <start_seed> <end_seed>
/// Output format: SEED <n>: turns=<t>, prng=<p>, winner=<w>

use pokemon_showdown::{Battle, BattleOptions, PlayerOptions, PRNG, PRNGSeed, ID, team_generator};
use pokemon_showdown::battle::TeamFormat;
use pokemon_showdown::dex::Dex;
use rayon::prelude::*;
use std::env;

fn run_battle(seed_num: u16, dex: &Dex) -> (u16, String) {
    (seed_num, run_battle_inner(seed_num, dex))
}

fn run_battle_inner(seed_num: u16, dex: &Dex) -> String {
    // Generate teams using the seed
    let mut prng = PRNG::new(Some(PRNGSeed::Gen5([0, 0, 0, seed_num])));

    let team1 = team_generator::generate_random_team(&mut prng, dex);
    let team2 = team_generator::generate_random_team(&mut prng, dex);

    // Create battle with fresh PRNG (same seed)
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
        _ => "none",
    };

    format!(
        "turns={}, prng={}, winner={}",
        battle.turn, battle.prng.call_count, winner
    )
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let start_seed: u16 = args.get(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);

    let end_seed: u16 = args.get(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(start_seed);

    // Configure thread pool with larger stack size
    rayon::ThreadPoolBuilder::new()
        .num_threads(10)
        .stack_size(1280 * 1024 * 1024) // 1280MB stack per thread
        .build_global()
        .ok();

    // Load dex once using global cache
    let dex = Dex::global();

    // Run battles in parallel, collect results
    let mut results: Vec<(u16, String)> = (start_seed..=end_seed)
        .into_par_iter()
        .map(|seed_num| run_battle(seed_num, &dex))
        .collect();

    // Sort by seed number to ensure consistent output order
    results.sort_by_key(|(seed, _)| *seed);

    // Print results in order
    for (seed_num, summary) in results {
        println!("SEED {}: {}", seed_num, summary);
    }
}
