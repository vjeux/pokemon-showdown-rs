//! Synchronoise Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryImmunity(target, source) {
///     return target.hasType(source.getTypes());
/// }
pub fn on_try_immunity(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>) -> EventResult {
    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let source_pos = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // Get source types
    let source_types = if let Some(source) = battle.pokemon_at(source_pos.0, source_pos.1) {
        source.get_types()
    } else {
        return EventResult::Continue;
    };

    // Check if target has any of the source's types
    let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    EventResult::Bool(target.has_type(&source_types))
}

