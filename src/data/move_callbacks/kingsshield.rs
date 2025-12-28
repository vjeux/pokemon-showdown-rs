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
pub fn on_prepare_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let pokemon = pokemon_pos;

    // return !!this.queue.willAct() && this.runEvent('StallMove', pokemon);
    let will_act = battle.queue.will_act();
    if !will_act {
        return EventResult::Boolean(false);
    }

    let stall_result = battle.run_event("StallMove", Some(pokemon), None, None, None);
    EventResult::Boolean(will_act && stall_result.unwrap_or(0) != 0)
}

/// onHit(pokemon) {
///     pokemon.addVolatile('stall');
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    let pokemon = pokemon_pos;

    // pokemon.addVolatile('stall');
    battle.add_volatile(pokemon, &ID::from("stall"), None, None);

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
        let target_arg = crate::battle::Arg::Pos(target.0, target.1);
        battle.add("-singleturn", &[target_arg, "Protect".into()]);

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
    pub fn on_try_hit(battle: &mut Battle, source_pos: (usize, usize), target_pos: (usize, usize)) -> EventResult {
        use crate::dex_data::ID;

        let source = source_pos;
        let target = target_pos;

        // if (!move.flags['protect'] || move.category === 'Status') {
        let (has_protect, is_status, move_id, is_z, is_max) = {
            if let Some(ref active_move) = battle.active_move {
                (
                    active_move.flags.protect,
                    active_move.category == crate::move_types::MoveCategory::Status,
                    active_move.clone(),
                    active_move.is_z,
                    active_move.is_max
                )
            } else {
                return EventResult::Continue;
            }
        };

        if !has_protect || is_status {
            //     if (['gmaxoneblow', 'gmaxrapidflow'].includes(move.id)) return;
            if move_id == ID::from("gmaxoneblow") || move_id == ID::from("gmaxrapidflow") {
                return EventResult::Continue;
            }

            //     if (move.isZ || move.isMax) target.getMoveHitData(move).zBrokeProtect = true;
            if is_z || is_max {
                let target_pokemon = match battle.pokemon_at_mut(target.0, target.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                if let Some(ref move_id) = battle.active_move.as_ref().map(|m| m.clone()) {
                    if let Some(hit_data) = target_pokemon.get_move_hit_data_mut(move_id) {
                        hit_data.z_broke_protect = true;
                    }
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
        let smart_target = battle.active_move.as_ref().map(|m| m.smart_target).unwrap_or(false);
        if smart_target {
            if let Some(ref mut active_move) = battle.active_move {
                active_move.smart_target = false;
            }
        } else {
            let target_arg = crate::battle::Arg::Pos(target.0, target.1);
            battle.add("-activate", &[target_arg, "move: Protect".into()]);
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
            if let Some(lockedmove_volatile) = source_pokemon.volatiles.get(&ID::from("lockedmove")) {
                lockedmove_volatile.duration == 2
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
        let makes_contact = battle.check_move_makes_contact(None, source, target);
        if makes_contact {
            let mut boosts = std::collections::HashMap::new();
            boosts.insert("atk".to_string(), -1);
            battle.boost(boosts, source, Some(target), Some(&ID::from("kingsshield")));
        }

        //     return this.NOT_FAIL;
        EventResult::NotFail
    }

    /// onHit(target, source, move) {
    ///     if (move.isZOrMaxPowered && this.checkMoveMakesContact(move, source, target)) {
    ///         this.boost({ atk: -1 }, source, target, this.dex.getActiveMove("King's Shield"));
    ///     }
    /// }
    pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
        use crate::dex_data::ID;

        let target = pokemon_pos;
        let source = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // if (move.isZOrMaxPowered && this.checkMoveMakesContact(move, source, target)) {
        //     this.boost({ atk: -1 }, source, target, this.dex.getActiveMove("King's Shield"));
        // }
        let is_z_or_max_powered = battle.active_move.as_ref().map(|m| m.is_z_or_max_powered).unwrap_or(false);

        if is_z_or_max_powered {
            let makes_contact = battle.check_move_makes_contact(None, source, target);
            if makes_contact {
                let mut boosts = std::collections::HashMap::new();
                boosts.insert("atk".to_string(), -1);
                battle.boost(boosts, source, Some(target), Some(&ID::from("kingsshield")));
            }
        }

        EventResult::Continue
    }
}
