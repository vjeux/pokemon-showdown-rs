//! Mean Look Move
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
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let source = pokemon_pos;

    // return target.addVolatile('trapped', source, move, 'trapper');
    // JavaScript: target.addVolatile('trapped', source, move, 'trapper')
    // ✅ NOW PASSING: source_pos = Some(source), source_effect = Some("meanlook"), linked_status = Some("trapper")
    let result = Pokemon::add_volatile(
            battle,
            target,
            ID::from("trapped"),
            Some(source),
            Some(&ID::new("meanlook")),
            Some(ID::from("trapper")),
            None,
        );

    EventResult::Boolean(result)
}
