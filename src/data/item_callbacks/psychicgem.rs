//! Psychic Gem Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

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
        // Get source pokemon mutably to use item
        let source_pokemon = match battle.pokemon_at_mut(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        if source_pokemon.use_item().is_some() {
            // source.addVolatile('gem');
            source_pokemon.add_volatile("gem".into());
        }
    }

    EventResult::Continue
}
