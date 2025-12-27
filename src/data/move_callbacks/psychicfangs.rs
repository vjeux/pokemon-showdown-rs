//! Psychic Fangs Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(pokemon) {
///     // will shatter screens through sub, before you hit
///     pokemon.side.removeSideCondition('reflect');
///     pokemon.side.removeSideCondition('lightscreen');
///     pokemon.side.removeSideCondition('auroraveil');
/// }
pub fn on_try_hit(battle: &mut Battle, source_pos: (usize, usize), target_pos: (usize, usize)) -> EventResult {
    use crate::dex_data::ID;

    let pokemon = target_pos;

    // will shatter screens through sub, before you hit
    // pokemon.side.removeSideCondition('reflect');
    battle.remove_side_condition(&ID::from("reflect"), pokemon.0);

    // pokemon.side.removeSideCondition('lightscreen');
    battle.remove_side_condition(&ID::from("lightscreen"), pokemon.0);

    // pokemon.side.removeSideCondition('auroraveil');
    battle.remove_side_condition(&ID::from("auroraveil"), pokemon.0);

    EventResult::Continue
}

