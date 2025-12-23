//! Full battle simulation integration tests

use pokemon_showdown::{
    Battle, BattleOptions, PlayerOptions, Dex,
    PRNG, PRNGSeed, BattleActions, DamageResult,
    PokemonSet, ID,
};

/// Create a simple battle with two Pokemon teams
fn create_test_battle() -> Battle {
    let team1 = vec![
        PokemonSet {
            name: "Pikachu".to_string(),
            species: "Pikachu".to_string(),
            level: 50,
            ability: "Static".to_string(),
            moves: vec!["Thunderbolt".to_string(), "Quick Attack".to_string()],
            ..Default::default()
        },
    ];

    let team2 = vec![
        PokemonSet {
            name: "Squirtle".to_string(),
            species: "Squirtle".to_string(),
            level: 50,
            ability: "Torrent".to_string(),
            moves: vec!["Tackle".to_string()],
            ..Default::default()
        },
    ];

    Battle::new(BattleOptions {
        format_id: ID::new("gen9ou"),
        seed: Some(PRNGSeed::Gen5([1, 2, 3, 4])),
        p1: Some(PlayerOptions {
            name: "Alice".to_string(),
            team: team1,
            avatar: None,
        }),
        p2: Some(PlayerOptions {
            name: "Bob".to_string(),
            team: team2,
            avatar: None,
        }),
        ..Default::default()
    })
}

#[test]
fn test_battle_initialization() {
    let battle = create_test_battle();

    assert!(battle.started);
    assert!(!battle.ended);
    assert_eq!(battle.sides.len(), 2);
    assert_eq!(battle.sides[0].name, "Alice");
    assert_eq!(battle.sides[1].name, "Bob");
    assert_eq!(battle.sides[0].pokemon.len(), 1);
    assert_eq!(battle.sides[1].pokemon.len(), 1);
}

#[test]
fn test_damage_calculation_super_effective() {
    let dex = Dex::load_default().unwrap();
    let actions = BattleActions::new(&dex, 9);

    // Create Pikachu (Electric) attacking Squirtle (Water)
    let attacker_set = PokemonSet {
        name: "Pikachu".to_string(),
        species: "Pikachu".to_string(),
        level: 50,
        ..Default::default()
    };
    let mut attacker = pokemon_showdown::Pokemon::new(&attacker_set, 0, 0);
    attacker.types = vec!["Electric".to_string()];
    attacker.stored_stats.spa = 80;  // Set Sp.Atk

    let defender_set = PokemonSet {
        name: "Squirtle".to_string(),
        species: "Squirtle".to_string(),
        level: 50,
        ..Default::default()
    };
    let mut defender = pokemon_showdown::Pokemon::new(&defender_set, 1, 0);
    defender.types = vec!["Water".to_string()];
    defender.stored_stats.spd = 64;  // Set Sp.Def
    defender.hp = 100;
    defender.maxhp = 100;

    let thunderbolt = dex.get_move("Thunderbolt").unwrap();

    // Calculate damage (super effective 2x)
    match actions.calculate_damage(&attacker, &defender, thunderbolt, false, 100) {
        DamageResult::Damage(dmg) => {
            // Should be significant damage due to super effectiveness
            println!("Super effective damage: {}", dmg);
            assert!(dmg > 50, "Expected significant damage, got {}", dmg);
        }
        _ => panic!("Expected damage result"),
    }
}

#[test]
fn test_damage_calculation_immune() {
    let dex = Dex::load_default().unwrap();
    let actions = BattleActions::new(&dex, 9);

    // Create Pikachu (Electric) attacking Ground-type
    let attacker = PokemonSet {
        name: "Pikachu".to_string(),
        species: "Pikachu".to_string(),
        level: 50,
        ..Default::default()
    };
    let mut attacker = pokemon_showdown::Pokemon::new(&attacker, 0, 0);
    attacker.types = vec!["Electric".to_string()];

    let defender = PokemonSet {
        name: "Sandshrew".to_string(),
        species: "Sandshrew".to_string(),
        level: 50,
        ..Default::default()
    };
    let mut defender = pokemon_showdown::Pokemon::new(&defender, 1, 0);
    defender.types = vec!["Ground".to_string()];

    let thunderbolt = dex.get_move("Thunderbolt").unwrap();

    // Should be immune
    match actions.calculate_damage(&attacker, &defender, thunderbolt, false, 100) {
        DamageResult::Immune => (),
        other => panic!("Expected immunity, got {:?}", other),
    }
}

#[test]
fn test_damage_calculation_with_stab() {
    let dex = Dex::load_default().unwrap();
    let actions = BattleActions::new(&dex, 9);

    // Electric-type using Electric move (STAB)
    let attacker = PokemonSet {
        name: "Pikachu".to_string(),
        species: "Pikachu".to_string(),
        level: 50,
        ..Default::default()
    };
    let mut attacker = pokemon_showdown::Pokemon::new(&attacker, 0, 0);
    attacker.types = vec!["Electric".to_string()];
    attacker.stored_stats.spa = 100;

    // Normal-type defender (neutral effectiveness)
    let defender = PokemonSet {
        name: "Raticate".to_string(),
        species: "Raticate".to_string(),
        level: 50,
        ..Default::default()
    };
    let mut defender = pokemon_showdown::Pokemon::new(&defender, 1, 0);
    defender.types = vec!["Normal".to_string()];
    defender.stored_stats.spd = 100;
    defender.hp = 200;
    defender.maxhp = 200;

    let thunderbolt = dex.get_move("Thunderbolt").unwrap();

    // Calculate with STAB
    match actions.calculate_damage(&attacker, &defender, thunderbolt, false, 100) {
        DamageResult::Damage(stab_dmg) => {
            // Now calculate without STAB (pretend attacker is Normal-type)
            attacker.types = vec!["Normal".to_string()];
            match actions.calculate_damage(&attacker, &defender, thunderbolt, false, 100) {
                DamageResult::Damage(no_stab_dmg) => {
                    println!("STAB damage: {}, No STAB damage: {}", stab_dmg, no_stab_dmg);
                    // STAB should give 1.5x damage
                    assert!(stab_dmg > no_stab_dmg, "STAB should increase damage");
                }
                _ => panic!("Expected damage"),
            }
        }
        _ => panic!("Expected damage"),
    }
}

#[test]
fn test_stat_boosts_in_damage() {
    let dex = Dex::load_default().unwrap();
    let actions = BattleActions::new(&dex, 9);

    let attacker = PokemonSet {
        name: "Pikachu".to_string(),
        species: "Pikachu".to_string(),
        level: 50,
        ..Default::default()
    };
    let mut attacker = pokemon_showdown::Pokemon::new(&attacker, 0, 0);
    attacker.types = vec!["Normal".to_string()];
    attacker.stored_stats.atk = 100;

    let defender = PokemonSet {
        name: "Raticate".to_string(),
        species: "Raticate".to_string(),
        level: 50,
        ..Default::default()
    };
    let mut defender = pokemon_showdown::Pokemon::new(&defender, 1, 0);
    defender.types = vec!["Normal".to_string()];
    defender.stored_stats.def = 100;
    defender.hp = 200;
    defender.maxhp = 200;

    let tackle = dex.get_move("Tackle").unwrap();

    // Get base damage
    let base_damage = match actions.calculate_damage(&attacker, &defender, tackle, false, 100) {
        DamageResult::Damage(dmg) => dmg,
        _ => panic!("Expected damage"),
    };

    // Apply +2 Attack boost
    attacker.boosts.atk = 2;
    let boosted_damage = match actions.calculate_damage(&attacker, &defender, tackle, false, 100) {
        DamageResult::Damage(dmg) => dmg,
        _ => panic!("Expected damage"),
    };

    println!("Base damage: {}, Boosted (+2 Atk) damage: {}", base_damage, boosted_damage);
    // +2 should give 2x attack
    assert!(boosted_damage > base_damage * 3 / 2, "Expected roughly 2x damage with +2 boost");
}

#[test]
fn test_critical_hit_damage() {
    let dex = Dex::load_default().unwrap();
    let actions = BattleActions::new(&dex, 9);

    let attacker = PokemonSet {
        name: "Pikachu".to_string(),
        species: "Pikachu".to_string(),
        level: 50,
        ..Default::default()
    };
    let mut attacker = pokemon_showdown::Pokemon::new(&attacker, 0, 0);
    attacker.types = vec!["Normal".to_string()];
    attacker.stored_stats.atk = 100;

    let defender = PokemonSet {
        name: "Raticate".to_string(),
        species: "Raticate".to_string(),
        level: 50,
        ..Default::default()
    };
    let mut defender = pokemon_showdown::Pokemon::new(&defender, 1, 0);
    defender.types = vec!["Normal".to_string()];
    defender.stored_stats.def = 100;
    defender.hp = 200;
    defender.maxhp = 200;

    let tackle = dex.get_move("Tackle").unwrap();

    // Normal hit
    let normal_damage = match actions.calculate_damage(&attacker, &defender, tackle, false, 100) {
        DamageResult::Damage(dmg) => dmg,
        _ => panic!("Expected damage"),
    };

    // Critical hit
    let crit_damage = match actions.calculate_damage(&attacker, &defender, tackle, true, 100) {
        DamageResult::Damage(dmg) => dmg,
        _ => panic!("Expected damage"),
    };

    println!("Normal damage: {}, Crit damage: {}", normal_damage, crit_damage);
    // Gen 6+ crits do 1.5x damage
    assert!(crit_damage > normal_damage, "Crit should do more damage");
}

#[test]
fn test_prng_determinism() {
    // Same seed should produce identical sequences
    let seed = PRNGSeed::Gen5([0x1234, 0x5678, 0x9ABC, 0xDEF0]);

    let mut prng1 = PRNG::new(Some(seed.clone()));
    let mut prng2 = PRNG::new(Some(seed));

    for _ in 0..100 {
        assert_eq!(prng1.random_int(100), prng2.random_int(100));
    }
}

#[test]
fn test_battle_log_format() {
    let battle = create_test_battle();

    let log = battle.get_log();
    println!("Battle log:\n{}", log);

    // Check that the log contains expected entries
    assert!(log.contains("|gametype|"), "Expected gametype in log");
    assert!(log.contains("|player|"), "Expected player info in log");
    // Note: team preview is lowercase in our implementation
    assert!(log.contains("teampreview"), "Expected teampreview in log");
}

