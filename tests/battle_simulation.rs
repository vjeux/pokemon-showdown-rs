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
