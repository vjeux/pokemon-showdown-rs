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
///
/// NOTE: dispatch_on_hit calls on_hit(battle, target_pos, source_pos)
/// So the first parameter is the target, second is the source!
pub fn on_hit(
    battle: &mut Battle,
    target_pos: (usize, usize),           // First param from dispatch is target
    source_pos: Option<(usize, usize)>,   // Second param from dispatch is source
) -> EventResult {
    let target = target_pos;
    let source = match source_pos {
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
            target_pokemon.has_ability(battle, &["mirrorarmor"])
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