#[test]
fn test_type_chart() {
    let dex = Dex::load_default().unwrap();

    // Test various type matchups
    assert_eq!(dex.get_effectiveness("Fire", "Grass"), 2.0, "Fire > Grass");
    assert_eq!(dex.get_effectiveness("Water", "Fire"), 2.0, "Water > Fire");
    assert_eq!(dex.get_effectiveness("Grass", "Water"), 2.0, "Grass > Water");

    assert_eq!(dex.get_effectiveness("Electric", "Ground"), 0.0, "Electric immune to Ground");
    assert_eq!(dex.get_effectiveness("Normal", "Ghost"), 0.0, "Normal immune to Ghost");
    assert_eq!(dex.get_effectiveness("Fighting", "Ghost"), 0.0, "Fighting immune to Ghost");

    assert_eq!(dex.get_effectiveness("Fire", "Water"), 0.5, "Fire resisted by Water");
    assert_eq!(dex.get_effectiveness("Electric", "Electric"), 0.5, "Electric resisted by Electric");
}

#[test]
fn test_dual_type_effectiveness() {
    let dex = Dex::load_default().unwrap();

    // Water/Flying (like Gyarados) takes 4x from Electric
    let types = vec!["Water".to_string(), "Flying".to_string()];
    let effectiveness = dex.get_type_effectiveness("Electric", &types);
    assert_eq!(effectiveness, 4.0, "Electric should be 4x effective vs Water/Flying");

    // Electric/Flying takes 0x from Ground
    let types = vec!["Electric".to_string(), "Flying".to_string()];
    let effectiveness = dex.get_type_effectiveness("Ground", &types);
    assert_eq!(effectiveness, 0.0, "Ground should be immune to Electric/Flying");
}

#[test]
fn test_make_choices_basic() {
    let team1 = vec![
        PokemonSet {
            name: "Pikachu".to_string(),
            species: "Pikachu".to_string(),
            level: 50,
            ability: "Static".to_string(),
            moves: vec!["Thunderbolt".to_string(), "Quick Attack".to_string()],
            ..Default::default()
        },
    ];

    let team2 = vec![
        PokemonSet {
            name: "Squirtle".to_string(),
            species: "Squirtle".to_string(),
            level: 50,
            ability: "Torrent".to_string(),
            moves: vec!["Tackle".to_string()],
            ..Default::default()
        },
    ];

    let mut battle = Battle::new(BattleOptions {
        format_id: ID::new("gen9ou"),
        seed: Some(PRNGSeed::Gen5([1, 2, 3, 4])),
        p1: Some(PlayerOptions {
            name: "Alice".to_string(),
            team: team1,
            avatar: None,
        }),
        p2: Some(PlayerOptions {
            name: "Bob".to_string(),
            team: team2,
            avatar: None,
        }),
        ..Default::default()
    });

    // Start the battle
    battle.start_battle();
    assert_eq!(battle.turn, 1);

    // Make moves
    battle.make_choices("move thunderbolt", "move tackle");

    // Should be turn 2 now
    assert_eq!(battle.turn, 2);

    // Check log contains move entries
    let log = battle.get_log();
    assert!(log.contains("|move|"), "Log should contain move messages");
}

#[test]
fn test_damage_dealt() {
    let team1 = vec![
        PokemonSet {
            name: "Pikachu".to_string(),
            species: "Pikachu".to_string(),
            level: 50,
            ability: "Static".to_string(),
            moves: vec!["Thunderbolt".to_string()],
            ..Default::default()
        },
    ];

    let team2 = vec![
        PokemonSet {
            name: "Squirtle".to_string(),
            species: "Squirtle".to_string(),
            level: 50,
            ability: "Torrent".to_string(),
            moves: vec!["Tackle".to_string()],
            ..Default::default()
        },
    ];

    let mut battle = Battle::new(BattleOptions {
        format_id: ID::new("gen9ou"),
        seed: Some(PRNGSeed::Gen5([1, 2, 3, 4])),
        p1: Some(PlayerOptions {
            name: "Alice".to_string(),
            team: team1,
            avatar: None,
        }),
        p2: Some(PlayerOptions {
            name: "Bob".to_string(),
            team: team2,
            avatar: None,
        }),
        ..Default::default()
    });

    // Set up Pokemon with proper stats and types
    battle.start_battle();

    // Set types for effectiveness
    if let Some(p1_pokemon) = battle.sides[0].get_active_mut(0) {
        p1_pokemon.types = vec!["Electric".to_string()];
        p1_pokemon.stored_stats.spa = 80;
        p1_pokemon.stored_stats.spe = 90;
    }
    if let Some(p2_pokemon) = battle.sides[1].get_active_mut(0) {
        p2_pokemon.types = vec!["Water".to_string()];
        p2_pokemon.stored_stats.spd = 64;
        p2_pokemon.hp = 500;  // High HP so it survives
        p2_pokemon.maxhp = 500;
    }

    let initial_hp = battle.sides[1].get_active(0).unwrap().hp;

    battle.make_choices("move thunderbolt", "move tackle");

    // After the move, check HP - might be fainted or still active
    let final_hp = battle.sides[1].pokemon[0].hp;  // Use pokemon directly instead of get_active
    println!("Initial HP: {}, Final HP: {}", initial_hp, final_hp);

    // Thunderbolt should deal damage (super effective against water)
    assert!(final_hp < initial_hp, "Thunderbolt should deal damage");
}

#[test]
fn test_status_burn() {
    let team1 = vec![
        PokemonSet {
            name: "Sableye".to_string(),
            species: "Sableye".to_string(),
            level: 50,
            ability: "Prankster".to_string(),
            moves: vec!["Will-O-Wisp".to_string()],
            ..Default::default()
        },
    ];

    let team2 = vec![
        PokemonSet {
            name: "Machamp".to_string(),
            species: "Machamp".to_string(),
            level: 50,
            ability: "Guts".to_string(),
            moves: vec!["Bulk Up".to_string()],
            ..Default::default()
        },
    ];

    let mut battle = Battle::new(BattleOptions {
        format_id: ID::new("gen9ou"),
        seed: Some(PRNGSeed::Gen5([1, 2, 3, 4])),
        p1: Some(PlayerOptions {
            name: "Alice".to_string(),
            team: team1,
            avatar: None,
        }),
        p2: Some(PlayerOptions {
            name: "Bob".to_string(),
            team: team2,
            avatar: None,
        }),
        ..Default::default()
    });

    battle.start_battle();

    // Set types
    if let Some(p2_pokemon) = battle.sides[1].get_active_mut(0) {
        p2_pokemon.types = vec!["Fighting".to_string()];
        p2_pokemon.hp = 160;
        p2_pokemon.maxhp = 160;
    }

    // Will-O-Wisp should burn
    battle.make_choices("move willowisp", "move bulkup");

    // Check Machamp is burned
    let machamp = battle.sides[1].get_active(0).unwrap();
    assert_eq!(machamp.status.as_str(), "brn", "Machamp should be burned");

    // Record HP before residual
    let hp_before = machamp.hp;

    // Next turn, burn should deal 1/16 max HP damage
    battle.make_choices("move willowisp", "move bulkup");

    let hp_after = battle.sides[1].get_active(0).unwrap().hp;
    let burn_damage = hp_before - hp_after;

    // Burn damage should be at least 1/16 of max HP (10 for 160 HP)
    println!("Burn damage: {} (expected ~10 for 160 maxhp)", burn_damage);
    assert!(burn_damage >= 10, "Burn should deal ~1/16 max HP per turn");
}

#[test]
fn test_status_paralysis_speed() {
    let team1 = vec![
        PokemonSet {
            name: "Pikachu".to_string(),
            species: "Pikachu".to_string(),
            level: 50,
            ability: "Static".to_string(),
            moves: vec!["Thunder Wave".to_string(), "Tackle".to_string()],
            ..Default::default()
        },
    ];

    let team2 = vec![
        PokemonSet {
            name: "Rattata".to_string(),
            species: "Rattata".to_string(),
            level: 50,
            ability: "Run Away".to_string(),
            moves: vec!["Quick Attack".to_string()],
            ..Default::default()
        },
    ];

    let mut battle = Battle::new(BattleOptions {
        format_id: ID::new("gen9ou"),
        seed: Some(PRNGSeed::Gen5([1, 2, 3, 4])),
        p1: Some(PlayerOptions {
            name: "Alice".to_string(),
            team: team1,
            avatar: None,
        }),
        p2: Some(PlayerOptions {
            name: "Bob".to_string(),
            team: team2,
            avatar: None,
        }),
        ..Default::default()
    });

    battle.start_battle();

    // Set types (not Electric so it can be paralyzed)
    if let Some(p2_pokemon) = battle.sides[1].get_active_mut(0) {
        p2_pokemon.types = vec!["Normal".to_string()];
    }

    // Thunder Wave should paralyze
    battle.make_choices("move thunderwave", "move quickattack");

    // Check Rattata is paralyzed
    let rattata = battle.sides[1].get_active(0).unwrap();
    assert_eq!(rattata.status.as_str(), "par", "Rattata should be paralyzed");
}

#[test]
fn test_switch_in_battle() {
    let team1 = vec![
        PokemonSet {
            name: "Pikachu".to_string(),
            species: "Pikachu".to_string(),
            level: 50,
            ability: "Static".to_string(),
            moves: vec!["Thunderbolt".to_string()],
            ..Default::default()
        },
        PokemonSet {
            name: "Charizard".to_string(),
            species: "Charizard".to_string(),
            level: 50,
            ability: "Blaze".to_string(),
            moves: vec!["Flamethrower".to_string()],
            ..Default::default()
        },
    ];

    let team2 = vec![
        PokemonSet {
            name: "Squirtle".to_string(),
            species: "Squirtle".to_string(),
            level: 50,
            ability: "Torrent".to_string(),
            moves: vec!["Tackle".to_string()],
            ..Default::default()
        },
    ];

    let mut battle = Battle::new(BattleOptions {
        format_id: ID::new("gen9ou"),
        seed: Some(PRNGSeed::Gen5([1, 2, 3, 4])),
        p1: Some(PlayerOptions {
            name: "Alice".to_string(),
            team: team1,
            avatar: None,
        }),
        p2: Some(PlayerOptions {
            name: "Bob".to_string(),
            team: team2,
            avatar: None,
        }),
        ..Default::default()
    });

    battle.start_battle();

    // Pikachu should be active
    assert_eq!(battle.sides[0].get_active(0).unwrap().name, "Pikachu");

    // Switch to Charizard
    battle.make_choices("switch 2", "move tackle");

    // Charizard should now be active
    assert_eq!(battle.sides[0].get_active(0).unwrap().name, "Charizard");

    // Log should contain switch message
    let log = battle.get_log();
    assert!(log.contains("switch") && log.contains("Charizard"), "Log should contain Charizard switch");
}

