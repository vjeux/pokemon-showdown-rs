
use pokemon_showdown::{Battle, BattleOptions, PlayerOptions, PokemonSet, PRNGSeed, ID};
use pokemon_showdown::dex_data::StatsTable;
use std::fs;

fn main() {
    // Load test teams
    let test_teams_json = fs::read_to_string("test-teams.json").unwrap();

    #[derive(serde::Deserialize)]
    struct TestTeams {
        p1: Vec<TestPokemon>,
        p2: Vec<TestPokemon>,
    }

    #[derive(serde::Deserialize)]
    struct TestPokemon {
        species: String,
        level: u8,
        ability: String,
        item: String,
        nature: String,
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

    let test_teams: TestTeams = serde_json::from_str(&test_teams_json).unwrap();

    let team1: Vec<_> = test_teams.p1.iter().map(|set| PokemonSet {
        name: set.species.clone(),
        species: set.species.clone(),
        level: set.level,
        ability: set.ability.clone(),
        item: set.item.clone(),
        nature: set.nature.clone(),
        moves: set.moves.clone(),
        evs: StatsTable::new(set.evs.hp, set.evs.atk, set.evs.def, set.evs.spa, set.evs.spd, set.evs.spe),
        ivs: StatsTable::new(set.ivs.hp, set.ivs.atk, set.ivs.def, set.ivs.spa, set.ivs.spd, set.ivs.spe),
        ..Default::default()
    }).collect();

    let team2: Vec<_> = test_teams.p2.iter().map(|set| PokemonSet {
        name: set.species.clone(),
        species: set.species.clone(),
        level: set.level,
        ability: set.ability.clone(),
        item: set.item.clone(),
        nature: set.nature.clone(),
        moves: set.moves.clone(),
        evs: StatsTable::new(set.evs.hp, set.evs.atk, set.evs.def, set.evs.spa, set.evs.spd, set.evs.spe),
        ivs: StatsTable::new(set.ivs.hp, set.ivs.atk, set.ivs.def, set.ivs.spa, set.ivs.spd, set.ivs.spe),
        ..Default::default()
    }).collect();

    let mut battle = Battle::new(BattleOptions {
        format_id: ID::new("gen9randombattle"),
        seed: Some(PRNGSeed::Gen5([12345, 23456, 11111, 22222])),
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

    // Make team preview choices
    println!("RUST: Making team preview choices...");
    battle.make_choices(&["default", "default"]);

    println!("RUST: After team preview, turn: {}", battle.turn);
    if let Some(p1_active) = battle.sides[0].get_active(0) {
        println!("RUST: P1 active: {}", p1_active.name);
        println!("RUST: P1 HP: {}/{}", p1_active.hp, p1_active.maxhp);
    }
    if let Some(p2_active) = battle.sides[1].get_active(0) {
        println!("RUST: P2 active: {}", p2_active.name);
        println!("RUST: P2 HP: {}/{}", p2_active.hp, p2_active.maxhp);
    }

    // Make turn 1 choices
    println!("RUST: Making turn 1 choices (move 1, move 1)...");
    battle.make_choices(&["move 1", "move 1"]);

    println!("RUST: After turn 1, turn: {}", battle.turn);
    if let Some(p1_active) = battle.sides[0].get_active(0) {
        println!("RUST: P1 HP: {}/{}", p1_active.hp, p1_active.maxhp);
    }
    if let Some(p2_active) = battle.sides[1].get_active(0) {
        println!("RUST: P2 HP: {}/{}", p2_active.hp, p2_active.maxhp);
    }

    // Make turn 2 choices
    println!("RUST: Making turn 2 choices (move 1, move 1)...");
    battle.make_choices(&["move 1", "move 1"]);

    println!("RUST: After turn 2, turn: {}", battle.turn);
    if let Some(p1_active) = battle.sides[0].get_active(0) {
        println!("RUST: P1 HP: {}/{}", p1_active.hp, p1_active.maxhp);
    }
    if let Some(p2_active) = battle.sides[1].get_active(0) {
        println!("RUST: P2 HP: {}/{}", p2_active.hp, p2_active.maxhp);
    }

    println!("\nRUST LOG:\n{}", battle.get_log());
}
