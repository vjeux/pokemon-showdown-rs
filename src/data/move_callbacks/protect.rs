//! Protect Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onPrepareHit(pokemon) {
///     return !!this.queue.willAct() && this.runEvent('StallMove', pokemon);
/// }
pub fn on_prepare_hit(battle: &mut Battle, pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    let pokemon = pokemon_pos;

    // return !!this.queue.willAct() && this.runEvent('StallMove', pokemon);
    // TODO: Implement queue_will_act in Battle
    let will_act = true; // battle.queue_will_act();

    if !will_act {
        return EventResult::Boolean(false);
    }

    let stall_result = battle.run_event("StallMove", Some(pokemon), None, None, None);

    if stall_result == Some(0) {
        return EventResult::Boolean(false);
    }

    EventResult::Boolean(true)
}

/// onHit(pokemon) {
///     pokemon.addVolatile('stall');
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    let pokemon = pokemon_pos;

    // pokemon.addVolatile('stall');
    {

        let pokemon = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {

            Some(p) => p,

            None => return EventResult::Continue,

        };

        pokemon.add_volatile(ID::from("stall"));

    }

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onStart(target) {
    ///     this.add('-singleturn', target, 'Protect');
    /// }
    pub fn on_start(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // this.add('-singleturn', target, 'Protect');
        let target_arg = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.get_slot()
        };

        battle.add("-singleturn", &[target_arg.into(), "Protect".into()]);

        EventResult::Continue
    }

    /// onTryHit(target, source, move) {
    ///     if (!move.flags['protect']) {
    ///         if (['gmaxoneblow', 'gmaxrapidflow'].includes(move.id)) return;
    ///         if (move.isZ || move.isMax) target.getMoveHitData(move).zBrokeProtect = true;
    ///         return;
    ///     }
    ///     if (move.smartTarget) {
    ///         move.smartTarget = false;
    ///     } else {
    ///         this.add('-activate', target, 'move: Protect');
    ///     }
    ///     const lockedmove = source.getVolatile('lockedmove');
    ///     if (lockedmove) {
    ///         // Outrage counter is reset
    ///         if (source.volatiles['lockedmove'].duration === 2) {
    ///             delete source.volatiles['lockedmove'];
    ///         }
    ///     }
    ///     return this.NOT_FAIL;
    /// }
    pub fn on_try_hit(battle: &mut Battle, source_pos: (usize, usize), target_pos: (usize, usize)) -> EventResult {
        use crate::dex_data::ID;

        let source = source_pos;
        let target = target_pos;

        // if (!move.flags['protect']) {
        let has_protect_flag = {
            let active_move = match &battle.active_move {
                Some(active_move) => active_move,
                None => return EventResult::Continue,
            };
            active_move.flags.protect
        };

        if !has_protect_flag {
            // if (['gmaxoneblow', 'gmaxrapidflow'].includes(move.id)) return;
            let move_id = {
                let active_move = match &battle.active_move {
                    Some(active_move) => active_move,
                    None => return EventResult::Continue,
                };
                active_move.clone()
            };

            if move_id.id == ID::from("gmaxoneblow") || move_id.id == ID::from("gmaxrapidflow") {
                return EventResult::Continue;
            }

            // if (move.isZ || move.isMax) target.getMoveHitData(move).zBrokeProtect = true;
            let (is_z, is_max) = {
                let active_move = match &battle.active_move {
                    Some(active_move) => active_move,
                    None => return EventResult::Continue,
                };
                (active_move.is_z, active_move.is_max)
            };

            if is_z || is_max {
                let _move_id = {
                    let active_move = match &battle.active_move {
                        Some(active_move) => active_move,
                        None => return EventResult::Continue,
                    };
                    active_move.clone()
                };

                // Set z_broke_protect on the target pokemon's move hit data
                // TODO: MoveHitData doesn't have z_broke_protect field yet
                // Need to add this field or track it differently
                // let target_pokemon = match battle.pokemon_at_mut(target.0, target.1) {
                //     Some(p) => p,
                //     None => return EventResult::Continue,
                // };
                // target_pokemon.get_move_hit_data(&move_id.id).z_broke_protect = true;
            }

            // return;
            return EventResult::Continue;
        }

        // if (move.smartTarget) {
        //     move.smartTarget = false;
        // } else {
        //     this.add('-activate', target, 'move: Protect');
        // }
        let smart_target = {
            let active_move = match &battle.active_move {
                Some(active_move) => active_move,
                None => return EventResult::Continue,
            };
            active_move.smart_target
        };

        if smart_target.unwrap_or(false) {
            // move.smartTarget = false;
            if let Some(ref mut active_move) = battle.active_move {
                active_move.smart_target = Some(false);
            }
        } else {
            // this.add('-activate', target, 'move: Protect');
            let target_arg = {
                let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                target_pokemon.get_slot()
            };

            battle.add("-activate", &[target_arg.into(), "move: Protect".into()]);
        }

        // const lockedmove = source.getVolatile('lockedmove');
        // if (lockedmove) {
        //     // Outrage counter is reset
        //     if (source.volatiles['lockedmove'].duration === 2) {
        //         delete source.volatiles['lockedmove'];
        //     }
        // }
        let has_lockedmove = {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            source_pokemon.volatiles.contains_key(&ID::from("lockedmove"))
        };

        if has_lockedmove {
            let duration = {
                let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                source_pokemon.volatiles.get(&ID::from("lockedmove"))
                    .and_then(|v| v.duration)
                    .unwrap_or(0)
            };

            if duration == 2 {
                // delete source.volatiles['lockedmove'];
                {
                    let pokemon = match battle.pokemon_at_mut(source.0, source.1) {
                        Some(p) => p,
                        None => return EventResult::Continue,
                    };
                    pokemon.remove_volatile(&ID::from("lockedmove"));
                }
            }
        }

        // return this.NOT_FAIL;
        EventResult::NotFail
    }
}
