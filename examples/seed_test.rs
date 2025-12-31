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

    // Team preview
    battle.make_choices(&["default", "default"]);

    // Output initial state
    let p1_active = battle.sides[0].get_active(0);
    let p2_active = battle.sides[1].get_active(0);

    println!("RESULT:{{\"turn\":{},\"p1_name\":\"{}\",\"p1_hp\":{},\"p1_maxhp\":{},\"p2_name\":\"{}\",\"p2_hp\":{},\"p2_maxhp\":{}}}",
        battle.turn,
        p1_active.map(|p| p.name.as_str()).unwrap_or(""),
        p1_active.map(|p| p.hp).unwrap_or(0),
        p1_active.map(|p| p.maxhp).unwrap_or(0),
        p2_active.map(|p| p.name.as_str()).unwrap_or(""),
        p2_active.map(|p| p.hp).unwrap_or(0),
        p2_active.map(|p| p.maxhp).unwrap_or(0)
    );

    // Run 3 turns
    for turn_num in 1..=5 {
        if battle.ended {
            break;
        }

        battle.make_choices(&["move 1", "move 1"]);

        let p1_active = battle.sides[0].get_active(0);
        let p2_active = battle.sides[1].get_active(0);

        println!("Turn {}: {}({}/{}) vs {}({}/{})",
            battle.turn,
            p1_active.map(|p| p.name.as_str()).unwrap_or("none"),
            p1_active.map(|p| p.hp).unwrap_or(0),
            p1_active.map(|p| p.maxhp).unwrap_or(0),
            p2_active.map(|p| p.name.as_str()).unwrap_or("none"),
            p2_active.map(|p| p.hp).unwrap_or(0),
            p2_active.map(|p| p.maxhp).unwrap_or(0)
        );

        println!("RESULT:{{\"turn\":{},\"p1_name\":\"{}\",\"p1_hp\":{},\"p1_maxhp\":{},\"p2_name\":\"{}\",\"p2_hp\":{},\"p2_maxhp\":{},\"ended\":{}}}",
            battle.turn,
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
