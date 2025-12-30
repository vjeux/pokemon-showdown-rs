//! Test PRNG synchronization with JavaScript

use pokemon_showdown::{Battle, BattleOptions, PRNGSeed, PlayerOptions, PokemonSet, ID};
use pokemon_showdown::dex_data::StatsTable;

fn main() {
    // Manually create teams matching test-teams.json
    let p1_team = vec![
        PokemonSet {
            name: "Passimian".to_string(),
            species: "Passimian".to_string(),
            level: 83,
            ability: "Defiant".to_string(),
            item: "Leftovers".to_string(),
            nature: "Hardy".to_string(),
            moves: vec!["knockoff".to_string(), "bulkup".to_string(), "gunkshot".to_string(), "drainpunch".to_string()],
            evs: StatsTable::new(85, 85, 85, 85, 85, 85),
            ivs: StatsTable::new(31, 31, 31, 31, 31, 31),
            ..Default::default()
        },
    ];

    let p2_team = vec![
        PokemonSet {
            name: "Passimian".to_string(),
            species: "Passimian".to_string(),
            level: 83,
            ability: "Defiant".to_string(),
            item: "Leftovers".to_string(),
            nature: "Hardy".to_string(),
            moves: vec!["knockoff".to_string(), "bulkup".to_string(), "gunkshot".to_string(), "drainpunch".to_string()],
            evs: StatsTable::new(85, 85, 85, 85, 85, 85),
            ivs: StatsTable::new(31, 31, 31, 31, 31, 31),
            ..Default::default()
        },
    ];

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
    battle.make_choices(&["default", "default"]);

    // Print final HP
    if let Some(p1_idx) = battle.sides[0].active[0] {
        let p1_active = &battle.sides[0].pokemon[p1_idx];
        println!("\nP1: {} {} / {}", p1_active.name, p1_active.hp, p1_active.maxhp);
    }

    if let Some(p2_idx) = battle.sides[1].active[0] {
        let p2_active = &battle.sides[1].pokemon[p2_idx];
        println!("P2: {} {} / {}", p2_active.name, p2_active.hp, p2_active.maxhp);
    }
}
