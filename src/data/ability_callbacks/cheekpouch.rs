//! Cheek Pouch Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::{Battle, Effect, hp_fraction};
use crate::event::EventResult;

/// onEatItem(item, pokemon) {
///     this.heal(pokemon.baseMaxhp / 3);
/// }
pub fn on_eat_item(battle: &mut Battle, _item_id: Option<&str>, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect: Option<&Effect>) -> EventResult {
    // Heal 1/3 max HP when eating an item
    let (heal_amount, _current_hp, _max_hp) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (hp_fraction(pokemon.base_maxhp, 3), pokemon.hp, pokemon.base_maxhp)
    };

    debug_elog!("[CHEEKPOUCH] Turn {}: on_eat_item called! Healing {} HP (current {}/{})",
        battle.turn, heal_amount, _current_hp, _max_hp);

    battle.heal(heal_amount, Some(pokemon_pos), None, None);
    EventResult::Continue
}