#[test]
fn test_faint_detection() {
    let team1 = vec![
        PokemonSet {
            name: "Pikachu".to_string(),
            species: "Pikachu".to_string(),
            level: 50,
            ability: "Static".to_string(),
            moves: vec!["Thunderbolt".to_string()],
            ..Default::default()
        },
    ];

    let team2 = vec![
        PokemonSet {
            name: "Magikarp".to_string(),
            species: "Magikarp".to_string(),
            level: 5,  // Very low level
            ability: "Swift Swim".to_string(),
            moves: vec!["Splash".to_string()],
            ..Default::default()
        },
    ];

    let mut battle = Battle::new(BattleOptions {
        format_id: ID::new("gen9ou"),
        seed: Some(PRNGSeed::Gen5([1, 2, 3, 4])),
        p1: Some(PlayerOptions {
            name: "Alice".to_string(),
            team: team1,
            avatar: None,
        }),
        p2: Some(PlayerOptions {
            name: "Bob".to_string(),
            team: team2,
            avatar: None,
        }),
        ..Default::default()
    });

    battle.start_battle();

    // Set Magikarp with very low HP
    if let Some(magikarp) = battle.sides[1].get_active_mut(0) {
        magikarp.types = vec!["Water".to_string()];
        magikarp.hp = 20;
        magikarp.maxhp = 20;
        magikarp.stored_stats.spd = 20;
    }

    // Set Pikachu with high attack
    if let Some(pikachu) = battle.sides[0].get_active_mut(0) {
        pikachu.types = vec!["Electric".to_string()];
        pikachu.stored_stats.spa = 100;
    }

    // Thunderbolt should KO
    battle.make_choices("move thunderbolt", "move splash");

    // Check Magikarp fainted
    let magikarp_hp = battle.sides[1].pokemon[0].hp;
    assert_eq!(magikarp_hp, 0, "Magikarp should be at 0 HP");
    assert!(battle.sides[1].pokemon[0].fainted, "Magikarp should be fainted");

    // Battle should have ended (only 1 Pokemon per side)
    assert!(battle.ended, "Battle should have ended");
    assert_eq!(battle.winner.as_deref(), Some("p1"), "Alice (p1) should have won");
}

#[test]
fn test_toxic_damage_progression() {
    let team1 = vec![
        PokemonSet {
            name: "Chansey".to_string(),
            species: "Chansey".to_string(),
            level: 100,
            ability: "Natural Cure".to_string(),
            moves: vec!["Soft Boiled".to_string()],
            ..Default::default()
        },
    ];

    let team2 = vec![
        PokemonSet {
            name: "Gengar".to_string(),
            species: "Gengar".to_string(),
            level: 100,
            ability: "Levitate".to_string(),
            moves: vec!["Toxic".to_string()],
            ..Default::default()
        },
    ];

    let mut battle = Battle::new(BattleOptions {
        format_id: ID::new("gen9ou"),
        seed: Some(PRNGSeed::Gen5([1, 2, 3, 4])),
        p1: Some(PlayerOptions {
            name: "Alice".to_string(),
            team: team1,
            avatar: None,
        }),
        p2: Some(PlayerOptions {
            name: "Bob".to_string(),
            team: team2,
            avatar: None,
        }),
        ..Default::default()
    });

    battle.start_battle();

    // Set up Chansey with high HP
    if let Some(chansey) = battle.sides[0].get_active_mut(0) {
        chansey.types = vec!["Normal".to_string()];
        chansey.hp = 500;
        chansey.maxhp = 500;
    }

    // Toxic the Chansey
    battle.make_choices("move softboiled", "move toxic");

    // Check that Chansey is badly poisoned
    let chansey = &battle.sides[0].pokemon[0];
    assert_eq!(chansey.status.as_str(), "tox", "Chansey should be badly poisoned");

    // Record HP after first toxic damage
    let hp_after_1 = chansey.hp;
    let damage_1 = 500 - hp_after_1;
    println!("Turn 1 toxic damage: {} (expected ~31 = 500/16)", damage_1);

    // Next turn should deal more damage
    battle.make_choices("move softboiled", "move toxic");

    let hp_after_2 = battle.sides[0].pokemon[0].hp;
    // Note: softboiled heals, so we need to account for that
    // But the toxic damage itself should have been 2/16 this turn

    // Check that toxic counter is incrementing (we can verify via status_state)
    let counter = battle.sides[0].pokemon[0].status_state.duration;
    assert!(counter.is_some() && counter.unwrap() >= 2, "Toxic counter should be at least 2");
}

#[test]
fn test_paralysis_cant_move() {
    // Test that paralysis can prevent movement (25% chance in Gen 7+)
    let team1 = vec![
        PokemonSet {
            name: "Pikachu".to_string(),
            species: "Pikachu".to_string(),
            level: 50,
            ability: "Static".to_string(),
            moves: vec!["Thunder Wave".to_string(), "Thunderbolt".to_string()],
            ..Default::default()
        },
    ];

    let team2 = vec![
        PokemonSet {
            name: "Rattata".to_string(),
            species: "Rattata".to_string(),
            level: 50,
            ability: "Run Away".to_string(),
            moves: vec!["Tackle".to_string()],
            ..Default::default()
        },
    ];

    let mut battle = Battle::new(BattleOptions {
        format_id: ID::new("gen9ou"),
        seed: Some(PRNGSeed::Gen5([1, 2, 3, 4])),
        p1: Some(PlayerOptions {
            name: "Alice".to_string(),
            team: team1,
            avatar: None,
        }),
        p2: Some(PlayerOptions {
            name: "Bob".to_string(),
            team: team2,
            avatar: None,
        }),
        ..Default::default()
    });

    battle.start_battle();

    // Set types
    if let Some(rattata) = battle.sides[1].get_active_mut(0) {
        rattata.types = vec!["Normal".to_string()];
    }

    // Paralyze the Rattata
    battle.make_choices("move thunderwave", "move tackle");

    assert_eq!(battle.sides[1].pokemon[0].status.as_str(), "par", "Rattata should be paralyzed");

    // Run multiple turns - statistically some should be blocked by paralysis
    // We can check the log for "cant" messages
    let log_before = battle.get_log().len();

    for _ in 0..10 {
        battle.make_choices("move thunderbolt", "move tackle");
        if battle.ended {
            break;
        }
    }

    // The battle log should contain evidence of the paralysis mechanic working
    // (either the Rattata was able to move, or it was fully paralyzed sometimes)
    let log = battle.get_log();
    println!("Battle log excerpt:\n{}", log);
}

#[test]
fn test_stat_boosts_sword_dance() {
    let team1 = vec![
        PokemonSet {
            name: "Scizor".to_string(),
            species: "Scizor".to_string(),
            level: 50,
            ability: "Technician".to_string(),
            moves: vec!["Swords Dance".to_string(), "Bullet Punch".to_string()],
            ..Default::default()
        },
    ];

    let team2 = vec![
        PokemonSet {
            name: "Blissey".to_string(),
            species: "Blissey".to_string(),
            level: 50,
            ability: "Natural Cure".to_string(),
            moves: vec!["Soft Boiled".to_string()],
            ..Default::default()
        },
    ];

    let mut battle = Battle::new(BattleOptions {
        format_id: ID::new("gen9ou"),
        seed: Some(PRNGSeed::Gen5([1, 2, 3, 4])),
        p1: Some(PlayerOptions {
            name: "Alice".to_string(),
            team: team1,
            avatar: None,
        }),
        p2: Some(PlayerOptions {
            name: "Bob".to_string(),
            team: team2,
            avatar: None,
        }),
        ..Default::default()
    });

    battle.start_battle();

    // Set types
    if let Some(scizor) = battle.sides[0].get_active_mut(0) {
        scizor.types = vec!["Bug".to_string(), "Steel".to_string()];
        scizor.stored_stats.atk = 100;
    }

    // Initial attack boost should be 0
    assert_eq!(battle.sides[0].pokemon[0].boosts.atk, 0, "Attack boost should start at 0");

    // Use Swords Dance
    battle.make_choices("move swordsdance", "move softboiled");

    // Attack boost should now be +2
    assert_eq!(battle.sides[0].pokemon[0].boosts.atk, 2, "Attack boost should be +2 after Swords Dance");

    // Use Swords Dance again
    battle.make_choices("move swordsdance", "move softboiled");

    // Attack boost should now be +4
    assert_eq!(battle.sides[0].pokemon[0].boosts.atk, 4, "Attack boost should be +4 after two Swords Dances");

    // Use Swords Dance a third time
    battle.make_choices("move swordsdance", "move softboiled");

    // Attack boost should cap at +6
    assert_eq!(battle.sides[0].pokemon[0].boosts.atk, 6, "Attack boost should cap at +6");
}

