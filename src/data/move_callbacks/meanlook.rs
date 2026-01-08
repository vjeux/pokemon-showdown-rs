//! Mean Look Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Effect};
use crate::dex_data::ID;
use crate::event::EventResult;
use crate::pokemon::Pokemon;

/// onHit(target, source, move) {
///     return target.addVolatile('trapped', source, move, 'trapper');
/// }
pub fn on_hit(
    battle: &mut Battle,
    target_pos: (usize, usize),  // First param is target
    source_pos: Option<(usize, usize)>,  // Second param is source
) -> EventResult {
    let target = target_pos;
    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // return target.addVolatile('trapped', source, move, 'trapper');
    // JavaScript: target.addVolatile('trapped', source, move, 'trapper')
    // ✅ NOW PASSING: source_pos = Some(source), source_effect = Some("meanlook"), linked_status = Some("trapper")
    let result = Pokemon::add_volatile(
            battle,
            target,
            ID::from("trapped"),
            Some(source),
            Some(&Effect::move_(ID::new("meanlook"))),
            Some(ID::from("trapper")),
            None,
        );

    EventResult::Boolean(result)
}
