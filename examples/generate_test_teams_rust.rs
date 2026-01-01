/// Generate random battle teams in Rust
///
/// This generates teams using Rust's team generation logic,
/// allowing comparison with JavaScript team generation.
///
/// Usage: cargo run --example generate_test_teams_rust [seed_number]

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

fn main() {
    let seed_num: u16 = env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);

    eprintln!("Generating test teams for seed {}...", seed_num);

    // Load dex
    let dex = Dex::load_default().expect("Failed to load dex");

    // Create PRNG with seed
    let mut prng = PRNG::new(Some(PRNGSeed::Gen5([0, 0, 0, seed_num])));

    // Generate teams
    let team1 = team_generator::generate_random_team(&mut prng, &dex);
    let team2 = team_generator::generate_random_team(&mut prng, &dex);

    // Extract team data
    let export = TeamExport {
        p1: team1
            .iter()
            .map(|p| PokemonExport {
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
                    hp: p.evs.hp,
                    atk: p.evs.atk,
                    def: p.evs.def,
                    spa: p.evs.spa,
                    spd: p.evs.spd,
                    spe: p.evs.spe,
                },
                ivs: StatsExport {
                    hp: p.ivs.hp,
                    atk: p.ivs.atk,
                    def: p.ivs.def,
                    spa: p.ivs.spa,
                    spd: p.ivs.spd,
                    spe: p.ivs.spe,
                },
            })
            .collect(),
        p2: team2
            .iter()
            .map(|p| PokemonExport {
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
                    hp: p.evs.hp,
                    atk: p.evs.atk,
                    def: p.evs.def,
                    spa: p.evs.spa,
                    spd: p.evs.spd,
                    spe: p.evs.spe,
                },
                ivs: StatsExport {
                    hp: p.ivs.hp,
                    atk: p.ivs.atk,
                    def: p.ivs.def,
                    spa: p.ivs.spa,
                    spd: p.ivs.spd,
                    spe: p.ivs.spe,
                },
            })
            .collect(),
    };

    // Write to /tmp (container's /tmp)
    let filename = format!("/tmp/teams-seed{}-rust.json", seed_num);
    let json = serde_json::to_string_pretty(&export).unwrap();
    fs::write(&filename, json).unwrap();

    eprintln!("✓ Generated teams for seed {}", seed_num);
    eprintln!("  P1: {} ({})", export.p1[0].name, export.p1[0].species);
    eprintln!("  P2: {} ({})", export.p2[0].name, export.p2[0].species);
    eprintln!("  File: {}", filename);
}
