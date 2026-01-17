//! Sitrus Berry Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::{Battle, hp_fraction};
use crate::event::EventResult;
use crate::Pokemon;

/// onUpdate(pokemon) {
///     if (pokemon.hp <= pokemon.maxhp / 2) {
///         pokemon.eatItem();
///     }
/// }
pub fn on_update(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // if (pokemon.hp <= pokemon.maxhp / 2) {
    //     pokemon.eatItem();
    // }

    // Phase 1: Check HP condition
    let should_eat = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.hp <= pokemon.maxhp / 2
    };

    // Phase 2: Eat item if needed
    if should_eat {
        Pokemon::eat_item(battle, pokemon_pos, false, None, None);
    }

    EventResult::Continue
}

/// onTryEatItem(item, pokemon) {
///     if (!this.runEvent('TryHeal', pokemon, null, this.effect, pokemon.baseMaxhp / 4)) return false;
/// }
pub fn on_try_eat_item(battle: &mut Battle, _item_id: &str, pokemon_pos: (usize, usize)) -> EventResult {
    // if (!this.runEvent('TryHeal', pokemon, null, this.effect, pokemon.baseMaxhp / 4)) return false;

    let heal_amount = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        hp_fraction(pokemon.base_maxhp, 4)
    };

    let result = battle.run_event("TryHeal", Some(crate::event::EventTarget::Pokemon(pokemon_pos)), None, None, EventResult::Number(heal_amount), false, false);

    if matches!(result, EventResult::Boolean(false) | EventResult::Null | EventResult::Stop) {
        return EventResult::Boolean(false);
    }

    EventResult::Continue
}

/// onEat(pokemon) {
///     this.heal(pokemon.baseMaxhp / 4);
/// }
pub fn on_eat(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // this.heal(pokemon.baseMaxhp / 4);

    let heal_amount = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        hp_fraction(pokemon.base_maxhp, 4)
    };

    battle.heal(heal_amount, Some(pokemon_pos), None, None);

    EventResult::Continue
}
