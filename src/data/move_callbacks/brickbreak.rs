//! Brick Break Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;

/// onTryHit(pokemon) {
///     // will shatter screens through sub, before you hit
///     pokemon.side.removeSideCondition('reflect');
///     pokemon.side.removeSideCondition('lightscreen');
///     pokemon.side.removeSideCondition('auroraveil');
/// }
/// NOTE: dispatch_on_try_hit passes (target_pos, source_pos) per JS convention
pub fn on_try_hit(
    battle: &mut Battle,
    target_pos: (usize, usize),
    _source_pos: (usize, usize),
) -> EventResult {
    // will shatter screens through sub, before you hit
    // pokemon.side.removeSideCondition('reflect');
    // pokemon.side.removeSideCondition('lightscreen');
    // pokemon.side.removeSideCondition('auroraveil');

    let side_idx = target_pos.0;

    if side_idx < battle.sides.len() {
        battle.sides[side_idx].remove_side_condition(&ID::from("reflect"));
        battle.sides[side_idx].remove_side_condition(&ID::from("lightscreen"));
        battle.sides[side_idx].remove_side_condition(&ID::from("auroraveil"));
    }

    EventResult::Continue
}
