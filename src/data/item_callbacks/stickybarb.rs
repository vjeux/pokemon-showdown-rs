//! Sticky Barb Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onResidual(pokemon) {
///     this.damage(pokemon.baseMaxhp / 8);
/// }
pub fn on_residual(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // Get pokemon's base_maxhp
    let base_maxhp = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.base_maxhp
    };

    // this.damage(pokemon.baseMaxhp / 8);
    battle.damage(base_maxhp / 8, Some(pokemon_pos), None, None, false);

    EventResult::Continue
}

/// onHit(target, source, move) {
///     if (source && source !== target && !source.item && move && this.checkMoveMakesContact(move, source, target)) {
///         const barb = target.takeItem();
///         if (!barb) return; // Gen 4 Multitype
///         source.setItem(barb);
///         // no message for Sticky Barb changing hands
///     }
/// }
pub fn on_hit(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}
