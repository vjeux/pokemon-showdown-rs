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
pub fn on_try_hit(battle: &mut Battle, target_pos: (usize, usize), source_pos: (usize, usize)) -> EventResult {
    use crate::dex_data::ID;

    let pokemon_pos = target_pos;

    // pokemon.side.removeSideCondition('reflect');
    // pokemon.side.removeSideCondition('lightscreen');
    // pokemon.side.removeSideCondition('auroraveil');
    battle.remove_side_condition(pokemon_pos.0, &ID::from("reflect"));
    battle.remove_side_condition(pokemon_pos.0, &ID::from("lightscreen"));
    battle.remove_side_condition(pokemon_pos.0, &ID::from("auroraveil"));

    EventResult::Continue
}

