//! Jaw Lock Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target, source, move) {
///     source.addVolatile('trapped', target, move, 'trapper');
///     target.addVolatile('trapped', source, move, 'trapper');
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    let source = pokemon_pos;
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // source.addVolatile('trapped', target, move, 'trapper');
    battle.add_volatile_with_source_effect(source, &ID::from("trapped"), Some(target), Some("trapper"));

    // target.addVolatile('trapped', source, move, 'trapper');
    battle.add_volatile_with_source_effect(target, &ID::from("trapped"), Some(source), Some("trapper"));

    EventResult::Continue
}