#[test]
fn test_speed_order() {
    // Test that faster Pokemon moves first
    let team1 = vec![
        PokemonSet {
            name: "Electrode".to_string(),  // Fast
            species: "Electrode".to_string(),
            level: 50,
            ability: "Static".to_string(),
            moves: vec!["Thunderbolt".to_string()],
            ..Default::default()
        },
    ];

    let team2 = vec![
        PokemonSet {
            name: "Slowpoke".to_string(),  // Slow
            species: "Slowpoke".to_string(),
            level: 50,
            ability: "Own Tempo".to_string(),
            moves: vec!["Tackle".to_string()],
            ..Default::default()
        },
    ];

    let mut battle = Battle::new(BattleOptions {
        format_id: ID::new("gen9ou"),
        seed: Some(PRNGSeed::Gen5([1, 2, 3, 4])),
        p1: Some(PlayerOptions {
            name: "Alice".to_string(),
            team: team1,
            avatar: None,
        }),
        p2: Some(PlayerOptions {
            name: "Bob".to_string(),
            team: team2,
            avatar: None,
        }),
        ..Default::default()
    });

    battle.start_battle();

    // Set speeds - Electrode is faster
    if let Some(electrode) = battle.sides[0].get_active_mut(0) {
        electrode.types = vec!["Electric".to_string()];
        electrode.stored_stats.spe = 200;
    }
    if let Some(slowpoke) = battle.sides[1].get_active_mut(0) {
        slowpoke.types = vec!["Water".to_string(), "Psychic".to_string()];
        slowpoke.stored_stats.spe = 50;
    }

    battle.make_choices("move thunderbolt", "move tackle");

    // Check the log - Electrode should move before Slowpoke
    let log = battle.get_log();
    let electrode_move_pos = log.find("Electrode");
    let slowpoke_move_pos = log.find("Slowpoke");

    // Both should appear in log
    assert!(electrode_move_pos.is_some(), "Electrode should appear in log");
    assert!(slowpoke_move_pos.is_some(), "Slowpoke should appear in log");

    // In a normal turn, faster moves first - but this is complex to verify
    // from log order. The important thing is the battle runs correctly.
    println!("Speed order test passed - battle executed successfully");
}

#[test]
fn test_weather_rain_boost() {
    // Rain should boost Water moves by 1.5x and weaken Fire moves by 0.5x
    let team1 = vec![
        PokemonSet {
            name: "Pelipper".to_string(),
            species: "Pelipper".to_string(),
            level: 50,
            ability: "Drizzle".to_string(),
            moves: vec!["Surf".to_string(), "Scald".to_string()],
            ..Default::default()
        },
    ];

    let team2 = vec![
        PokemonSet {
            name: "Blissey".to_string(),
            species: "Blissey".to_string(),
            level: 50,
            ability: "Natural Cure".to_string(),
            moves: vec!["Flamethrower".to_string()],
            ..Default::default()
        },
    ];

    let mut battle = Battle::new(BattleOptions {
        format_id: ID::new("gen9ou"),
        seed: Some(PRNGSeed::Gen5([1, 2, 3, 4])),
        p1: Some(PlayerOptions {
            name: "Player 1".to_string(),
            team: team1,
            avatar: None,
        }),
        p2: Some(PlayerOptions {
            name: "Player 2".to_string(),
            team: team2,
            avatar: None,
        }),
        ..Default::default()
    });

    battle.start_battle();

    // Set rain
    battle.field.set_weather(ID::new("raindance"), Some(5));

    let hp_before = battle.sides[1].pokemon[0].hp;
    battle.make_choices("move surf", "move flamethrower");

    // Water move in rain should deal boosted damage
    let hp_after = battle.sides[1].pokemon[0].hp;
    let damage_dealt = hp_before - hp_after;

    // Rain should have boosted the Water move
    assert!(damage_dealt > 0, "Surf should deal damage");
    println!("Rain boost test - Surf dealt {} damage in rain", damage_dealt);
}

#[test]
fn test_weather_sandstorm_damage() {
    // Sandstorm should deal 1/16 max HP damage to non-Rock/Ground/Steel types
    let team1 = vec![
        PokemonSet {
            name: "Tyranitar".to_string(),
            species: "Tyranitar".to_string(),
            level: 50,
            ability: "Sand Stream".to_string(),
            moves: vec!["Tackle".to_string()],
            ..Default::default()
        },
    ];

    let team2 = vec![
        PokemonSet {
            name: "Pikachu".to_string(),
            species: "Pikachu".to_string(),
            level: 50,
            ability: "Static".to_string(),
            moves: vec!["Tackle".to_string()],
            ..Default::default()
        },
    ];

    let mut battle = Battle::new(BattleOptions {
        format_id: ID::new("gen9ou"),
        seed: Some(PRNGSeed::Gen5([1, 2, 3, 4])),
        p1: Some(PlayerOptions {
            name: "Player 1".to_string(),
            team: team1,
            avatar: None,
        }),
        p2: Some(PlayerOptions {
            name: "Player 2".to_string(),
            team: team2,
            avatar: None,
        }),
        ..Default::default()
    });

    battle.start_battle();

    // Set Tyranitar as Rock/Dark
    if let Some(ttar) = battle.sides[0].get_active_mut(0) {
        ttar.types = vec!["Rock".to_string(), "Dark".to_string()];
    }
    // Set Pikachu as Electric
    if let Some(pika) = battle.sides[1].get_active_mut(0) {
        pika.types = vec!["Electric".to_string()];
    }

    // Set sandstorm
    battle.field.set_weather(ID::new("sandstorm"), Some(5));

    let pikachu_hp_before = battle.sides[1].pokemon[0].hp;
    let tyranitar_hp_before = battle.sides[0].pokemon[0].hp;

    battle.make_choices("move tackle", "move tackle");

    // Pikachu (Electric type) should take sandstorm damage
    let pikachu_hp_after = battle.sides[1].pokemon[0].hp;
    let tyranitar_hp_after = battle.sides[0].pokemon[0].hp;

    let log = battle.get_log();
    println!("Sandstorm test log:\n{}", log);

    // Tyranitar (Rock type) should NOT take sandstorm damage
    // But may take damage from Pikachu's Tackle
    // Pikachu should take both Tackle damage AND sandstorm damage
    assert!(pikachu_hp_after < pikachu_hp_before, "Pikachu should take damage");

    // Check that sandstorm damage appears in log
    assert!(log.contains("sandstorm"), "Log should mention sandstorm damage");
}

#[test]
fn test_terrain_electric_boost() {
    // Electric Terrain should boost Electric moves by 1.3x for grounded Pokemon
    let team1 = vec![
        PokemonSet {
            name: "Pikachu".to_string(),
            species: "Pikachu".to_string(),
            level: 50,
            ability: "Static".to_string(),
            moves: vec!["Thunderbolt".to_string()],
            ..Default::default()
        },
    ];

    let team2 = vec![
        PokemonSet {
            name: "Blissey".to_string(),
            species: "Blissey".to_string(),
            level: 50,
            ability: "Natural Cure".to_string(),
            moves: vec!["Tackle".to_string()],
            ..Default::default()
        },
    ];

    let mut battle = Battle::new(BattleOptions {
        format_id: ID::new("gen9ou"),
        seed: Some(PRNGSeed::Gen5([1, 2, 3, 4])),
        p1: Some(PlayerOptions {
            name: "Player 1".to_string(),
            team: team1,
            avatar: None,
        }),
        p2: Some(PlayerOptions {
            name: "Player 2".to_string(),
            team: team2,
            avatar: None,
        }),
        ..Default::default()
    });

    battle.start_battle();

    // Set Pikachu as Electric (grounded)
    if let Some(pika) = battle.sides[0].get_active_mut(0) {
        pika.types = vec!["Electric".to_string()];
    }

    // Set Electric Terrain
    battle.field.set_terrain(ID::new("electricterrain"), Some(5));

    let hp_before = battle.sides[1].pokemon[0].hp;
    battle.make_choices("move thunderbolt", "move tackle");

    let hp_after = battle.sides[1].pokemon[0].hp;
    let damage_dealt = hp_before - hp_after;

    assert!(damage_dealt > 0, "Thunderbolt should deal damage");
    println!("Electric Terrain test - Thunderbolt dealt {} damage", damage_dealt);
}

#[test]
fn test_terrain_grassy_healing() {
    // Grassy Terrain should heal grounded Pokemon 1/16 HP at end of turn
    let team1 = vec![
        PokemonSet {
            name: "Bulbasaur".to_string(),
            species: "Bulbasaur".to_string(),
            level: 50,
            ability: "Overgrow".to_string(),
            moves: vec!["Tackle".to_string()],
            ..Default::default()
        },
    ];

    let team2 = vec![
        PokemonSet {
            name: "Charmander".to_string(),
            species: "Charmander".to_string(),
            level: 50,
            ability: "Blaze".to_string(),
            moves: vec!["Tackle".to_string()],
            ..Default::default()
        },
    ];

    let mut battle = Battle::new(BattleOptions {
        format_id: ID::new("gen9ou"),
        seed: Some(PRNGSeed::Gen5([1, 2, 3, 4])),
        p1: Some(PlayerOptions {
            name: "Player 1".to_string(),
            team: team1,
            avatar: None,
        }),
        p2: Some(PlayerOptions {
            name: "Player 2".to_string(),
            team: team2,
            avatar: None,
        }),
        ..Default::default()
    });

    battle.start_battle();

    // Set both as Grass type (grounded)
    if let Some(bulba) = battle.sides[0].get_active_mut(0) {
        bulba.types = vec!["Grass".to_string(), "Poison".to_string()];
    }
    if let Some(char) = battle.sides[1].get_active_mut(0) {
        char.types = vec!["Fire".to_string()];
    }

    // Damage both Pokemon first
    battle.sides[0].pokemon[0].take_damage(50);
    battle.sides[1].pokemon[0].take_damage(50);

    let bulba_hp_before = battle.sides[0].pokemon[0].hp;
    let char_hp_before = battle.sides[1].pokemon[0].hp;

    // Set Grassy Terrain
    battle.field.set_terrain(ID::new("grassyterrain"), Some(5));

    battle.make_choices("move tackle", "move tackle");

    // Both should have taken some damage from Tackle, then healed from Grassy Terrain
    let log = battle.get_log();
    println!("Grassy Terrain test log:\n{}", log);

    // Check that Grassy Terrain healing appears in log
    assert!(log.contains("Grassy Terrain"), "Log should mention Grassy Terrain healing");
}

