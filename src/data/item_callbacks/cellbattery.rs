//! Cell Battery Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

/// onDamagingHit(damage, target, source, move) {
///     if (move.type === 'Electric') {
///         target.useItem();
///     }
/// }
pub fn on_damaging_hit(battle: &mut Battle, _damage: i32, target_pos: (usize, usize), _source_pos: (usize, usize)) -> EventResult {
    // Get the active move
    let move_type = match &battle.active_move {
        Some(active_move) => {
            // Get the move data to check its type
            match battle.dex.moves().get_by_id(&active_move.id) {
                Some(move_data) => move_data.move_type.clone(),
                None => return EventResult::Continue,
            }
        }
        None => return EventResult::Continue,
    };

    // if (move.type === 'Electric')
    if move_type == "Electric" {
        // target.useItem();
        // Phase 1: Use the item (remove it from the pokemon)
        let item_id = {
            let pokemon = match battle.pokemon_at_mut(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            match Pokemon::use_item(battle, target_pos, None, None) {
                Some(id) => id,
                None => return EventResult::Continue,
            }
        };

        // Phase 2: Apply the item's boosts (Cell Battery gives +1 Attack)
        // In JavaScript, this is done automatically by useItem, but in Rust we do it manually
        battle.boost(&[("atk", 1)], target_pos, None, Some(item_id.as_str()), false, false);
    }

    EventResult::Continue
}
