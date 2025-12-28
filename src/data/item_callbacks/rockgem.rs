//! Rock Gem Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onSourceTryPrimaryHit(target, source, move) {
///     if (target === source || move.category === 'Status') return;
///     if (move.type === 'Rock' && source.useItem()) {
///         source.addVolatile('gem');
///     }
/// }
pub fn on_source_try_primary_hit(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    // if (target === source || move.category === 'Status') return;
    // if (move.type === 'Rock' && source.useItem()) {
    //     source.addVolatile('gem');
    // }
    // TODO: Need move.category, move.type, source.useItem(), and source.addVolatile('gem')
    // Gem boosts Rock-type move power by 1.3x (or 1.5x in gen 5) then consumed
    EventResult::Continue
}
