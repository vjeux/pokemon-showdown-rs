//! Gold Berry Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onResidual(pokemon) {
///     if (pokemon.hp <= pokemon.maxhp / 2) {
///         pokemon.eatItem();
///     }
/// }
pub fn on_residual(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // if (pokemon.hp <= pokemon.maxhp / 2)
    let (hp, maxhp) = if let Some(pokemon) = battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
        (pokemon.hp, pokemon.maxhp)
    } else {
        return EventResult::Continue;
    };

    if hp <= maxhp / 2 {
        // pokemon.eatItem();
        if let Some(pokemon) = battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            pokemon.eat_item(false);
        }
    }

    EventResult::Continue
}

/// onTryEatItem(item, pokemon) {
///     if (!this.runEvent('TryHeal', pokemon, null, this.effect, 30)) return false;
/// }
pub fn on_try_eat_item(battle: &mut Battle, item_id: &str, pokemon_pos: (usize, usize)) -> EventResult {
    // if (!this.runEvent('TryHeal', pokemon, null, this.effect, 30)) return false;
    let result = battle.run_event_bool(
        "TryHeal",
        Some(pokemon_pos),
        None,
        Some(&item_id.into()),
    );

    if !result {
        return EventResult::Boolean(false);
    }

    EventResult::Continue
}

/// onEat(pokemon) {
///     this.heal(30);
/// }
pub fn on_eat(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // this.heal(30);
    battle.heal(30, Some(pokemon_pos), None, None);
    EventResult::Continue
}
