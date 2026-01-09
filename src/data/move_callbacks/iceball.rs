//! Ice Ball Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;
use crate::pokemon::Pokemon;

/// basePowerCallback(pokemon, target, move) {
///     let bp = move.basePower;
///     const iceballData = pokemon.volatiles['iceball'];
///     if (iceballData?.hitCount) {
///         bp *= 2 ** iceballData.contactHitCount;
///     }
///     if (iceballData && pokemon.status !== 'slp') {
///         iceballData.hitCount++;
///         iceballData.contactHitCount++;
///         if (iceballData.hitCount < 5) {
///             iceballData.duration = 2;
///         }
///     }
///     if (pokemon.volatiles['defensecurl']) {
///         bp *= 2;
///     }
///     this.debug(`BP: ${bp}`);
///     return bp;
/// }
pub fn base_power_callback(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    let pokemon = pokemon_pos;

    // let bp = move.basePower;
    let mut bp = battle
        .active_move
        .as_ref()
        .map(|m| m.base_power)
        .unwrap_or(0);

    // const iceballData = pokemon.volatiles['iceball'];
    // if (iceballData?.hitCount) {
    //     bp *= 2 ** iceballData.contactHitCount;
    // }
    let (has_hit_count, contact_hit_count) = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        if let Some(iceball_volatile) = pokemon_pokemon.volatiles.get(&ID::from("iceball")) {
            let hit_count = iceball_volatile.hit_count.unwrap_or(0);
            let contact_hit_count = iceball_volatile.contact_hit_count.unwrap_or(0);
            (hit_count > 0, contact_hit_count as u32)
        } else {
            (false, 0)
        }
    };

    if has_hit_count {
        bp *= 2_i32.pow(contact_hit_count);
    }

    // if (iceballData && pokemon.status !== 'slp') {
    //     iceballData.hitCount++;
    //     iceballData.contactHitCount++;
    //     if (iceballData.hitCount < 5) {
    //         iceballData.duration = 2;
    //     }
    // }
    let has_iceball = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_pokemon.volatiles.contains_key(&ID::from("iceball"))
    };

    let is_sleeping = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_pokemon.status == ID::from("slp")
    };

    // NOTE: basePowerCallback is called outside the event dispatch system, so we can't use
    // battle.with_effect_state() as it requires battle.effect to be set. Instead, we directly
    // modify the pokemon's volatile state.
    if has_iceball && !is_sleeping {
        // Directly update the volatile state on the pokemon
        let pokemon_mut = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        if let Some(volatile_state) = pokemon_mut.volatiles.get_mut(&ID::from("iceball")) {
            volatile_state.hit_count = Some(volatile_state.hit_count.unwrap_or(0) + 1);
            volatile_state.contact_hit_count = Some(volatile_state.contact_hit_count.unwrap_or(0) + 1);
            if volatile_state.hit_count.unwrap_or(0) < 5 {
                volatile_state.duration = Some(2);
            }
        }
    }

    // if (pokemon.volatiles['defensecurl']) {
    //     bp *= 2;
    // }
    let has_defense_curl = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_pokemon
            .volatiles
            .contains_key(&ID::from("defensecurl"))
    };

    if has_defense_curl {
        bp *= 2;
    }

    // this.debug(`BP: ${bp}`);
    battle.debug(&format!("BP: {}", bp));

    // return bp;
    EventResult::Number(bp)
}

