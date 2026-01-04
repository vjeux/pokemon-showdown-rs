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
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    let source = pokemon_pos;
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // source.addVolatile('trapped', target, move, 'trapper');
    // JavaScript: source.addVolatile('trapped', target, move, 'trapper')
    // ✅ NOW PASSING: source_pos = Some(target), source_effect = Some("jawlock"), linked_status = Some("trapper")
    Pokemon::add_volatile(
            battle,
            source,
            ID::from("trapped"),
            Some(target),
            Some(&ID::new("jawlock")),
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
            Some(&ID::new("jawlock")),
            Some(ID::from("trapper")),
            None,
        );

    EventResult::Continue
}
