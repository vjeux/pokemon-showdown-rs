use pokemon_showdown::{Battle, BattleOptions, PlayerOptions, PokemonSet, PRNGSeed, ID};
use pokemon_showdown::dex_data::StatsTable;
use std::fs;
use std::env;

fn main() {
    // Get test file number from command line args (default to 1)
    let args: Vec<String> = env::args().collect();
    let test_num = if args.len() > 1 {
        args[1].parse::<usize>().unwrap_or(1)
    } else {
        1
    };

    let test_file = format!("test-teams-{}.json", test_num);

    // Load test teams
    let test_teams_json = fs::read_to_string(&test_file).unwrap_or_else(|_| {
        eprintln!("Could not read {}, using test-teams.json", test_file);
        fs::read_to_string("test-teams.json").unwrap()
    });

    #[derive(serde::Deserialize)]
    struct TestTeams {
        seed: Vec<u16>,
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

    let seed = PRNGSeed::Gen5([
        test_teams.seed[0],
        test_teams.seed[1],
        test_teams.seed[2],
        test_teams.seed[3],
    ]);

    let mut battle = Battle::new(BattleOptions {
        format_id: ID::new("gen9randombattle"),
        seed: Some(seed),
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

    // gen9randombattle format starts directly with move choices (no team preview)
    eprintln!("[DEBUG] Initial state:");
    if let Some(p1_active) = battle.sides[0].get_active(0) {
        eprintln!("  P1 active[0]: {} ({}/{})", p1_active.name, p1_active.hp, p1_active.maxhp);
    }
    eprintln!("  P1 pokemon[0]: {} ({}/{})",
        battle.sides[0].pokemon[0].name,
        battle.sides[0].pokemon[0].hp,
        battle.sides[0].pokemon[0].maxhp);
    if let Some(p2_active) = battle.sides[1].get_active(0) {
        eprintln!("  P2 active[0]: {} ({}/{})", p2_active.name, p2_active.hp, p2_active.maxhp);
    }
    eprintln!("  P2 pokemon[0]: {} ({}/{})",
        battle.sides[1].pokemon[0].name,
        battle.sides[1].pokemon[0].hp,
        battle.sides[1].pokemon[0].maxhp);

    // Run 3 turns (matching JavaScript test)
    for turn_num in 1..=3 {
        if battle.ended {
            break;
        }

        eprintln!("[DEBUG] BEFORE turn {} make_choices: battle.turn={}, P1 active HP={}/{}, P1 pokemon[0] HP={}/{}",
            turn_num,
            battle.turn,
            battle.sides[0].get_active(0).map(|p| p.hp).unwrap_or(0),
            battle.sides[0].get_active(0).map(|p| p.maxhp).unwrap_or(0),
            battle.sides[0].pokemon.get(0).map(|p| p.hp).unwrap_or(0),
            battle.sides[0].pokemon.get(0).map(|p| p.maxhp).unwrap_or(0));

        // Check if battle is waiting for switch choices
        use pokemon_showdown::battle::BattleRequestState;
        let needs_switch = match battle.request_state {
            BattleRequestState::Switch => true,
            _ => false,
        };

        if needs_switch {
            // Find first alive Pokemon for each side to switch to
            let mut p1_choice = "pass".to_string();
            let mut p2_choice = "pass".to_string();

            // Check P1 for alive Pokemon
            for (i, pokemon) in battle.sides[0].pokemon.iter().enumerate() {
                if i > 0 && pokemon.hp > 0 {
                    p1_choice = format!("switch {}", i + 1);
                    break;
                }
            }

            // Check P2 for alive Pokemon
            for (i, pokemon) in battle.sides[1].pokemon.iter().enumerate() {
                if i > 0 && pokemon.hp > 0 {
                    p2_choice = format!("switch {}", i + 1);
                    break;
                }
            }

            battle.make_choices(&[p1_choice.as_str(), p2_choice.as_str()]);
        }

        // Always make move choices after handling switches (matches JavaScript)
        battle.make_choices(&["move 1", "move 1"]);

        eprintln!("[DEBUG] AFTER turn {} make_choices: battle.turn={}, P1 active HP={}/{}, P1 pokemon[0] HP={}/{}",
            turn_num,
            battle.turn,
            battle.sides[0].get_active(0).map(|p| p.hp).unwrap_or(0),
            battle.sides[0].get_active(0).map(|p| p.maxhp).unwrap_or(0),
            battle.sides[0].pokemon.get(0).map(|p| p.hp).unwrap_or(0),
            battle.sides[0].pokemon.get(0).map(|p| p.maxhp).unwrap_or(0));

        let p1_active = battle.sides[0].get_active(0);
        let p2_active = battle.sides[1].get_active(0);

        println!("Turn {}: {}({}/{}) vs {}({}/{})",
            turn_num,
            p1_active.map(|p| p.name.as_str()).unwrap_or("none"),
            p1_active.map(|p| p.hp).unwrap_or(0),
            p1_active.map(|p| p.maxhp).unwrap_or(0),
            p2_active.map(|p| p.name.as_str()).unwrap_or("none"),
            p2_active.map(|p| p.hp).unwrap_or(0),
            p2_active.map(|p| p.maxhp).unwrap_or(0)
        );

        println!("RESULT:{{\"turn\":{},\"p1_name\":\"{}\",\"p1_hp\":{},\"p1_maxhp\":{},\"p2_name\":\"{}\",\"p2_hp\":{},\"p2_maxhp\":{},\"ended\":{}}}",
            turn_num,
            p1_active.map(|p| p.name.as_str()).unwrap_or(""),
            p1_active.map(|p| p.hp).unwrap_or(0),
            p1_active.map(|p| p.maxhp).unwrap_or(0),
            p2_active.map(|p| p.name.as_str()).unwrap_or(""),
            p2_active.map(|p| p.hp).unwrap_or(0),
            p2_active.map(|p| p.maxhp).unwrap_or(0),
            battle.ended
        );
    }
}
