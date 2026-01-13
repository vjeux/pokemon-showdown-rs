//! Block Move
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
    target_pos: (usize, usize),  // The target of the move (receives trapped)
    source_pos: Option<(usize, usize)>,  // The source/attacker (receives trapper)
) -> EventResult {
    // return target.addVolatile('trapped', source, move, 'trapper');
    // JavaScript: target.addVolatile('trapped', source, move, 'trapper')
    // target = target_pos (the Pokemon that was hit by Block)
    // source = source_pos (the Pokemon that used Block)
    // move = "block"
    // linkedStatus = 'trapper'
    let result = Pokemon::add_volatile(
            battle,
            target_pos,  // Add "trapped" to the TARGET (the one hit by Block)
            ID::from("trapped"),
            source_pos,  // Source is the attacker (who used Block)
            Some(&Effect::move_(ID::new("block"))),
            Some(ID::from("trapper")),
            None,
        );

    EventResult::Boolean(result)
}
