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
            stockpile_volatile
                .data
                .get("layers")
                .and_then(|v| v.as_i64())
                .unwrap_or(0) as i32
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
        if let Some(ref mut effect_state) = battle.current_effect_state {
            effect_state.data.insert(
                "layers".to_string(),
                serde_json::to_value(1).unwrap_or(serde_json::Value::Null),
            );
            effect_state.data.insert(
                "def".to_string(),
                serde_json::to_value(0).unwrap_or(serde_json::Value::Null),
            );
            effect_state.data.insert(
                "spd".to_string(),
                serde_json::to_value(0).unwrap_or(serde_json::Value::Null),
            );
        }

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
        battle.boost(&[("def", 1), ("spd", 1)], target, Some(target), None);

        // if (curDef !== target.boosts.def) this.effectState.def--;
        // if (curSpD !== target.boosts.spd) this.effectState.spd--;
        let (new_def, new_spd) = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            (target_pokemon.boosts.def, target_pokemon.boosts.spd)
        };

        if let Some(ref mut effect_state) = battle.current_effect_state {
            if cur_def != new_def {
                let current_def = effect_state
                    .data
                    .get("def")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0);
                effect_state.data.insert(
                    "def".to_string(),
                    serde_json::to_value(current_def - 1).unwrap_or(serde_json::Value::Null),
                );
            }

            if cur_spd != new_spd {
                let current_spd = effect_state
                    .data
                    .get("spd")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0);
                effect_state.data.insert(
                    "spd".to_string(),
                    serde_json::to_value(current_spd - 1).unwrap_or(serde_json::Value::Null),
                );
            }
        }

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
            .current_effect_state
            .as_ref()
            .and_then(|es| es.data.get("layers"))
            .and_then(|v| v.as_i64())
            .unwrap_or(0) as i32;

        if layers >= 3 {
            return EventResult::NotFail;
        }

        // this.effectState.layers++;
        let new_layers = layers + 1;
        if let Some(ref mut effect_state) = battle.current_effect_state {
            effect_state.data.insert(
                "layers".to_string(),
                serde_json::to_value(new_layers).unwrap_or(serde_json::Value::Null),
            );
        }

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
        battle.boost(&[("def", 1), ("spd", 1)], target, Some(target), None);

        // if (curDef !== target.boosts.def) this.effectState.def--;
        // if (curSpD !== target.boosts.spd) this.effectState.spd--;
        let (new_def, new_spd) = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            (target_pokemon.boosts.def, target_pokemon.boosts.spd)
        };

        if let Some(ref mut effect_state) = battle.current_effect_state {
            if cur_def != new_def {
                let current_def = effect_state
                    .data
                    .get("def")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0);
                effect_state.data.insert(
                    "def".to_string(),
                    serde_json::to_value(current_def - 1).unwrap_or(serde_json::Value::Null),
                );
            }

            if cur_spd != new_spd {
                let current_spd = effect_state
                    .data
                    .get("spd")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0);
                effect_state.data.insert(
                    "spd".to_string(),
                    serde_json::to_value(current_spd - 1).unwrap_or(serde_json::Value::Null),
                );
            }
        }

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
        let (def_value, spd_value, layers) = {
            if let Some(ref effect_state) = battle.current_effect_state {
                let def = effect_state
                    .data
                    .get("def")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0);
                let spd = effect_state
                    .data
                    .get("spd")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0);
                let layers = effect_state
                    .data
                    .get("layers")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0);
                (def, spd, layers)
            } else {
                (0, 0, 0)
            }
        };

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
            battle.boost(&boosts, target, Some(target), None);
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
