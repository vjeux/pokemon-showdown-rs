//! Petaya Berry Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;
use crate::Pokemon;

/// onUpdate(pokemon) {
///     if (pokemon.hp <= pokemon.maxhp / 4 || (pokemon.hp <= pokemon.maxhp / 2 &&
///         pokemon.hasAbility('gluttony') && pokemon.abilityState.gluttony)) {
///         pokemon.eatItem();
///     }
/// }
pub fn on_update(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // Check conditions for eating the item
    let should_eat = {
        if let Some(pokemon) = battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            let quarter_hp = pokemon.maxhp / 4;
            let half_hp = pokemon.maxhp / 2;

            // if (pokemon.hp <= pokemon.maxhp / 4 || ...)
            if pokemon.hp <= quarter_hp {
                true
            } else if pokemon.hp <= half_hp {
                // (pokemon.hp <= pokemon.maxhp / 2 && pokemon.hasAbility('gluttony') && pokemon.abilityState.gluttony)
                let has_gluttony = pokemon.has_ability(battle, &["gluttony"]);
                let gluttony_active = pokemon.ability_state.gluttony.unwrap_or(false);
                has_gluttony && gluttony_active
            } else {
                false
            }
        } else {
            false
        }
    };

    if should_eat {
        // pokemon.eatItem();
        Pokemon::eat_item(battle, pokemon_pos, false, None, None);
    }

    EventResult::Continue
}

/// onEat(pokemon) {
///     this.boost({ spa: 1 });
/// }
pub fn on_eat(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // this.boost({ spa: 1 });
    battle.boost(&[("spa", 1)], pokemon_pos, None, None, false, false);

    EventResult::Continue
}
