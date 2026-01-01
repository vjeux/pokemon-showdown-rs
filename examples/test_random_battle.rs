/// Test a random gen9 battle with a new seed
use pokemon_showdown::{Battle, BattleOptions, ID};

fn main() {
    // Use seed [1, 2, 3, 4] for a different random battle
    let mut battle = Battle::new(BattleOptions {
        format_id: ID::new("gen9randombattle"),
        seed: Some(pokemon_showdown::PRNGSeed::Gen5([1, 2, 3, 4])),
        ..Default::default()
    });

    eprintln!("Starting new random gen9 battle with seed [1, 2, 3, 4]");
    eprintln!("Battle format: {}", battle.format_id);

    // Run 50 turns to test for divergences
    for make_choices_num in 1..=50 {
        let prng_before = battle.prng.call_count;
        let turn_before = battle.turn;

        battle.make_choices(&["default", "default"]);

        let prng_after = battle.prng.call_count;
        let turn_after = battle.turn;

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

        eprintln!("#{}: turn={}→{}, prng={}→{} ({}), P1=[{}], P2=[{}]",
            make_choices_num, turn_before, turn_after,
            prng_before, prng_after, prng_after - prng_before,
            p1_active.join(", "), p2_active.join(", "));

        if battle.ended {
            eprintln!("Battle ended after {} makeChoices calls", make_choices_num);
            break;
        }

        if make_choices_num >= 50 {
            eprintln!("Reached 50 makeChoices calls, stopping test");
            break;
        }
    }

    eprintln!("\nBattle summary:");
    eprintln!("  Turn: {}", battle.turn);
    eprintln!("  PRNG calls: {}", battle.prng.call_count);
    eprintln!("  Ended: {}", battle.ended);
}
