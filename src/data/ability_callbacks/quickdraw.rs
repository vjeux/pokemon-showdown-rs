//! Quick Draw Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Quick Draw: 30% chance to move first in its priority bracket

use crate::battle::Battle;
use crate::event::EventResult;

/// onFractionalPriority(priority, pokemon, target, move) {
///     if (move.category !== "Status" && this.randomChance(3, 10)) {
///         this.add("-activate", pokemon, "ability: Quick Draw");
///         return 0.1;
///     }
/// }
pub fn on_fractional_priority(
    battle: &mut Battle,
    _priority: i32,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
    active_move: Option<&crate::battle_actions::ActiveMove>,
) -> EventResult {
    // JavaScript checks move.category (the active move's category, not the dex category)
    // Get category from active_move, or fallback to event's source_effect for FractionalPriority
    let is_status = if let Some(m) = active_move {
        m.category == "Status"
    } else if let Some(ref event) = battle.event {
        // Fallback: get move category from event's source_effect
        if let Some(ref effect) = event.effect {
            if let Some(move_data) = battle.dex.moves().get(effect.id.as_str()) {
                move_data.category == "Status"
            } else {
                // In JS, nonexistent move has category: undefined
                // undefined !== "Status" is true, so treat as non-Status
                false
            }
        } else {
            return EventResult::Continue;
        }
    } else {
        return EventResult::Continue;
    };

    // if (move.category !== "Status" && this.randomChance(3, 10))
    if !is_status && battle.random_chance(3.0, 10) {
        // this.add("-activate", pokemon, "ability: Quick Draw");
        if let Some(pokemon) = battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            let pokemon_ident = pokemon.get_slot();
            battle.add("-activate", &[pokemon_ident.as_str().into(), "ability: Quick Draw".into()]);
        }
        // return 0.1;
        return EventResult::Float(0.1);
    }

    EventResult::Continue
}
