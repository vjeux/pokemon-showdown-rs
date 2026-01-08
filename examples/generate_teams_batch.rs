/// Batch Team Generator (Rust)
///
/// Generates teams for multiple seeds in a single process.
///
/// Usage: cargo run --example generate_teams_batch <start_seed> <end_seed>

use pokemon_showdown::{Dex, PRNG, PRNGSeed, team_generator};
use serde::Serialize;
use std::fs;
use std::env;

#[derive(Serialize)]
struct TeamExport {
    p1: Vec<PokemonExport>,
    p2: Vec<PokemonExport>,
}

#[derive(Serialize)]
struct PokemonExport {
    name: String,
    species: String,
    level: u8,
    ability: String,
    item: String,
    nature: String,
    gender: String,
    moves: Vec<String>,
    evs: StatsExport,
    ivs: StatsExport,
}

#[derive(Serialize)]
struct StatsExport {
    hp: i32,
    atk: i32,
    def: i32,
    spa: i32,
    spd: i32,
    spe: i32,
}

fn generate_team(seed_num: u16, dex: &Dex) -> Result<(), String> {
    let mut prng = PRNG::new(Some(PRNGSeed::Gen5([0, 0, 0, seed_num])));
    
    let team1 = team_generator::generate_random_team(&mut prng, dex);
    let team2 = team_generator::generate_random_team(&mut prng, dex);
    
    let export = TeamExport {
        p1: team1.iter().map(|p| PokemonExport {
            name: p.name.clone(),
            species: p.species.clone(),
            level: p.level,
            ability: p.ability.clone(),
            item: p.item.clone(),
            nature: p.nature.clone(),
            gender: match p.gender {
                pokemon_showdown::dex_data::Gender::Male => "M".to_string(),
                pokemon_showdown::dex_data::Gender::Female => "F".to_string(),
                pokemon_showdown::dex_data::Gender::None => "".to_string(),
            },
            moves: p.moves.clone(),
            evs: StatsExport {
                hp: p.evs.hp, atk: p.evs.atk, def: p.evs.def,
                spa: p.evs.spa, spd: p.evs.spd, spe: p.evs.spe,
            },
            ivs: StatsExport {
                hp: p.ivs.hp, atk: p.ivs.atk, def: p.ivs.def,
                spa: p.ivs.spa, spd: p.ivs.spd, spe: p.ivs.spe,
            },
        }).collect(),
        p2: team2.iter().map(|p| PokemonExport {
            name: p.name.clone(),
            species: p.species.clone(),
            level: p.level,
            ability: p.ability.clone(),
            item: p.item.clone(),
            nature: p.nature.clone(),
            gender: match p.gender {
                pokemon_showdown::dex_data::Gender::Male => "M".to_string(),
                pokemon_showdown::dex_data::Gender::Female => "F".to_string(),
                pokemon_showdown::dex_data::Gender::None => "".to_string(),
            },
            moves: p.moves.clone(),
            evs: StatsExport {
                hp: p.evs.hp, atk: p.evs.atk, def: p.evs.def,
                spa: p.evs.spa, spd: p.evs.spd, spe: p.evs.spe,
            },
            ivs: StatsExport {
                hp: p.ivs.hp, atk: p.ivs.atk, def: p.ivs.def,
                spa: p.ivs.spa, spd: p.ivs.spd, spe: p.ivs.spe,
            },
        }).collect(),
    };
    
    let json = serde_json::to_string_pretty(&export)
        .map_err(|e| format!("Failed to serialize: {}", e))?;
    
    let output_file = format!("/tmp/teams-seed{}-rust.json", seed_num);
    fs::write(&output_file, json)
        .map_err(|e| format!("Failed to write: {}", e))?;
    
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let start_seed: u16 = args.get(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);
    
    let end_seed: u16 = args.get(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(start_seed);
    
    // Load dex once
    let dex = Dex::load_default().expect("Failed to load dex");
    
    for seed_num in start_seed..=end_seed {
        match generate_team(seed_num, &dex) {
            Ok(()) => println!("SEED {}: OK", seed_num),
            Err(e) => println!("SEED {}: ERROR - {}", seed_num, e),
        }
    }
}
