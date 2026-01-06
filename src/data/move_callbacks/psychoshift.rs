//! Psycho Shift Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(target, source, move) {
///     if (!source.status) return false;
///     move.status = source.status;
/// }
pub fn on_try_hit(
    battle: &mut Battle,
    source_pos: (usize, usize),
    _target_pos: (usize, usize),
) -> EventResult {
    use crate::dex_data::ID;

    let source = source_pos;

    // if (!source.status) return false;
    let source_status = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.status.clone()
    };

    if source_status == ID::from("") {
        return EventResult::Boolean(false);
    }

    // move.status = source.status;
    if let Some(ref mut active_move) = battle.active_move {
        active_move.status = Some(source_status.to_string());
    }

    EventResult::Continue
}

/// Self-targeting callbacks
/// These callbacks target the move user (source), not the move target
pub mod self_callbacks {
    use super::*;

    /// self.onHit(source)
    ///
    /// ```text
    /// JS Source (data/moves.ts):
    /// self: {
    ///     onHit(source) {
    ///         onHit(pokemon) {
    ///                 pokemon.cureStatus();
    ///               }
    ///     },
    /// }
    /// ```
    pub fn on_hit(
        _battle: &mut Battle,
        _target_pos: (usize, usize),
        _source_pos: Option<(usize, usize)>,
    ) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }
}
