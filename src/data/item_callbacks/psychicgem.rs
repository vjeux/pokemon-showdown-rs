//! Psychic Gem Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::pokemon::Pokemon;

/// onSourceTryPrimaryHit(target, source, move) {
///     if (target === source || move.category === 'Status') return;
///     if (move.type === 'Psychic' && source.useItem()) {
///         source.addVolatile('gem');
///     }
/// }
pub fn on_source_try_primary_hit(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, _move_id: &str) -> EventResult {
    // Get positions
    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };
    let source_pos = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (target === source || move.category === 'Status') return;
    if target_pos == source_pos {
        return EventResult::Continue;
    }

    // Get move category and type
    let (move_category, move_type) = match &battle.active_move {
        Some(active_move) => (active_move.category.clone(), active_move.move_type.clone()),
        None => return EventResult::Continue,
    };

    if move_category == "Status" {
        return EventResult::Continue;
    }

    // if (move.type === 'Psychic' && source.useItem())
    if move_type == "Psychic" {
        let used_item = Pokemon::use_item(battle, source_pos, None, None).is_some();

        if used_item {
            // source.addVolatile('gem');
            Pokemon::add_volatile(battle, source_pos, "gem".into(), None, None, None);
        }
    }

    EventResult::Continue
}
