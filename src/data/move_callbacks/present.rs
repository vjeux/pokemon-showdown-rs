//! Present Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyMove(move, pokemon, target) {
///     const rand = this.random(10);
///     if (rand < 2) {
///         move.heal = [1, 4];
///         move.infiltrates = true;
///     } else if (rand < 6) {
///         move.basePower = 40;
///     } else if (rand < 9) {
///         move.basePower = 80;
///     } else {
///         move.basePower = 120;
///     }
/// }
pub fn on_modify_move(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    debug_elog!("[PRESENT] onModifyMove called at turn={}", battle.turn);
    // const rand = this.random(10);
    let rand = battle.random(10);
    debug_elog!("[PRESENT] onModifyMove generated rand={}", rand);

    if rand < 2 {
        // move.heal = [1, 4];
        // move.infiltrates = true;
        if let Some(ref mut active_move) = battle.active_move {
            active_move.heal = Some((1, 4));
            active_move.infiltrates = true;
        }
    } else if rand < 6 {
        // move.basePower = 40;
        if let Some(ref mut active_move) = battle.active_move {
            active_move.base_power = 40;
        }
    } else if rand < 9 {
        // move.basePower = 80;
        if let Some(ref mut active_move) = battle.active_move {
            active_move.base_power = 80;
        }
    } else {
        // move.basePower = 120;
        if let Some(ref mut active_move) = battle.active_move {
            active_move.base_power = 120;
        }
    }

    EventResult::Continue
}
