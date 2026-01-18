/// Rust Battle Test Runner
///
/// Runs a random battle with a specific seed and outputs:
/// - Turn number
/// - PRNG call count before/after each turn
/// - HP of all active Pokemon
///
/// Output format: #<iteration>: turn=<turn>, prng=<before>-><after>, P1=[...], P2=[...]
///
/// Usage: cargo run --example test_battle_rust [seed_number]

use pokemon_showdown::{Battle, BattleOptions, PlayerOptions, PokemonSet, PRNGSeed, ID};
use pokemon_showdown::battle::TeamFormat;
use pokemon_showdown::dex_data::{StatsTable, Gender};
use std::fs;
use std::env;

fn print_battle_state(battle: &Battle, iteration: usize, prng_calls: usize) {
    eprintln!("");
    eprintln!("========== Turn {} (Iteration {}) ==========", battle.turn, iteration);

    // Field conditions
    let weather = if battle.field.weather.as_str().is_empty() { "none" } else { battle.field.weather.as_str() };
    let terrain = if battle.field.terrain.as_str().is_empty() { "none" } else { battle.field.terrain.as_str() };
    eprintln!("Field: Weather={}, Terrain={}, PRNG calls={}", weather, terrain, prng_calls);

    // Player 1 state
    eprintln!("");
    eprintln!("--- Player 1 ---");
    for (idx, active_idx) in battle.sides[0].active.iter().enumerate() {
        if let Some(poke_idx) = active_idx {
            if let Some(pokemon) = battle.sides[0].pokemon.get(*poke_idx) {
                let display_name = &pokemon.name;
                let hp_percent = (pokemon.hp as f32 / pokemon.maxhp as f32 * 100.0) as i32;

                eprintln!("  Active[{}]: {} ({})", idx, display_name, pokemon.species_id.as_str());

                let status_str = if pokemon.status.as_str().is_empty() { "none" } else { pokemon.status.as_str() };
                eprintln!("    HP: {}/{} ({}%) | Status: {}",
                    pokemon.hp, pokemon.maxhp, hp_percent, status_str);

                let item_name = if pokemon.item.as_str().is_empty() { "none" } else { pokemon.item.as_str() };
                let ability_name = pokemon.ability.as_str();
                eprintln!("    Item: {} | Ability: {}", item_name, ability_name);

                eprintln!("    Stats: Atk:{} Def:{} SpA:{} SpD:{} Spe:{}",
                    pokemon.stored_stats.atk, pokemon.stored_stats.def, pokemon.stored_stats.spa,
                    pokemon.stored_stats.spd, pokemon.stored_stats.spe);

                // Boosts
                let mut boosts_str = Vec::new();
                if pokemon.boosts.atk != 0 { boosts_str.push(format!("atk:{:+}", pokemon.boosts.atk)); }
                if pokemon.boosts.def != 0 { boosts_str.push(format!("def:{:+}", pokemon.boosts.def)); }
                if pokemon.boosts.spa != 0 { boosts_str.push(format!("spa:{:+}", pokemon.boosts.spa)); }
                if pokemon.boosts.spd != 0 { boosts_str.push(format!("spd:{:+}", pokemon.boosts.spd)); }
                if pokemon.boosts.spe != 0 { boosts_str.push(format!("spe:{:+}", pokemon.boosts.spe)); }
                if pokemon.boosts.accuracy != 0 { boosts_str.push(format!("accuracy:{:+}", pokemon.boosts.accuracy)); }
                if pokemon.boosts.evasion != 0 { boosts_str.push(format!("evasion:{:+}", pokemon.boosts.evasion)); }
                let boosts_display = if boosts_str.is_empty() { "none".to_string() } else { boosts_str.join(", ") };
                eprintln!("    Boosts: {}", boosts_display);

                // Volatiles
                let volatiles: Vec<_> = pokemon.volatiles.keys()
                    .filter(|v| v.as_str() != "lockedmove")
                    .map(|v| v.as_str())
                    .collect();
                let volatiles_display = if volatiles.is_empty() { "none" } else { &volatiles.join(", ") };
                eprintln!("    Volatiles: {}", volatiles_display);

                // Moves
                let moves: Vec<String> = pokemon.move_slots.iter()
                    .map(|m| format!("{}({}/{})", m.id.as_str(), m.pp, m.maxpp))
                    .collect();
                eprintln!("    Moves: {}", moves.join(", "));
            }
        }
    }

    // Side conditions for P1
    if !battle.sides[0].side_conditions.is_empty() {
        let conditions: Vec<_> = battle.sides[0].side_conditions.keys()
            .map(|c| c.as_str())
            .collect();
        eprintln!("  Side Conditions: {}", conditions.join(", "));
    }

    // Player 2 state
    eprintln!("");
    eprintln!("--- Player 2 ---");
    for (idx, active_idx) in battle.sides[1].active.iter().enumerate() {
        if let Some(poke_idx) = active_idx {
            if let Some(pokemon) = battle.sides[1].pokemon.get(*poke_idx) {
                let display_name = &pokemon.name;
                let hp_percent = (pokemon.hp as f32 / pokemon.maxhp as f32 * 100.0) as i32;

                eprintln!("  Active[{}]: {} ({})", idx, display_name, pokemon.species_id.as_str());

                let status_str = if pokemon.status.as_str().is_empty() { "none" } else { pokemon.status.as_str() };
                eprintln!("    HP: {}/{} ({}%) | Status: {}",
                    pokemon.hp, pokemon.maxhp, hp_percent, status_str);

                let item_name = if pokemon.item.as_str().is_empty() { "none" } else { pokemon.item.as_str() };
                let ability_name = pokemon.ability.as_str();
                eprintln!("    Item: {} | Ability: {}", item_name, ability_name);

                eprintln!("    Stats: Atk:{} Def:{} SpA:{} SpD:{} Spe:{}",
                    pokemon.stored_stats.atk, pokemon.stored_stats.def, pokemon.stored_stats.spa,
                    pokemon.stored_stats.spd, pokemon.stored_stats.spe);

                // Boosts
                let mut boosts_str = Vec::new();
                if pokemon.boosts.atk != 0 { boosts_str.push(format!("atk:{:+}", pokemon.boosts.atk)); }
                if pokemon.boosts.def != 0 { boosts_str.push(format!("def:{:+}", pokemon.boosts.def)); }
                if pokemon.boosts.spa != 0 { boosts_str.push(format!("spa:{:+}", pokemon.boosts.spa)); }
                if pokemon.boosts.spd != 0 { boosts_str.push(format!("spd:{:+}", pokemon.boosts.spd)); }
                if pokemon.boosts.spe != 0 { boosts_str.push(format!("spe:{:+}", pokemon.boosts.spe)); }
                if pokemon.boosts.accuracy != 0 { boosts_str.push(format!("accuracy:{:+}", pokemon.boosts.accuracy)); }
                if pokemon.boosts.evasion != 0 { boosts_str.push(format!("evasion:{:+}", pokemon.boosts.evasion)); }
                let boosts_display = if boosts_str.is_empty() { "none".to_string() } else { boosts_str.join(", ") };
                eprintln!("    Boosts: {}", boosts_display);

                // Volatiles
                let volatiles: Vec<_> = pokemon.volatiles.keys()
                    .filter(|v| v.as_str() != "lockedmove")
                    .map(|v| v.as_str())
                    .collect();
                let volatiles_display = if volatiles.is_empty() { "none" } else { &volatiles.join(", ") };
                eprintln!("    Volatiles: {}", volatiles_display);

                // Moves
                let moves: Vec<String> = pokemon.move_slots.iter()
                    .map(|m| format!("{}({}/{})", m.id.as_str(), m.pp, m.maxpp))
                    .collect();
                eprintln!("    Moves: {}", moves.join(", "));
            }
        }
    }

    // Side conditions for P2
    if !battle.sides[1].side_conditions.is_empty() {
        let conditions: Vec<_> = battle.sides[1].side_conditions.keys()
            .map(|c| c.as_str())
            .collect();
        eprintln!("  Side Conditions: {}", conditions.join(", "));
    }

    eprintln!("");
}