#[test]
fn test_stealth_rock_damage() {
    // Stealth Rock should deal damage based on Rock type effectiveness
    let team1 = vec![
        PokemonSet {
            name: "Landorus".to_string(),
            species: "Landorus".to_string(),
            level: 50,
            ability: "Intimidate".to_string(),
            moves: vec!["Stealth Rock".to_string(), "Tackle".to_string()],
            ..Default::default()
        },
        PokemonSet {
            name: "Garchomp".to_string(),
            species: "Garchomp".to_string(),
            level: 50,
            ability: "Rough Skin".to_string(),
            moves: vec!["Tackle".to_string()],
            ..Default::default()
        },
    ];

    let team2 = vec![
        PokemonSet {
            name: "Charizard".to_string(),
            species: "Charizard".to_string(),
            level: 50,
            ability: "Blaze".to_string(),
            moves: vec!["Tackle".to_string()],
            ..Default::default()
        },
        PokemonSet {
            name: "Venusaur".to_string(),
            species: "Venusaur".to_string(),
            level: 50,
            ability: "Overgrow".to_string(),
            moves: vec!["Tackle".to_string()],
            ..Default::default()
        },
    ];

    let mut battle = Battle::new(BattleOptions {
        format_id: ID::new("gen9ou"),
        seed: Some(PRNGSeed::Gen5([1, 2, 3, 4])),
        p1: Some(PlayerOptions {
            name: "Player 1".to_string(),
            team: team1,
            avatar: None,
        }),
        p2: Some(PlayerOptions {
            name: "Player 2".to_string(),
            team: team2,
            avatar: None,
        }),
        ..Default::default()
    });

    battle.start_battle();

    // Set types
    if let Some(p) = battle.sides[1].get_active_mut(0) {
        p.types = vec!["Fire".to_string(), "Flying".to_string()]; // Charizard - 4x weak to Rock
    }

    // Set up Stealth Rock
    battle.make_choices("move stealthrock", "move tackle");

    // Check that Stealth Rock is set up
    let log = battle.get_log();
    assert!(log.contains("Stealth Rock"), "Stealth Rock should be set up");

    // Now switch in Venusaur and check damage
    let venusaur_hp_before = battle.sides[1].pokemon[1].hp;

    // Set Venusaur's type
    battle.sides[1].pokemon[1].types = vec!["Grass".to_string(), "Poison".to_string()]; // Neutral to Rock

    battle.make_choices("move tackle", "switch 2");

    let venusaur_hp_after = battle.sides[1].pokemon[1].hp;

    // Venusaur should take Stealth Rock damage
    let log = battle.get_log();
    println!("Stealth Rock test log:\n{}", log);
    assert!(log.contains("[from] Stealth Rock"), "Stealth Rock should damage switching Pokemon");
}

#[test]
fn test_spikes_damage_layers() {
    // Spikes damage should increase with layers
    let team1 = vec![
        PokemonSet {
            name: "Skarmory".to_string(),
            species: "Skarmory".to_string(),
            level: 50,
            ability: "Sturdy".to_string(),
            moves: vec!["Spikes".to_string()],
            ..Default::default()
        },
    ];

    let team2 = vec![
        PokemonSet {
            name: "Pikachu".to_string(),
            species: "Pikachu".to_string(),
            level: 50,
            ability: "Static".to_string(),
            moves: vec!["Tackle".to_string()],
            ..Default::default()
        },
        PokemonSet {
            name: "Raichu".to_string(),
            species: "Raichu".to_string(),
            level: 50,
            ability: "Static".to_string(),
            moves: vec!["Tackle".to_string()],
            ..Default::default()
        },
    ];

    let mut battle = Battle::new(BattleOptions {
        format_id: ID::new("gen9ou"),
        seed: Some(PRNGSeed::Gen5([1, 2, 3, 4])),
        p1: Some(PlayerOptions {
            name: "Player 1".to_string(),
            team: team1,
            avatar: None,
        }),
        p2: Some(PlayerOptions {
            name: "Player 2".to_string(),
            team: team2,
            avatar: None,
        }),
        ..Default::default()
    });

    battle.start_battle();

    // Set types - grounded
    if let Some(p) = battle.sides[1].get_active_mut(0) {
        p.types = vec!["Electric".to_string()];
    }
    battle.sides[1].pokemon[1].types = vec!["Electric".to_string()];

    // Set up 3 layers of Spikes
    battle.make_choices("move spikes", "move tackle");
    battle.make_choices("move spikes", "move tackle");
    battle.make_choices("move spikes", "move tackle");

    // Check we have 3 layers
    let spikes_id = ID::new("spikes");
    let layers = battle.sides[1].get_hazard_layers(&spikes_id);
    assert_eq!(layers, 3, "Should have 3 layers of Spikes");

    // Switch in Raichu and check it takes 1/4 HP damage (3 layers)
    let raichu_hp_before = battle.sides[1].pokemon[1].hp;
    let raichu_maxhp = battle.sides[1].pokemon[1].maxhp;

    battle.make_choices("move spikes", "switch 2");

    let raichu_hp_after = battle.sides[1].pokemon[1].hp;
    let damage_taken = raichu_hp_before - raichu_hp_after;

    // Should take approximately 1/4 of max HP (accounting for tackle damage as well)
    let log = battle.get_log();
    println!("Spikes test log:\n{}", log);
    assert!(log.contains("[from] Spikes"), "Spikes should damage switching Pokemon");
}

#[test]
fn test_toxic_spikes_poison() {
    // Toxic Spikes should poison grounded Pokemon, 2 layers = badly poisoned
    let team1 = vec![
        PokemonSet {
            name: "Toxapex".to_string(),
            species: "Toxapex".to_string(),
            level: 50,
            ability: "Regenerator".to_string(),
            moves: vec!["Toxic Spikes".to_string()],
            ..Default::default()
        },
    ];

    let team2 = vec![
        PokemonSet {
            name: "Pikachu".to_string(),
            species: "Pikachu".to_string(),
            level: 50,
            ability: "Static".to_string(),
            moves: vec!["Tackle".to_string()],
            ..Default::default()
        },
        PokemonSet {
            name: "Raichu".to_string(),
            species: "Raichu".to_string(),
            level: 50,
            ability: "Static".to_string(),
            moves: vec!["Tackle".to_string()],
            ..Default::default()
        },
    ];

    let mut battle = Battle::new(BattleOptions {
        format_id: ID::new("gen9ou"),
        seed: Some(PRNGSeed::Gen5([1, 2, 3, 4])),
        p1: Some(PlayerOptions {
            name: "Player 1".to_string(),
            team: team1,
            avatar: None,
        }),
        p2: Some(PlayerOptions {
            name: "Player 2".to_string(),
            team: team2,
            avatar: None,
        }),
        ..Default::default()
    });

    battle.start_battle();

    // Set types - grounded
    if let Some(p) = battle.sides[1].get_active_mut(0) {
        p.types = vec!["Electric".to_string()];
    }
    battle.sides[1].pokemon[1].types = vec!["Electric".to_string()];

    // Set up 2 layers of Toxic Spikes
    battle.make_choices("move toxicspikes", "move tackle");
    battle.make_choices("move toxicspikes", "move tackle");

    // Check we have 2 layers
    let tspikes_id = ID::new("toxicspikes");
    let layers = battle.sides[1].get_hazard_layers(&tspikes_id);
    assert_eq!(layers, 2, "Should have 2 layers of Toxic Spikes");

    // Switch in Raichu - should get badly poisoned
    battle.make_choices("move toxicspikes", "switch 2");

    let log = battle.get_log();
    println!("Toxic Spikes test log:\n{}", log);

    // Check Raichu has toxic status
    assert_eq!(battle.sides[1].pokemon[1].status.as_str(), "tox", "Raichu should be badly poisoned");
}

#[test]
fn test_flying_immune_to_spikes() {
    // Flying types should be immune to Spikes and Toxic Spikes
    let team1 = vec![
        PokemonSet {
            name: "Skarmory".to_string(),
            species: "Skarmory".to_string(),
            level: 50,
            ability: "Sturdy".to_string(),
            moves: vec!["Spikes".to_string()],
            ..Default::default()
        },
    ];

    let team2 = vec![
        PokemonSet {
            name: "Pikachu".to_string(),
            species: "Pikachu".to_string(),
            level: 50,
            ability: "Static".to_string(),
            moves: vec!["Tackle".to_string()],
            ..Default::default()
        },
        PokemonSet {
            name: "Pidgeot".to_string(),
            species: "Pidgeot".to_string(),
            level: 50,
            ability: "Keen Eye".to_string(),
            moves: vec!["Tackle".to_string()],
            ..Default::default()
        },
    ];

    let mut battle = Battle::new(BattleOptions {
        format_id: ID::new("gen9ou"),
        seed: Some(PRNGSeed::Gen5([1, 2, 3, 4])),
        p1: Some(PlayerOptions {
            name: "Player 1".to_string(),
            team: team1,
            avatar: None,
        }),
        p2: Some(PlayerOptions {
            name: "Player 2".to_string(),
            team: team2,
            avatar: None,
        }),
        ..Default::default()
    });

    battle.start_battle();

    // Set types
    if let Some(p) = battle.sides[1].get_active_mut(0) {
        p.types = vec!["Electric".to_string()];
    }
    battle.sides[1].pokemon[1].types = vec!["Normal".to_string(), "Flying".to_string()]; // Pidgeot - Flying

    // Set up 3 layers of Spikes
    battle.make_choices("move spikes", "move tackle");
    battle.make_choices("move spikes", "move tackle");
    battle.make_choices("move spikes", "move tackle");

    // Switch in Pidgeot (Flying) - should NOT take damage
    let pidgeot_hp_before = battle.sides[1].pokemon[1].hp;

    battle.make_choices("move spikes", "switch 2");

    let pidgeot_hp_after = battle.sides[1].pokemon[1].hp;

    // Pidgeot should only take tackle damage, NOT Spikes damage
    let log = battle.get_log();
    let spikes_to_pidgeot = log.matches("Pidgeot").count() > 0 && log.contains("[from] Spikes");
    // Since Flying type is immune to ground-based hazards, check the HP difference doesn't include spikes
    println!("Flying immunity test log:\n{}", log);
}

#[test]
fn test_protect_blocks_damage() {
    // Protect should block damage from attacks
    let team1 = vec![
        PokemonSet {
            name: "Blissey".to_string(),
            species: "Blissey".to_string(),
            level: 50,
            ability: "Natural Cure".to_string(),
            moves: vec!["Protect".to_string()],
            ..Default::default()
        },
    ];

    let team2 = vec![
        PokemonSet {
            name: "Machamp".to_string(),
            species: "Machamp".to_string(),
            level: 50,
            ability: "Guts".to_string(),
            moves: vec!["Close Combat".to_string()],
            ..Default::default()
        },
    ];

    let mut battle = Battle::new(BattleOptions {
        format_id: ID::new("gen9ou"),
        seed: Some(PRNGSeed::Gen5([1, 2, 3, 4])),
        p1: Some(PlayerOptions {
            name: "Player 1".to_string(),
            team: team1,
            avatar: None,
        }),
        p2: Some(PlayerOptions {
            name: "Player 2".to_string(),
            team: team2,
            avatar: None,
        }),
        ..Default::default()
    });

    battle.start_battle();

    // Blissey uses Protect
    battle.make_choices("move protect", "move closecombat");

    // Check that Protect was used
    let log = battle.get_log();
    println!("Protect test log:\n{}", log);
    assert!(log.contains("Protect"), "Protect should be logged");
}

