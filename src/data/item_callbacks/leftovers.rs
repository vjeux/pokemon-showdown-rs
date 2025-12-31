//! Leftovers Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onResidual(pokemon) {
///     this.heal(pokemon.baseMaxhp / 16);
/// }
pub fn on_residual(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    eprintln!("[LEFTOVERS DEBUG] on_residual called for Pokemon at {:?}", pokemon_pos);

    // Get pokemon's base_maxhp
    let base_maxhp = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        eprintln!("[LEFTOVERS DEBUG] Pokemon {} has base_maxhp={}, hp={}", pokemon.name, pokemon.base_maxhp, pokemon.hp);
        pokemon.base_maxhp
    };

    let heal_amount = base_maxhp / 16;
    eprintln!("[LEFTOVERS DEBUG] Healing {} HP ({} / 16)", heal_amount, base_maxhp);

    // this.heal(pokemon.baseMaxhp / 16);
    battle.heal(heal_amount, Some(pokemon_pos), None, None);

    EventResult::Continue
}
