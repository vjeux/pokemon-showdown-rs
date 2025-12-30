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

    // Test seeds 1-100
    for seed in 1..=100 {
        let mut battle = Battle::new(BattleOptions {
            format_id: ID::new("gen9randombattle"),
            seed: Some(PRNGSeed::Gen5([0, 0, 0, seed])),
            p1: Some(PlayerOptions {
                name: "Player 1".to_string(),
                team: p1_team.clone(),
                avatar: None,
                rating: None,
            }),
            p2: Some(PlayerOptions {
                name: "Player 2".to_string(),
                team: p2_team.clone(),
                avatar: None,
                rating: None,
            }),
            ..Default::default()
        });

        battle.make_choices(&["default", "default"]);

        if let (Some(p1_idx), Some(p2_idx)) = (battle.sides[0].active[0], battle.sides[1].active[0]) {
            let p1_active = &battle.sides[0].pokemon[p1_idx];
            let p2_active = &battle.sides[1].pokemon[p2_idx];
            println!("{},{},{}", seed, p1_active.hp, p2_active.hp);
        }
    }
}