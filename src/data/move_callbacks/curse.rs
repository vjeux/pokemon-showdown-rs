//! Curse Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyMove(move, source, target) {
///     if (!source.hasType('Ghost')) {
///         move.target = move.nonGhostTarget!;
///     } else if (source.isAlly(target)) {
///         move.target = 'randomNormal';
///     }
/// }
pub fn on_modify_move(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    // if (!source.hasType('Ghost')) {
    //     move.target = move.nonGhostTarget!;
    // } else if (source.isAlly(target)) {
    //     move.target = 'randomNormal';
    // }
    let source = pokemon_pos;

    let source_has_ghost = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.has_type(&ID::from("ghost"), battle)
    };

    if !source_has_ghost {
        // move.target = move.nonGhostTarget!;
        // TODO: Modify the current move's target to nonGhostTarget
        // This requires access to the current move state which may not be directly available
        // For now, we'll leave this as a placeholder
    } else if let Some(target) = target_pos {
        // else if (source.isAlly(target)) {
        //     move.target = 'randomNormal';
        // }
        let is_ally = battle.is_ally(source, target);
        if is_ally {
            // move.target = 'randomNormal';
            // TODO: Modify the current move's target to randomNormal
        }
    }

    EventResult::Continue
}

/// onTryHit(target, source, move) {
///     if (!source.hasType('Ghost')) {
///         delete move.volatileStatus;
///         delete move.onHit;
///         move.self = { boosts: { spe: -1, atk: 1, def: 1 } };
///     } else if (move.volatileStatus && target.volatiles['curse']) {
///         return false;
///     }
/// }
pub fn on_try_hit(battle: &mut Battle, source_pos: (usize, usize), target_pos: (usize, usize)) -> EventResult {
    use crate::dex_data::ID;

    // if (!source.hasType('Ghost')) {
    //     delete move.volatileStatus;
    //     delete move.onHit;
    //     move.self = { boosts: { spe: -1, atk: 1, def: 1 } };
    // } else if (move.volatileStatus && target.volatiles['curse']) {
    //     return false;
    // }
    let source = source_pos;
    let target = target_pos;

    let source_has_ghost = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.has_type(&ID::from("ghost"), battle)
    };

    if !source_has_ghost {
        // delete move.volatileStatus;
        // delete move.onHit;
        // move.self = { boosts: { spe: -1, atk: 1, def: 1 } };
        // TODO: Modify the current move's properties
        // This would require access to the active move state which isn't directly available here
        // The move modification would happen in the move execution logic
    } else {
        // else if (move.volatileStatus && target.volatiles['curse']) {
        //     return false;
        // }
        // Check if target already has curse volatile
        let target_has_curse = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.volatiles.contains_key(&ID::from("curse"))
        };

        if target_has_curse {
            // TODO: Check if move has volatileStatus
            // For now, we'll assume if target has curse, we should return false
            return EventResult::Boolean(false);
        }
    }

    EventResult::Continue
}

/// onHit(target, source) {
///     this.directDamage(source.maxhp / 2, source, source);
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // this.directDamage(source.maxhp / 2, source, source);
    let source = pokemon_pos;

    let damage = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.maxhp / 2
    };

    battle.direct_damage(damage, source, Some(source));

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onStart(pokemon, source) {
    ///     this.add('-start', pokemon, 'Curse', `[of] ${source}`);
    /// }
    pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>) -> EventResult {
        // this.add('-start', pokemon, 'Curse', `[of] ${source}`);
        let pokemon = pokemon_pos;
        let source = match source_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        let (pokemon_arg, source_arg) = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };

            (crate::battle::Arg::from(pokemon_pokemon), crate::battle::Arg::from(source_pokemon))
        };

        battle.add("-start", &[
            pokemon_arg,
            "Curse".into(),
            format!("[of] {}", source_arg.to_protocol_string()).into(),
        ]);

        EventResult::Continue
    }

    /// onResidual(pokemon) {
    ///     this.damage(pokemon.baseMaxhp / 4);
    /// }
    pub fn on_residual(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // this.damage(pokemon.baseMaxhp / 4);
        let pokemon = pokemon_pos;

        let damage = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_pokemon.base_maxhp / 4
        };

        battle.damage(damage, pokemon);

        EventResult::Continue
    }
}
