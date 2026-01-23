/// Battle Profiling Tool
///
/// Runs battles and measures time spent in different phases.
/// Uses a single thread for accurate timing.
///
/// Usage:
///   cargo run --release --example profile_battle <num_battles>

use pokemon_showdown::{Battle, BattleOptions, PlayerOptions, PRNG, PRNGSeed, ID, team_generator};
use pokemon_showdown::battle::TeamFormat;
use pokemon_showdown::dex::Dex;
use std::env;
use std::time::{Duration, Instant};

struct ProfilingStats {
    total_battles: u32,
    total_turns: u32,
    dex_load_time: Duration,
    team_gen_time: Duration,
    battle_create_time: Duration,
    battle_run_time: Duration,
    total_time: Duration,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let num_battles: u32 = args.get(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(1000);

    println!("Profiling {} battles (single-threaded)...\n", num_battles);

    let overall_start = Instant::now();

    // Time dex loading
    let dex_start = Instant::now();
    let dex = Dex::global();
    let dex_load_time = dex_start.elapsed();

    let mut team_gen_time = Duration::ZERO;
    let mut battle_create_time = Duration::ZERO;
    let mut battle_run_time = Duration::ZERO;
    let mut total_turns = 0i32;

    for seed_num in 1..=num_battles {
        // Time team generation
        let team_start = Instant::now();
        let mut prng = PRNG::new(Some(PRNGSeed::Gen5([0, 0, 0, seed_num])));
        let team1 = team_generator::generate_random_team(&mut prng, &dex);
        let team2 = team_generator::generate_random_team(&mut prng, &dex);
        team_gen_time += team_start.elapsed();

        // Time battle creation
        let create_start = Instant::now();
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
        battle_create_time += create_start.elapsed();

        // Time battle execution
        let run_start = Instant::now();
        for i in 1..=100 {
            battle.make_choices(&["default", "default"]);
            battle.sent_log_pos = battle.log.len();
            if battle.ended || i >= 100 {
                break;
            }
        }
        battle_run_time += run_start.elapsed();
        total_turns += battle.turn;
    }

    let total_time = overall_start.elapsed();

    // Print results
    println!("=== Profiling Results ===\n");
    println!("Battles:     {:>8}", num_battles);
    println!("Total turns: {:>8}", total_turns);
    println!("Avg turns:   {:>8.1}", total_turns as f64 / num_battles as f64);
    println!();
    println!("Timing breakdown:");
    println!("  Dex load:       {:>8.2}ms ({:>5.1}%)",
        dex_load_time.as_secs_f64() * 1000.0,
        dex_load_time.as_secs_f64() / total_time.as_secs_f64() * 100.0);
    println!("  Team gen:       {:>8.2}ms ({:>5.1}%)",
        team_gen_time.as_secs_f64() * 1000.0,
        team_gen_time.as_secs_f64() / total_time.as_secs_f64() * 100.0);
    println!("  Battle create:  {:>8.2}ms ({:>5.1}%)",
        battle_create_time.as_secs_f64() * 1000.0,
        battle_create_time.as_secs_f64() / total_time.as_secs_f64() * 100.0);
    println!("  Battle run:     {:>8.2}ms ({:>5.1}%)",
        battle_run_time.as_secs_f64() * 1000.0,
        battle_run_time.as_secs_f64() / total_time.as_secs_f64() * 100.0);
    println!("  ─────────────────────────────");
    println!("  Total:          {:>8.2}ms", total_time.as_secs_f64() * 1000.0);
    println!();
    println!("Performance:");
    println!("  Battles/sec:    {:>8.1}", num_battles as f64 / total_time.as_secs_f64());
    println!("  Turns/sec:      {:>8.1}", total_turns as f64 / total_time.as_secs_f64());
    println!("  ms/battle:      {:>8.2}", total_time.as_secs_f64() * 1000.0 / num_battles as f64);
    println!("  ms/turn:        {:>8.3}", total_time.as_secs_f64() * 1000.0 / total_turns as f64);
}
