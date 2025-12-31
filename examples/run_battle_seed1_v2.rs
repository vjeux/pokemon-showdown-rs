/// Run battle comparison with seed 1 - NO team preview choice

use pokemon_showdown::{Battle, BattleOptions, PlayerOptions, PokemonSet, PRNGSeed, ID};
use pokemon_showdown::dex_data::{StatsTable, Gender};
use std::fs;

fn main() {
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

    println!("RUST: Battle created");
    println!("RUST: Turn: {}", battle.turn);

    if let Some(p1_active) = battle.sides[0].get_active(0) {
        println!("RUST: P1 active: {}", p1_active.name);
        println!("RUST: P1 HP: {}/{}", p1_active.hp, p1_active.maxhp);
    } else {
        println!("RUST: P1 has no active Pokemon yet");
    }
    if let Some(p2_active) = battle.sides[1].get_active(0) {
        println!("RUST: P2 active: {}", p2_active.name);
        println!("RUST: P2 HP: {}/{}", p2_active.hp, p2_active.maxhp);
    } else {
        println!("RUST: P2 has no active Pokemon yet");
    }

    // Run battle until it ends (max 50 turns)
    let mut turn_num = 1;
    while !battle.ended && turn_num <= 50 {
        println!("RUST: Making turn {} choices...", turn_num);

        // Use default choice which handles both moves and switches
        battle.make_choices(&["default", "default"]);

        println!("RUST: After turn {}, turn: {}", turn_num, battle.turn);
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

        if battle.ended {
            println!("RUST: Battle ended!");
            println!("RUST: Winner: {:?}", battle.winner);
            break;
        }

        turn_num += 1;
    }

    let log = battle.get_log();
    println!("\nRUST LOG:");
    println!("{}", log);

    // Save log to file
    fs::write("battle-rust-v2.log", log).unwrap();
    println!("\nLog saved to battle-rust-v2.log");
}
