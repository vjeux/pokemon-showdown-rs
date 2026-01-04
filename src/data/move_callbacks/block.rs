//! Block Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;
use crate::pokemon::Pokemon;

/// onHit(target, source, move) {
///     return target.addVolatile('trapped', source, move, 'trapper');
/// }
pub fn on_hit(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    // Get the target
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // return target.addVolatile('trapped', source, move, 'trapper');
    // JavaScript: target.addVolatile('trapped', source, move, 'trapper')
    // source = pokemon_pos
    // move = "block"
    // linkedStatus = 'trapper'
    // ✅ NOW PASSING: source_pos = Some(pokemon_pos), source_effect = Some("block"), linked_status = Some("trapper")
    let result = Pokemon::add_volatile(
            battle,
            target,
            ID::from("trapped"),
            Some(pokemon_pos),
            Some(&ID::new("block")),
            Some(ID::from("trapper")),
            None,
        );

    EventResult::Boolean(result)
}
