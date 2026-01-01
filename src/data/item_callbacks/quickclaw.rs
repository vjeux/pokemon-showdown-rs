//! Quick Claw Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onFractionalPriority(priority, pokemon, target, move) {
///     if (move.category === "Status" && pokemon.hasAbility("myceliummight")) return;
///     if (priority <= 0 && this.randomChance(1, 5)) {
///         this.add('-activate', pokemon, 'item: Quick Claw');
///         return 0.1;
///     }
/// }
pub fn on_fractional_priority(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    priority: f64,
) -> EventResult {
    // if (priority <= 0 && this.randomChance(1, 5))
    if priority <= 0.0 {
        let activate = {
            // Use random chance: 1 in 5 (20%)
            battle.random_chance(1, 5)
        };

        if activate {
            // this.add('-activate', pokemon, 'item: Quick Claw');
            let pokemon_slot = {
                let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                pokemon.get_slot()
            };

            battle.add(
                "-activate",
                &[
                    crate::battle::Arg::from(pokemon_slot),
                    crate::battle::Arg::from("item: Quick Claw"),
                ],
            );

            // return 0.1;
            return EventResult::Float(0.1);
        }
    }

    EventResult::Continue
}
