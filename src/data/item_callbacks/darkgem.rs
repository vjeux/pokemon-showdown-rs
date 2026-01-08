//! Dark Gem Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;
use crate::pokemon::Pokemon;

/// onSourceTryPrimaryHit(target, source, move) {
///     if (target === source || move.category === 'Status') return;
///     if (move.type === 'Dark' && source.useItem()) {
///         source.addVolatile('gem');
///     }
/// }
pub fn on_source_try_primary_hit(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, _active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // if (target === source || move.category === 'Status') return;
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    if target == source {
        return EventResult::Continue;
    }

    let is_status = match &battle.active_move {
        Some(active_move) => active_move.category == "Status",
        None => return EventResult::Continue,
    };

    if is_status {
        return EventResult::Continue;
    }

    // if (move.type === 'Dark' && source.useItem())
    let is_dark = match &battle.active_move {
        Some(active_move) => active_move.move_type == "Dark",
        None => return EventResult::Continue,
    };

    if is_dark {
        let used_item = {
            let _source_pokemon = match battle.pokemon_at_mut(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            Pokemon::use_item(battle, source, None, None).is_some()
        };

        if used_item {
            Pokemon::add_volatile(battle, source, ID::new("gem"), None, None, None, None);
        }
    }

    EventResult::Continue
}
