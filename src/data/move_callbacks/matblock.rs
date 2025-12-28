//! Mat Block Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTry(source) {
///     if (source.activeMoveActions > 1) {
///         this.hint("Mat Block only works on your first turn out.");
///         return false;
///     }
///     return !!this.queue.willAct();
/// }
pub fn on_try(battle: &mut Battle, source_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let source = source_pos;

    // if (source.activeMoveActions > 1) {
    //     this.hint("Mat Block only works on your first turn out.");
    //     return false;
    // }
    let active_move_actions = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.active_move_actions
    };

    if active_move_actions > 1 {
        battle.hint("Mat Block only works on your first turn out.", true, None);
        return EventResult::Boolean(false);
    }

    // return !!this.queue.willAct();
    let will_act = battle.queue.will_act();
    EventResult::Boolean(will_act)
}

pub mod condition {
    use super::*;

    /// onSideStart(target, source) {
    ///     this.add('-singleturn', source, 'Mat Block');
    /// }
    pub fn on_side_start(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>) -> EventResult {
        let source = match source_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // this.add('-singleturn', source, 'Mat Block');
        let source_arg = {

            let pokemon = match battle.pokemon_at(source.0, source.1) {

                Some(p) => p,

                None => return EventResult::Continue,

            };

            crate::battle::Arg::from(pokemon)

        };
        battle.add("-singleturn", &[source_arg, "Mat Block".into()]);

        EventResult::Continue
    }

    /// onTryHit(target, source, move) {
    ///     if (!move.flags['protect']) {
    ///         if (['gmaxoneblow', 'gmaxrapidflow'].includes(move.id)) return;
    ///         if (move.isZ || move.isMax) target.getMoveHitData(move).zBrokeProtect = true;
    ///         return;
    ///     }
    ///     if (move && (move.target === 'self' || move.category === 'Status')) return;
    ///     this.add('-activate', target, 'move: Mat Block', move.name);
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
        let has_protect_flag = battle.active_move.as_ref()
            .map(|m| m.flags.protect)
            .unwrap_or(false);

        if !has_protect_flag {
            // if (['gmaxoneblow', 'gmaxrapidflow'].includes(move.id)) return;
            let move_id = battle.active_move.as_ref().map(|m| m.id.as_str()).unwrap_or("");
            if move_id == "gmaxoneblow" || move_id == "gmaxrapidflow" {
                return EventResult::Continue;
            }

            // if (move.isZ || move.isMax) target.getMoveHitData(move).zBrokeProtect = true;
            let is_z_or_max = battle.active_move.as_ref()
                .map(|m| m.is_z || m.is_max)
                .unwrap_or(false);

            if is_z_or_max {
                let move_id = match &battle.active_move {
                    Some(active_move) => active_move.id.clone(),
                    None => return EventResult::Continue,
                };

                let target_pokemon = match battle.pokemon_at_mut(target.0, target.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };

                // TODO: Implement move_hit_data system
                // In JavaScript: target.getMoveHitData(move).zBrokeProtect = true;
                // Needs proper MoveHitData tracking on Pokemon or Battle
                /*
                if let Some(hit_data) = target_pokemon.get_move_hit_data_mut(&move_id) {
                    hit_data.z_broke_protect = true;
                }
                */
            }

            // return;
            return EventResult::Continue;
        }

        // if (move && (move.target === 'self' || move.category === 'Status')) return;
        let move_data = battle.active_move.as_ref()
            .and_then(|m| battle.dex.get_move_by_id(&m.id));

        if let Some(m) = move_data {
            if m.target == "self" || m.category == "Status" {
                return EventResult::Continue;
            }
        }

        // this.add('-activate', target, 'move: Mat Block', move.name);
        let move_name = move_data.map(|m| m.name.as_str()).unwrap_or("");
        let target_arg = {

            let pokemon = match battle.pokemon_at(target.0, target.1) {

                Some(p) => p,

                None => return EventResult::Continue,

            };

            crate::battle::Arg::from(pokemon)

        };
        battle.add("-activate", &[target_arg, "move: Mat Block".into(), move_name.into()]);

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
                None => return EventResult::NotFail,
            };

            if let Some(lockedmove) = source_pokemon.volatiles.get(&ID::from("lockedmove")) {
                lockedmove.duration == Some(2)
            } else {
                false
            }
        };

        if has_lockedmove_duration_2 {
            let source_pokemon = match battle.pokemon_at_mut(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::NotFail,
            };
            source_pokemon.remove_volatile(&ID::from("lockedmove"));
        }

        // return this.NOT_FAIL;
        EventResult::NotFail
    }
}
