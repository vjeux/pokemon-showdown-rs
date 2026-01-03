/// Debug Battle State - Outputs detailed battle state at each turn
///
/// Usage: cargo run --example debug_battle_state [seed_number]

use pokemon_showdown::{Battle, BattleOptions, PlayerOptions, PokemonSet, PRNGSeed, ID};
use pokemon_showdown::battle::TeamFormat;
use pokemon_showdown::dex_data::{StatsTable, Gender};
use serde_json::json;
use std::fs;
use std::env;

fn main() {
    let seed_num: u32 = env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);

    let teams_file = format!("/tmp/teams-seed{}-rust.json", seed_num);

    if !std::path::Path::new(&teams_file).exists() {
        eprintln!("ERROR: Team file not found: {}", teams_file);
        eprintln!("Run: docker exec pokemon-rust-dev bash -c \"cd /home/builder/workspace && cargo run --example generate_test_teams_rust {}\" first", seed_num);
        std::process::exit(1);
    }

    let teams_json = fs::read_to_string(&teams_file).unwrap();

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

    let teams: Teams = serde_json::from_str(&teams_json).unwrap();

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
        }),
        p2: Some(PlayerOptions {
            name: "Player 2".to_string(),
            team: TeamFormat::Sets(team2),
            avatar: None,
            rating: None,
        }),
        ..Default::default()
    });

    eprintln!("# Rust Battle Debug - Seed {}", seed_num);
    eprintln!("# P1: {} vs P2: {}", teams.p1[0].name, teams.p2[0].name);
    eprintln!("");

    // Output initial state
    println!("{}", get_battle_state(&battle, 0));
    println!("---");

    // Run battle
    for i in 1..=100 {
        battle.make_choices(&["default", "default"]);

        // Output state after each turn
        println!("{}", get_battle_state(&battle, i));
        println!("---");

        if battle.ended || i >= 100 {
            eprintln!("# Battle ended: {}, Turn: {}, Total PRNG: {}",
                battle.ended, battle.turn, battle.prng.call_count);
            break;
        }
    }
}

fn get_battle_state(battle: &Battle, iteration: i32) -> String {
    let weather = if !battle.field.weather.is_empty() {
        battle.field.weather.as_str()
    } else {
        "none"
    };

    let terrain = if !battle.field.terrain.is_empty() {
        battle.field.terrain.as_str()
    } else {
        "none"
    };

    let mut p1_active = vec![];
    for active_idx in &battle.sides[0].active {
        if let Some(poke_idx) = active_idx {
            if let Some(pokemon) = battle.sides[0].pokemon.get(*poke_idx) {
                if !pokemon.fainted {
                    p1_active.push(json!({
                        "species": pokemon.species_id.as_str(),
                        "hp": pokemon.hp,
                        "maxhp": pokemon.maxhp,
                        "status": if pokemon.status.is_empty() { "none" } else { pokemon.status.as_str() },
                        "volatiles": pokemon.volatiles.keys().map(|k| k.as_str()).collect::<Vec<_>>()
                    }));
                }
            }
        }
    }

    let mut p1_team = vec![];
    for pokemon in &battle.sides[0].pokemon {
        p1_team.push(json!({
            "species": pokemon.species_id.as_str(),
            "hp": pokemon.hp,
            "maxhp": pokemon.maxhp,
            "status": if pokemon.status.is_empty() { "none" } else { pokemon.status.as_str() },
            "fainted": pokemon.fainted
        }));
    }

    let p1_side_conditions: Vec<_> = battle.sides[0].side_conditions.keys()
        .map(|k| k.as_str()).collect();

    let mut p2_active = vec![];
    for active_idx in &battle.sides[1].active {
        if let Some(poke_idx) = active_idx {
            if let Some(pokemon) = battle.sides[1].pokemon.get(*poke_idx) {
                if !pokemon.fainted {
                    p2_active.push(json!({
                        "species": pokemon.species_id.as_str(),
                        "hp": pokemon.hp,
                        "maxhp": pokemon.maxhp,
                        "status": if pokemon.status.is_empty() { "none" } else { pokemon.status.as_str() },
                        "volatiles": pokemon.volatiles.keys().map(|k| k.as_str()).collect::<Vec<_>>()
                    }));
                }
            }
        }
    }

    let mut p2_team = vec![];
    for pokemon in &battle.sides[1].pokemon {
        p2_team.push(json!({
            "species": pokemon.species_id.as_str(),
            "hp": pokemon.hp,
            "maxhp": pokemon.maxhp,
            "status": if pokemon.status.is_empty() { "none" } else { pokemon.status.as_str() },
            "fainted": pokemon.fainted
        }));
    }

    let p2_side_conditions: Vec<_> = battle.sides[1].side_conditions.keys()
        .map(|k| k.as_str()).collect();

    let state = json!({
        "iteration": iteration,
        "turn": battle.turn,
        "prng": battle.prng.call_count,
        "weather": weather,
        "terrain": terrain,
        "log": battle.log.clone(),
        "p1": {
            "active": p1_active,
            "team": p1_team,
            "sideConditions": p1_side_conditions
        },
        "p2": {
            "active": p2_active,
            "team": p2_team,
            "sideConditions": p2_side_conditions
        }
    });

    serde_json::to_string_pretty(&state).unwrap()
}
