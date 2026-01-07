//! Stockpile Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTry(source) {
///     if (source.volatiles['stockpile'] && source.volatiles['stockpile'].layers >= 3) return false;
/// }
pub fn on_try(
    battle: &mut Battle,
    source_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    use crate::dex_data::ID;

    let source = source_pos;

    // if (source.volatiles['stockpile'] && source.volatiles['stockpile'].layers >= 3) return false;
    let layers = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        if let Some(stockpile_volatile) = source_pokemon.volatiles.get(&ID::from("stockpile")) {
            stockpile_volatile.layers.unwrap_or(0)
        } else {
            0
        }
    };

    if layers >= 3 {
        return EventResult::NotFail;
    }

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onStart(target) {
    ///     this.effectState.layers = 1;
    ///     this.effectState.def = 0;
    ///     this.effectState.spd = 0;
    ///     this.add('-start', target, 'stockpile' + this.effectState.layers);
    ///     const [curDef, curSpD] = [target.boosts.def, target.boosts.spd];
    ///     this.boost({ def: 1, spd: 1 }, target, target);
    ///     if (curDef !== target.boosts.def) this.effectState.def--;
    ///     if (curSpD !== target.boosts.spd) this.effectState.spd--;
    /// }
    pub fn on_start(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // this.effectState.layers = 1;
        // this.effectState.def = 0;
        // this.effectState.spd = 0;
        battle.with_effect_state(|state| {
            state.layers = Some(1);
            state.def = Some(0);
            state.spd = Some(0);
        });

        // this.add('-start', target, 'stockpile' + this.effectState.layers);
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
                crate::battle::Arg::from("stockpile1"),
            ],
        );

        // const [curDef, curSpD] = [target.boosts.def, target.boosts.spd];
        let (cur_def, cur_spd) = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            (target_pokemon.boosts.def, target_pokemon.boosts.spd)
        };

        // this.boost({ def: 1, spd: 1 }, target, target);
        battle.boost(&[("def", 1), ("spd", 1)], target, Some(target), None, false, false);

        // if (curDef !== target.boosts.def) this.effectState.def--;
        // if (curSpD !== target.boosts.spd) this.effectState.spd--;
        let (new_def, new_spd) = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            (target_pokemon.boosts.def, target_pokemon.boosts.spd)
        };

        battle.with_effect_state(|state| {
            if cur_def != new_def {
                let current_def = state.def.unwrap_or(0);
                state.def = Some(current_def - 1);
            }

            if cur_spd != new_spd {
                let current_spd = state.spd.unwrap_or(0);
                state.spd = Some(current_spd - 1);
            }
        });

        EventResult::Continue
    }

    /// onRestart(target) {
    ///     if (this.effectState.layers >= 3) return false;
    ///     this.effectState.layers++;
    ///     this.add('-start', target, 'stockpile' + this.effectState.layers);
    ///     const curDef = target.boosts.def;
    ///     const curSpD = target.boosts.spd;
    ///     this.boost({ def: 1, spd: 1 }, target, target);
    ///     if (curDef !== target.boosts.def) this.effectState.def--;
    ///     if (curSpD !== target.boosts.spd) this.effectState.spd--;
    /// }
    pub fn on_restart(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // if (this.effectState.layers >= 3) return false;
        let layers = battle
            .with_effect_state_ref(|state| state.layers.unwrap_or(0))
            .unwrap_or(0);

        if layers >= 3 {
            return EventResult::NotFail;
        }

        // this.effectState.layers++;
        let new_layers = layers + 1;
        battle.with_effect_state(|state| {
            state.layers = Some(new_layers);
        });

        // this.add('-start', target, 'stockpile' + this.effectState.layers);
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
                crate::battle::Arg::from(format!("stockpile{}", new_layers)),
            ],
        );

        // const curDef = target.boosts.def;
        // const curSpD = target.boosts.spd;
        let (cur_def, cur_spd) = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            (target_pokemon.boosts.def, target_pokemon.boosts.spd)
        };

        // this.boost({ def: 1, spd: 1 }, target, target);
        battle.boost(&[("def", 1), ("spd", 1)], target, Some(target), None, false, false);

        // if (curDef !== target.boosts.def) this.effectState.def--;
        // if (curSpD !== target.boosts.spd) this.effectState.spd--;
        let (new_def, new_spd) = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            (target_pokemon.boosts.def, target_pokemon.boosts.spd)
        };

        battle.with_effect_state(|state| {
            if cur_def != new_def {
                let current_def = state.def.unwrap_or(0);
                state.def = Some(current_def - 1);
            }

            if cur_spd != new_spd {
                let current_spd = state.spd.unwrap_or(0);
                state.spd = Some(current_spd - 1);
            }
        });

        EventResult::Continue
    }

    /// onEnd(target) {
    ///     if (this.effectState.def || this.effectState.spd) {
    ///         const boosts: SparseBoostsTable = {};
    ///         if (this.effectState.def) boosts.def = this.effectState.def;
    ///         if (this.effectState.spd) boosts.spd = this.effectState.spd;
    ///         this.boost(boosts, target, target);
    ///     }
    ///     this.add('-end', target, 'Stockpile');
    ///     if (this.effectState.def !== this.effectState.layers * -1 || this.effectState.spd !== this.effectState.layers * -1) {
    ///         this.hint("In Gen 7, Stockpile keeps track of how many times it successfully altered each stat individually.");
    ///     }
    /// }
    pub fn on_end(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // if (this.effectState.def || this.effectState.spd) {
        let (def_value, spd_value, layers) = battle.with_effect_state_ref(|state| {
            let def = state.def.unwrap_or(0) as i64;
            let spd = state.spd.unwrap_or(0) as i64;
            let layers = state.layers.unwrap_or(0) as i64;
            (def, spd, layers)
        }).unwrap_or((0, 0, 0));

        if def_value != 0 || spd_value != 0 {
            // const boosts: SparseBoostsTable = {};
            // if (this.effectState.def) boosts.def = this.effectState.def;
            // if (this.effectState.spd) boosts.spd = this.effectState.spd;
            // this.boost(boosts, target, target);
            let mut boosts = Vec::new();
            if def_value != 0 {
                boosts.push(("def", def_value as i8));
            }
            if spd_value != 0 {
                boosts.push(("spd", spd_value as i8));
            }
            battle.boost(&boosts, target, Some(target), None, false, false);
        }

        // this.add('-end', target, 'Stockpile');
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
                crate::battle::Arg::from("Stockpile"),
            ],
        );

        // if (this.effectState.def !== this.effectState.layers * -1 || this.effectState.spd !== this.effectState.layers * -1) {
        //     this.hint("In Gen 7, Stockpile keeps track of how many times it successfully altered each stat individually.");
        // }
        let expected = layers * -1;
        if def_value != expected || spd_value != expected {
            battle.hint(
                "In Gen 7, Stockpile keeps track of how many times it successfully altered each stat individually.",
                true,
                None,
            );
        }

        EventResult::Continue
    }
}
