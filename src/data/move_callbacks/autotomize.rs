//! Autotomize Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(pokemon) {
///     const hasContrary = pokemon.hasAbility('contrary');
///     if ((!hasContrary && pokemon.boosts.spe === 6) || (hasContrary && pokemon.boosts.spe === -6)) {
///         return false;
///     }
/// }
pub fn on_try_hit(battle: &mut Battle, _source_pos: (usize, usize), target_pos: (usize, usize)) -> EventResult {
    // Get the pokemon (target is the user for this move)
    let pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    // const hasContrary = pokemon.hasAbility('contrary');
    let has_contrary = pokemon.has_ability(&["contrary"]);

    // if ((!hasContrary && pokemon.boosts.spe === 6) || (hasContrary && pokemon.boosts.spe === -6)) {
    //     return false;
    // }
    let spe_boost = pokemon.boosts.spe;
    if (!has_contrary && spe_boost == 6) || (has_contrary && spe_boost == -6) {
        return EventResult::Boolean(false);
    }

    EventResult::Continue
}

/// onHit(pokemon) {
///     if (pokemon.weighthg > 1) {
///         pokemon.weighthg = Math.max(1, pokemon.weighthg - 1000);
///         this.add('-start', pokemon, 'Autotomize');
///     }
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // Get the pokemon (use target_pos if available, otherwise pokemon_pos)
    let target = target_pos.unwrap_or(pokemon_pos);

    // Get pokemon to check weight
    let pokemon = match battle.pokemon_at(target.0, target.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    // if (pokemon.weighthg > 1) {
    if pokemon.weight_hg > 1 {
        // pokemon.weighthg = Math.max(1, pokemon.weighthg - 1000);
        let new_weight = std::cmp::max(1, pokemon.weight_hg - 1000);

        // Get mutable reference to update weight
        let pokemon_mut = match battle.pokemon_at_mut(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_mut.weight_hg = new_weight;

        // Get pokemon identifier for battle.add
        let pokemon_ident = {
            let pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.get_slot()
        };

        // this.add('-start', pokemon, 'Autotomize');
        battle.add("-start", &[pokemon_ident.as_str().into(), "Autotomize".into()]);
    }

    EventResult::Continue
}

