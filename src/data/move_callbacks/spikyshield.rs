//! Spiky Shield Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onPrepareHit(pokemon) {
///     return !!this.queue.willAct() && this.runEvent('StallMove', pokemon);
/// }
pub fn on_prepare_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // onPrepareHit(pokemon) {
    //     return !!this.queue.willAct() && this.runEvent('StallMove', pokemon);
    // }
    let pokemon = pokemon_pos;

    // return !!this.queue.willAct() && this.runEvent('StallMove', pokemon);
    // TODO: Implement queue_will_act in Battle
    let will_act = true; // battle.queue_will_act();
    if !will_act {
        return EventResult::Boolean(false);
    }

    let stall_result = battle.run_event("StallMove", Some(pokemon), None, None, None);
    EventResult::Boolean(stall_result != Some(0) && stall_result.is_some())
}

/// onHit(pokemon) {
///     pokemon.addVolatile('stall');
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    // onHit(pokemon) {
    //     pokemon.addVolatile('stall');
    // }
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
    ///     this.add('-singleturn', target, 'move: Protect');
    /// }
    pub fn on_start(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        // onStart(target) {
        //     this.add('-singleturn', target, 'move: Protect');
        // }
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // this.add('-singleturn', target, 'move: Protect');
        let target_arg = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            crate::battle::Arg::from(target_pokemon)
        };

        battle.add("-singleturn", &[
            target_arg,
            "move: Protect".into(),
        ]);

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
    ///     if (this.checkMoveMakesContact(move, source, target)) {
    ///         this.damage(source.baseMaxhp / 8, source, target);
    ///     }
    ///     return this.NOT_FAIL;
    /// }
    pub fn on_try_hit(battle: &mut Battle, source_pos: (usize, usize), target_pos: (usize, usize)) -> EventResult {
        use crate::dex_data::ID;

        // onTryHit(target, source, move) {
        //     if (!move.flags['protect']) {
        //         if (['gmaxoneblow', 'gmaxrapidflow'].includes(move.id)) return;
        //         if (move.isZ || move.isMax) target.getMoveHitData(move).zBrokeProtect = true;
        //         return;
        //     }
        //     if (move.smartTarget) {
        //         move.smartTarget = false;
        //     } else {
        //         this.add('-activate', target, 'move: Protect');
        //     }
        //     const lockedmove = source.getVolatile('lockedmove');
        //     if (lockedmove) {
        //         // Outrage counter is reset
        //         if (source.volatiles['lockedmove'].duration === 2) {
        //             delete source.volatiles['lockedmove'];
        //         }
        //     }
        //     if (this.checkMoveMakesContact(move, source, target)) {
        //         this.damage(source.baseMaxhp / 8, source, target);
        //     }
        //     return this.NOT_FAIL;
        // }
        let target = target_pos;
        let source = source_pos;

        // if (!move.flags['protect']) {
        let (has_protect, move_id, is_z, is_max, smart_target) = {
            let active_move = match &battle.active_move {
                Some(active_move) => active_move,
                None => return EventResult::Continue,
            };
            (
                active_move.flags.protect,
                active_move.id.clone(),
                active_move.is_z,
                active_move.is_max,
                active_move.smart_target,
            )
        };

        if !has_protect {
            // if (['gmaxoneblow', 'gmaxrapidflow'].includes(move.id)) return;
            if move_id == ID::from("gmaxoneblow") || move_id == ID::from("gmaxrapidflow") {
                return EventResult::Continue;
            }

            // if (move.isZ || move.isMax) target.getMoveHitData(move).zBrokeProtect = true;
            if is_z || is_max {
                // TODO: Implement move_hit_data system
                // In JavaScript: target.getMoveHitData(move).zBrokeProtect = true;
                // Needs proper MoveHitData tracking on Pokemon or Battle
                // battle.set_move_hit_data_z_broke_protect(target, true);
            }

            // return;
            return EventResult::Continue;
        }

        // if (move.smartTarget) {
        //     move.smartTarget = false;
        // } else {
        //     this.add('-activate', target, 'move: Protect');
        // }
        if smart_target.unwrap_or(false) {
            if let Some(ref mut active_move) = battle.active_move {
                active_move.smart_target = Some(false);
            }
        } else {
            let target_arg = {
                let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                crate::battle::Arg::from(target_pokemon)
            };

            battle.add("-activate", &[
                target_arg,
                "move: Protect".into(),
            ]);
        }

        // const lockedmove = source.getVolatile('lockedmove');
        // if (lockedmove) {
        //     // Outrage counter is reset
        //     if (source.volatiles['lockedmove'].duration === 2) {
        //         delete source.volatiles['lockedmove'];
        //     }
        // }
        let has_lockedmove_duration_2 = {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };

            if let Some(volatile) = source_pokemon.volatiles.get(&ID::from("lockedmove")) {
                volatile.duration == Some(2)
            } else {
                false
            }
        };

        if has_lockedmove_duration_2 {
            {
                let pokemon = match battle.pokemon_at_mut(source.0, source.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                pokemon.remove_volatile(&ID::from("lockedmove"));
            }
        }

        // if (this.checkMoveMakesContact(move, source, target)) {
        //     this.damage(source.baseMaxhp / 8, source, target);
        // }
        let makes_contact = battle.check_move_makes_contact(&move_id, source);
        if makes_contact {
            let base_max_hp = {
                let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                source_pokemon.base_maxhp
            };

            battle.damage(base_max_hp / 8, Some(source), None, None, false);
        }

        // return this.NOT_FAIL;
        EventResult::NotFail
    }

    /// onHit(target, source, move) {
    ///     if (move.isZOrMaxPowered && this.checkMoveMakesContact(move, source, target)) {
    ///         this.damage(source.baseMaxhp / 8, source, target);
    ///     }
    /// }
    pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
        // onHit(target, source, move) {
        //     if (move.isZOrMaxPowered && this.checkMoveMakesContact(move, source, target)) {
        //         this.damage(source.baseMaxhp / 8, source, target);
        //     }
        // }
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };
        let source = pokemon_pos;

        // if (move.isZOrMaxPowered && this.checkMoveMakesContact(move, source, target)) {
        let (is_z_or_max_powered, move_id) = {
            let active_move = match &battle.active_move {
                Some(active_move) => active_move,
                None => return EventResult::Continue,
            };
            (active_move.is_z_or_max_powered, active_move.id.clone())
        };

        let makes_contact = battle.check_move_makes_contact(&move_id, source);

        if is_z_or_max_powered && makes_contact {
            // this.damage(source.baseMaxhp / 8, source, target);
            let base_max_hp = {
                let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                source_pokemon.base_maxhp
            };

            battle.damage(base_max_hp / 8, Some(source), None, None, false);
        }

        EventResult::Continue
    }
}
