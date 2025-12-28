//! Uproar Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(target) {
///     const activeTeam = target.side.activeTeam();
///     const foeActiveTeam = target.side.foe.activeTeam();
///     for (const [i, allyActive] of activeTeam.entries()) {
///         if (allyActive && allyActive.status === 'slp') allyActive.cureStatus();
///         const foeActive = foeActiveTeam[i];
///         if (foeActive && foeActive.status === 'slp') foeActive.cureStatus();
///     }
/// }
pub fn on_try_hit(
    battle: &mut Battle,
    _source_pos: (usize, usize),
    target_pos: (usize, usize),
) -> EventResult {
    let target = target_pos;

    // Get the target's side index
    let target_side_index = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.side_index
    };

    // const activeTeam = target.side.activeTeam();
    let active_team = battle.sides[target_side_index].active_team();

    // const foeActiveTeam = target.side.foe.activeTeam();
    let foe_side_index = 1 - target_side_index; // 0 -> 1, 1 -> 0
    let foe_active_team = battle.sides[foe_side_index].active_team();

    // for (const [i, allyActive] of activeTeam.entries())
    for slot_index in active_team {
        // if (allyActive && allyActive.status === 'slp') allyActive.cureStatus();
        let has_sleep = {
            let pokemon = match battle.pokemon_at(target_side_index, slot_index) {
                Some(p) => p,
                None => continue,
            };
            pokemon.status.as_str() == "slp"
        };

        if has_sleep {
            battle.cure_status((target_side_index, slot_index));
        }
    }

    // Iterate through foe active team
    for slot_index in foe_active_team {
        // if (foeActive && foeActive.status === 'slp') foeActive.cureStatus();
        let has_sleep = {
            let pokemon = match battle.pokemon_at(foe_side_index, slot_index) {
                Some(p) => p,
                None => continue,
            };
            pokemon.status.as_str() == "slp"
        };

        if has_sleep {
            battle.cure_status((foe_side_index, slot_index));
        }
    }

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onStart(target) {
    ///     this.add('-start', target, 'Uproar');
    /// }
    pub fn on_start(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // this.add('-start', target, 'Uproar');
        let target_slot = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.get_slot()
        };

        battle.add(
            "-start",
            &[
                crate::battle::Arg::from(target_slot),
                crate::battle::Arg::from("Uproar"),
            ],
        );

        EventResult::Continue
    }

    /// onResidual(target) {
    ///     if (target.volatiles['throatchop']) {
    ///         target.removeVolatile('uproar');
    ///         return;
    ///     }
    ///     if (target.lastMove && target.lastMove.id === 'struggle') {
    ///         // don't lock
    ///         delete target.volatiles['uproar'];
    ///     }
    ///     this.add('-start', target, 'Uproar', '[upkeep]');
    /// }
    pub fn on_residual(_battle: &mut Battle, _target_pos: Option<(usize, usize)>) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onEnd(target) {
    ///     this.add('-end', target, 'Uproar');
    /// }
    pub fn on_end(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // this.add('-end', target, 'Uproar');
        let target_slot = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.get_slot()
        };

        battle.add(
            "-end",
            &[
                crate::battle::Arg::from(target_slot),
                crate::battle::Arg::from("Uproar"),
            ],
        );

        EventResult::Continue
    }

    /// onAnySetStatus(status, pokemon) {
    ///     if (status.id === 'slp') {
    ///         if (pokemon === this.effectState.target) {
    ///             this.add('-fail', pokemon, 'slp', '[from] Uproar', '[msg]');
    ///         } else {
    ///             this.add('-fail', pokemon, 'slp', '[from] Uproar');
    ///         }
    ///         return null;
    ///     }
    /// }
    pub fn on_any_set_status(
        _battle: &mut Battle,
        _status: Option<&str>,
        _pokemon_pos: (usize, usize),
    ) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }
}
