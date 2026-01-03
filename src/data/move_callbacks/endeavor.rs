//! Endeavor Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// damageCallback(pokemon, target) {
///     return target.getUndynamaxedHP() - pokemon.hp;
/// }
pub fn damage_callback(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    let pokemon = pokemon_pos;
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // return target.getUndynamaxedHP() - pokemon.hp;
    let target_hp = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.get_undynamaxed_hp(None)
    };

    let pokemon_hp = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_pokemon.hp
    };

    let damage = target_hp - pokemon_hp;

    EventResult::Number(damage)
}

/// onTryImmunity(target, pokemon) {
///     return pokemon.hp < target.hp;
/// }
pub fn on_try_immunity(
    battle: &mut Battle,
    target_pos: Option<(usize, usize)>,
    pokemon_pos: (usize, usize),
) -> EventResult {
    let pokemon = pokemon_pos;
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // return pokemon.hp < target.hp;
    let pokemon_hp = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_pokemon.hp
    };

    let target_hp = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.hp
    };

    EventResult::Boolean(pokemon_hp < target_hp)
}
