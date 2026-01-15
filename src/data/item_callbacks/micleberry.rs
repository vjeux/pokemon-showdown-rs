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

            eprintln!("[MICLEBERRY_RESIDUAL] pokemon={:?}, hp={}, maxhp={}, quarter_hp={}, half_hp={}",
                      pokemon_pos, pokemon.hp, pokemon.maxhp, quarter_hp, half_hp);

            if pokemon.hp <= quarter_hp {
                eprintln!("[MICLEBERRY_RESIDUAL] hp <= quarter_hp, should_eat=true");
                true
            } else if pokemon.hp <= half_hp {
                let has_gluttony = pokemon.has_ability(battle, &["gluttony"]);
                let gluttony_active = pokemon.ability_state.gluttony.unwrap_or(false);
                eprintln!("[MICLEBERRY_RESIDUAL] hp <= half_hp, has_gluttony={}, gluttony_active={}",
                          has_gluttony, gluttony_active);
                has_gluttony && gluttony_active
            } else {
                eprintln!("[MICLEBERRY_RESIDUAL] hp > half_hp, should_eat=false");
                false
            }
        } else {
            false
        }
    };

    if should_eat {
        Pokemon::eat_item(battle, pokemon_pos, false, None, None);
    }

    EventResult::Continue
}

/// onEat(pokemon) {
///     pokemon.addVolatile('micleberry');
/// }
pub fn on_eat(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // pokemon.addVolatile('micleberry');
    Pokemon::add_volatile(battle, pokemon_pos, "micleberry".into(), None, None, None, None);
    EventResult::Continue
}
