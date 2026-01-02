//! Fighting Gem Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::pokemon::Pokemon;

/// onSourceTryPrimaryHit(target, source, move) {
///     if (target === source || move.category === 'Status') return;
///     if (move.type === 'Fighting' && source.useItem()) {
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

    // Check if target === source
    if target == source {
        return EventResult::Continue;
    }

    // Check move.category === 'Status'
    let active_move = match &battle.active_move {
        Some(m) => m,
        None => return EventResult::Continue,
    };

    if active_move.category == "Status" {
        return EventResult::Continue;
    }

    // if (move.type === 'Fighting' && source.useItem()) { source.addVolatile('gem'); }
    if active_move.move_type == "Fighting" {
        // Two-phase borrow: first get item, then modify pokemon
        let item_used = {
            let source_pokemon = match battle.pokemon_at_mut(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            source_pokemon.use_item(None, None)
        };

        if item_used.is_some() {
            Pokemon::add_volatile(battle, source, "gem".into(), None, None, None);
        }
    }

    EventResult::Continue
}
