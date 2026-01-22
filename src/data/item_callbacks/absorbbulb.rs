//! Absorb Bulb Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;
use crate::Pokemon;

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
        if let Some(_target_pokemon) = battle.pokemon_at_mut(target_pos.0, target_pos.1) {
            Pokemon::use_item(battle, target_pos, None, None);
        }
    }

    EventResult::Continue
}
