//! Test PRNG synchronization with JavaScript

use pokemon_showdown::{Battle, BattleOptions, PRNGSeed, PlayerOptions, PokemonSet, ID};
use std::fs;

fn main() {
    // Load test teams
    let teams_json = fs::read_to_string("test-teams.json")
        .expect("Failed to read test-teams.json");
    let teams: serde_json::Value = serde_json::from_str(&teams_json)
        .expect("Failed to parse test-teams.json");

    // Extract teams
    let p1_team: Vec<PokemonSet> = serde_json::from_value(teams["p1"].clone())
        .expect("Failed to parse P1 team");
    let p2_team: Vec<PokemonSet> = serde_json::from_value(teams["p2"].clone())
        .expect("Failed to parse P2 team");

    // Create battle with seed [0,0,0,2]
    let mut battle = Battle::new(BattleOptions {
        format_id: ID::new("gen9randombattle"),
        seed: Some(PRNGSeed::Gen5([0, 0, 0, 2])),
        p1: Some(PlayerOptions {
            name: "Player 1".to_string(),
            team: p1_team,
            avatar: None,
            rating: None,
        }),
        p2: Some(PlayerOptions {
            name: "Player 2".to_string(),
            team: p2_team,
            avatar: None,
            rating: None,
        }),
        ..Default::default()
    });

    // Make default choices (both attack)
    battle.make_choices("default", "default");

    // Print final HP
    let p1_active = &battle.sides[0].pokemon[battle.sides[0].active[0]];
    let p2_active = &battle.sides[1].pokemon[battle.sides[1].active[0]];

    println!("\nP1: {} {} / {}", p1_active.name, p1_active.hp, p1_active.max_hp);
    println!("P2: {} {} / {}", p2_active.name, p2_active.hp, p2_active.max_hp);
}
