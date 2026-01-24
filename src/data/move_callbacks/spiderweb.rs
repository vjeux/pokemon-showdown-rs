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
    target_pos: (usize, usize),  // First param is target
    source_pos: Option<(usize, usize)>,  // Second param is source
) -> EventResult {
    use crate::dex_data::ID;
    use crate::pokemon::Pokemon;

    // onHit(target, source, move) {
    //     return target.addVolatile('trapped', source, move, 'trapper');
    // }
    let target = target_pos;
    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // return target.addVolatile('trapped', source, move, 'trapper');
    // JavaScript: target.addVolatile('trapped', source, move, 'trapper')
    // ✅ NOW PASSING: source_pos = Some(source), source_effect = Some("spiderweb"), linked_status = Some("trapper")
    let move_effect = battle.make_move_effect(&ID::from("spiderweb"));
    let result = Pokemon::add_volatile(
            battle,
            target,
            ID::from("trapped"),
            Some(source),
            Some(&move_effect),
            Some(ID::from("trapper")),
            None,
        );

    // add_volatile returns Option<bool>:
    // - Some(true) -> EventResult::Boolean(true)
    // - Some(false) -> EventResult::Boolean(false)
    // - None (restart) -> EventResult::Continue (undefined in JS)
    match result {
        Some(b) => EventResult::Boolean(b),
        None => EventResult::Continue,
    }
}
