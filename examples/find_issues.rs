use pokemon_showdown::{Battle, BattleOptions, PlayerOptions, PokemonSet, PRNGSeed, ID};

fn create_random_team(seed_offset: u16) -> Vec<PokemonSet> {
    let mut prng = pokemon_showdown::PRNG::new(Some(PRNGSeed::Gen5([seed_offset, seed_offset + 1, seed_offset + 2, seed_offset + 3])));

    let species_list = vec![
        "Pikachu", "Charizard", "Blastoise", "Venusaur", "Gengar", "Alakazam",
        "Dragonite", "Mewtwo", "Mew", "Tyranitar", "Lugia", "Ho-Oh",
        "Garchomp", "Salamence", "Metagross", "Latios", "Rayquaza", "Dialga",
        "Palkia", "Giratina", "Arceus", "Reshiram", "Zekrom", "Kyurem",
        "Xerneas", "Yveltal", "Zygarde", "Solgaleo", "Lunala", "Necrozma",
        "Zacian", "Zamazenta", "Eternatus", "Calyrex", "Koraidon", "Miraidon",
    ];

    let ability_list = vec![
        "Static", "Blaze", "Torrent", "Overgrow", "Levitate", "Magic Guard",
        "Intimidate", "Pressure", "Synchronize", "Inner Focus", "Sand Stream",
        "Drizzle", "Drought", "Snow Warning", "Electric Surge", "Grassy Surge",
        "Psychic Surge", "Misty Surge", "Orichalcum Pulse", "Hadron Engine",
    ];

    let mut team = Vec::new();
    for _ in 0..6 {
        let species_idx = prng.next() as usize % species_list.len();
        let ability_idx = prng.next() as usize % ability_list.len();

        team.push(PokemonSet {
            name: species_list[species_idx].to_string(),
            species: species_list[species_idx].to_string(),
            ability: ability_list[ability_idx].to_string(),
            moves: vec!["Tackle".to_string(), "Thunder Shock".to_string()],
            level: 50,
            ..Default::default()
        });
    }

    team
}

fn main() {
    println!("Running 100 random battles to find issues...\n");

    for seed_num in 0..100 {
        let seed = PRNGSeed::Gen5([
            (seed_num * 4) as u16,
            (seed_num * 4 + 1) as u16,
            (seed_num * 4 + 2) as u16,
            (seed_num * 4 + 3) as u16,
        ]);

        print!("Seed {:3}: ", seed_num);

        let team1 = create_random_team(seed_num as u16 * 100);
        let team2 = create_random_team(seed_num as u16 * 100 + 50);

        let mut battle = Battle::new(BattleOptions {
            format_id: ID::new("gen9ou"),
            seed: Some(seed.clone()),
            debug: false,
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
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            battle.make_choices(&["default", "default"]);
        })) {
            Ok(_) => {},
            Err(e) => {
                println!("PANIC during team preview: {:?}", e);
                continue;
            }
        }

        // Run 5 turns
        let mut turn_ok = true;
        for turn in 1..=5 {
            if battle.ended {
                break;
            }

            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                battle.make_choices(&["move 1", "move 1"]);
            })) {
                Ok(_) => {},
                Err(e) => {
                    println!("PANIC at turn {}: {:?}", turn, e);
                    turn_ok = false;
                    break;
                }
            }
        }

        if turn_ok {
            let p1_hp = battle.sides[0].get_active(0).map(|p| p.hp).unwrap_or(0);
            let p2_hp = battle.sides[1].get_active(0).map(|p| p.hp).unwrap_or(0);
            println!("OK (turn {}, P1:{} P2:{})", battle.turn, p1_hp, p2_hp);
        }
    }

    println!("\n✓ Completed 100 battles");
}