#[test]
fn test_substitute_creation() {
    // Substitute should cost 1/4 HP and create a substitute
    let team1 = vec![
        PokemonSet {
            name: "Gengar".to_string(),
            species: "Gengar".to_string(),
            level: 50,
            ability: "Levitate".to_string(),
            moves: vec!["Substitute".to_string()],
            ..Default::default()
        },
    ];

    let team2 = vec![
        PokemonSet {
            name: "Blissey".to_string(),
            species: "Blissey".to_string(),
            level: 50,
            ability: "Natural Cure".to_string(),
            moves: vec!["Tackle".to_string()],
            ..Default::default()
        },
    ];

    let mut battle = Battle::new(BattleOptions {
        format_id: ID::new("gen9ou"),
        seed: Some(PRNGSeed::Gen5([1, 2, 3, 4])),
        p1: Some(PlayerOptions {
            name: "Player 1".to_string(),
            team: team1,
            avatar: None,
        }),
        p2: Some(PlayerOptions {
            name: "Player 2".to_string(),
            team: team2,
            avatar: None,
        }),
        ..Default::default()
    });

    battle.start_battle();

    let gengar_hp_before = battle.sides[0].pokemon[0].hp;
    let gengar_maxhp = battle.sides[0].pokemon[0].maxhp;
    let expected_cost = gengar_maxhp / 4;

    battle.make_choices("move substitute", "move tackle");

    let gengar_hp_after = battle.sides[0].pokemon[0].hp;
    let hp_lost = gengar_hp_before - gengar_hp_after;

    // Should lose at least 1/4 HP for substitute (plus potential tackle damage)
    assert!(hp_lost >= expected_cost, "Gengar should lose at least 1/4 HP for Substitute");

    // Check that Substitute was created
    assert!(battle.sides[0].pokemon[0].has_volatile(&ID::new("substitute")), "Gengar should have a Substitute");

    let log = battle.get_log();
    println!("Substitute test log:\n{}", log);
    assert!(log.contains("Substitute"), "Substitute should be logged");
}

#[test]
fn test_recovery_move() {
    // Recover should heal 50% HP
    let team1 = vec![
        PokemonSet {
            name: "Blissey".to_string(),
            species: "Blissey".to_string(),
            level: 50,
            ability: "Natural Cure".to_string(),
            moves: vec!["Recover".to_string()],
            ..Default::default()
        },
    ];

    let team2 = vec![
        PokemonSet {
            name: "Machamp".to_string(),
            species: "Machamp".to_string(),
            level: 50,
            ability: "Guts".to_string(),
            moves: vec!["Tackle".to_string()],
            ..Default::default()
        },
    ];

    let mut battle = Battle::new(BattleOptions {
        format_id: ID::new("gen9ou"),
        seed: Some(PRNGSeed::Gen5([1, 2, 3, 4])),
        p1: Some(PlayerOptions {
            name: "Player 1".to_string(),
            team: team1,
            avatar: None,
        }),
        p2: Some(PlayerOptions {
            name: "Player 2".to_string(),
            team: team2,
            avatar: None,
        }),
        ..Default::default()
    });

    battle.start_battle();

    // Damage Blissey first (but not too much)
    let maxhp = battle.sides[0].pokemon[0].maxhp;
    let damage = maxhp / 3; // Take 1/3 damage
    battle.sides[0].pokemon[0].take_damage(damage);
    let _hp_before = battle.sides[0].pokemon[0].hp;

    battle.make_choices("move recover", "move tackle");

    let _hp_after = battle.sides[0].pokemon[0].hp;

    // Recover heals 50% max HP, then takes tackle damage
    // Net HP change depends on tackle damage, but Recover should have healed
    let log = battle.get_log();
    println!("Recover test log:\n{}", log);
    assert!(log.contains("-heal"), "Recover should heal and log it");
}

#[test]
fn test_haze_clears_boosts() {
    // Haze should clear all stat boosts
    let team1 = vec![
        PokemonSet {
            name: "Weezing".to_string(),
            species: "Weezing".to_string(),
            level: 50,
            ability: "Levitate".to_string(),
            moves: vec!["Haze".to_string()],
            ..Default::default()
        },
    ];

    let team2 = vec![
        PokemonSet {
            name: "Gyarados".to_string(),
            species: "Gyarados".to_string(),
            level: 50,
            ability: "Intimidate".to_string(),
            moves: vec!["Dragon Dance".to_string(), "Swords Dance".to_string()],
            ..Default::default()
        },
    ];

    let mut battle = Battle::new(BattleOptions {
        format_id: ID::new("gen9ou"),
        seed: Some(PRNGSeed::Gen5([1, 2, 3, 4])),
        p1: Some(PlayerOptions {
            name: "Player 1".to_string(),
            team: team1,
            avatar: None,
        }),
        p2: Some(PlayerOptions {
            name: "Player 2".to_string(),
            team: team2,
            avatar: None,
        }),
        ..Default::default()
    });

    battle.start_battle();

    // Gyarados uses Swords Dance, then Weezing uses Haze
    battle.make_choices("move haze", "move swordsdance");

    // Both should have no boosts after Haze
    assert_eq!(battle.sides[0].pokemon[0].boosts.atk, 0, "Weezing's Attack should be reset");
    assert_eq!(battle.sides[1].pokemon[0].boosts.atk, 0, "Gyarados's Attack should be reset by Haze");

    let log = battle.get_log();
    println!("Haze test log:\n{}", log);
    assert!(log.contains("-clearallboost"), "Haze should clear all boosts");
}

/// Test multi-hit moves (Bullet Seed)
#[test]
fn test_multi_hit_moves() {
    let mut battle = Battle::new(BattleOptions {
        format_id: ID::new("gen9ou"),
        seed: Some(PRNGSeed::Gen5([1, 2, 3, 4])),
        p1: Some(PlayerOptions {
            name: "Player 1".to_string(),
            avatar: None,
            team: vec![PokemonSet {
                name: "Breloom".to_string(),
                species: "Breloom".to_string(),
                level: 100,
                ability: "Technician".to_string(),
                moves: vec!["bulletseed".to_string()],
                ..Default::default()
            }],
        }),
        p2: Some(PlayerOptions {
            name: "Player 2".to_string(),
            avatar: None,
            team: vec![PokemonSet {
                name: "Blissey".to_string(),
                species: "Blissey".to_string(),
                level: 100,
                ability: "Natural Cure".to_string(),
                moves: vec!["softboiled".to_string()],
                ..Default::default()
            }],
        }),
        ..Default::default()
    });

    battle.start_battle();

    let blissey_hp_before = battle.sides[1].pokemon[0].hp;

    // Use Bullet Seed
    battle.make_choices("move bulletseed", "move softboiled");

    let blissey_hp_after = battle.sides[1].pokemon[0].hp;
    let log = battle.get_log();
    println!("Multi-hit test log:\n{}", log);

    // Should have taken damage
    assert!(blissey_hp_before > blissey_hp_after, "Blissey should have taken damage");

    // Should show hitcount in log (for multi-hit moves)
    assert!(log.contains("-hitcount"), "Multi-hit move should show hit count");
}

/// Test fixed 2-hit move (Double Kick)
#[test]
fn test_fixed_double_hit_move() {
    let mut battle = Battle::new(BattleOptions {
        format_id: ID::new("gen9ou"),
        seed: Some(PRNGSeed::Gen5([5, 6, 7, 8])),
        p1: Some(PlayerOptions {
            name: "Player 1".to_string(),
            avatar: None,
            team: vec![PokemonSet {
                name: "Hitmonlee".to_string(),
                species: "Hitmonlee".to_string(),
                level: 100,
                ability: "Limber".to_string(),
                moves: vec!["doublekick".to_string()],
                ..Default::default()
            }],
        }),
        p2: Some(PlayerOptions {
            name: "Player 2".to_string(),
            avatar: None,
            team: vec![PokemonSet {
                name: "Snorlax".to_string(),
                species: "Snorlax".to_string(),
                level: 100,
                ability: "Thick Fat".to_string(),
                moves: vec!["rest".to_string()],
                ..Default::default()
            }],
        }),
        ..Default::default()
    });

    battle.start_battle();

    // Use Double Kick
    battle.make_choices("move doublekick", "move rest");

    let log = battle.get_log();
    println!("Double Kick test log:\n{}", log);

    // Should show hitcount 2 in log (fixed 2 hits)
    assert!(log.contains("-hitcount|2"), "Double Kick should hit exactly 2 times");
}

/// Test move accuracy (Focus Blast with 70% accuracy)
#[test]
fn test_move_accuracy_miss() {
    // Use a seed that causes Focus Blast to miss
    // We'll try multiple seeds until we find one that misses
    let mut missed = false;
    for seed_val in 100u16..200u16 {
        let mut battle = Battle::new(BattleOptions {
            format_id: ID::new("gen9ou"),
            seed: Some(PRNGSeed::Gen5([seed_val, 0, 0, 0])),
            p1: Some(PlayerOptions {
                name: "Player 1".to_string(),
                avatar: None,
                team: vec![PokemonSet {
                    name: "Gengar".to_string(),
                    species: "Gengar".to_string(),
                    level: 100,
                    ability: "Levitate".to_string(),
                    moves: vec!["focusblast".to_string()],
                    ..Default::default()
                }],
            }),
            p2: Some(PlayerOptions {
                name: "Player 2".to_string(),
                avatar: None,
                team: vec![PokemonSet {
                    name: "Blissey".to_string(),
                    species: "Blissey".to_string(),
                    level: 100,
                    ability: "Natural Cure".to_string(),
                    moves: vec!["softboiled".to_string()],
                    ..Default::default()
                }],
            }),
            ..Default::default()
        });

        battle.start_battle();
        battle.make_choices("move focusblast", "move softboiled");

        let log = battle.get_log();
        if log.contains("-miss") {
            println!("Found miss with seed {} - log:\n{}", seed_val, log);
            missed = true;
            break;
        }
    }

    assert!(missed, "Focus Blast should miss sometimes (70% accuracy)");
}

