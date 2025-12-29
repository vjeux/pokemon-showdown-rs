use pokemon_showdown::{Battle, BattleOptions, PlayerOptions, PRNGSeed};
use std::fs;

#[test]
fn test_prng_sequence() {
    // Load test teams
    let test_teams_json = fs::read_to_string("test-teams.json").expect("test-teams.json not found");
    let test_teams: serde_json::Value = serde_json::from_str(&test_teams_json).unwrap();

    let p1_team: Vec<_> = test_teams["p1"].as_array().unwrap().iter().map(|p| {
        pokemon_showdown::PokemonSet {
            species: p["species"].as_str().unwrap().to_string(),
            level: p["level"].as_u64().unwrap() as u8,
            ability: p["ability"].as_str().unwrap().to_string(),
            item: p["item"].as_str().unwrap().to_string(),
            nature: p["nature"].as_str().unwrap().to_string(),
            moves: p["moves"].as_array().unwrap().iter()
                .map(|m| m.as_str().unwrap().to_string()).collect(),
            evs: pokemon_showdown::dex_data::StatsTable::new(
                p["evs"]["hp"].as_i64().unwrap() as i32,
                p["evs"]["atk"].as_i64().unwrap() as i32,
                p["evs"]["def"].as_i64().unwrap() as i32,
                p["evs"]["spa"].as_i64().unwrap() as i32,
                p["evs"]["spd"].as_i64().unwrap() as i32,
                p["evs"]["spe"].as_i64().unwrap() as i32,
            ),
            ivs: pokemon_showdown::dex_data::StatsTable::new(
                p["ivs"]["hp"].as_i64().unwrap() as i32,
                p["ivs"]["atk"].as_i64().unwrap() as i32,
                p["ivs"]["def"].as_i64().unwrap() as i32,
                p["ivs"]["spa"].as_i64().unwrap() as i32,
                p["ivs"]["spd"].as_i64().unwrap() as i32,
                p["ivs"]["spe"].as_i64().unwrap() as i32,
            ),
            ..Default::default()
        }
    }).collect();

    let p2_team = p1_team.clone();

    let mut battle = Battle::new(BattleOptions {
        format_id: pokemon_showdown::ID::new("gen9randombattle"),
        seed: Some(PRNGSeed::Gen5([12345, 23456, 11111, 22222])),
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

    // Match JavaScript test structure:
    // JavaScript calls make_choices twice:
    // 1. First call executes moves (during "team preview" phase)
    // 2. Second call executes Turn 1

    // First make_choices - executes moves
    battle.make_choices("move 1", "move 2");

    // Second make_choices - Turn 1
    battle.make_choices("move 1", "move 2");

    // Print HP values to compare with JavaScript
    if let Some(p1) = battle.sides[0].pokemon.get(0) {
        println!("P1 HP: {}/{}", p1.hp, p1.maxhp);
    }
    if let Some(p2) = battle.sides[1].pokemon.get(0) {
        println!("P2 HP: {}/{}", p2.hp, p2.maxhp);
    }

    println!("Test completed");
}
