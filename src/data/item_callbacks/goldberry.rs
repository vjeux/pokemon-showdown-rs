//! Gold Berry Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::{Battle, Effect};
use crate::event::EventResult;
use crate::Pokemon;

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
        Pokemon::eat_item(battle, pokemon_pos, false, None, None);
    }

    EventResult::Continue
}

/// onTryEatItem(item, pokemon) {
///     if (!this.runEvent('TryHeal', pokemon, null, this.effect, 30)) return false;
/// }
pub fn on_try_eat_item(battle: &mut Battle, item_id: &str, pokemon_pos: (usize, usize)) -> EventResult {
    // if (!this.runEvent('TryHeal', pokemon, null, this.effect, 30)) return false;
    use crate::dex_data::ID;
    let result = battle.run_event(
        "TryHeal",
        Some(crate::event::EventTarget::Pokemon(pokemon_pos)),
        None,
        Some(&Effect::item(ID::from(item_id))),
        crate::event::EventResult::Number(1),
        false,
        false,
    ).is_truthy();

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
