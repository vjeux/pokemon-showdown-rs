//! Burning Bulwark Move
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
    if !will_act {
        return EventResult::Boolean(false);
    }

    let stall_move_result = battle.run_event("StallMove", Some(crate::event::EventTarget::Pokemon(pokemon_pos)), None, None, EventResult::Continue, false, false);

    // Convert stall_result to boolean: Boolean(true/false) or Number(!=0) means success
    let stall_success = match stall_move_result {
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
    // pokemon.addVolatile('stall');
    // Use battle.add_volatile_to_pokemon to properly set duration from dex.conditions
    Pokemon::add_volatile(battle, pokemon_pos, ID::from("stall"), None, None, None, None);

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onStart(target) {
    ///     this.add('-singleturn', target, 'move: Protect');
    /// }
    pub fn on_start(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        _source_pos: Option<(usize, usize)>,
        _effect: Option<&crate::battle::Effect>,
    ) -> EventResult {
        // this.add('-singleturn', target, 'move: Protect');
        let target = pokemon_pos;

        let target_ident = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.get_slot()
        };

        battle.add(
            "-singleturn",
            &[target_ident.as_str().into(), "move: Protect".into()],
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
    ///         source.trySetStatus('brn', target);
    ///     }
    ///     return this.NOT_FAIL;
    /// }
    pub fn on_try_hit(
        battle: &mut Battle,
        source_pos: (usize, usize),
        target_pos: (usize, usize),
        _move_id: Option<&str>,
    ) -> EventResult {
        // Get the active move
        let move_id = match &battle.active_move {
            Some(active_move) => active_move.borrow().id.clone(),
            None => return EventResult::Continue,
        };

        let move_data = match battle.dex.moves().get_by_id(&move_id) {
            Some(m) => m,
            None => return EventResult::Continue,
        };

        // if (!move.flags['protect'] || move.category === 'Status') {
        // IMPORTANT: Check the ACTIVE move's runtime flags, not the dex data
        // Abilities like Unseen Fist modify active_move.borrow().flags.protect at runtime
        let (has_protect_flag, move_category) = match &battle.active_move {
            Some(active_move) => {
                let am = active_move.borrow();
                (am.flags.protect, am.category.clone())
            },
            None => (true, String::new()), // Default to having protect if no active move
        };

        if !has_protect_flag || move_category == "Status" {
            // if (['gmaxoneblow', 'gmaxrapidflow'].includes(move.id)) return;
            if move_id.as_str() == "gmaxoneblow" || move_id.as_str() == "gmaxrapidflow" {
                // return;
                return EventResult::Continue;
            }

            // if (move.isZ || move.isMax) target.getMoveHitData(move).zBrokeProtect = true;
            if move_data.is_z.is_some() || move_data.is_max.is_some() {
                // Set zBrokeProtect in move hit data
                battle.with_move_hit_data_mut(target_pos, |hit_data| {
                    hit_data.z_broke_protect = true;
                });
            }

            // return;
            return EventResult::Continue;
        }

        // if (move.smartTarget) {
        //     move.smartTarget = false;
        // } else {
        //     this.add('-activate', target, 'move: Protect');
        // }
        let target_ident = {
            let target_pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.get_slot()
        };

        let has_smart_target = {
            if let Some(ref active_move) = battle.active_move {
                active_move.borrow().smart_target == Some(true)
            } else {
                false
            }
        };

        if has_smart_target {
            if let Some(ref active_move) = battle.active_move {
                active_move.borrow_mut().smart_target = Some(false);
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
            let source_pokemon = match battle.pokemon_at(source_pos.0, source_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };

            if let Some(volatile) = source_pokemon.volatiles.get(&lockedmove_id) {
                volatile.borrow().duration == Some(2)
            } else {
                false
            }
        };

        if should_remove_lockedmove {
            Pokemon::remove_volatile(battle, source_pos, &lockedmove_id);
        }

        // if (this.checkMoveMakesContact(move, source, target)) {
        //     source.trySetStatus('brn', target);
        // }
        // Note: target_pos (the Burning Bulwark user) is the source of the burn status
        // This is important for Synchronize to know who to pass the status back to
        // Use check_move_makes_contact_with_active_move to check the active move's flags
        let active_move_clone = battle.active_move.as_ref().map(|am| am.borrow().clone());
        let makes_contact = battle.check_move_makes_contact_with_active_move(
            active_move_clone.as_ref(),
            source_pos,
            target_pos,
            false,
        );
        if makes_contact {
            Pokemon::try_set_status(battle, source_pos, ID::from("brn"), Some(target_pos), None);
        }

        // return this.NOT_FAIL;
        EventResult::NotFail
    }

    /// onHit(target, source, move) {
    ///     if (move.isZOrMaxPowered && this.checkMoveMakesContact(move, source, target)) {
    ///         source.trySetStatus('brn', target);
    ///     }
    /// }
    pub fn on_hit(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        target_pos: Option<(usize, usize)>,
    ) -> EventResult {
        use crate::dex_data::ID;

        // Get source from target_pos (in condition context, pokemon_pos is the protected pokemon, target is the attacker)
        let source = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // Get the active move - use runtime flags, not dex lookup
        let is_z_or_max_powered = match &battle.active_move {
            Some(active_move) => active_move.borrow().is_z_or_max_powered,
            None => return EventResult::Continue,
        };

        // if (move.isZOrMaxPowered && this.checkMoveMakesContact(move, source, target)) {
        //     source.trySetStatus('brn', target);
        // }
        // Note: is_z_or_max_powered is a runtime flag on ActiveMove, not a dex property
        // pokemon_pos (the Burning Bulwark user) is the source of the burn status
        // Use check_move_makes_contact_with_active_move to check the active move's flags
        if is_z_or_max_powered {
            let active_move_clone = battle.active_move.as_ref().map(|am| am.borrow().clone());
            let makes_contact = battle.check_move_makes_contact_with_active_move(
                active_move_clone.as_ref(),
                source,
                pokemon_pos,
                false,
            );
            if makes_contact {
                Pokemon::try_set_status(battle, source, ID::from("brn"), Some(pokemon_pos), None);
            }
        }

        EventResult::Continue
    }
}
