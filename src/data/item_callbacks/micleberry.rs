//! Micle Berry Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::pokemon::Pokemon;

/// onResidual(pokemon) {
///     if (pokemon.hp <= pokemon.maxhp / 4 || (pokemon.hp <= pokemon.maxhp / 2 &&
///         pokemon.hasAbility('gluttony') && pokemon.abilityState.gluttony)) {
///         pokemon.eatItem();
///     }
/// }
pub fn on_residual(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // if (pokemon.hp <= pokemon.maxhp / 4 || (pokemon.hp <= pokemon.maxhp / 2 &&
    //     pokemon.hasAbility('gluttony') && pokemon.abilityState.gluttony)) {
    //     pokemon.eatItem();
    // }

    let should_eat = {
        if let Some(pokemon) = battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            let quarter_hp = pokemon.maxhp / 4;
            let half_hp = pokemon.maxhp / 2;

            if pokemon.hp <= quarter_hp {
                true
            } else if pokemon.hp <= half_hp {
                let has_gluttony = pokemon.has_ability(battle, &["gluttony"]);
                let gluttony_active = pokemon.ability_state.data.get("gluttony")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                has_gluttony && gluttony_active
            } else {
                false
            }
        } else {
            false
        }
    };

    if should_eat {
        if let Some(pokemon) = battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            pokemon.eat_item(false);
        }
    }

    EventResult::Continue
}

/// onEat(pokemon) {
///     pokemon.addVolatile('micleberry');
/// }
pub fn on_eat(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // pokemon.addVolatile('micleberry');
    Pokemon::add_volatile(battle, pokemon_pos, "micleberry".into(), None, None, None);
    EventResult::Continue
}