/// onModifyMove(move, pokemon, target) {
///     if (pokemon.volatiles['iceball'] || pokemon.status === 'slp' || !target) return;
///     pokemon.addVolatile('iceball');
///     if (move.sourceEffect) pokemon.lastMoveTargetLoc = pokemon.getLocOf(target);
/// }
pub fn on_modify_move(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    let pokemon = pokemon_pos;
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (pokemon.volatiles['iceball'] || pokemon.status === 'slp' || !target) return;
    let has_iceball = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_pokemon.volatiles.contains_key(&ID::from("iceball"))
    };

    let is_sleeping = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_pokemon.status == ID::from("slp")
    };

    if has_iceball || is_sleeping {
        return EventResult::Continue;
    }

    // pokemon.addVolatile('iceball');
    Pokemon::add_volatile(battle, pokemon, ID::from("iceball"), None, None, None, None);

    // if (move.sourceEffect) pokemon.lastMoveTargetLoc = pokemon.getLocOf(target);
    if battle
        .active_move
        .as_ref()
        .and_then(|m| m.source_effect.as_ref())
        .is_some()
    {
        let target_loc = {
            let active_per_half = battle.active_per_half;
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_pokemon.get_loc_of(target.0, target.1, active_per_half)
        };

        let pokemon_pokemon = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_pokemon.last_move_target_loc = Some(target_loc);
    }

    EventResult::Continue
}

/// onAfterMove(source, target, move) {
///     const iceballData = source.volatiles["iceball"];
///     if (
///         iceballData &&
///         iceballData.hitCount === 5 &&
///         iceballData.contactHitCount < 5
///         // this conditions can only be met in gen7 and gen8dlc1
///         // see `disguise` and `iceface` abilities in the resp mod folders
///     ) {
///         source.addVolatile("rolloutstorage");
///         source.volatiles["rolloutstorage"].contactHitCount =
///         iceballData.contactHitCount;
///     }
/// }
pub fn on_after_move(
    battle: &mut Battle,
    source_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    let source = source_pos;

    // const iceballData = source.volatiles["iceball"];
    // if (
    //     iceballData &&
    //     iceballData.hitCount === 5 &&
    //     iceballData.contactHitCount < 5
    //     // this conditions can only be met in gen7 and gen8dlc1
    //     // see `disguise` and `iceface` abilities in the resp mod folders
    // ) {
    let (has_iceball, hit_count_is_5, contact_hit_count) = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        if let Some(iceball_volatile) = source_pokemon.volatiles.get(&ID::from("iceball")) {
            let hit_count = iceball_volatile.hit_count.unwrap_or(0);
            let contact_hit_count = iceball_volatile.contact_hit_count.unwrap_or(0);
            (true, hit_count == 5, contact_hit_count)
        } else {
            (false, false, 0)
        }
    };

    if has_iceball && hit_count_is_5 && contact_hit_count < 5 {
        // source.addVolatile("rolloutstorage");
        Pokemon::add_volatile(battle, source, ID::from("rolloutstorage"), None, None, None, None);

        // source.volatiles["rolloutstorage"].contactHitCount =
        // iceballData.contactHitCount;
        let source_pokemon = match battle.pokemon_at_mut(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        if let Some(rollout_volatile) = source_pokemon
            .volatiles
            .get_mut(&ID::from("rolloutstorage"))
        {
            rollout_volatile.contact_hit_count = Some(contact_hit_count);
        }
    }

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onStart() {
    ///     this.effectState.hitCount = 0;
    ///     this.effectState.contactHitCount = 0;
    /// }
    pub fn on_start(battle: &mut Battle) -> EventResult {
        // this.effectState.hitCount = 0;
        // this.effectState.contactHitCount = 0;
        battle.with_effect_state(|state| {
            state.hit_count = Some(0);
            state.contact_hit_count = Some(0);
        });

        EventResult::Continue
    }

    /// onResidual(target) {
    ///     if (target.lastMove && target.lastMove.id === 'struggle') {
    ///         // don't lock
    ///         delete target.volatiles['iceball'];
    ///     }
    /// }
    pub fn on_residual(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        use crate::dex_data::ID;

        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // if (target.lastMove && target.lastMove.id === 'struggle') {
        //     // don't lock
        //     delete target.volatiles['iceball'];
        // }
        let last_move_is_struggle = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.last_move.as_ref().map(|m| m.as_str()) == Some("struggle")
        };

        if last_move_is_struggle {
            let target_pokemon = match battle.pokemon_at_mut(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.volatiles.remove(&ID::from("iceball"));
        }

        EventResult::Continue
    }
}
