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
        // use_item() automatically applies the item's boosts from data/items.json
        // Cell Battery has boosts: {atk: 1}, so use_item() handles the +1 Attack boost
        Pokemon::use_item(battle, target_pos, None, None);
    }

    EventResult::Continue
}