fn main() {
    let seed_num: u32 = env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);

    let teams_file = format!("/tmp/teams-seed{}-rust.json", seed_num);

    if !std::path::Path::new(&teams_file).exists() {
        eprintln!("ERROR: Team file not found: {}", teams_file);
        eprintln!("Run: docker exec pokemon-rust-dev bash -c \"cd /home/builder/workspace && cargo run --example generate_test_teams_rust {}\" first", seed_num);
        std::process::exit(1);
    }

    let teams_json = fs::read_to_string(&teams_file).unwrap();

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

    // Split u32 seed into two u16 values for PRNGSeed::Gen5
    // This matches the JavaScript behavior where large seed numbers are stored properly
    let seed_lo = (seed_num & 0xFFFF) as u16;
    let seed_hi = ((seed_num >> 16) & 0xFFFF) as u16;

    let mut battle = Battle::new(BattleOptions {
        format_id: ID::new("gen9randombattle"),
        seed: Some(PRNGSeed::Gen5([0, 0, seed_hi, seed_lo])),
        p1: Some(PlayerOptions {
            name: "Player 1".to_string(),
            team: TeamFormat::Sets(team1),
            avatar: None,
            rating: None,
            seed: None,
        }),
        p2: Some(PlayerOptions {
            name: "Player 2".to_string(),
            team: TeamFormat::Sets(team2),
            avatar: None,
            rating: None,
            seed: None,
        }),
        ..Default::default()
    });

    eprintln!("# Rust Battle Test - Seed {}", seed_num);
    eprintln!("# P1: {} vs P2: {}", teams.p1[0].name, teams.p2[0].name);

    // Run battle for up to 100 turns
    for i in 1..=100 {
        let prng_before = battle.prng.call_count;

        // Print detailed state before turn
        print_battle_state(&battle, i, prng_before);

        eprintln!(">>> Making choices for turn {}...", battle.turn);
        battle.make_choices(&["default", "default"]);

        // Reset log position to prevent "LINE LIMIT EXCEEDED" check from failing
        // In JavaScript, this is updated as logs are sent to clients, but in test mode we don't send logs
        battle.sent_log_pos = battle.log.len();

        let prng_after = battle.prng.call_count;

        // Get active Pokemon HP
        let mut p1_active = Vec::new();
        let mut p2_active = Vec::new();

        for active_idx in &battle.sides[0].active {
            if let Some(poke_idx) = active_idx {
                if let Some(pokemon) = battle.sides[0].pokemon.get(*poke_idx) {
                    let display_name = &pokemon.name;
                    p1_active.push(format!("{}({}/{})", display_name, pokemon.hp, pokemon.maxhp));
                }
            } else {
                p1_active.push("none".to_string());
            }
        }

        for active_idx in &battle.sides[1].active {
            if let Some(poke_idx) = active_idx {
                if let Some(pokemon) = battle.sides[1].pokemon.get(*poke_idx) {
                    let display_name = &pokemon.name;
                    eprintln!("[DEBUG_OUTPUT] P2 active: poke_idx={}, name={}, hp={}/{}",
                        poke_idx, display_name, pokemon.hp, pokemon.maxhp);
                    p2_active.push(format!("{}({}/{})", display_name, pokemon.hp, pokemon.maxhp));
                }
            } else {
                p2_active.push("none".to_string());
            }
        }

        println!("#{}: turn={}, prng={}->{}, P1=[{}], P2=[{}]",
            i, battle.turn, prng_before, prng_after,
            p1_active.join(", "), p2_active.join(", "));
        eprintln!(">>> Turn {} completed. PRNG: {}->{} (+{} calls)",
            battle.turn, prng_before, prng_after, prng_after - prng_before);

        if battle.ended || i >= 100 {
            eprintln!("");
            eprintln!("========================================");
            eprintln!("Battle ended: {}", battle.ended);
            eprintln!("Final turn: {}", battle.turn);
            eprintln!("Total PRNG calls: {}", battle.prng.call_count);
            eprintln!("========================================");
            eprintln!("# Battle ended: {}, Turn: {}, Total PRNG: {}",
                battle.ended, battle.turn, battle.prng.call_count);
            break;
        }
    }
}
