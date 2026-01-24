//! Silk Trap Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

/// onPrepareHit(pokemon) {
///     return !!this.queue.willAct() && this.runEvent('StallMove', pokemon);
/// }
pub fn on_prepare_hit(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    // onPrepareHit(pokemon) {
    //     return !!this.queue.willAct() && this.runEvent('StallMove', pokemon);
    // }
    let pokemon = pokemon_pos;

    let will_act = battle.queue.will_act().is_some();
    if !will_act {
        return EventResult::Boolean(false);
    }

    let stall_result = battle.run_event("StallMove", Some(crate::event::EventTarget::Pokemon(pokemon)), None, None, EventResult::Continue, false, false);

    // Convert stall_result to boolean: Boolean(true/false) or Number(!=0) means success
    let stall_success = match stall_result {
        EventResult::Boolean(b) => b,
        EventResult::Number(n) => n != 0,
        _ => false,
    };

    EventResult::Boolean(will_act && stall_success)
}

/// onHit(pokemon) {
///     pokemon.addVolatile('stall');
/// }
pub fn on_hit(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    use crate::dex_data::ID;
    use crate::pokemon::Pokemon;

    // onHit(pokemon) {
    //     pokemon.addVolatile('stall');
    // }

    // pokemon.addVolatile('stall');
    // Use battle.add_volatile_to_pokemon to properly set duration from dex.conditions
    Pokemon::add_volatile(battle, pokemon_pos, ID::from("stall"), None, None, None, None);

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onStart(target) {
    ///     this.add('-singleturn', target, 'Protect');
    /// }
    pub fn on_start(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        _source_pos: Option<(usize, usize)>,
        _effect: Option<&crate::battle::Effect>,
    ) -> EventResult {
        // onStart(target) {
        //     this.add('-singleturn', target, 'Protect');
        // }
        let target = pokemon_pos;

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
    pub fn on_try_hit(
        battle: &mut Battle,
        source_pos: (usize, usize),
        target_pos: (usize, usize),
        _move_id: Option<&str>,
    ) -> EventResult {
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
                Some(active_move) => active_move,
                None => return EventResult::Continue,
            };
            let has_protect = active_move.flags.protect;
            let is_status = active_move.category == "Status";
            let is_z = active_move.is_z.is_some();
            let is_max = active_move.is_max.is_some();
            (has_protect, is_status, is_z, is_max)
        };

        if !has_protect_flag || is_status {
            // if (move.isZ || move.isMax) target.getMoveHitData(move).zBrokeProtect = true;
            if is_z || is_max {
                if let Some(hit_data) = battle.get_move_hit_data_mut(target) {
                    hit_data.z_broke_protect = true;
                }
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
            if let Some(ref mut active_move) = battle.active_move {
                active_move.smart_target = Some(false);
            }
        } else {
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
            source_pokemon
                .volatiles
                .contains_key(&ID::from("lockedmove"))
        };

        if has_lockedmove {
            let duration = {
                let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                if let Some(volatile) = source_pokemon.volatiles.get(&ID::from("lockedmove")) {
                    volatile.borrow().duration.unwrap_or(0)
                } else {
                    0
                }
            };

            if duration == 2 {
                Pokemon::remove_volatile(battle, source, &ID::from("lockedmove"));
            }
        }

        // if (this.checkMoveMakesContact(move, source, target)) {
        //     this.boost({ spe: -1 }, source, target, this.dex.getActiveMove("Silk Trap"));
        // }
        // Use check_move_makes_contact_with_active_move to check the active move's flags
        // (which may have been modified by onModifyMove, e.g., skydrop removing contact)
        let active_move = battle.active_move.clone();
        let makes_contact = battle.check_move_makes_contact_with_active_move(
            active_move.as_ref(),
            source,
            target,
            false,
        );
        if makes_contact {
            battle.boost(&[("spe", -1)], source, Some(target), Some("silktrap"), false, false);
        }

        // return this.NOT_FAIL;
        // NOT_FAIL is a special value (empty string '') that means "blocked but doesn't count as failure"
        // This prevents moves like Stomping Tantrum from getting the power boost
        EventResult::NotFail
    }

    /// onHit(target, source, move) {
    ///     if (move.isZOrMaxPowered && this.checkMoveMakesContact(move, source, target)) {
    ///         this.boost({ spe: -1 }, source, target, this.dex.getActiveMove("Silk Trap"));
    ///     }
    /// }
    pub fn on_hit(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        target_pos: Option<(usize, usize)>,
    ) -> EventResult {
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
                Some(active_move) => active_move,
                None => return EventResult::Continue,
            };
            active_move.is_z_or_max_powered
        };

        if is_z_or_max_powered {
            // Use check_move_makes_contact_with_active_move to check the active move's flags
            let active_move = battle.active_move.clone();
            let makes_contact = battle.check_move_makes_contact_with_active_move(
                active_move.as_ref(),
                source,
                target,
                false,
            );
            if makes_contact {
                // this.boost({ spe: -1 }, source, target, this.dex.getActiveMove("Silk Trap"));
                battle.boost(&[("spe", -1)], source, Some(target), Some("silktrap"), false, false);
            }
        }

        EventResult::Continue
    }
}
