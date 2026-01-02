//! Spider Web Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target, source, move) {
///     return target.addVolatile('trapped', source, move, 'trapper');
/// }
pub fn on_hit(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    use crate::dex_data::ID;
    use crate::pokemon::Pokemon;

    // onHit(target, source, move) {
    //     return target.addVolatile('trapped', source, move, 'trapper');
    // }
    let source = pokemon_pos;
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // return target.addVolatile('trapped', source, move, 'trapper');
    let result = Pokemon::add_volatile(battle, target, ID::from("trapped"), Some(source), None);

    EventResult::Boolean(result)
}
