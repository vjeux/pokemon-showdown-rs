//! Rocky Helmet Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onDamagingHit(damage, target, source, move) {
///     if (this.checkMoveMakesContact(move, source, target)) {
///         this.damage(source.baseMaxhp / 6, source, target);
///     }
/// }
pub fn on_damaging_hit(battle: &mut Battle, _damage: i32, target_pos: (usize, usize), source_pos: (usize, usize)) -> EventResult {
    // if (this.checkMoveMakesContact(move, source, target)) {
    //     this.damage(source.baseMaxhp / 6, source, target);
    // }

    // Check if move makes contact
    let is_contact = match &battle.active_move {
        Some(active_move) => active_move.flags.contact,
        None => return EventResult::Continue,
    };

    if is_contact {
        // Get source's base_maxhp
        let source_base_maxhp = {
            let source_pokemon = match battle.pokemon_at(source_pos.0, source_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            source_pokemon.base_maxhp
        };

        // this.damage(source.baseMaxhp / 6, source, target);
        battle.damage(source_base_maxhp / 6, Some(source_pos), Some(target_pos), None, false);
    }

    EventResult::Continue
}
