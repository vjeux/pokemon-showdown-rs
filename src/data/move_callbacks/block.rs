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
    target_pos: (usize, usize),  // The target of the move (receives trapped)
    source_pos: Option<(usize, usize)>,  // The source/attacker (receives trapper)
) -> EventResult {
    // return target.addVolatile('trapped', source, move, 'trapper');
    // JavaScript: target.addVolatile('trapped', source, move, 'trapper')
    // target = target_pos (the Pokemon that was hit by Block)
    // source = source_pos (the Pokemon that used Block)
    // move = "block"
    // linkedStatus = 'trapper'
    let move_effect = battle.make_move_effect(&ID::new("block"));
    let result = Pokemon::add_volatile(
            battle,
            target_pos,  // Add "trapped" to the TARGET (the one hit by Block)
            ID::from("trapped"),
            source_pos,  // Source is the attacker (who used Block)
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
