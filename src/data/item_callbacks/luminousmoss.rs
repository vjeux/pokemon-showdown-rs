//! Luminous Moss Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

/// onDamagingHit(damage, target, source, move) {
///     if (move.type === 'Water') {
///         target.useItem();
///     }
/// }
pub fn on_damaging_hit(battle: &mut Battle, _damage: i32, target_pos: (usize, usize), _source_pos: (usize, usize)) -> EventResult {
    // if (move.type === 'Water') {
    //     target.useItem();
    // }

    let is_water_move = match &battle.active_move {
        Some(active_move) => active_move.borrow().move_type == "Water",
        None => return EventResult::Continue,
    };

    if is_water_move {
        let _pokemon_mut = match battle.pokemon_at_mut(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        Pokemon::use_item(battle, target_pos, None, None);
    }

    EventResult::Continue
}
