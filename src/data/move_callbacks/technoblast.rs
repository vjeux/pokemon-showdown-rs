//! Techno Blast Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Effect};
use crate::event::EventResult;

/// onModifyType(move, pokemon) {
///     if (pokemon.ignoringItem()) return;
///     move.type = this.runEvent('Drive', pokemon, null, move, 'Normal');
/// }
pub fn on_modify_type(
    battle: &mut Battle,
    _move_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    let pokemon = pokemon_pos;

    // if (pokemon.ignoringItem()) return;
    let is_ignoring_item = {
        let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_ref.ignoring_item(battle, false)
    };

    if is_ignoring_item {
        return EventResult::Continue;
    }

    // move.type = this.runEvent('Drive', pokemon, null, move, 'Normal');
    // Extract the move ID before calling run_event to avoid borrow issues
    let source_effect = battle.active_move.as_ref().map(|m| Effect::move_(m.id.clone()));
    let move_type = match battle.run_event(
        "Drive",
        Some(crate::event::EventTarget::Pokemon(pokemon)),
        None,
        source_effect.as_ref(),
        EventResult::String("Normal".to_string()),
        false,
        false,
    ) {
        EventResult::String(s) => s,
        _ => "Normal".to_string(),
    };

    if let Some(ref mut active_move) = battle.active_move {
        active_move.move_type = move_type;
    }

    EventResult::Continue
}
