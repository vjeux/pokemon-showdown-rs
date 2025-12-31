/// Run battle comparison with seed 1 - PRNG Tracing

use pokemon_showdown::{Battle, BattleOptions, PlayerOptions, PokemonSet, PRNGSeed, ID};
use pokemon_showdown::dex_data::{StatsTable, Gender};
use std::fs;

fn main() {
    // Enable PRNG logging
    std::env::set_var("RUST_LOG_PRNG", "1");

    // Load the teams we generated
    let teams_json = fs::read_to_string("teams-rust.json").unwrap();

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
        seed: Some(PRNGSeed::Gen5([0, 0, 0, 1])),
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

    println!("RUST: Battle created, turn: {}", battle.turn);

    if let Some(p1_active) = battle.sides[0].get_active(0) {
        println!("RUST: P1 active: {} {}/{}", p1_active.name, p1_active.hp, p1_active.maxhp);
    }
    if let Some(p2_active) = battle.sides[1].get_active(0) {
        println!("RUST: P2 active: {} {}/{}", p2_active.name, p2_active.hp, p2_active.maxhp);
    }

    // Run only up to turn 13
    for turn_num in 1..=13 {
        println!("\n=== RUST: Making turn {} choices ===", turn_num);
        let prng_before = battle.prng.call_count;

        battle.make_choices(&["default", "default"]);

        let prng_after = battle.prng.call_count;
        println!("RUST: Turn {} used {} PRNG calls (total: {})", turn_num, prng_after - prng_before, prng_after);
        println!("RUST: After turn {}, battle.turn: {}", turn_num, battle.turn);

        if let Some(p1_active) = battle.sides[0].get_active(0) {
            if p1_active.hp > 0 {
                println!("RUST: P1 HP: {}/{}", p1_active.hp, p1_active.maxhp);
            } else {
                println!("RUST: P1 fainted");
            }
        }
        if let Some(p2_active) = battle.sides[1].get_active(0) {
            if p2_active.hp > 0 {
                println!("RUST: P2 HP: {}/{}", p2_active.hp, p2_active.maxhp);
            } else {
                println!("RUST: P2 fainted");
            }
        }
    }

    println!("\nTotal PRNG calls: {}", battle.prng.call_count);
}
