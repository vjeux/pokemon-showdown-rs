//! Absorb Bulb Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onDamagingHit(damage, target, source, move) {
///     if (move.type === 'Water') {
///         target.useItem();
///     }
/// }
pub fn on_damaging_hit(battle: &mut Battle, _damage: i32, target_pos: (usize, usize), _source_pos: (usize, usize)) -> EventResult {
    // if (move.type === 'Water')
    let is_water = {
        if let Some(ref active_move) = battle.active_move {
            active_move.move_type == "Water"
        } else {
            false
        }
    };

    if is_water {
        // target.useItem();
        if let Some(target_pokemon) = battle.pokemon_at_mut(target_pos.0, target_pos.1) {
            target_pokemon.use_item(None, None);
        }
    }

    EventResult::Continue
}
