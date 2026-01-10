//! Water Gem Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::pokemon::Pokemon;

/// onSourceTryPrimaryHit(target, source, move) {
///     if (target === source || move.category === 'Status' || move.flags['pledgecombo']) return;
///     if (move.type === 'Water' && source.useItem()) {
///         source.addVolatile('gem');
///     }
/// }
pub fn on_source_try_primary_hit(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // if (target === source || move.category === 'Status' || move.flags['pledgecombo']) return;
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // Get active_move from parameter
    let active_move_ref = match active_move {
        Some(m) => m,
        None => return EventResult::Continue,
    };

    // target === source
    if target == source {
        return EventResult::Continue;
    }

    // move.category === 'Status' || move.flags['pledgecombo']
    if active_move_ref.category == "Status" || active_move_ref.flags.pledgecombo {
        return EventResult::Continue;
    }

    // if (move.type === 'Water' && source.useItem())
    if active_move_ref.move_type == "Water" {
        let used_item = Pokemon::use_item(battle, source, None, None).is_some();

        if used_item {
            // source.addVolatile('gem');
            Pokemon::add_volatile(battle, source, "gem".into(), None, None, None, None);
        }
    }

    EventResult::Continue
}
