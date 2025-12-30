//! Grass Gem Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onSourceTryPrimaryHit(target, source, move) {
///     if (target === source || move.category === 'Status' || move.flags['pledgecombo']) return;
///     if (move.type === 'Grass' && source.useItem()) {
///         source.addVolatile('gem');
///     }
/// }
pub fn on_source_try_primary_hit(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, _move_id: &str) -> EventResult {
    // if (target === source || move.category === 'Status' || move.flags['pledgecombo']) return;
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

    // move.flags['pledgecombo']
    let has_pledgecombo = battle.active_move.as_ref()
        .map(|m| m.flags.pledgecombo)
        .unwrap_or(false);

    if has_pledgecombo {
        return EventResult::Continue;
    }

    // if (move.type === 'Grass' && source.useItem())
    let is_grass = battle.active_move.as_ref()
        .map(|m| m.move_type.as_str() == "Grass")
        .unwrap_or(false);

    if is_grass {
        // source.useItem()
        let used_item = if let Some(source_pokemon) = battle.pokemon_at_mut(source.0, source.1) {
            source_pokemon.use_item().is_some()
        } else {
            false
        };

        if used_item {
            // source.addVolatile('gem');
            if let Some(source_pokemon) = battle.pokemon_at_mut(source.0, source.1) {
                source_pokemon.add_volatile("gem".into());
            }
        }
    }

    EventResult::Continue
}
