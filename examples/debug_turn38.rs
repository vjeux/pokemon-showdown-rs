/// Run seed 1 battle and show PRNG calls for Turn 38
use pokemon_showdown::{Battle, BattleOptions, PlayerOptions, PokemonSet, PRNGSeed, ID};
use pokemon_showdown::dex_data::{StatsTable, Gender};
use std::fs;

fn main() {
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

    // Run up to turn 38
    for turn_num in 1..=40 {
        let prng_before = battle.prng.call_count;

        // Print Pokemon status before turn 37 and 38
        if turn_num == 37 || turn_num == 38 {
            eprintln!("\n=== Before Turn {} ===", turn_num);
            for (side_idx, side) in battle.sides.iter().enumerate() {
                eprintln!("Side {}:", side_idx);
                for (poke_idx, pokemon) in side.pokemon.iter().enumerate() {
                    eprintln!("  [{}] {}: HP={}/{} fainted={} active={}",
                        poke_idx, pokemon.name, pokemon.hp, pokemon.maxhp,
                        pokemon.fainted, pokemon.is_active);
                }
            }
        }

        battle.make_choices(&["default", "default"]);

        let prng_after = battle.prng.call_count;
        eprintln!("Turn {}: {} PRNG calls (cumulative: {} -> {})",
            turn_num, prng_after - prng_before, prng_before, prng_after);

        // Print Pokemon status after turn 37 and 38
        if turn_num == 37 || turn_num == 38 {
            eprintln!("\n=== After Turn {} ===", turn_num);
            for (side_idx, side) in battle.sides.iter().enumerate() {
                eprintln!("Side {}:", side_idx);
                for (poke_idx, pokemon) in side.pokemon.iter().enumerate() {
                    eprintln!("  [{}] {}: HP={}/{} fainted={} active={}",
                        poke_idx, pokemon.name, pokemon.hp, pokemon.maxhp,
                        pokemon.fainted, pokemon.is_active);
                }
            }
        }

        if battle.ended {
            eprintln!("Battle ended at turn {}", turn_num);
            break;
        }
    }

    eprintln!("\nTotal PRNG calls: {}", battle.prng.call_count);
}
