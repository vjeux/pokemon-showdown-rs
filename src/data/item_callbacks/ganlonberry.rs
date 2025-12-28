//! Ganlon Berry Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onUpdate(pokemon) {
///     if (pokemon.hp <= pokemon.maxhp / 4 || (pokemon.hp <= pokemon.maxhp / 2 &&
///         pokemon.hasAbility('gluttony') && pokemon.abilityState.gluttony)) {
///         pokemon.eatItem();
///     }
/// }
pub fn on_update(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // if (pokemon.hp <= pokemon.maxhp / 4 || (pokemon.hp <= pokemon.maxhp / 2 &&
    //     pokemon.hasAbility('gluttony') && pokemon.abilityState.gluttony))

    let (hp, maxhp, has_gluttony) = if let Some(pokemon) = battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
        (pokemon.hp, pokemon.maxhp, pokemon.has_ability(&["gluttony"]))
    } else {
        return EventResult::Continue;
    };

    let should_eat = hp <= maxhp / 4 || (hp <= maxhp / 2 && has_gluttony);

    if should_eat {
        // pokemon.eatItem();
        if let Some(pokemon) = battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            pokemon.eat_item(false);
        }
    }

    EventResult::Continue
}

/// onEat(pokemon) {
///     this.boost({ def: 1 });
/// }
pub fn on_eat(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // this.boost({ def: 1 });
    battle.boost(&[("def", 1)], pokemon_pos, None, None);
    EventResult::Continue
}
