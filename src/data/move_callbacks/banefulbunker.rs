//! Baneful Bunker Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::dex_data::ID;
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
    // return !!this.queue.willAct() && this.runEvent('StallMove', pokemon);
    let will_act = battle.queue.will_act().is_some();
    let stall_move_result = battle.run_event("StallMove", Some(pokemon_pos), None, None, None);

    // Convert Option<i32> to bool: Some(x) where x != 0 is true
    let stall_move_ok = stall_move_result.unwrap_or(0) != 0;
    let result = will_act && stall_move_ok;
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
    // pokemon.addVolatile('stall');
    // Use battle.add_volatile_to_pokemon to properly set duration from dex.conditions
    Pokemon::add_volatile(battle, pokemon_pos, ID::from("stall"), None, None, None);

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onStart(target) {
    ///     this.add('-singleturn', target, 'move: Protect');
    /// }
    pub fn on_start(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        // Get target
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // Get target pokemon identifier
        let target_ident = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.get_slot()
        };

        // this.add('-singleturn', target, 'move: Protect');
        battle.add(
            "-singleturn",
            &[target_ident.as_str().into(), "move: Protect".into()],
        );

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
    ///         source.trySetStatus('psn', target);
    ///     }
    ///     return this.NOT_FAIL;
    /// }
    pub fn on_try_hit(
        battle: &mut Battle,
        source_pos: (usize, usize),
        target_pos: (usize, usize),
    ) -> EventResult {
        // Get the active move
        let move_id = match &battle.active_move {
            Some(active_move) => active_move.id.clone(),
            None => return EventResult::Continue,
        };

        // Get the move data
        let move_data = match battle.dex.moves().get_by_id(&move_id) {
            Some(m) => m,
            None => return EventResult::Continue,
        };

        // if (!move.flags['protect']) {
        if !move_data.flags.contains_key("protect") {
            // if (['gmaxoneblow', 'gmaxrapidflow'].includes(move.id)) return;
            if move_id == ID::from("gmaxoneblow") || move_id == ID::from("gmaxrapidflow") {
                return EventResult::Continue;
            }

            // if (move.isZ || move.isMax) target.getMoveHitData(move).zBrokeProtect = true;
            if move_data.is_z.is_some() || move_data.is_max.is_some() {
                // Set zBrokeProtect in move hit data
                if let Some(hit_data) = battle.get_move_hit_data_mut(target_pos) {
                    hit_data.z_broke_protect = true;
                }
            }
            // return;
            return EventResult::Continue;
        }

        // Get target pokemon identifier
        let target_ident = {
            let target_pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.get_slot()
        };

        // if (move.smartTarget) {
        //     move.smartTarget = false;
        // } else {
        //     this.add('-activate', target, 'move: Protect');
        // }
        let has_smart_target = {
            if let Some(ref active_move) = battle.active_move {
                active_move.smart_target == Some(true)
            } else {
                false
            }
        };

        if has_smart_target {
            if let Some(ref mut active_move) = battle.active_move {
                active_move.smart_target = Some(false);
            }
        } else {
            battle.add(
                "-activate",
                &[target_ident.as_str().into(), "move: Protect".into()],
            );
        }

        // const lockedmove = source.getVolatile('lockedmove');
        // if (lockedmove) {
        //     // Outrage counter is reset
        //     if (source.volatiles['lockedmove'].duration === 2) {
        //         delete source.volatiles['lockedmove'];
        //     }
        // }
        let lockedmove_id = ID::from("lockedmove");
        let should_remove_lockedmove = {
            let source_pokemon = battle.pokemon_at(source_pos.0, source_pos.1);
            if let Some(pokemon) = source_pokemon {
                if let Some(volatile) = pokemon.volatiles.get(&lockedmove_id) {
                    volatile.duration == Some(2)
                } else {
                    false
                }
            } else {
                false
            }
        };

        if should_remove_lockedmove {
            Pokemon::remove_volatile(battle, source_pos, &lockedmove_id);
        }

        // if (this.checkMoveMakesContact(move, source, target)) {
        //     source.trySetStatus('psn', target);
        // }
        if battle.check_move_makes_contact(&move_id, source_pos, target_pos, false) {
            Pokemon::try_set_status(battle, source_pos, ID::from("psn"), None);
        }

        // return this.NOT_FAIL;
        EventResult::NotFail
    }

    /// onHit(target, source, move) {
    ///     if (move.isZOrMaxPowered && this.checkMoveMakesContact(move, source, target)) {
    ///         source.trySetStatus('psn', target);
    ///     }
    /// }
    pub fn on_hit(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        target_pos: Option<(usize, usize)>,
    ) -> EventResult {
        // Get source from target_pos (in condition context, pokemon_pos is the protected pokemon, target is the attacker)
        let source = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // Get the active move
        let move_id = match &battle.active_move {
            Some(active_move) => active_move.id.clone(),
            None => return EventResult::Continue,
        };

        // Get the move data
        let move_data = match battle.dex.moves().get_by_id(&move_id) {
            Some(m) => m,
            None => return EventResult::Continue,
        };

        // if (move.isZOrMaxPowered && this.checkMoveMakesContact(move, source, target)) {
        //     source.trySetStatus('psn', target);
        // }
        if move_data.is_z_or_max_powered && battle.check_move_makes_contact(&move_id, source, pokemon_pos, false) {
            let _source_pokemon = match battle.pokemon_at_mut(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            Pokemon::try_set_status(battle, pokemon_pos, ID::from("psn"), None);
        }

        EventResult::Continue
    }
}
