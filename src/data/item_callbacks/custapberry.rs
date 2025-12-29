//! Custap Berry Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onFractionalPriority(priority, pokemon) {
///     if (
///         priority <= 0 &&
///         (pokemon.hp <= pokemon.maxhp / 4 || (pokemon.hp <= pokemon.maxhp / 2 &&
///             pokemon.hasAbility('gluttony') && pokemon.abilityState.gluttony))
///     ) {
///         if (pokemon.eatItem()) {
///             this.add('-activate', pokemon, 'item: Custap Berry', '[consumed]');
///             return 0.1;
///         }
///     }
/// }
pub fn on_fractional_priority(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    priority: f64,
) -> EventResult {
    // if (priority <= 0 && ...)
    if priority > 0.0 {
        return EventResult::Continue;
    }

    // Check HP thresholds
    let (hp, maxhp, has_gluttony) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (pokemon.hp, pokemon.maxhp, pokemon.has_ability(&["gluttony"]))
    };

    // pokemon.hp <= pokemon.maxhp / 4 || (pokemon.hp <= pokemon.maxhp / 2 && pokemon.hasAbility('gluttony') ...)
    let should_activate = hp <= maxhp / 4 || (hp <= maxhp / 2 && has_gluttony);

    if should_activate {
        // if (pokemon.eatItem())
        let ate_item = {
            let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_mut.eat_item(false).is_some()
        };

        if ate_item {
            // this.add('-activate', pokemon, 'item: Custap Berry', '[consumed]');
            let pokemon_slot = {
                let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                pokemon.get_slot()
            };

            battle.add(
                "-activate",
                &[
                    crate::battle::Arg::from(pokemon_slot),
                    crate::battle::Arg::from("item: Custap Berry"),
                    crate::battle::Arg::from("[consumed]"),
                ],
            );

            // return 0.1;
            return EventResult::Float(0.1);
        }
    }

    EventResult::Continue
}

/// onEat() {
///     num: 210,
///     gen: 4,
/// }
pub fn on_eat(_battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    // onEat is just metadata in JS (num, gen) - no actual logic
    EventResult::Continue
}
