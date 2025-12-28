//! Max Guard Move
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

    let stall_move_result = battle.run_event("StallMove", pokemon, None, None, None);
    EventResult::Boolean(stall_move_result)
}

/// onHit(pokemon) {
///     pokemon.addVolatile('stall');
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    let pokemon = pokemon_pos;

    // pokemon.addVolatile('stall');
    let pokemon_pokemon = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };
    pokemon_pokemon.add_volatile(&ID::from("stall"), battle);

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
        let target_arg = crate::battle::Arg::Pos(target.0, target.1);
        battle.add("-singleturn", &[target_arg, "Max Guard".into()]);

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
    pub fn on_try_hit(battle: &mut Battle, source_pos: (usize, usize), target_pos: (usize, usize)) -> EventResult {
        use crate::dex_data::ID;

        let source = source_pos;
        let target = target_pos;

        // const bypassesMaxGuard = [...]
        // if (bypassesMaxGuard.includes(move.id)) return;
        let move_id = battle.active_move.as_ref().map(|m| m.as_str()).unwrap_or("");
        let bypasses_max_guard = matches!(move_id,
            "acupressure" | "afteryou" | "allyswitch" | "aromatherapy" | "aromaticmist" |
            "coaching" | "confide" | "copycat" | "curse" | "decorate" | "doomdesire" |
            "feint" | "futuresight" | "gmaxoneblow" | "gmaxrapidflow" | "healbell" |
            "holdhands" | "howl" | "junglehealing" | "lifedew" | "meanlook" | "perishsong" |
            "playnice" | "powertrick" | "roar" | "roleplay" | "tearfullook"
        );

        if bypasses_max_guard {
            return EventResult::Continue;
        }

        // if (move.smartTarget) {
        //     move.smartTarget = false;
        // } else {
        //     this.add('-activate', target, 'move: Max Guard');
        // }
        let smart_target = battle.active_move.as_ref()
            .and_then(|m| battle.dex.get_move_by_id(m))
            .and_then(|m| m.smart_target);

        if let Some(true) = smart_target {
            // Set smartTarget to false
            // TODO: Once battle infrastructure supports modifying active move directly,
            // this should set move.smartTarget = false
            // For now, we'll track this in effect state
            if let Some(ref mut effect_state) = battle.current_effect_state {
                effect_state.data.insert(
                    "smartTargetDisabled".to_string(),
                    serde_json::to_value(true).unwrap_or(serde_json::Value::Null),
                );
            }
        } else {
            // this.add('-activate', target, 'move: Max Guard');
            let target_arg = crate::battle::Arg::Pos(target.0, target.1);
            battle.add("-activate", &[target_arg, "move: Max Guard".into()]);
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
