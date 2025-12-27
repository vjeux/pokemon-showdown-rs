//! Silk Trap Move
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

    let will_act = battle.queue_will_act();
    if !will_act {
        return EventResult::Bool(false);
    }

    let stall_result = battle.run_event("StallMove", pokemon, None, None);
    EventResult::Bool(stall_result)
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

    battle.add_volatile(&ID::from("stall"), pokemon, None, None);

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onStart(target) {
    ///     this.add('-singleturn', target, 'Protect');
    /// }
    pub fn on_start(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        // onStart(target) {
        //     this.add('-singleturn', target, 'Protect');
        // }
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        let target_arg = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            crate::battle::Arg::from(target_pokemon)
        };

        battle.add("-singleturn", &[
            target_arg,
            "Protect".into(),
        ]);

        EventResult::Continue
    }

    /// onTryHit(target, source, move) {
    ///     if (!move.flags['protect'] || move.category === 'Status') {
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
    ///         this.boost({ spe: -1 }, source, target, this.dex.getActiveMove("Silk Trap"));
    ///     }
    ///     return this.NOT_FAIL;
    /// }
    pub fn on_try_hit(battle: &mut Battle, source_pos: (usize, usize), target_pos: (usize, usize)) -> EventResult {
        use crate::dex_data::ID;

        // onTryHit(target, source, move) {
        //     if (!move.flags['protect'] || move.category === 'Status') {
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
        //         this.boost({ spe: -1 }, source, target, this.dex.getActiveMove("Silk Trap"));
        //     }
        //     return this.NOT_FAIL;
        // }
        let target = target_pos;
        let source = source_pos;

        // if (!move.flags['protect'] || move.category === 'Status') {
        let (has_protect_flag, is_status, is_z, is_max) = {
            let active_move = match &battle.active_move {
                Some(m) => m,
                None => return EventResult::Continue,
            };
            let has_protect = active_move.flags.protect.unwrap_or(0) != 0;
            let is_status = active_move.category == "Status";
            let is_z = active_move.is_z;
            let is_max = active_move.is_max;
            (has_protect, is_status, is_z, is_max)
        };

        if !has_protect_flag || is_status {
            // if (move.isZ || move.isMax) target.getMoveHitData(move).zBrokeProtect = true;
            if is_z || is_max {
                battle.set_move_hit_data_z_broke_protect(target, true);
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
                Some(m) => m,
                None => return EventResult::Continue,
            };
            active_move.smart_target
        };

        if smart_target {
            if let Some(ref mut active_move) = battle.active_move {
                active_move.smart_target = false;
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
                if let Some(volatile) = source_pokemon.volatiles.get(&ID::from("lockedmove")) {
                    volatile.duration.unwrap_or(0)
                } else {
                    0
                }
            };

            if duration == 2 {
                battle.remove_volatile(&ID::from("lockedmove"), source);
            }
        }

        // if (this.checkMoveMakesContact(move, source, target)) {
        //     this.boost({ spe: -1 }, source, target, this.dex.getActiveMove("Silk Trap"));
        // }
        let makes_contact = battle.check_move_makes_contact(source, target);
        if makes_contact {
            let mut boosts = std::collections::HashMap::new();
            boosts.insert("spe".to_string(), -1);
            battle.boost(&boosts, source, Some(target), Some(&ID::from("silktrap")));
        }

        // return this.NOT_FAIL;
        EventResult::Bool(false)
    }

    /// onHit(target, source, move) {
    ///     if (move.isZOrMaxPowered && this.checkMoveMakesContact(move, source, target)) {
    ///         this.boost({ spe: -1 }, source, target, this.dex.getActiveMove("Silk Trap"));
    ///     }
    /// }
    pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
        use crate::dex_data::ID;

        // onHit(target, source, move) {
        //     if (move.isZOrMaxPowered && this.checkMoveMakesContact(move, source, target)) {
        //         this.boost({ spe: -1 }, source, target, this.dex.getActiveMove("Silk Trap"));
        //     }
        // }
        let target = pokemon_pos;
        let source = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // if (move.isZOrMaxPowered && this.checkMoveMakesContact(move, source, target)) {
        let is_z_or_max_powered = {
            let active_move = match &battle.active_move {
                Some(m) => m,
                None => return EventResult::Continue,
            };
            active_move.is_z_or_max_powered
        };

        if is_z_or_max_powered {
            let makes_contact = battle.check_move_makes_contact(source, target);
            if makes_contact {
                // this.boost({ spe: -1 }, source, target, this.dex.getActiveMove("Silk Trap"));
                let mut boosts = std::collections::HashMap::new();
                boosts.insert("spe".to_string(), -1);
                battle.boost(&boosts, source, Some(target), Some(&ID::from("silktrap")));
            }
        }

        EventResult::Continue
    }
}
