#!/bin/bash

cd /Users/vjeux/random/showdown/pokemon-showdown-rs

# Create a simple Rust program to test seed 1
docker exec pokemon-rust-dev bash -c 'cat > /home/builder/workspace/examples/test_seed_1.rs << '"'"'RUSTEOF'"'"'
use pokemon_showdown::battle::Battle;
use std::fs;

fn main() {
    // Load teams
    let teams_json = fs::read_to_string("test-teams-1.json").expect("Failed to read teams file");
    let teams: serde_json::Value = serde_json::from_str(&teams_json).expect("Failed to parse JSON");

    // Create battle
    let mut battle = Battle::new(
        "gen9randombattle",
        [0, 0, 0, 1], // seed
    );

    // Set teams
    battle.set_player(0, "Player 1", teams["p1"].as_array().unwrap());
    battle.set_player(1, "Player 2", teams["p2"].as_array().unwrap());

    // Start battle
    battle.start();

    // Make choices for turn 1
    battle.choose(pokemon_showdown::battle::SideID::P1, "default");
    battle.choose(pokemon_showdown::battle::SideID::P2, "default");

    // Get pokemon status after turn 1
    let p1_pokemon = &battle.sides[0].pokemon[0];
    let p2_pokemon = &battle.sides[1].pokemon[0];

    println!("After Turn 1:");
    println!("P1 {}: {}/{} HP", p1_pokemon.name, p1_pokemon.hp, p1_pokemon.max_hp);
    println!("P2 {}: {}/{} HP", p2_pokemon.name, p2_pokemon.hp, p2_pokemon.max_hp);

    // Calculate damage taken
    let p1_damage = p1_pokemon.max_hp - p1_pokemon.hp;
    let p2_damage = p2_pokemon.max_hp - p2_pokemon.hp;

    println!("");
    println!("Damage taken:");
    println!("P1 {}: {} damage", p1_pokemon.name, p1_damage);
    println!("P2 {}: {} damage", p2_pokemon.name, p2_damage);
}
RUSTEOF
'

# Compile and run
docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && cargo run --example test_seed_1 2>&1" | grep -E 'After Turn|P1 |P2 |Damage|FLUFFY|HANDLE_ABILITY_EVENT.*SourceModifyDamage'
