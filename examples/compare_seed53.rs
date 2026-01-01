/// Compare battle with seed 53 - Rust version
use pokemon_showdown::{Battle, BattleOptions, PlayerOptions, PokemonSet, PRNGSeed, ID};
use pokemon_showdown::dex_data::{StatsTable, Gender};
use std::fs;

fn main() {
    let teams_json = fs::read_to_string("teams-seed53-rust.json").unwrap();

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
        seed: Some(PRNGSeed::Gen5([0, 0, 0, 53])),
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

    eprintln!("RUST: Running battle with seed [0, 0, 0, 53]");

    for make_choices_num in 1..=100 {
        let prng_before = battle.prng.call_count;

        battle.make_choices(&["default", "default"]);

        let prng_after = battle.prng.call_count;

        // Print HP of all active Pokemon
        let mut p1_active = Vec::new();
        let mut p2_active = Vec::new();

        for active_idx in &battle.sides[0].active {
            if let Some(poke_idx) = active_idx {
                if let Some(pokemon) = battle.sides[0].pokemon.get(*poke_idx) {
                    p1_active.push(format!("{}({}/{})", pokemon.name, pokemon.hp, pokemon.maxhp));
                }
            }
        }

        for active_idx in &battle.sides[1].active {
            if let Some(poke_idx) = active_idx {
                if let Some(pokemon) = battle.sides[1].pokemon.get(*poke_idx) {
                    p2_active.push(format!("{}({}/{})", pokemon.name, pokemon.hp, pokemon.maxhp));
                }
            }
        }

        eprintln!("#{}: turn={}, prng={}->{}, P1=[{}], P2=[{}]",
            make_choices_num, battle.turn, prng_before, prng_after,
            p1_active.join(", "), p2_active.join(", "));

        if battle.ended || make_choices_num >= 100 {
            eprintln!("\nRUST Battle status:");
            eprintln!("  Ended: {}", battle.ended);
            eprintln!("  Turn: {}", battle.turn);
            eprintln!("  Total PRNG calls: {}", battle.prng.call_count);
            break;
        }
    }
}
