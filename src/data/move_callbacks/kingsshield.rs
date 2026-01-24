//! King's Shield Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onPrepareHit(pokemon) {
///     return !!this.queue.willAct() && this.runEvent('StallMove', pokemon);
/// }
pub fn on_prepare_hit(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    debug_elog!("[KINGSSHIELD::ON_PREPARE_HIT] Called for pokemon {:?}", pokemon_pos);

    let pokemon = pokemon_pos;

    // return !!this.queue.willAct() && this.runEvent('StallMove', pokemon);
    let will_act = battle.queue.will_act().is_some();
    debug_elog!("[KINGSSHIELD::ON_PREPARE_HIT] will_act={}", will_act);

    if !will_act {
        debug_elog!("[KINGSSHIELD::ON_PREPARE_HIT] Returning false (will_act=false)");
        return EventResult::Boolean(false);
    }

    debug_elog!("[KINGSSHIELD::ON_PREPARE_HIT] Calling run_event StallMove");
    let stall_result = battle.run_event("StallMove", Some(crate::event::EventTarget::Pokemon(pokemon)), None, None, EventResult::Continue, false, false);
    debug_elog!("[KINGSSHIELD::ON_PREPARE_HIT] StallMove result: {:?}", stall_result);

    // Convert stall_result to boolean: Boolean(true/false) or Number(!=0) means success
    let stall_success = match stall_result {
        EventResult::Boolean(b) => b,
        EventResult::Number(n) => n != 0,
        _ => false,
    };
    let result = will_act && stall_success;
    debug_elog!("[KINGSSHIELD::ON_PREPARE_HIT] Returning {}", result);
    EventResult::Boolean(result)
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

    debug_elog!("[KINGSSHIELD::ON_HIT] Called with pokemon_pos={:?}", pokemon_pos);

    let pokemon = pokemon_pos;
    let stall_id = ID::from("stall");

    // JavaScript: pokemon.addVolatile('stall');
    // NOTE: add_volatile handles both cases:
    // - If volatile exists: calls on_restart
    // - If volatile doesn't exist: adds volatile and calls on_start
    // We should NOT manually check for stall existence here!
    Pokemon::add_volatile(battle, pokemon, stall_id.clone(), None, None, None, None);

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
        let target = pokemon_pos;

        // this.add('-singleturn', target, 'Protect');
        let target_ident = {
            let pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,

                None => return EventResult::Continue,
            };

            pokemon.get_slot()
        };
        battle.add(
            "-singleturn",
            &[target_ident.as_str().into(), "Protect".into()],
        );

        EventResult::Continue
    }

    /// onTryHit(target, source, move) {
    ///     if (!move.flags['protect'] || move.category === 'Status') {
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
    ///         this.boost({ atk: -1 }, source, target, this.dex.getActiveMove("King's Shield"));
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

        debug_elog!("[KINGSSHIELD::CONDITION::ON_TRY_HIT] Called with source={:?}, target={:?}", source_pos, target_pos);

        let source = source_pos;
        let target = target_pos;

        // if (!move.flags['protect'] || move.category === 'Status') {
        let (has_protect, is_status, move_id, is_z, is_max) = {
            if let Some(ref active_move) = battle.active_move {
                debug_elog!("[KINGSSHIELD::CONDITION::ON_TRY_HIT] active_move exists: {}", active_move.id);
                (
                    active_move.flags.protect,
                    active_move.category == "Status",
                    active_move.clone(),
                    active_move.is_z.is_some(),
                    active_move.is_max.is_some(),
                )
            } else {
                debug_elog!("[KINGSSHIELD::CONDITION::ON_TRY_HIT] No active_move, returning Continue");
                return EventResult::Continue;
            }
        };

        debug_elog!("[KINGSSHIELD::CONDITION::ON_TRY_HIT] has_protect={}, is_status={}", has_protect, is_status);

        if !has_protect || is_status {
            //     if (['gmaxoneblow', 'gmaxrapidflow'].includes(move.id)) return;
            if move_id.id == ID::from("gmaxoneblow") || move_id.id == ID::from("gmaxrapidflow") {
                return EventResult::Continue;
            }

            //     if (move.isZ || move.isMax) target.getMoveHitData(move).zBrokeProtect = true;
            if is_z || is_max {
                // Set zBrokeProtect in move hit data
                if let Some(hit_data) = battle.get_move_hit_data_mut(target) {
                    hit_data.z_broke_protect = true;
                }
            }

            //     return;
            return EventResult::Continue;
        }

        //     if (move.smartTarget) {
        //         move.smartTarget = false;
        //     } else {
        //         this.add('-activate', target, 'move: Protect');
        //     }
        let smart_target = battle
            .active_move
            .as_ref()
            .and_then(|m| m.smart_target)
            .unwrap_or(false);
        if smart_target {
            if let Some(ref mut active_move) = battle.active_move {
                active_move.smart_target = Some(false);
            }
        } else {
            let target_ident = {
                let pokemon = match battle.pokemon_at(target.0, target.1) {
                    Some(p) => p,

                    None => return EventResult::Continue,
                };

                pokemon.get_slot()
            };
            battle.add(
                "-activate",
                &[target_ident.as_str().into(), "move: Protect".into()],
            );
        }

        //     const lockedmove = source.getVolatile('lockedmove');
        //     if (lockedmove) {
        //         // Outrage counter is reset
        //         if (source.volatiles['lockedmove'].duration === 2) {
        //             delete source.volatiles['lockedmove'];
        //         }
        //     }
        let should_remove_lockedmove = {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::NotFail,
            };
            if let Some(lockedmove_volatile) = source_pokemon.volatiles.get(&ID::from("lockedmove"))
            {
                lockedmove_volatile.borrow().duration == Some(2)
            } else {
                false
            }
        };

        if should_remove_lockedmove {
            let source_pokemon = match battle.pokemon_at_mut(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::NotFail,
            };
            source_pokemon.volatiles.remove(&ID::from("lockedmove"));
        }

        //     if (this.checkMoveMakesContact(move, source, target)) {
        //         this.boost({ atk: -1 }, source, target, this.dex.getActiveMove("King's Shield"));
        //     }
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
            battle.boost(&[("atk", -1)], source, Some(target), Some("kingsshield"), false, false);
        }

        debug_elog!("[KINGSSHIELD::CONDITION::ON_TRY_HIT] Returning NotFail");
        //     return this.NOT_FAIL;
        EventResult::NotFail
    }

    /// onHit(target, source, move) {
    ///     if (move.isZOrMaxPowered && this.checkMoveMakesContact(move, source, target)) {
    ///         this.boost({ atk: -1 }, source, target, this.dex.getActiveMove("King's Shield"));
    ///     }
    /// }
    pub fn on_hit(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        target_pos: Option<(usize, usize)>,
    ) -> EventResult {
        

        debug_elog!("[KINGSSHIELD::CONDITION::ON_HIT] Called with pokemon_pos={:?}, target_pos={:?}", pokemon_pos, target_pos);

        let target = pokemon_pos;
        let source = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // if (move.isZOrMaxPowered && this.checkMoveMakesContact(move, source, target)) {
        //     this.boost({ atk: -1 }, source, target, this.dex.getActiveMove("King's Shield"));
        // }
        let is_z_or_max_powered = battle
            .active_move
            .as_ref()
            .map(|m| m.is_z_or_max_powered)
            .unwrap_or(false);

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
                battle.boost(&[("atk", -1)], source, Some(target), Some("kingsshield"), false, false);
            }
        }

        EventResult::Continue
    }
}
