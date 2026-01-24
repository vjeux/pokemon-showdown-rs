//! Jaw Lock Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;
use crate::pokemon::Pokemon;

/// onHit(target, source, move) {
///     source.addVolatile('trapped', target, move, 'trapper');
///     target.addVolatile('trapped', source, move, 'trapper');
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

    // source.addVolatile('trapped', target, move, 'trapper');
    // JavaScript: source.addVolatile('trapped', target, move, 'trapper')
    // ✅ NOW PASSING: source_pos = Some(target), source_effect = Some("jawlock"), linked_status = Some("trapper")
    let move_effect = battle.make_move_effect(&ID::from("jawlock"));
    Pokemon::add_volatile(
            battle,
            source,
            ID::from("trapped"),
            Some(target),
            Some(&move_effect),
            Some(ID::from("trapper")),
            None,
        );

    // target.addVolatile('trapped', source, move, 'trapper');
    // JavaScript: target.addVolatile('trapped', source, move, 'trapper')
    // ✅ NOW PASSING: source_pos = Some(source), source_effect = Some("jawlock"), linked_status = Some("trapper")
    Pokemon::add_volatile(
            battle,
            target,
            ID::from("trapped"),
            Some(source),
            Some(&move_effect),
            Some(ID::from("trapper")),
            None,
        );

    EventResult::Continue
}
