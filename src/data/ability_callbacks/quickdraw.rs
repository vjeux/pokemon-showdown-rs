//! Quick Draw Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onFractionalPriority(priority, pokemon, target, move) {
///     if (move.category !== "Status" && this.randomChance(3, 10)) {
///         this.add('-activate', pokemon, 'ability: Quick Draw');
///         return 0.1;
///     }
/// }
pub fn on_fractional_priority(battle: &mut Battle, _priority: i32, pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>, _move_id: &str) -> EventResult {
    use crate::battle::Arg;

    // if (move.category !== "Status" && this.randomChance(3, 10))
    let is_not_status = if let Some(ref active_move) = battle.active_move {
        active_move.category != "Status"
    } else {
        return EventResult::Continue;
    };

    if is_not_status && battle.random_chance(3, 10) {
        // this.add('-activate', pokemon, 'ability: Quick Draw');
        let pokemon_slot = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.get_slot()
        };

        battle.add("-activate", &[
            Arg::String(pokemon_slot),
            Arg::Str("ability: Quick Draw"),
        ]);

        // return 0.1;
        return EventResult::Number(1); // 0.1 * 10 = 1 (fractional priority is multiplied by 10 internally)
    }

    EventResult::Continue
}

