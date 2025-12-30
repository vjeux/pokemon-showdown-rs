//! Parting Shot Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target, source, move) {
///     const success = this.boost({ atk: -1, spa: -1 }, target, source);
///     if (!success && !target.hasAbility('mirrorarmor')) {
///         delete move.selfSwitch;
///     }
/// }
pub fn on_hit(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    let source = pokemon_pos;
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // const success = this.boost({ atk: -1, spa: -1 }, target, source);
    let success = battle.boost(&[("atk", -1), ("spa", -1)], target, Some(source), None, false, false);

    // if (!success && !target.hasAbility('mirrorarmor')) {
    if !success {
        let has_mirrorarmor = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.has_ability(&["mirrorarmor"])
        };

        if !has_mirrorarmor {
            // delete move.selfSwitch;
            if let Some(ref mut active_move) = battle.active_move {
                active_move.self_switch = None;
            }
        }
    }

    EventResult::Continue
}
