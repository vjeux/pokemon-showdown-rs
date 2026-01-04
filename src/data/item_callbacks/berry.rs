//! Berry Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

/// onResidual(pokemon) {
///     if (pokemon.hp <= pokemon.maxhp / 2) {
///         pokemon.eatItem();
///     }
/// }
pub fn on_residual(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // if (pokemon.hp <= pokemon.maxhp / 2) {
    //     pokemon.eatItem();
    // }

    let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    if pokemon_mut.hp <= pokemon_mut.maxhp / 2 {
        Pokemon::eat_item(battle, pokemon_pos, false, None, None);
    }

    EventResult::Continue
}

/// onTryEatItem(item, pokemon) {
///     if (!this.runEvent('TryHeal', pokemon, null, this.effect, 10)) return false;
/// }
pub fn on_try_eat_item(battle: &mut Battle, _item_id: &str, pokemon_pos: (usize, usize)) -> EventResult {
    // if (!this.runEvent('TryHeal', pokemon, null, this.effect, 10)) return false;

    let result = battle.run_event("TryHeal", Some(pokemon_pos), None, None, EventResult::Number(10), false, false);

    if matches!(result, EventResult::Null | EventResult::Continue) {
        return EventResult::Boolean(false);
    }

    EventResult::Continue
}

/// onEat(pokemon) {
///     this.heal(10);
/// }
pub fn on_eat(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // this.heal(10);
    battle.heal(10, Some(pokemon_pos), None, None);

    EventResult::Continue
}
