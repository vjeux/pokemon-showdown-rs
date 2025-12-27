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
pub fn damage_callback(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onTryImmunity(target, pokemon) {
///     return pokemon.hp < target.hp;
/// }
pub fn on_try_immunity(battle: &mut Battle, target_pos: Option<(usize, usize)>, pokemon_pos: (usize, usize)) -> EventResult {
    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    // Only works if pokemon HP is less than target HP
    EventResult::Bool(pokemon.hp < target.hp)
}

