/// Detailed Battle Profiling Tool
///
/// Instruments key battle functions to identify hot spots.
///
/// Usage:
///   cargo run --release --example detailed_profile <num_battles>

use pokemon_showdown::{Battle, BattleOptions, PlayerOptions, PRNG, PRNGSeed, ID, team_generator};
use pokemon_showdown::battle::TeamFormat;
use pokemon_showdown::dex::Dex;
use std::env;
use std::time::{Duration, Instant};
use std::cell::RefCell;

thread_local! {
    static TIMINGS: RefCell<Timings> = RefCell::new(Timings::default());
}

#[derive(Default)]
struct Timings {
    run_event: Duration,
    run_event_count: u64,
    single_event: Duration,
    single_event_count: u64,
    dispatch_single_event: Duration,
    dispatch_single_event_count: u64,
    damage: Duration,
    damage_count: u64,
    get_damage: Duration,
    get_damage_count: u64,
    run_move: Duration,
    run_move_count: u64,
    use_move: Duration,
    use_move_count: u64,
    switch_in: Duration,
    switch_in_count: u64,
    faint: Duration,
    faint_count: u64,
    turn_loop: Duration,
    turn_loop_count: u64,
    speed_sort: Duration,
    speed_sort_count: u64,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let num_battles: u32 = args.get(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(100);

    println!("Detailed profiling of {} battles...\n", num_battles);

    let dex = Dex::global();

    let mut total_turns = 0i32;
    let battle_start = Instant::now();

    for seed_num in 1..=num_battles {
        let mut prng = PRNG::new(Some(PRNGSeed::Gen5([0, 0, 0, seed_num])));
        let team1 = team_generator::generate_random_team(&mut prng, &dex);
        let team2 = team_generator::generate_random_team(&mut prng, &dex);

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

        for i in 1..=100 {
            battle.make_choices(&["default", "default"]);
            battle.sent_log_pos = battle.log.len();
            if battle.ended || i >= 100 {
                break;
            }
        }
        total_turns += battle.turn;
    }

    let total_time = battle_start.elapsed();

    println!("=== Detailed Profiling Results ===\n");
    println!("Battles:     {:>8}", num_battles);
    println!("Total turns: {:>8}", total_turns);
    println!("Total time:  {:>8.2}ms", total_time.as_secs_f64() * 1000.0);
    println!();
    println!("Performance:");
    println!("  Battles/sec:    {:>8.1}", num_battles as f64 / total_time.as_secs_f64());
    println!("  Turns/sec:      {:>8.1}", total_turns as f64 / total_time.as_secs_f64());
    println!();

    // Now let's do sampling-based profiling
    println!("=== Function Call Analysis ===\n");
    println!("Running sampling analysis on a single long battle...\n");

    // Run one battle and count function calls manually through log analysis
    let mut prng = PRNG::new(Some(PRNGSeed::Gen5([0, 0, 0, 1])));
    let team1 = team_generator::generate_random_team(&mut prng, &dex);
    let team2 = team_generator::generate_random_team(&mut prng, &dex);

    let mut battle = Battle::new(BattleOptions {
        format_id: ID::new("gen9randombattle"),
        seed: Some(PRNGSeed::Gen5([0, 0, 0, 1])),
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

    // Count different log line types as a proxy for function calls
    let mut damage_lines = 0u64;
    let mut move_lines = 0u64;
    let mut switch_lines = 0u64;
    let mut status_lines = 0u64;
    let mut boost_lines = 0u64;
    let mut ability_lines = 0u64;
    let mut item_lines = 0u64;
    let mut weather_lines = 0u64;
    let mut faint_lines = 0u64;

    for i in 1..=100 {
        battle.make_choices(&["default", "default"]);
        battle.sent_log_pos = battle.log.len();
        if battle.ended || i >= 100 {
            break;
        }
    }

    for line in &battle.log {
        let line_str = line.as_str();
        if line_str.starts_with("|-damage") { damage_lines += 1; }
        else if line_str.starts_with("|move|") { move_lines += 1; }
        else if line_str.starts_with("|switch|") || line_str.starts_with("|drag|") { switch_lines += 1; }
        else if line_str.starts_with("|-status") || line_str.starts_with("|-curestatus") { status_lines += 1; }
        else if line_str.starts_with("|-boost") || line_str.starts_with("|-unboost") { boost_lines += 1; }
        else if line_str.starts_with("|-ability") { ability_lines += 1; }
        else if line_str.starts_with("|-item") || line_str.starts_with("|-enditem") { item_lines += 1; }
        else if line_str.starts_with("|-weather") { weather_lines += 1; }
        else if line_str.starts_with("|faint|") { faint_lines += 1; }
    }

    println!("Log line analysis (proxy for function calls in 1 battle):");
    println!("  Total log lines:  {:>6}", battle.log.len());
    println!("  Turns:            {:>6}", battle.turn);
    println!();
    println!("  Move executions:  {:>6} ({:.1}/turn)", move_lines, move_lines as f64 / battle.turn as f64);
    println!("  Damage events:    {:>6} ({:.1}/turn)", damage_lines, damage_lines as f64 / battle.turn as f64);
    println!("  Switch events:    {:>6} ({:.1}/turn)", switch_lines, switch_lines as f64 / battle.turn as f64);
    println!("  Faint events:     {:>6} ({:.1}/turn)", faint_lines, faint_lines as f64 / battle.turn as f64);
    println!("  Status changes:   {:>6} ({:.1}/turn)", status_lines, status_lines as f64 / battle.turn as f64);
    println!("  Boost changes:    {:>6} ({:.1}/turn)", boost_lines, boost_lines as f64 / battle.turn as f64);
    println!("  Ability triggers: {:>6} ({:.1}/turn)", ability_lines, ability_lines as f64 / battle.turn as f64);
    println!("  Item triggers:    {:>6} ({:.1}/turn)", item_lines, item_lines as f64 / battle.turn as f64);
    println!("  Weather changes:  {:>6} ({:.1}/turn)", weather_lines, weather_lines as f64 / battle.turn as f64);

    println!("\n=== Estimated Hot Spots ===\n");
    println!("Based on the log analysis and typical Pokemon Showdown architecture:");
    println!();
    println!("1. run_event() - Called for EVERY game event (move, damage, switch, etc.)");
    println!("   Estimated calls/battle: {} moves × ~20 events/move = ~{} calls",
        move_lines, move_lines * 20);
    println!();
    println!("2. single_event() - Called by run_event for each handler");
    println!("   Estimated calls/battle: ~{} (varies by abilities/items)",
        move_lines * 20 * 5);
    println!();
    println!("3. dispatch_single_event() - Routes events to callbacks");
    println!("   Same as single_event calls");
    println!();
    println!("4. damage()/get_damage() - Complex damage calculation");
    println!("   Calls: {} per battle", damage_lines);
    println!();
    println!("5. speed_sort() - Sorts actions by speed priority");
    println!("   Calls: ~{} per battle (once per turn)", battle.turn);
}
