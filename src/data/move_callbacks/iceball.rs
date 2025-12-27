//! Ice Ball Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

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
pub fn base_power_callback(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    let pokemon = pokemon_pos;

    // let bp = move.basePower;
    let mut bp = battle.active_move.as_ref().map(|m| m.base_power).unwrap_or(0);

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
            let hit_count = iceball_volatile.data.get("hitCount").and_then(|v| v.as_i64()).unwrap_or(0);
            let contact_hit_count = iceball_volatile.data.get("contactHitCount").and_then(|v| v.as_i64()).unwrap_or(0);
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
        pokemon_pokemon.status.as_ref().map(|s| s.as_str()) == Some("slp")
    };

    if has_iceball && !is_sleeping {
        let pokemon_pokemon = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        if let Some(iceball_volatile) = pokemon_pokemon.volatiles.get_mut(&ID::from("iceball")) {
            let hit_count = iceball_volatile.data.get("hitCount").and_then(|v| v.as_i64()).unwrap_or(0);
            iceball_volatile.data.insert("hitCount".to_string(), serde_json::json!(hit_count + 1));

            let contact_hit_count = iceball_volatile.data.get("contactHitCount").and_then(|v| v.as_i64()).unwrap_or(0);
            iceball_volatile.data.insert("contactHitCount".to_string(), serde_json::json!(contact_hit_count + 1));

            if hit_count + 1 < 5 {
                iceball_volatile.duration = 2;
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
        pokemon_pokemon.volatiles.contains_key(&ID::from("defensecurl"))
    };

    if has_defense_curl {
        bp *= 2;
    }

    // this.debug(`BP: ${bp}`);
    battle.debug(&format!("BP: {}", bp));

    // return bp;
    EventResult::Int(bp)
}

/// onModifyMove(move, pokemon, target) {
///     if (pokemon.volatiles['iceball'] || pokemon.status === 'slp' || !target) return;
///     pokemon.addVolatile('iceball');
///     if (move.sourceEffect) pokemon.lastMoveTargetLoc = pokemon.getLocOf(target);
/// }
pub fn on_modify_move(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

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
        pokemon_pokemon.status.as_ref().map(|s| s.as_str()) == Some("slp")
    };

    if has_iceball || is_sleeping {
        return EventResult::Continue;
    }

    // pokemon.addVolatile('iceball');
    battle.add_volatile(pokemon, &ID::from("iceball"), None, None);

    // if (move.sourceEffect) pokemon.lastMoveTargetLoc = pokemon.getLocOf(target);
    if battle.active_move.as_ref().and_then(|m| m.source_effect.as_ref()).is_some() {
        let target_loc = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_pokemon.get_loc_of(target)
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
pub fn on_after_move(battle: &mut Battle, source_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

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
            let hit_count = iceball_volatile.data.get("hitCount").and_then(|v| v.as_i64()).unwrap_or(0);
            let contact_hit_count = iceball_volatile.data.get("contactHitCount").and_then(|v| v.as_i64()).unwrap_or(0);
            (true, hit_count == 5, contact_hit_count)
        } else {
            (false, false, 0)
        }
    };

    if has_iceball && hit_count_is_5 && contact_hit_count < 5 {
        // source.addVolatile("rolloutstorage");
        battle.add_volatile(source, &ID::from("rolloutstorage"), None, None);

        // source.volatiles["rolloutstorage"].contactHitCount =
        // iceballData.contactHitCount;
        let source_pokemon = match battle.pokemon_at_mut(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        if let Some(rollout_volatile) = source_pokemon.volatiles.get_mut(&ID::from("rolloutstorage")) {
            rollout_volatile.data.insert("contactHitCount".to_string(), serde_json::json!(contact_hit_count));
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
        if let Some(ref mut effect_state) = battle.current_effect_state {
            effect_state.data.insert("hitCount".to_string(), serde_json::json!(0));
            effect_state.data.insert("contactHitCount".to_string(), serde_json::json!(0));
        }

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
