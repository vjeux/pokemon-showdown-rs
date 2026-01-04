//! Ice Gem Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::pokemon::Pokemon;

/// onSourceTryPrimaryHit(target, source, move) {
///     if (target === source || move.category === 'Status') return;
///     if (move.type === 'Ice' && source.useItem()) {
///         source.addVolatile('gem');
///     }
/// }
pub fn on_source_try_primary_hit(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, _move_id: &str) -> EventResult {
    // if (target === source || move.category === 'Status') return;
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };
    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // target === source
    if target == source {
        return EventResult::Continue;
    }

    // move.category === 'Status'
    let is_status = battle.active_move.as_ref()
        .map(|m| m.category.as_str() == "Status")
        .unwrap_or(false);

    if is_status {
        return EventResult::Continue;
    }

    // if (move.type === 'Ice' && source.useItem())
    let is_ice = battle.active_move.as_ref()
        .map(|m| m.move_type.as_str() == "Ice")
        .unwrap_or(false);

    if is_ice {
        // source.useItem()
        let used_item = Pokemon::use_item(battle, source, None, None).is_some();

        if used_item {
            // source.addVolatile('gem');
            Pokemon::add_volatile(battle, source, "gem".into(), None, None, None, None);
        }
    }

    EventResult::Continue
}
