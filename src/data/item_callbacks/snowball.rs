//! Snowball Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onDamagingHit(damage, target, source, move) {
///     if (move.type === 'Ice') {
///         target.useItem();
///     }
/// }
pub fn on_damaging_hit(battle: &mut Battle, damage: i32, target_pos: (usize, usize), source_pos: (usize, usize)) -> EventResult {
    // if (move.type === 'Ice')
    let is_ice = {
        if let Some(ref active_move) = battle.active_move {
            active_move.move_type == "Ice"
        } else {
            false
        }
    };

    if is_ice {
        // target.useItem();
        if let Some(target_pokemon) = battle.pokemon_at_mut(target_pos.0, target_pos.1) {
            target_pokemon.use_item();
        }
    }

    EventResult::Continue
}
