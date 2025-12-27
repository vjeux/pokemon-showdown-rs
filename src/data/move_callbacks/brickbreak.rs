//! Brick Break Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::dex_data::ID;

/// onTryHit(pokemon) {
///     // will shatter screens through sub, before you hit
///     pokemon.side.removeSideCondition('reflect');
///     pokemon.side.removeSideCondition('lightscreen');
///     pokemon.side.removeSideCondition('auroraveil');
/// }
pub fn on_try_hit(battle: &mut Battle, source_pos: (usize, usize), target_pos: (usize, usize)) -> EventResult {
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