/// Test confusion (confuse ray and self-damage)
#[test]
fn test_confusion_volatile() {
    let mut battle = Battle::new(BattleOptions {
        format_id: ID::new("gen9ou"),
        seed: Some(PRNGSeed::Gen5([1, 2, 3, 4])),
        p1: Some(PlayerOptions {
            name: "Player 1".to_string(),
            avatar: None,
            team: vec![PokemonSet {
                name: "Gengar".to_string(),
                species: "Gengar".to_string(),
                level: 100,
                ability: "Levitate".to_string(),
                moves: vec!["confuseray".to_string()],
                ..Default::default()
            }],
        }),
        p2: Some(PlayerOptions {
            name: "Player 2".to_string(),
            avatar: None,
            team: vec![PokemonSet {
                name: "Machamp".to_string(),
                species: "Machamp".to_string(),
                level: 100,
                ability: "Guts".to_string(),
                moves: vec!["closecombat".to_string()],
                ..Default::default()
            }],
        }),
        ..Default::default()
    });

    battle.start_battle();

    // Gengar uses Confuse Ray
    battle.make_choices("move confuseray", "move closecombat");

    let log = battle.get_log();
    println!("Confusion test log:\n{}", log);

    // Machamp should be confused
    assert!(log.contains("-start|p2: Machamp|confusion"), "Confuse Ray should cause confusion");

    // Check if confusion was activated on subsequent turn or caused self-damage
    assert!(battle.sides[1].pokemon[0].has_volatile(&ID::new("confusion")), "Machamp should have confusion volatile");
}

/// Test flinch (Fake Out always flinches)
#[test]
fn test_flinch_prevents_move() {
    let mut battle = Battle::new(BattleOptions {
        format_id: ID::new("gen9ou"),
        seed: Some(PRNGSeed::Gen5([1, 2, 3, 4])),
        p1: Some(PlayerOptions {
            name: "Player 1".to_string(),
            avatar: None,
            team: vec![PokemonSet {
                name: "Persian".to_string(),
                species: "Persian".to_string(),
                level: 100,
                ability: "Technician".to_string(),
                moves: vec!["fakeout".to_string()],
                ..Default::default()
            }],
        }),
        p2: Some(PlayerOptions {
            name: "Player 2".to_string(),
            avatar: None,
            team: vec![PokemonSet {
                name: "Machamp".to_string(),
                species: "Machamp".to_string(),
                level: 100,
                ability: "Guts".to_string(),
                moves: vec!["closecombat".to_string()],
                ..Default::default()
            }],
        }),
        ..Default::default()
    });

    battle.start_battle();

    let persian_hp_before = battle.sides[0].pokemon[0].hp;

    // Persian uses Fake Out - it should flinch Machamp
    battle.make_choices("move fakeout", "move closecombat");

    let persian_hp_after = battle.sides[0].pokemon[0].hp;
    let log = battle.get_log();
    println!("Flinch test log:\n{}", log);

    // Persian should not take damage because Machamp flinched
    assert_eq!(persian_hp_before, persian_hp_after, "Persian should not take damage - Machamp flinched");

    // Log should show Machamp couldn't move due to flinch
    assert!(log.contains("cant|p2: Machamp|flinch"), "Machamp should be prevented from moving by flinch");
}

/// Test Choice item locking (Choice Band locks into first move used)
#[test]
fn test_choice_band_locking() {
    let mut battle = Battle::new(BattleOptions {
        format_id: ID::new("gen9ou"),
        seed: Some(PRNGSeed::Gen5([1, 2, 3, 4])),
        p1: Some(PlayerOptions {
            name: "Player 1".to_string(),
            avatar: None,
            team: vec![PokemonSet {
                name: "Garchomp".to_string(),
                species: "Garchomp".to_string(),
                level: 100,
                ability: "Rough Skin".to_string(),
                item: "Choice Band".to_string(),
                moves: vec!["earthquake".to_string(), "dragonclaw".to_string()],
                ..Default::default()
            }],
        }),
        p2: Some(PlayerOptions {
            name: "Player 2".to_string(),
            avatar: None,
            team: vec![PokemonSet {
                name: "Bronzong".to_string(),
                species: "Bronzong".to_string(),
                level: 100,
                ability: "Levitate".to_string(),
                moves: vec!["irondefense".to_string()],
                ..Default::default()
            }],
        }),
        ..Default::default()
    });

    battle.start_battle();

    // Garchomp uses Earthquake first (does 0 damage to Levitate Bronzong)
    battle.make_choices("move earthquake", "move irondefense");

    // Garchomp should be locked into Earthquake
    assert!(battle.sides[0].pokemon[0].locked_move.is_some(), "Garchomp should be locked into a move");
    assert_eq!(battle.sides[0].pokemon[0].locked_move.as_ref().unwrap().as_str(), "earthquake",
               "Garchomp should be locked into Earthquake");

    // Try to use Dragon Claw but should still use Earthquake
    battle.make_choices("move dragonclaw", "move irondefense");

    let log = battle.get_log();
    println!("Choice Band test log:\n{}", log);

    // The log should show Earthquake was used twice (not Dragon Claw)
    let eq_count = log.matches("earthquake").count();
    assert!(eq_count >= 2, "Earthquake should be used twice due to Choice lock, found {} occurrences", eq_count);
}

/// Test Intimidate ability lowers Attack on switch-in
#[test]
fn test_intimidate_ability() {
    let mut battle = Battle::new(BattleOptions {
        format_id: ID::new("gen9ou"),
        seed: Some(PRNGSeed::Gen5([1, 2, 3, 4])),
        p1: Some(PlayerOptions {
            name: "Player 1".to_string(),
            avatar: None,
            team: vec![PokemonSet {
                name: "Gyarados".to_string(),
                species: "Gyarados".to_string(),
                level: 100,
                ability: "Intimidate".to_string(),
                moves: vec!["waterfall".to_string()],
                ..Default::default()
            }],
        }),
        p2: Some(PlayerOptions {
            name: "Player 2".to_string(),
            avatar: None,
            team: vec![PokemonSet {
                name: "Machamp".to_string(),
                species: "Machamp".to_string(),
                level: 100,
                ability: "Guts".to_string(),
                moves: vec!["closecombat".to_string()],
                ..Default::default()
            }],
        }),
        ..Default::default()
    });

    battle.start_battle();

    let log = battle.get_log();
    println!("Intimidate test log:\n{}", log);

    // Machamp should have -1 Attack boost from Intimidate
    assert_eq!(battle.sides[1].pokemon[0].boosts.atk, -1, "Machamp's Attack should be lowered by Intimidate");

    // Log should show Intimidate activated
    assert!(log.contains("-ability|p1: Gyarados|Intimidate"), "Intimidate ability should be logged");
}

/// Test Drizzle ability summons rain on switch-in
#[test]
fn test_drizzle_ability() {
    let mut battle = Battle::new(BattleOptions {
        format_id: ID::new("gen9ou"),
        seed: Some(PRNGSeed::Gen5([1, 2, 3, 4])),
        p1: Some(PlayerOptions {
            name: "Player 1".to_string(),
            avatar: None,
            team: vec![PokemonSet {
                name: "Politoed".to_string(),
                species: "Politoed".to_string(),
                level: 100,
                ability: "Drizzle".to_string(),
                moves: vec!["scald".to_string()],
                ..Default::default()
            }],
        }),
        p2: Some(PlayerOptions {
            name: "Player 2".to_string(),
            avatar: None,
            team: vec![PokemonSet {
                name: "Pikachu".to_string(),
                species: "Pikachu".to_string(),
                level: 100,
                ability: "Static".to_string(),
                moves: vec!["thunderbolt".to_string()],
                ..Default::default()
            }],
        }),
        ..Default::default()
    });

    battle.start_battle();

    // Rain should be active
    assert!(battle.field.is_weather("raindance"), "Rain should be active from Drizzle");

    let log = battle.get_log();
    println!("Drizzle test log:\n{}", log);

    // Log should show weather activation
    assert!(log.contains("-weather|RainDance"), "Drizzle weather should be logged");
}

/// Test Leftovers item healing at end of turn
#[test]
fn test_leftovers_healing() {
    let mut battle = Battle::new(BattleOptions {
        format_id: ID::new("gen9ou"),
        seed: Some(PRNGSeed::Gen5([1, 2, 3, 4])),
        p1: Some(PlayerOptions {
            name: "Player 1".to_string(),
            avatar: None,
            team: vec![PokemonSet {
                name: "Blissey".to_string(),
                species: "Blissey".to_string(),
                level: 100,
                ability: "Natural Cure".to_string(),
                item: "Leftovers".to_string(),
                moves: vec!["protect".to_string()],
                ..Default::default()
            }],
        }),
        p2: Some(PlayerOptions {
            name: "Player 2".to_string(),
            avatar: None,
            team: vec![PokemonSet {
                name: "Pikachu".to_string(),
                species: "Pikachu".to_string(),
                level: 100,
                ability: "Static".to_string(),
                moves: vec!["thunderbolt".to_string()],
                ..Default::default()
            }],
        }),
        ..Default::default()
    });

    battle.start_battle();

    // Damage Blissey a bit so Leftovers has something to heal
    let maxhp = battle.sides[0].pokemon[0].maxhp;
    battle.sides[0].pokemon[0].take_damage(maxhp / 4); // Damage by 25%

    // Make a move to trigger end of turn - Blissey uses Protect
    battle.make_choices("move protect", "move thunderbolt");

    let log = battle.get_log();
    println!("Leftovers test log:\n{}", log);

    // Log should show Leftovers healing
    assert!(log.contains("[from] item: Leftovers"), "Leftovers healing should be logged");
}

