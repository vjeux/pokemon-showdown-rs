//! Obstruct Move
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
    let pokemon = pokemon_pos;

    // return !!this.queue.willAct() && this.runEvent('StallMove', pokemon);
    let will_act = battle.queue.will_act().is_some();

    if !will_act {
        return EventResult::Boolean(false);
    }

    // this.runEvent('StallMove', pokemon)
    let stall_result = battle.run_event("StallMove", Some(pokemon), None, None, None);
    EventResult::Boolean(will_act && stall_result.unwrap_or(0) != 0)
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

    // pokemon.addVolatile('stall');
    // Use battle.add_volatile_to_pokemon to properly set duration from dex.conditions
    Pokemon::add_volatile(battle, pokemon_pos, ID::from("stall"), None, None, None);

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
    ///         this.boost({ def: -2 }, source, target, this.dex.getActiveMove("Obstruct"));
    ///     }
    ///     return this.NOT_FAIL;
    /// }
    pub fn on_try_hit(
        battle: &mut Battle,
        source_pos: (usize, usize),
        target_pos: (usize, usize),
    ) -> EventResult {
        use crate::dex_data::ID;

        let source = source_pos;
        let target = target_pos;

        // if (!move.flags['protect'] || move.category === 'Status') {
        let (has_protect_flag, is_status) = {
            let active_move = match &battle.active_move {
                Some(active_move) => active_move,
                None => return EventResult::Continue,
            };
            let has_protect = active_move.flags.protect;
            let is_status = active_move.category == "Status";
            (has_protect, is_status)
        };

        if !has_protect_flag || is_status {
            // if (['gmaxoneblow', 'gmaxrapidflow'].includes(move.id)) return;
            let move_id = {
                match &battle.active_move {
                    Some(active_move) => active_move.id.clone(),
                    None => return EventResult::Continue,
                }
            };

            if move_id == ID::from("gmaxoneblow") || move_id == ID::from("gmaxrapidflow") {
                return EventResult::Continue;
            }

            // if (move.isZ || move.isMax) target.getMoveHitData(move).zBrokeProtect = true;
            let is_z_or_max = {
                let active_move = match &battle.active_move {
                    Some(active_move) => active_move,
                    None => return EventResult::Continue,
                };
                active_move.is_z || active_move.is_max
            };

            if is_z_or_max {
                // target.getMoveHitData(move).zBrokeProtect = true;
                if let Some(hit_data) = battle.get_move_hit_data_mut(target) {
                    hit_data.z_broke_protect = true;
                }
            }

            // return;
            return EventResult::Continue;
        }

        // if (move.smartTarget) {
        let smart_target = {
            match &battle.active_move {
                Some(m) => m.smart_target,
                None => return EventResult::Continue,
            }
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
            // if (source.volatiles['lockedmove'].duration === 2) {
            let duration_is_2 = {
                let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                source_pokemon
                    .volatiles
                    .get(&ID::from("lockedmove"))
                    .and_then(|v| v.duration)
                    .map(|d| d == 2)
                    .unwrap_or(false)
            };

            if duration_is_2 {
                // delete source.volatiles['lockedmove'];
                Pokemon::remove_volatile(battle, source, &ID::from("lockedmove"));
            }
        }

        // if (this.checkMoveMakesContact(move, source, target)) {
        let empty_id = ID::from("");
        let move_id = battle
            .active_move
            .as_ref()
            .map(|m| m.id.clone())
            .unwrap_or(empty_id);
        if battle.check_move_makes_contact(&move_id, source, target, false) {
            // this.boost({ def: -2 }, source, target, this.dex.getActiveMove("Obstruct"));
            battle.boost(&[("def", -2)], source, Some(target), Some("obstruct"), false, false);
        }

        // return this.NOT_FAIL;
        EventResult::NotFail
    }

    /// onHit(target, source, move) {
    ///     if (move.isZOrMaxPowered && this.checkMoveMakesContact(move, source, target)) {
    ///         this.boost({ def: -2 }, source, target, this.dex.getActiveMove("Obstruct"));
    ///     }
    /// }
    pub fn on_hit(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        target_pos: Option<(usize, usize)>,
    ) -> EventResult {
        use crate::dex_data::ID;

        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };
        let source = pokemon_pos;

        // if (move.isZOrMaxPowered && this.checkMoveMakesContact(move, source, target)) {
        let is_z_or_max_powered = {
            let active_move = match &battle.active_move {
                Some(active_move) => active_move,
                None => return EventResult::Continue,
            };
            active_move.is_z || active_move.is_max
        };

        if is_z_or_max_powered {
            let empty_id = ID::from("");
            let move_id = battle
                .active_move
                .as_ref()
                .map(|m| m.id.clone())
                .unwrap_or(empty_id);
            if battle.check_move_makes_contact(&move_id, source, target, false) {
                // this.boost({ def: -2 }, source, target, this.dex.getActiveMove("Obstruct"));
                battle.boost(&[("def", -2)], source, Some(target), Some("obstruct"), false, false);
            }
        }

        EventResult::Continue
    }
}
