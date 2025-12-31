/// Trace PRNG calls for first complete Pokemon in Rust

use pokemon_showdown::{Dex, PRNG, PRNGSeed};
use pokemon_showdown::dex_data::{StatsTable, Gender};

fn main() {
    let dex = Dex::load_default().expect("Failed to load dex");
    let mut prng = PRNG::new(Some(PRNGSeed::Gen5([0, 0, 0, 1])));

    println!("=== COMPLETE POKEMON #1 GENERATION ===\n");

    let mut all_species: Vec<_> = dex.species.values().collect();
    all_species.sort_by_key(|s| &s.name);

    let mut all_moves: Vec<_> = dex.moves.keys().map(|id| id.as_str()).collect();
    all_moves.sort();

    let mut all_items: Vec<_> = dex.items.keys().map(|id| id.as_str()).collect();
    all_items.sort();

    let mut all_natures: Vec<_> = dex.natures.keys().map(|id| id.as_str()).collect();
    all_natures.sort();

    let mut call_count = 0;

    // Wrap PRNG to count calls
    macro_rules! count_call {
        ($method:expr, $call:expr) => {{
            call_count += 1;
            let result = $call;
            println!("[{}] {}", call_count, $method);
            result
        }};
    }

    // 1. Species
    let idx = count_call!("random_range(0, 1515)", prng.random_range(0, all_species.len() as i32));
    let species = all_species[idx as usize];
    println!("=> Species: {}\n", species.name);

    // 2. Level
    let level = count_call!("random_range(50, 101)", prng.random_range(50, 101));
    println!("=> Level: {}\n", level);

    // 3. Item
    let idx = count_call!("random_range(0, 583)", prng.random_range(0, all_items.len() as i32));
    let item = all_items[idx as usize];
    println!("=> Item: {}\n", item);

    // 4. Nature
    let idx = count_call!("random_range(0, 25)", prng.random_range(0, all_natures.len() as i32));
    let nature = all_natures[idx as usize];
    println!("=> Nature: {}\n", nature);

    // 5. Gender
    let gender = if let Some(ref ratio) = species.gender_ratio {
        if ratio.m > 0.0 && ratio.f > 0.0 {
            if count_call!("random_chance(1, 2)", prng.random_chance(1, 2)) {
                Gender::Male
            } else {
                Gender::Female
            }
        } else if ratio.m > 0.0 {
            Gender::Male
        } else if ratio.f > 0.0 {
            Gender::Female
        } else {
            Gender::None
        }
    } else if let Some(ref gender_str) = species.gender {
        // No gender_ratio, but gender field is set (M, F, or N)
        match gender_str.as_str() {
            "M" => Gender::Male,
            "F" => Gender::Female,
            "N" => Gender::None,
            _ => {
                // Default to 50/50 like JavaScript
                if count_call!("random_chance(1, 2)", prng.random_chance(1, 2)) {
                    Gender::Male
                } else {
                    Gender::Female
                }
            }
        }
    } else {
        // No gender_ratio and no gender field - default to 50/50 like JavaScript
        if count_call!("random_chance(1, 2)", prng.random_chance(1, 2)) {
            Gender::Male
        } else {
            Gender::Female
        }
    };
    println!("=> Gender: {:?}\n", gender);

    // 6. Moves
    println!("=> Moves:");
    let mut moves = Vec::new();
    for _ in 0..4 {
        let idx = count_call!("random_range(0, 954)", prng.random_range(0, all_moves.len() as i32));
        let move_name = all_moves[idx as usize].to_string();
        if !moves.contains(&move_name) {
            moves.push(move_name);
        }
    }
    while moves.len() < 4 {
        moves.push("tackle".to_string());
    }
    println!("   {:?}\n", moves);

    // 7. EVs
    println!("=> EVs:");
    let mut evs = [0i32; 6];
    let mut total_evs = 0;
    while total_evs < 510 {
        let mut available_stats = Vec::new();
        for (i, &ev) in evs.iter().enumerate() {
            if ev < 252 {
                available_stats.push(i);
            }
        }
        if available_stats.is_empty() {
            break;
        }

        let idx = count_call!("random_range(0, available)", prng.random_range(0, available_stats.len() as i32));
        let stat_idx = available_stats[idx as usize];

        let amount = count_call!("random_range(1, 5)", prng.random_range(1, 5))
            .min(252 - evs[stat_idx])
            .min(510 - total_evs);
        evs[stat_idx] += amount;
        total_evs += amount;
    }
    let evs_table = StatsTable::new(evs[0], evs[1], evs[2], evs[3], evs[4], evs[5]);
    println!("   hp:{} atk:{} def:{} spa:{} spd:{} spe:{}\n", evs[0], evs[1], evs[2], evs[3], evs[4], evs[5]);

    // 8. IVs
    println!("=> IVs:");
    let iv_hp = count_call!("random_range(0, 32)", prng.random_range(0, 32));
    let iv_atk = count_call!("random_range(0, 32)", prng.random_range(0, 32));
    let iv_def = count_call!("random_range(0, 32)", prng.random_range(0, 32));
    let iv_spa = count_call!("random_range(0, 32)", prng.random_range(0, 32));
    let iv_spd = count_call!("random_range(0, 32)", prng.random_range(0, 32));
    let iv_spe = count_call!("random_range(0, 32)", prng.random_range(0, 32));
    println!("   hp:{} atk:{} def:{} spa:{} spd:{} spe:{}\n", iv_hp, iv_atk, iv_def, iv_spa, iv_spd, iv_spe);

    println!("\n=== TOTAL PRNG CALLS: {} ===\n", call_count);

    // Next call for Pokemon #2
    println!("=== NEXT CALL (for Pokemon #2 species selection) ===");
    let used_species = vec![species.name.as_str()];
    let available_species2: Vec<_> = all_species.iter()
        .filter(|s| !used_species.contains(&s.name.as_str()))
        .copied()
        .collect();

    call_count += 1;
    let idx2 = prng.random_range(0, available_species2.len() as i32);
    println!("[{}] random_range(0, {})", call_count, available_species2.len());
    println!("Pokemon #2 should be: {}", available_species2[idx2 as usize].name);
}