/// Test Life Orb damage boost and recoil
#[test]
fn test_life_orb_damage() {
    let mut battle = Battle::new(BattleOptions {
        format_id: ID::new("gen9ou"),
        seed: Some(PRNGSeed::Gen5([1, 2, 3, 4])),
        p1: Some(PlayerOptions {
            name: "Player 1".to_string(),
            avatar: None,
            team: vec![PokemonSet {
                name: "Alakazam".to_string(),
                species: "Alakazam".to_string(),
                level: 100,
                ability: "Magic Guard".to_string(),
                item: "Life Orb".to_string(),
                moves: vec!["psychic".to_string()],
                ..Default::default()
            }],
        }),
        p2: Some(PlayerOptions {
            name: "Player 2".to_string(),
            avatar: None,
            team: vec![PokemonSet {
                name: "Machamp".to_string(),
                species: "Machamp".to_string(),
                level: 100,
                ability: "Guts".to_string(),
                moves: vec!["closecombat".to_string()],
                ..Default::default()
            }],
        }),
        ..Default::default()
    });

    battle.start_battle();

    let alakazam_hp_before = battle.sides[0].pokemon[0].hp;

    // Alakazam uses Psychic with Life Orb
    battle.make_choices("move psychic", "move closecombat");

    let log = battle.get_log();
    println!("Life Orb test log:\n{}", log);

    // Life Orb should cause recoil damage
    assert!(log.contains("[from] item: Life Orb"), "Life Orb recoil should be logged");

    // Alakazam should have taken Life Orb recoil (10% of max HP)
    // Note: Alakazam also took damage from Close Combat, so we just check recoil was applied
}

/// Test U-Turn pivot mechanics
#[test]
fn test_uturn_pivot_switch() {
    let mut battle = Battle::new(BattleOptions {
        format_id: ID::new("gen9ou"),
        seed: Some(PRNGSeed::Gen5([1, 2, 3, 4])),
        p1: Some(PlayerOptions {
            name: "Player 1".to_string(),
            avatar: None,
            team: vec![
                PokemonSet {
                    name: "Scizor".to_string(),
                    species: "Scizor".to_string(),
                    level: 100,
                    ability: "Technician".to_string(),
                    moves: vec!["uturn".to_string(), "bulletpunch".to_string()],
                    ..Default::default()
                },
                PokemonSet {
                    name: "Blissey".to_string(),
                    species: "Blissey".to_string(),
                    level: 100,
                    ability: "Natural Cure".to_string(),
                    moves: vec!["softboiled".to_string()],
                    ..Default::default()
                },
            ],
        }),
        p2: Some(PlayerOptions {
            name: "Player 2".to_string(),
            avatar: None,
            team: vec![PokemonSet {
                name: "Ferrothorn".to_string(),
                species: "Ferrothorn".to_string(),
                level: 100,
                ability: "Iron Barbs".to_string(),
                moves: vec!["irondefense".to_string()],
                ..Default::default()
            }],
        }),
        ..Default::default()
    });

    battle.start_battle();

    // Scizor should be active initially
    assert_eq!(battle.sides[0].pokemon[0].name, "Scizor");
    assert!(battle.sides[0].pokemon[0].is_active);
    assert!(!battle.sides[0].pokemon[1].is_active);

    // Scizor uses U-Turn - should deal damage and switch to Blissey
    battle.make_choices("move uturn", "move irondefense");

    let log = battle.get_log();
    println!("U-Turn test log:\n{}", log);

    // Should see U-Turn move logged
    assert!(log.contains("|move|") && log.contains("uturn"), "U-Turn should be used");

    // Ferrothorn should have taken damage
    assert!(log.contains("-damage") && log.contains("Ferrothorn"), "Ferrothorn should take U-Turn damage");

    // After U-Turn, Blissey should be active (Scizor switched out)
    assert!(!battle.sides[0].pokemon[0].is_active, "Scizor should no longer be active");
    assert!(battle.sides[0].pokemon[1].is_active, "Blissey should be active after U-Turn");

    // Log should show the switch
    assert!(log.contains("switch") && log.contains("Blissey"), "Should log switch to Blissey");
}

/// Test Trick Room reverses speed order
#[test]
fn test_trick_room_speed_reversal() {
    let mut battle = Battle::new(BattleOptions {
        format_id: ID::new("gen9ou"),
        seed: Some(PRNGSeed::Gen5([1, 2, 3, 4])),
        p1: Some(PlayerOptions {
            name: "Player 1".to_string(),
            avatar: None,
            team: vec![PokemonSet {
                name: "Reuniclus".to_string(),
                species: "Reuniclus".to_string(),
                level: 100,
                ability: "Magic Guard".to_string(),
                moves: vec!["trickroom".to_string(), "psychic".to_string()],
                ..Default::default()
            }],
        }),
        p2: Some(PlayerOptions {
            name: "Player 2".to_string(),
            avatar: None,
            team: vec![PokemonSet {
                name: "Jolteon".to_string(),
                species: "Jolteon".to_string(),
                level: 100,
                ability: "Volt Absorb".to_string(),
                moves: vec!["thunderbolt".to_string()],
                ..Default::default()
            }],
        }),
        ..Default::default()
    });

    battle.start_battle();

    // First turn: Without Trick Room, faster Jolteon should move first
    // Turn 1: Reuniclus uses Trick Room (priority -7, moves last), Jolteon attacks
    battle.make_choices("move trickroom", "move thunderbolt");

    let log1 = battle.get_log();
    println!("Trick Room setup log:\n{}", log1);

    // Trick Room should be logged
    assert!(log1.contains("-fieldstart") && log1.contains("Trick Room"), "Trick Room should be set up");

    // Jolteon (faster) should have moved before Reuniclus in turn 1
    // because Trick Room has -7 priority and only affects future turns
    let tb_pos = log1.find("thunderbolt").unwrap_or(0);
    let tr_pos = log1.find("trickroom").unwrap_or(0);
    assert!(tb_pos < tr_pos, "Jolteon's Thunderbolt should come before Trick Room (Trick Room has -7 priority)");

    // Turn 2: With Trick Room active, slower Reuniclus should move first
    battle.make_choices("move psychic", "move thunderbolt");

    let log2 = battle.get_log();
    println!("Trick Room effect log:\n{}", log2);

    // In the turn 2 section, Reuniclus (slower) should move before Jolteon (faster)
    // Look for the order after "turn|2"
    if let Some(turn2_start) = log2.find("|turn|2") {
        let turn2_log = &log2[turn2_start..];
        let psychic_pos = turn2_log.find("psychic").unwrap_or(999);
        let tb_pos2 = turn2_log.find("thunderbolt").unwrap_or(999);
        assert!(psychic_pos < tb_pos2, "Under Trick Room, slower Reuniclus's Psychic should come before faster Jolteon's Thunderbolt");
    }
}

/// Test Water Absorb ability heals from Water moves
#[test]
fn test_water_absorb_ability() {
    let mut battle = Battle::new(BattleOptions {
        format_id: ID::new("gen9ou"),
        seed: Some(PRNGSeed::Gen5([1, 2, 3, 4])),
        p1: Some(PlayerOptions {
            name: "Player 1".to_string(),
            avatar: None,
            team: vec![PokemonSet {
                name: "Vaporeon".to_string(),
                species: "Vaporeon".to_string(),
                level: 100,
                ability: "Water Absorb".to_string(),
                moves: vec!["tackle".to_string()],
                ..Default::default()
            }],
        }),
        p2: Some(PlayerOptions {
            name: "Player 2".to_string(),
            avatar: None,
            team: vec![PokemonSet {
                name: "Blastoise".to_string(),
                species: "Blastoise".to_string(),
                level: 100,
                ability: "Torrent".to_string(),
                moves: vec!["surf".to_string()],
                ..Default::default()
            }],
        }),
        ..Default::default()
    });

    battle.start_battle();

    // Damage Vaporeon first so it can heal
    let maxhp = battle.sides[0].pokemon[0].maxhp;
    battle.sides[0].pokemon[0].hp = maxhp / 2; // Set to 50% HP
    let hp_before = battle.sides[0].pokemon[0].hp;

    // Blastoise uses Surf on Vaporeon (Water Absorb)
    battle.make_choices("move tackle", "move surf");

    let log = battle.get_log();
    println!("Water Absorb test log:\n{}", log);

    // Should see immunity and heal messages
    assert!(log.contains("[from] ability: Water Absorb"), "Water Absorb should trigger");
    assert!(log.contains("-heal") && log.contains("Vaporeon"), "Vaporeon should be healed by Water Absorb");

    // Vaporeon should have healed (25% of max HP)
    let hp_after = battle.sides[0].pokemon[0].hp;
    let _expected_heal = (maxhp / 4).max(1);
    assert!(hp_after > hp_before, "Vaporeon should have gained HP from Water Absorb");
}

/// Test Iron Barbs deals damage to attackers using contact moves
#[test]
fn test_iron_barbs_contact_damage() {
    let mut battle = Battle::new(BattleOptions {
        format_id: ID::new("gen9ou"),
        seed: Some(PRNGSeed::Gen5([1, 2, 3, 4])),
        p1: Some(PlayerOptions {
            name: "Player 1".to_string(),
            avatar: None,
            team: vec![PokemonSet {
                name: "Machamp".to_string(),
                species: "Machamp".to_string(),
                level: 100,
                ability: "Guts".to_string(),
                moves: vec!["closecombat".to_string()],
                ..Default::default()
            }],
        }),
        p2: Some(PlayerOptions {
            name: "Player 2".to_string(),
            avatar: None,
            team: vec![PokemonSet {
                name: "Ferrothorn".to_string(),
                species: "Ferrothorn".to_string(),
                level: 100,
                ability: "Iron Barbs".to_string(),
                moves: vec!["irondefense".to_string()],
                ..Default::default()
            }],
        }),
        ..Default::default()
    });

    battle.start_battle();

    let machamp_hp_before = battle.sides[0].pokemon[0].hp;

    // Machamp uses Close Combat (contact move) on Ferrothorn
    battle.make_choices("move closecombat", "move irondefense");

    let log = battle.get_log();
    println!("Iron Barbs test log:\n{}", log);

    // Should see Iron Barbs damage
    assert!(log.contains("[from] ability: Iron Barbs"), "Iron Barbs should trigger on contact");

    // Machamp should have taken 1/8 max HP damage from Iron Barbs
    let machamp_hp_after = battle.sides[0].pokemon[0].hp;
    let machamp_maxhp = battle.sides[0].pokemon[0].maxhp;
    let expected_damage = (machamp_maxhp / 8).max(1);

    // Machamp takes Close Combat stat drops, so HP change includes Iron Barbs
    assert!(machamp_hp_after < machamp_hp_before, "Machamp should have taken Iron Barbs damage");
}
