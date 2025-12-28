//! Bug Gem Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onSourceTryPrimaryHit(target, source, move) {
///     if (target === source || move.category === 'Status') return;
///     if (move.type === 'Bug' && source.useItem()) {
///         source.addVolatile('gem');
///     }
/// }
pub fn on_source_try_primary_hit(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    use crate::dex_data::ID;

    // if (target === source || move.category === 'Status') return;
    if target_pos == source_pos {
        return EventResult::Continue;
    }

    let move_category = {
        if let Some(ref active_move) = battle.active_move {
            active_move.category.as_str()
        } else {
            ""
        }
    };

    if move_category == "Status" {
        return EventResult::Continue;
    }

    // if (move.type === 'Bug' && source.useItem())
    let is_bug = {
        if let Some(ref active_move) = battle.active_move {
            active_move.move_type == "Bug"
        } else {
            false
        }
    };

    if is_bug {
        let used_item = {
            if let Some(src_pos) = source_pos {
                if let Some(source) = battle.pokemon_at_mut(src_pos.0, src_pos.1) {
                    source.use_item().is_some()
                } else {
                    false
                }
            } else {
                false
            }
        };

        if used_item {
            // source.addVolatile('gem');
            if let Some(src_pos) = source_pos {
                if let Some(source) = battle.pokemon_at_mut(src_pos.0, src_pos.1) {
                    source.add_volatile(ID::from("gem"));
                }
            }
        }
    }

    EventResult::Continue
}
