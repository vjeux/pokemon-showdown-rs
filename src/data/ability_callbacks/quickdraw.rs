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
    // Get move_id from active_move, or fallback to event's source_effect
    // FractionalPriority is called during action resolution before active_move is set,
    // so the move is passed via source_effect (4th arg to run_event in JavaScript)
    let move_id_owned: String;
    let move_id = if let Some(m) = active_move {
        m.id.as_str()
    } else {
        // Fallback: get move ID from event's source_effect
        move_id_owned = battle.event.as_ref()
            .and_then(|e| e.effect.as_ref())
            .map(|eff| eff.id.to_string())
            .unwrap_or_default();
        &move_id_owned
    };

    // Get the move data from move_id
    let is_status = if let Some(move_data) = battle.dex.moves().get(move_id) {
        move_data.category == "Status"
    } else {
        return EventResult::Continue;
    };

    // if (move.category !== "Status" && this.randomChance(3, 10))
    if !is_status && battle.random_chance(3, 10) {
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
