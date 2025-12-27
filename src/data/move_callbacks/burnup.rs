//! Burn Up Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryMove(pokemon, target, move) {
///     if (pokemon.hasType('Fire')) return;
///     this.add('-fail', pokemon, 'move: Burn Up');
///     this.attrLastMove('[still]');
///     return null;
/// }
pub fn on_try_move(battle: &mut Battle, source_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let pokemon = match battle.pokemon_at(source_pos.0, source_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    if pokemon.has_type("Fire") {
        return EventResult::Continue;
    }

    // TODO: battle.add('-fail', pokemon, 'move: Burn Up');
    // TODO: battle.attrLastMove('[still]');
    EventResult::Null
}

