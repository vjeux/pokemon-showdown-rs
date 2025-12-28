//! Burning Bulwark Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::event::EventResult;
use crate::dex_data::ID;

/// onPrepareHit(pokemon) {
///     return !!this.queue.willAct() && this.runEvent('StallMove', pokemon);
/// }
pub fn on_prepare_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // return !!this.queue.willAct() && this.runEvent('StallMove', pokemon);
    let will_act = battle.queue.will_act();
    if !will_act {
        return EventResult::Boolean(false);
    }

    let stall_move_result = battle.run_event("StallMove", Some(pokemon_pos), None, None);
    EventResult::Boolean(stall_move_result)
}

/// onHit(pokemon) {
///     pokemon.addVolatile('stall');
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // pokemon.addVolatile('stall');
    let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    pokemon.add_volatile(ID::from("stall"));

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onStart(target) {
    ///     this.add('-singleturn', target, 'move: Protect');
    /// }
    pub fn on_start(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        // this.add('-singleturn', target, 'move: Protect');
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        let target_arg = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            Arg::from(target_pokemon)
        };

        battle.add("-singleturn", &[target_arg, "move: Protect".into()]);

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
    ///         source.trySetStatus('brn', target);
    ///     }
    ///     return this.NOT_FAIL;
    /// }
    pub fn on_try_hit(battle: &mut Battle, source_pos: (usize, usize), target_pos: (usize, usize)) -> EventResult {
        // Get the active move
        let move_id = match &battle.active_move {
            Some(id) => id.clone(),
            None => return EventResult::Continue,
        };

        let move_data = match battle.dex.get_move_by_id(&move_id) {
            Some(m) => m,
            None => return EventResult::Continue,
        };

        // if (!move.flags['protect'] || move.category === 'Status') {
        if !move_data.flags.contains_key("protect") || move_data.category == "Status" {
            // if (['gmaxoneblow', 'gmaxrapidflow'].includes(move.id)) return;
            if move_id.as_str() == "gmaxoneblow" || move_id.as_str() == "gmaxrapidflow" {
                // return;
                return EventResult::Continue;
            }

            // if (move.isZ || move.isMax) target.getMoveHitData(move).zBrokeProtect = true;
            // TODO: isZ, isMax, getMoveHitData not yet implemented

            // return;
            return EventResult::Continue;
        }

        // if (move.smartTarget) {
        //     move.smartTarget = false;
        // } else {
        //     this.add('-activate', target, 'move: Protect');
        // }
        // TODO: smartTarget not yet implemented, always add activate
        let (target_arg, source_arg) = {
            let target_pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            let source_pokemon = match battle.pokemon_at(source_pos.0, source_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            (Arg::from(target_pokemon), Arg::from(source_pokemon))
        };

        battle.add("-activate", &[target_arg, "move: Protect".into()]);

        // const lockedmove = source.getVolatile('lockedmove');
        // if (lockedmove) {
        //     // Outrage counter is reset
        //     if (source.volatiles['lockedmove'].duration === 2) {
        //         delete source.volatiles['lockedmove'];
        //     }
        // }
        let lockedmove_id = ID::from("lockedmove");
        let should_remove_lockedmove = {
            let source_pokemon = match battle.pokemon_at(source_pos.0, source_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };

            if let Some(volatile) = source_pokemon.volatiles.get(&lockedmove_id) {
                volatile.duration == 2
            } else {
                false
            }
        };

        if should_remove_lockedmove {
            let source_pokemon = match battle.pokemon_at_mut(source_pos.0, source_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            source_pokemon.remove_volatile(&lockedmove_id);
        }

        // if (this.checkMoveMakesContact(move, source, target)) {
        //     source.trySetStatus('brn', target);
        // }
        if battle.check_move_makes_contact(&move_id, source_pos, target_pos) {
            let source_pokemon = match battle.pokemon_at_mut(source_pos.0, source_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            source_pokemon.try_set_status(ID::from("brn"));
        }

        // return this.NOT_FAIL;
        EventResult::NotFail
    }

    /// onHit(target, source, move) {
    ///     if (move.isZOrMaxPowered && this.checkMoveMakesContact(move, source, target)) {
    ///         source.trySetStatus('brn', target);
    ///     }
    /// }
    pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
        // Get source and target
        let (source_pos, target) = match target_pos {
            Some(t) => (pokemon_pos, t),
            None => return EventResult::Continue,
        };

        // Get the active move
        let move_id = match &battle.active_move {
            Some(id) => id.clone(),
            None => return EventResult::Continue,
        };

        // if (move.isZOrMaxPowered && this.checkMoveMakesContact(move, source, target)) {
        //     source.trySetStatus('brn', target);
        // }
        // TODO: isZOrMaxPowered not yet implemented
        // For now, skip this callback

        EventResult::Continue
    }
}
