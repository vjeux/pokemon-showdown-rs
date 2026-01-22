//! Quick Claw Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::battle::Effect;
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
    // if (move.category === "Status" && pokemon.hasAbility("myceliummight")) return;
    let has_mycelium_might = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.has_ability(battle, &["myceliummight"])
    };

    // During FractionalPriority (action resolution), active_move is not yet set.
    // The move is passed via event.effect. We need to check both like Mycelium Might does.
    let is_status = if let Some(ref mv) = battle.active_move {
        mv.category == "Status"
    } else if let Some(ref event) = battle.event {
        // Try to get move category from event's source_effect
        if let Some(ref effect) = event.effect {
            battle.dex.moves().get(effect.id.as_str())
                .map(|move_data| move_data.category == "Status")
                .unwrap_or(false)
        } else {
            false
        }
    } else {
        false
    };

    if is_status && has_mycelium_might {
        return EventResult::Continue;
    }

    // if (priority <= 0 && this.randomChance(1, 5))
    if priority <= 0.0 {
        let activate = {
            // Use random chance: 1 in 5 (20%)
            battle.random_chance(1.0, 5)
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
