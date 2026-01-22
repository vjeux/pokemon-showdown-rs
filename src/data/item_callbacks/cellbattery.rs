//! Cell Battery Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;
use crate::Pokemon;

/// onDamagingHit(damage, target, source, move) {
///     if (move.type === 'Electric') {
///         target.useItem();
///     }
/// }
pub fn on_damaging_hit(battle: &mut Battle, _damage: i32, target_pos: (usize, usize), _source_pos: (usize, usize)) -> EventResult {
    // JavaScript checks move.type (the active move's type, not the dex type)
    let is_electric = battle.active_move.as_ref()
        .map(|m| m.move_type == "Electric")
        .unwrap_or(false);

    // if (move.type === 'Electric')
    if is_electric {
        // target.useItem();
        // use_item() automatically applies the item's boosts from data/items.json
        // Cell Battery has boosts: {atk: 1}, so use_item() handles the +1 Attack boost
        Pokemon::use_item(battle, target_pos, None, None);
    }

    EventResult::Continue
}
