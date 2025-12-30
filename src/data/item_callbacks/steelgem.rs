//! Steel Gem Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onSourceTryPrimaryHit(target, source, move) {
///     if (target === source || move.category === 'Status') return;
///     if (move.type === 'Steel' && source.useItem()) {
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
    let is_status = match &battle.active_move {
        Some(active_move) => active_move.category == "Status",
        None => return EventResult::Continue,
    };

    if is_status {
        return EventResult::Continue;
    }

    // if (move.type === 'Steel' && source.useItem())
    let is_steel = match &battle.active_move {
        Some(active_move) => active_move.move_type == "Steel",
        None => return EventResult::Continue,
    };

    if is_steel {
        let used_item = {
            let source_pokemon = match battle.pokemon_at_mut(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            source_pokemon.use_item().is_some()
        };

        if used_item {
            // source.addVolatile('gem');
            let source_pokemon = match battle.pokemon_at_mut(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            source_pokemon.add_volatile(crate::dex_data::ID::new("gem"));
        }
    }

    EventResult::Continue
}
