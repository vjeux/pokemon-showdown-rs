//! Uproar Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

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
        let (has_sleep, pokemon_ident, pokemon_name) = {
            let pokemon = match battle.pokemon_at(target_side_index, slot_index) {
                Some(p) => p,
                None => continue,
            };
            (pokemon.status.as_str() == "slp", pokemon.get_slot(), pokemon.name.clone())
        };

        if has_sleep {
            let _pokemon_mut = match battle.pokemon_at_mut(target_side_index, slot_index) {
                Some(p) => p,
                None => continue,
            };

            if let Some((status, removed_nightmare, _silent)) = Pokemon::cure_status(battle, (target_side_index, slot_index), false) {
                let full_name = format!("{}: {}", pokemon_ident, pokemon_name);
                battle.add("-curestatus", &[full_name.as_str().into(), status.as_str().into(), "[msg]".into()]);
                if removed_nightmare {
                    battle.add("-end", &[full_name.as_str().into(), "Nightmare".into(), "[silent]".into()]);
                }
            }
        }
    }

    // Iterate through foe active team
    for slot_index in foe_active_team {
        // if (foeActive && foeActive.status === 'slp') foeActive.cureStatus();
        let (has_sleep, pokemon_ident, pokemon_name) = {
            let pokemon = match battle.pokemon_at(foe_side_index, slot_index) {
                Some(p) => p,
                None => continue,
            };
            (pokemon.status.as_str() == "slp", pokemon.get_slot(), pokemon.name.clone())
        };

        if has_sleep {
            let _pokemon_mut = match battle.pokemon_at_mut(foe_side_index, slot_index) {
                Some(p) => p,
                None => continue,
            };

            if let Some((status, removed_nightmare, _silent)) = Pokemon::cure_status(battle, (foe_side_index, slot_index), false) {
                let full_name = format!("{}: {}", pokemon_ident, pokemon_name);
                battle.add("-curestatus", &[full_name.as_str().into(), status.as_str().into(), "[msg]".into()]);
                if removed_nightmare {
                    battle.add("-end", &[full_name.as_str().into(), "Nightmare".into(), "[silent]".into()]);
                }
            }
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
    pub fn on_residual(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        use crate::dex_data::ID;

        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // if (target.volatiles['throatchop'])
        let has_throatchop = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.has_volatile(&ID::from("throatchop"))
        };

        if has_throatchop {
            // target.removeVolatile('uproar');
            Pokemon::remove_volatile(battle, target, &ID::from("uproar"));
            return EventResult::Continue;
        }

        // if (target.lastMove && target.lastMove.id === 'struggle')
        let last_move_is_struggle = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.last_move.as_ref().map(|m| m.as_str()) == Some("struggle")
        };

        if last_move_is_struggle {
            // delete target.volatiles['uproar'];
            Pokemon::remove_volatile(battle, target, &ID::from("uproar"));
        }

        // this.add('-start', target, 'Uproar', '[upkeep]');
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
                crate::battle::Arg::from("[upkeep]"),
            ],
        );

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
        battle: &mut Battle,
        status: Option<&str>,
        pokemon_pos: (usize, usize),
    ) -> EventResult {
        let pokemon = pokemon_pos;

        // if (status.id === 'slp')
        if status == Some("slp") {
            // Get effectState.target
            let effect_target = battle
                .with_effect_state_ref(|state| state.target)
                .flatten();

            let pokemon_slot = {
                let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                pokemon_ref.get_slot()
            };

            // if (pokemon === this.effectState.target)
            if Some(pokemon) == effect_target {
                // this.add('-fail', pokemon, 'slp', '[from] Uproar', '[msg]');
                battle.add(
                    "-fail",
                    &[
                        crate::battle::Arg::from(pokemon_slot),
                        crate::battle::Arg::from("slp"),
                        crate::battle::Arg::from("[from] Uproar"),
                        crate::battle::Arg::from("[msg]"),
                    ],
                );
            } else {
                // this.add('-fail', pokemon, 'slp', '[from] Uproar');
                battle.add(
                    "-fail",
                    &[
                        crate::battle::Arg::from(pokemon_slot),
                        crate::battle::Arg::from("slp"),
                        crate::battle::Arg::from("[from] Uproar"),
                    ],
                );
            }

            // return null;
            return EventResult::Null;
        }

        EventResult::Continue
    }
}
