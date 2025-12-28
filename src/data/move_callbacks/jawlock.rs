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
pub fn on_hit(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    use crate::dex_data::ID;

    let source = pokemon_pos;
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // source.addVolatile('trapped', target, move, 'trapper');
    {
        let pokemon = match battle.pokemon_at_mut(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.add_volatile(ID::from("trapped"));
        // TODO: Set source/trapper information on the volatile
    }

    // target.addVolatile('trapped', source, move, 'trapper');
    {
        let pokemon = match battle.pokemon_at_mut(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.add_volatile(ID::from("trapped"));
        // TODO: Set source/trapper information on the volatile
    }

    EventResult::Continue
}
