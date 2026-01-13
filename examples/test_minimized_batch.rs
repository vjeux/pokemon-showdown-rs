/// Batch Battle Test Runner for Minimized Seeds (Rust)
///
/// Runs all minimized seeds listed in a file in a single process for efficiency.
///
/// Usage: cargo run --release --example test_minimized_batch <seeds_file>
/// where seeds_file contains one seed number per line
/// Output: Results written to /tmp/rust-battle-seed<N>.txt for each seed

use pokemon_showdown::{Battle, BattleOptions, PlayerOptions, PokemonSet, PRNGSeed, ID};
use pokemon_showdown::battle::TeamFormat;
use pokemon_showdown::dex_data::{StatsTable, Gender};
use std::fs;
use std::env;
use std::io::Write;

fn run_battle(seed_num: u32) -> Result<usize, String> {
    let teams_file = format!("/tmp/teams-seed{}-rust.json", seed_num);

    if !std::path::Path::new(&teams_file).exists() {
        return Err(format!("Team file not found: {}", teams_file));
    }

    let teams_json = fs::read_to_string(&teams_file)
        .map_err(|e| format!("Failed to read team file: {}", e))?;

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

    let mut battle = Battle::new(BattleOptions {
        format_id: ID::new("gen9randombattle"),
        seed: Some(PRNGSeed::Gen5([0, 0, 0, seed_num as u16])),
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

    let mut results = Vec::new();

    for i in 1..=100 {
        let prng_before = battle.prng.call_count;
        battle.make_choices(&["default", "default"]);

        // Reset log position to prevent "LINE LIMIT EXCEEDED" check from failing
        battle.sent_log_pos = battle.log.len();

        let prng_after = battle.prng.call_count;

        let p1_active: Vec<String> = battle.sides[0].active.iter()
            .map(|slot| {
                if let Some(poke_idx) = slot {
                    if let Some(pokemon) = battle.sides[0].pokemon.get(*poke_idx) {
                        format!("{}({}/{})", pokemon.name, pokemon.hp, pokemon.maxhp)
                    } else {
                        "none".to_string()
                    }
                } else {
                    "none".to_string()
                }
            })
            .collect();

        let p2_active: Vec<String> = battle.sides[1].active.iter()
            .map(|slot| {
                if let Some(poke_idx) = slot {
                    if let Some(pokemon) = battle.sides[1].pokemon.get(*poke_idx) {
                        format!("{}({}/{})", pokemon.name, pokemon.hp, pokemon.maxhp)
                    } else {
                        "none".to_string()
                    }
                } else {
                    "none".to_string()
                }
            })
            .collect();

        results.push(format!(
            "#{}: turn={}, prng={}->{}, P1=[{}], P2=[{}]",
            i,
            battle.turn,
            prng_before,
            prng_after,
            p1_active.join(", "),
            p2_active.join(", ")
        ));

        if battle.ended || i >= 100 {
            break;
        }
    }

    let output_file = format!("/tmp/rust-battle-seed{}.txt", seed_num);
    let mut file = fs::File::create(&output_file)
        .map_err(|e| format!("Failed to create output file: {}", e))?;

    for line in &results {
        writeln!(file, "{}", line).map_err(|e| format!("Failed to write: {}", e))?;
    }

    Ok(results.len())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <seeds_file>", args[0]);
        eprintln!("  seeds_file: file containing one seed number per line");
        std::process::exit(1);
    }

    let seeds_file = &args[1];
    let seeds_content = fs::read_to_string(seeds_file)
        .expect("Failed to read seeds file");

    let seeds: Vec<u32> = seeds_content
        .lines()
        .filter_map(|line| line.trim().parse().ok())
        .collect();

    println!("Running {} minimized seeds...", seeds.len());

    let mut success_count = 0;
    let mut error_count = 0;

    for seed_num in seeds {
        match run_battle(seed_num) {
            Ok(_turns) => {
                success_count += 1;
            }
            Err(e) => {
                eprintln!("Seed {}: ERROR - {}", seed_num, e);
                error_count += 1;
            }
        }
    }

    println!("Done: {} successful, {} errors", success_count, error_count);
}
