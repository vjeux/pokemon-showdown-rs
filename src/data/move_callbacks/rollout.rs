//! Rollout Move
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
///     const rolloutData = pokemon.volatiles['rollout'];
///     if (rolloutData?.hitCount) {
///         bp *= 2 ** rolloutData.contactHitCount;
///     }
///     if (rolloutData && pokemon.status !== 'slp') {
///         rolloutData.hitCount++;
///         rolloutData.contactHitCount++;
///         if (rolloutData.hitCount < 5) {
///             rolloutData.duration = 2;
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
    let mut bp = {
        let active_move = match &battle.active_move {
            Some(active_move) => active_move,
            None => return EventResult::Continue,
        };
        active_move.base_power
    };

    // const rolloutData = pokemon.volatiles['rollout'];
    // if (rolloutData?.hitCount) {
    //     bp *= 2 ** rolloutData.contactHitCount;
    // }
    let rollout_data = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_pokemon.volatiles.get(&ID::from("rollout")).cloned()
    };

    if let Some(ref data) = rollout_data {
        if data.hit_count.unwrap_or(0) > 0 {
            bp *= 2_i32.pow(data.contact_hit_count.unwrap_or(0) as u32);
        }
    }

    // if (rolloutData && pokemon.status !== 'slp') {
    //     rolloutData.hitCount++;
    //     rolloutData.contactHitCount++;
    //     if (rolloutData.hitCount < 5) {
    //         rolloutData.duration = 2;
    //     }
    // }
    if rollout_data.is_some() {
        let status = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_pokemon.status.clone()
        };

        if status != ID::from("slp") {
            let pokemon_pokemon = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };

            if let Some(data) = pokemon_pokemon.volatiles.get_mut(&ID::from("rollout")) {
                data.hit_count = Some(data.hit_count.unwrap_or(0) + 1);
                data.contact_hit_count = Some(data.contact_hit_count.unwrap_or(0) + 1);
                if data.hit_count.unwrap_or(0) < 5 {
                    data.duration = Some(2);
                }
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
///     if (pokemon.volatiles['rollout'] || pokemon.status === 'slp' || !target) return;
///     pokemon.addVolatile('rollout');
///     if (move.sourceEffect) pokemon.lastMoveTargetLoc = pokemon.getLocOf(target);
/// }
pub fn on_modify_move(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    let pokemon = pokemon_pos;
    let target = target_pos;

    // if (pokemon.volatiles['rollout'] || pokemon.status === 'slp' || !target) return;
    let (has_rollout, is_sleeping) = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (
            pokemon_pokemon.volatiles.contains_key(&ID::from("rollout")),
            pokemon_pokemon.status == ID::from("slp"),
        )
    };

    if has_rollout || is_sleeping || target.is_none() {
        return EventResult::Continue;
    }

    // pokemon.addVolatile('rollout');
    Pokemon::add_volatile(battle, pokemon, ID::from("rollout"), None);

    // if (move.sourceEffect) pokemon.lastMoveTargetLoc = pokemon.getLocOf(target);
    let has_source_effect = {
        let active_move = match &battle.active_move {
            Some(active_move) => active_move,
            None => return EventResult::Continue,
        };
        active_move.source_effect.is_some()
    };

    if has_source_effect {
        let target = target.unwrap();
        let target_loc = if let Some(pokemon_pokemon) = battle.sides.get(pokemon.0).and_then(|s| s.pokemon.get(pokemon.1)) {
            pokemon_pokemon.get_loc_of(target.0, target.1, battle.active_per_half) as i32
        } else {
            0
        };

        let pokemon_pokemon = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_pokemon.last_move_target_loc = Some(target_loc as i8);
    }

    EventResult::Continue
}

/// onAfterMove(source, target, move) {
///     const rolloutData = source.volatiles["rollout"];
///     if (
///         rolloutData &&
///         rolloutData.hitCount === 5 &&
///         rolloutData.contactHitCount < 5
///         // this conditions can only be met in gen7 and gen8dlc1
///         // see `disguise` and `iceface` abilities in the resp mod folders
///     ) {
///         source.addVolatile("rolloutstorage");
///         source.volatiles["rolloutstorage"].contactHitCount =
///             rolloutData.contactHitCount;
///     }
/// }
pub fn on_after_move(
    battle: &mut Battle,
    source_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    let source = source_pos;

    // const rolloutData = source.volatiles["rollout"];
    // if (
    //     rolloutData &&
    //     rolloutData.hitCount === 5 &&
    //     rolloutData.contactHitCount < 5
    //     // this conditions can only be met in gen7 and gen8dlc1
    //     // see `disguise` and `iceface` abilities in the resp mod folders
    // ) {
    let rollout_data = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.volatiles.get(&ID::from("rollout")).cloned()
    };

    if let Some(data) = rollout_data {
        if data.hit_count.unwrap_or(0) == 5 && data.contact_hit_count.unwrap_or(0) < 5 {
            // source.addVolatile("rolloutstorage");
            Pokemon::add_volatile(battle, source, ID::from("rolloutstorage"), None);

            // source.volatiles["rolloutstorage"].contactHitCount = rolloutData.contactHitCount;
            let source_pokemon = match battle.pokemon_at_mut(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };

            if let Some(storage_data) = source_pokemon
                .volatiles
                .get_mut(&ID::from("rolloutstorage"))
            {
                storage_data.contact_hit_count = data.contact_hit_count;
            }
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
        let effect_state = match &mut battle.current_effect_state {
            Some(es) => es,
            None => return EventResult::Continue,
        };

        effect_state.hit_count = Some(0);
        effect_state.contact_hit_count = Some(0);

        EventResult::Continue
    }

    /// onResidual(target) {
    ///     if (target.lastMove && target.lastMove.id === 'struggle') {
    ///         // don't lock
    ///         delete target.volatiles['rollout'];
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
        //     delete target.volatiles['rollout'];
        // }
        let last_move_is_struggle = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon
                .last_move
                .as_ref()
                .map(|m| *m == ID::from("struggle"))
                .unwrap_or(false)
        };

        if last_move_is_struggle {
            {
                let pokemon = match battle.pokemon_at_mut(target.0, target.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                pokemon.remove_volatile(&ID::from("rollout"));
            }
        }

        EventResult::Continue
    }
}
