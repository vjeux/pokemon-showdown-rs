//! Foresight Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(target) {
///     if (target.volatiles['miracleeye']) return false;
/// }
pub fn on_try_hit(
    battle: &mut Battle,
    _source_pos: (usize, usize),
    target_pos: (usize, usize),
) -> EventResult {
    use crate::dex_data::ID;

    let target = target_pos;

    // if (target.volatiles['miracleeye']) return false;
    let has_miracle_eye = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.has_volatile(&ID::from("miracleeye"))
    };

    if has_miracle_eye {
        return EventResult::NotFail;
    }

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onStart(pokemon) {
    ///     this.add('-start', pokemon, 'Foresight');
    /// }
    pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        let pokemon = pokemon_pos;

        // this.add('-start', pokemon, 'Foresight');
        let pokemon_arg = {
            let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_ref.get_slot()
        };

        battle.add(
            "-start",
            &[pokemon_arg.into(), "Foresight".into()],
        );

        EventResult::Continue
    }

    /// onNegateImmunity(pokemon, type) {
    ///     if (pokemon.hasType('Ghost') && ['Normal', 'Fighting'].includes(type)) return false;
    /// }
    pub fn on_negate_immunity(_battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onModifyBoost(boosts) {
    ///     if (boosts.evasion && boosts.evasion > 0) {
    ///         boosts.evasion = 0;
    ///     }
    /// }
    pub fn on_modify_boost(_battle: &mut Battle) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }
}
