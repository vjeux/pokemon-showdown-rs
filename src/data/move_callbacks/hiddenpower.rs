//! Hidden Power Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyType(move, pokemon) {
///     move.type = pokemon.hpType || 'Dark';
/// }
pub fn on_modify_type(battle: &mut Battle, move_id: &str, pokemon_pos: (usize, usize)) -> EventResult {
    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    // Set move type to Hidden Power type, or Dark if not set
    let hp_type = pokemon.hp_type.as_ref()
        .map(|s| s.as_str())
        .unwrap_or("Dark");

    // TODO: Need battle.dex.getActiveMove(move_id) to get mutable move reference
    // For now, this is a placeholder - we need ActiveMove mutation system
    battle.debug(&format!("Hidden Power type: {}", hp_type));

    EventResult::Continue
}

