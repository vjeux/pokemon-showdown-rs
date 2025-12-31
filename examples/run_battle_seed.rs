/// Run battle with configurable seed
/// Usage: cargo run --example run_battle_seed [seed]

use pokemon_showdown::{Battle, BattleOptions, PlayerOptions, PokemonSet, PRNGSeed, ID};
use pokemon_showdown::dex_data::{StatsTable, Gender};
use std::fs;

fn main() {
    // Get seed from command line, default to 1
    let seed_value: u16 = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);

    // Load the teams we generated
    let teams_file = if seed_value == 1 {
        "teams-rust.json".to_string()
    } else {
        format!("teams-rust-seed{}.json", seed_value)
    };
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
        seed: Some(PRNGSeed::Gen5([0, 0, 0, seed_value])),
        p1: Some(PlayerOptions {
            name: "Player 1".to_string(),
            team: team1,
            avatar: None,
            rating: None,
        }),
        p2: Some(PlayerOptions {
            name: "Player 2".to_string(),
            team: team2,
            avatar: None,
            rating: None,
        }),
        ..Default::default()
    });

    println!("RUST: Seed {} - Battle created", seed_value);

    // Run 20 turns
    for turn_num in 1..=20 {
        let prng_before = battle.prng.call_count;
        battle.make_choices(&["default", "default"]);
        let prng_after = battle.prng.call_count;

        println!("RUST: Seed {} Turn {} - PRNG calls: {} (total: {})",
            seed_value, turn_num, prng_after - prng_before, prng_after);
    }

    println!("RUST: Seed {} - Total PRNG calls after 20 turns: {}", seed_value, battle.prng.call_count);
}
