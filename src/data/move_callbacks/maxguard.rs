//! Max Guard Move
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

    let stall_move_result = battle.run_event("StallMove", Some(pokemon), None, None, None);
    // Convert Option<i32> to bool
    EventResult::Boolean(stall_move_result.unwrap_or(0) != 0)
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
    Pokemon::add_volatile(battle, pokemon_pos, ID::from("stall"), None, None, None, None);

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onStart(target) {
    ///     this.add('-singleturn', target, 'Max Guard');
    /// }
    pub fn on_start(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // this.add('-singleturn', target, 'Max Guard');
        let target_arg = {
            let pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,

                None => return EventResult::Continue,
            };

            pokemon.get_slot()
        };
        battle.add("-singleturn", &[target_arg.into(), "Max Guard".into()]);

        EventResult::Continue
    }

    /// onTryHit(target, source, move) {
    ///     const bypassesMaxGuard = [
    ///         'acupressure', 'afteryou', 'allyswitch', 'aromatherapy', 'aromaticmist', 'coaching', 'confide', 'copycat', 'curse', 'decorate', 'doomdesire', 'feint', 'futuresight', 'gmaxoneblow', 'gmaxrapidflow', 'healbell', 'holdhands', 'howl', 'junglehealing', 'lifedew', 'meanlook', 'perishsong', 'playnice', 'powertrick', 'roar', 'roleplay', 'tearfullook',
    ///     ];
    ///     if (bypassesMaxGuard.includes(move.id)) return;
    ///     if (move.smartTarget) {
    ///         move.smartTarget = false;
    ///     } else {
    ///         this.add('-activate', target, 'move: Max Guard');
    ///     }
    ///     const lockedmove = source.getVolatile('lockedmove');
    ///     if (lockedmove) {
    ///         // Outrage counter is reset
    ///         if (source.volatiles['lockedmove'].duration === 2) {
    ///             delete source.volatiles['lockedmove'];
    ///         }
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

        // const bypassesMaxGuard = [...]
        // if (bypassesMaxGuard.includes(move.id)) return;
        let move_id = battle
            .active_move
            .as_ref()
            .map(|m| m.id.as_str())
            .unwrap_or("");
        let bypasses_max_guard = matches!(
            move_id,
            "acupressure"
                | "afteryou"
                | "allyswitch"
                | "aromatherapy"
                | "aromaticmist"
                | "coaching"
                | "confide"
                | "copycat"
                | "curse"
                | "decorate"
                | "doomdesire"
                | "feint"
                | "futuresight"
                | "gmaxoneblow"
                | "gmaxrapidflow"
                | "healbell"
                | "holdhands"
                | "howl"
                | "junglehealing"
                | "lifedew"
                | "meanlook"
                | "perishsong"
                | "playnice"
                | "powertrick"
                | "roar"
                | "roleplay"
                | "tearfullook"
        );

        if bypasses_max_guard {
            return EventResult::Continue;
        }

        // if (move.smartTarget) {
        //     move.smartTarget = false;
        // } else {
        //     this.add('-activate', target, 'move: Max Guard');
        // }
        let smart_target = battle.active_move.as_ref().and_then(|m| m.smart_target);

        if let Some(true) = smart_target {
            // Set smartTarget to false
            battle.modify_active_move_smart_target(false);
        } else {
            // this.add('-activate', target, 'move: Max Guard');
            let target_arg = {
                let pokemon = match battle.pokemon_at(target.0, target.1) {
                    Some(p) => p,

                    None => return EventResult::Continue,
                };

                pokemon.get_slot()
            };
            battle.add("-activate", &[target_arg.into(), "move: Max Guard".into()]);
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
                None => return EventResult::NotFail,
            };

            if let Some(lockedmove) = source_pokemon.volatiles.get(&ID::from("lockedmove")) {
                lockedmove.duration == Some(2)
            } else {
                false
            }
        };

        if has_lockedmove_duration_2 {
            Pokemon::remove_volatile(battle, source, &ID::from("lockedmove"));
        }

        // return this.NOT_FAIL;
        EventResult::NotFail
    }
}
