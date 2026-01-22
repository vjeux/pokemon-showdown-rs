//! Shadow Tag Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;
use crate::pokemon::Pokemon;

/// onFoeTrapPokemon(pokemon) {
///     if (!pokemon.hasAbility('shadowtag') && pokemon.isAdjacent(this.effectState.target)) {
///         pokemon.tryTrap(true);
///     }
/// }
pub fn on_foe_trap_pokemon(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // Get ability holder position from effectState.target
    let ability_holder_pos = match battle.effect_state.target {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (!pokemon.hasAbility('shadowtag') && pokemon.isAdjacent(this.effectState.target))
    let (has_shadowtag, is_adjacent) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (pokemon.has_ability(battle, &["shadowtag"]), battle.is_adjacent(pokemon_pos, ability_holder_pos))
    };

    if !has_shadowtag && is_adjacent {
        // pokemon.tryTrap(true);
        Pokemon::try_trap(battle, pokemon_pos, true);
    }

    EventResult::Continue
}

/// onFoeMaybeTrapPokemon(pokemon, source) {
///     if (!source) source = this.effectState.target;
///     if (!source || !pokemon.isAdjacent(source)) return;
///     if (!pokemon.hasAbility('shadowtag')) {
///         pokemon.maybeTrapped = true;
///     }
/// }
pub fn on_foe_maybe_trap_pokemon(battle: &mut Battle, pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>) -> EventResult {
    // if (!source) source = this.effectState.target;
    let source_pos = source_pos.or(battle.effect_state.target);

    // if (!source || !pokemon.isAdjacent(source)) return;
    let source_pos = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    if !battle.is_adjacent(pokemon_pos, source_pos) {
        return EventResult::Continue;
    }

    // if (!pokemon.hasAbility('shadowtag'))
    let has_shadowtag = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.has_ability(battle, &["shadowtag"])
    };

    if !has_shadowtag {
        // pokemon.maybeTrapped = true;
        let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_mut.maybe_trapped = true;
    }

    EventResult::Continue
}

