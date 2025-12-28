//! Helping Hand Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(target) {
///     if (!target.newlySwitched && !this.queue.willMove(target)) return false;
/// }
pub fn on_try_hit(
    battle: &mut Battle,
    _source_pos: (usize, usize),
    target_pos: (usize, usize),
) -> EventResult {
    let target = target_pos;

    // if (!target.newlySwitched && !this.queue.willMove(target)) return false;
    let (newly_switched, will_move) = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (
            target_pokemon.newly_switched,
            battle.queue.will_move(target.0, target.1).is_some(),
        )
    };

    if !newly_switched && !will_move {
        return EventResult::NotFail;
    }

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onStart(target, source) {
    ///     this.effectState.multiplier = 1.5;
    ///     this.add('-singleturn', target, 'Helping Hand', `[of] ${source}`);
    /// }
    pub fn on_start(
        battle: &mut Battle,
        target_pos: Option<(usize, usize)>,
        source_pos: Option<(usize, usize)>,
    ) -> EventResult {
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        let source = match source_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // this.effectState.multiplier = 1.5;
        if let Some(ref mut effect_state) = battle.current_effect_state {
            effect_state.data.insert(
                "multiplier".to_string(),
                serde_json::to_value(1.5).unwrap_or(serde_json::Value::Null),
            );
        }

        // this.add('-singleturn', target, 'Helping Hand', `[of] ${source}`);
        let (target_slot, source_slot) = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            (target_pokemon.get_slot(), source_pokemon.get_slot())
        };

        battle.add(
            "-singleturn",
            &[
                crate::battle::Arg::from(target_slot),
                crate::battle::Arg::from("Helping Hand"),
                crate::battle::Arg::from(format!("[of] {}", source_slot)),
            ],
        );

        EventResult::Continue
    }

    /// onRestart(target, source) {
    ///     this.effectState.multiplier *= 1.5;
    ///     this.add('-singleturn', target, 'Helping Hand', `[of] ${source}`);
    /// }
    pub fn on_restart(
        battle: &mut Battle,
        target_pos: Option<(usize, usize)>,
        source_pos: Option<(usize, usize)>,
    ) -> EventResult {
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        let source = match source_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // this.effectState.multiplier *= 1.5;
        if let Some(ref mut effect_state) = battle.current_effect_state {
            let current_multiplier = effect_state
                .data
                .get("multiplier")
                .and_then(|v| v.as_f64())
                .unwrap_or(1.0);

            let new_multiplier = current_multiplier * 1.5;

            effect_state.data.insert(
                "multiplier".to_string(),
                serde_json::to_value(new_multiplier).unwrap_or(serde_json::Value::Null),
            );
        }

        // this.add('-singleturn', target, 'Helping Hand', `[of] ${source}`);
        let (target_slot, source_slot) = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            (target_pokemon.get_slot(), source_pokemon.get_slot())
        };

        battle.add(
            "-singleturn",
            &[
                crate::battle::Arg::from(target_slot),
                crate::battle::Arg::from("Helping Hand"),
                crate::battle::Arg::from(format!("[of] {}", source_slot)),
            ],
        );

        EventResult::Continue
    }

    /// onBasePower(basePower) {
    ///     this.debug('Boosting from Helping Hand: ' + this.effectState.multiplier);
    ///     return this.chainModify(this.effectState.multiplier);
    /// }
    pub fn on_base_power(
        battle: &mut Battle,
        _base_power: i32,
        _pokemon_pos: (usize, usize),
        _target_pos: Option<(usize, usize)>,
    ) -> EventResult {
        // this.debug('Boosting from Helping Hand: ' + this.effectState.multiplier);
        let multiplier = if let Some(ref effect_state) = battle.current_effect_state {
            effect_state
                .data
                .get("multiplier")
                .and_then(|v| v.as_f64())
                .unwrap_or(1.0)
        } else {
            1.0
        };

        battle.debug(&format!("Boosting from Helping Hand: {}", multiplier));

        // return this.chainModify(this.effectState.multiplier);
        EventResult::Number(battle.chain_modify(multiplier as f32))
    }
}
