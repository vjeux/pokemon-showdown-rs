//! Rocky Helmet Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::{Battle, Effect, hp_fraction};
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

    // JavaScript uses this.checkMoveMakesContact(move, source, target) which checks for Protective Pads
    // We need to clone active_move first to avoid borrow issues
    let active_move_clone = battle.active_move.clone();
    if !battle.check_move_makes_contact_with_active_move(active_move_clone.as_ref(), source_pos, target_pos, false) {
        return EventResult::Continue;
    }

    // Get source's base_maxhp
    let source_base_maxhp = {
        let source_pokemon = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.base_maxhp
    };

    // this.damage(source.baseMaxhp / 6, source, target);
    battle.damage(hp_fraction(source_base_maxhp, 6), Some(source_pos), Some(target_pos), None, false);

    EventResult::Continue
}
