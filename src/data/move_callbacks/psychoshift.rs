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
    _target_pos: (usize, usize),
    source_pos: (usize, usize),
) -> EventResult {
    use crate::dex_data::ID;

    // JavaScript: onTryHit(target, source, move) - target comes first, source second
    let source = source_pos;

    // if (!source.status) return false;
    let source_status = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.status.clone()
    };

    debug_elog!("[PSYCHOSHIFT_TRYHIT] source={:?}, source_status={:?}", source, source_status);

    if source_status == ID::from("") {
        debug_elog!("[PSYCHOSHIFT_TRYHIT] source has no status, returning false");
        return EventResult::Boolean(false);
    }

    // move.status = source.status;
    if let Some(ref mut active_move) = battle.active_move {
        active_move.borrow_mut().status = Some(source_status.to_string());
        debug_elog!("[PSYCHOSHIFT_TRYHIT] Set move.status to {:?}", active_move.borrow().status);
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
        battle: &mut Battle,
        _target_pos: (usize, usize),
        source_pos: Option<(usize, usize)>,
        _source_effect: Option<&crate::battle::Effect>,
    ) -> EventResult {
        use crate::pokemon::Pokemon;

        debug_elog!("[PSYCHOSHIFT_SELF_ONHIT] Called with source_pos={:?}", source_pos);

        // pokemon.cureStatus();
        let source = match source_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        debug_elog!("[PSYCHOSHIFT_SELF_ONHIT] About to call cure_status for {:?}", source);
        Pokemon::cure_status(battle, source, false);

        EventResult::Continue
    }
}
