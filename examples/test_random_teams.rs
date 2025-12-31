//! Test PRNG synchronization with JavaScript using randomly generated teams

use pokemon_showdown::{Battle, BattleOptions, PRNGSeed, PlayerOptions, PokemonSet, ID};
use std::fs;

fn main() {
    println!("Testing with randomly generated teams for each seed...\n");

    // Focus on seed 1 for debugging
    for seed in 1..=1 {
        // Load the team file for this seed
        let team_file = format!("test-teams-{}.json", seed);
        let teams_json = match fs::read_to_string(&team_file) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Failed to read {}: {}", team_file, e);
                continue;
            }
        };

        let teams: serde_json::Value = match serde_json::from_str(&teams_json) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Failed to parse {}: {}", team_file, e);
                continue;
            }
        };

        // Parse teams
        let p1_team: Vec<PokemonSet> = match serde_json::from_value(teams["p1"].clone()) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Failed to parse P1 team from {}: {}", team_file, e);
                continue;
            }
        };

        let p2_team: Vec<PokemonSet> = match serde_json::from_value(teams["p2"].clone()) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Failed to parse P2 team from {}: {}", team_file, e);
                continue;
            }
        };

        // Create battle
        let mut battle = Battle::new(BattleOptions {
            format_id: ID::new("gen9randombattle"),
            seed: Some(PRNGSeed::Gen5([0, 0, 0, seed as u16])),
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

        battle.make_choices(&["default", "default"]);

        // Print battle log
        for line in &battle.log {
            println!("{}", line);
        }

        if let (Some(p1_idx), Some(p2_idx)) = (battle.sides[0].active[0], battle.sides[1].active[0]) {
            let p1_active = &battle.sides[0].pokemon[p1_idx];
            let p2_active = &battle.sides[1].pokemon[p2_idx];
            println!("Seed {}: {} ({}/{}) vs {} ({}/{})",
                seed,
                p1_active.name, p1_active.hp, p1_active.maxhp,
                p2_active.name, p2_active.hp, p2_active.maxhp
            );
        }
    }
}
