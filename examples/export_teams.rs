/// Example: Generate and export random teams to JSON for comparison with JavaScript
///
/// Usage: cargo run --example export_teams [seed]
/// Default seed is 1 if not provided

use pokemon_showdown::{Dex, PRNG, PRNGSeed};
use serde::Serialize;

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
    // Get seed from command line argument, default to 1
    let seed_value: u16 = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);

    // Load dex
    let dex = Dex::load_default().expect("Failed to load dex");

    // Create PRNG with seed [0, 0, 0, seed_value]
    let mut prng = PRNG::new(Some(PRNGSeed::Gen5([0, 0, 0, seed_value])));

    // Generate teams
    let (team1, team2) = pokemon_showdown::team_generator::generate_random_teams(&mut prng, &dex);

    // Convert to export format
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
                    pokemon_showdown::Gender::Male => "M".to_string(),
                    pokemon_showdown::Gender::Female => "F".to_string(),
                    pokemon_showdown::Gender::None => "".to_string(),
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
                    pokemon_showdown::Gender::Male => "M".to_string(),
                    pokemon_showdown::Gender::Female => "F".to_string(),
                    pokemon_showdown::Gender::None => "".to_string(),
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

    // Export to JSON
    let json = serde_json::to_string_pretty(&export).expect("Failed to serialize");
    let filename = if seed_value == 1 {
        "teams-rust.json".to_string()
    } else {
        format!("teams-rust-seed{}.json", seed_value)
    };
    std::fs::write(&filename, json).expect("Failed to write file");

    println!("Teams exported to {}", filename);
    println!(
        "P1 team: {}",
        export
            .p1
            .iter()
            .map(|p| p.species.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!(
        "P2 team: {}",
        export
            .p2
            .iter()
            .map(|p| p.species.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    );
}
