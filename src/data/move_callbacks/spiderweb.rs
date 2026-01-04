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
    // JavaScript: target.addVolatile('trapped', source, move, 'trapper')
    // ✅ NOW PASSING: source_pos = Some(source), source_effect = Some("spiderweb"), linked_status = Some("trapper")
    let result = Pokemon::add_volatile(
            battle,
            target,
            ID::from("trapped"),
            Some(source),
            Some(&ID::new("spiderweb")),
            Some(ID::from("trapper")),
            None,
        );

    EventResult::Boolean(result)
}
