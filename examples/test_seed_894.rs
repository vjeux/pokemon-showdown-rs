use pokemon_showdown::battle::{Battle, PlayerOptions, BattleOptions};
use pokemon_showdown::dex_data::{SideID, ID};
use pokemon_showdown::pokemon::PokemonSet;
use pokemon_showdown::prng::PRNGSeed;
use std::fs;

fn main() {
    // Load teams
    let teams_json = fs::read_to_string("test-teams-1.json").expect("Failed to read teams file");
    let teams: serde_json::Value = serde_json::from_str(&teams_json).expect("Failed to parse JSON");

    // Parse teams
    let p1_team: Vec<PokemonSet> = serde_json::from_value(teams["p1"].clone()).expect("Failed to parse P1 team");
    let p2_team: Vec<PokemonSet> = serde_json::from_value(teams["p2"].clone()).expect("Failed to parse P2 team");

    // Create battle with seed 894
    let mut battle = Battle::new(BattleOptions {
        format_id: ID::new("gen9randombattle"),
        format_name: None,
        game_type: None,
        seed: Some(PRNGSeed::Gen5([0, 0, 0, 894])),  // Seed 894
        rated: None,
        debug: false,
        strict_choices: false,
        force_random_chance: None,
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
        p3: None,
        p4: None,
    });

    // Make choices for turn 1
    battle.choose(SideID::P1, "default");
    battle.choose(SideID::P2, "default");

    // Get pokemon status after turn 1
    let p1_pokemon = &battle.sides[0].pokemon[0];
    let p2_pokemon = &battle.sides[1].pokemon[0];

    println!("After Turn 1:");
    println!("P1 {}: {}/{} HP", p1_pokemon.name, p1_pokemon.hp, p1_pokemon.maxhp);
    println!("P2 {}: {}/{} HP", p2_pokemon.name, p2_pokemon.hp, p2_pokemon.maxhp);

    // Calculate damage taken
    let p1_damage = p1_pokemon.maxhp - p1_pokemon.hp;
    let p2_damage = p2_pokemon.maxhp - p2_pokemon.hp;

    println!("");
    println!("Damage taken:");
    println!("P1 {}: {} damage", p1_pokemon.name, p1_damage);
    println!("P2 {}: {} damage", p2_pokemon.name, p2_damage);
}
