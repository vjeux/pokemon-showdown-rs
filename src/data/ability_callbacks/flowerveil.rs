//! Flower Veil Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onAllyTryBoost(boost, target, source, effect) {
///     if ((source && target === source) || !target.hasType('Grass')) return;
///     let showMsg = false;
///     let i: BoostID;
///     for (i in boost) {
///         if (boost[i]! < 0) {
///             delete boost[i];
///             showMsg = true;
///         }
///     }
///     if (showMsg && !(effect as ActiveMove).secondaries) {
///         const effectHolder = this.effectState.target;
///         this.add('-block', target, 'ability: Flower Veil', `[of] ${effectHolder}`);
///     }
/// }
pub fn on_ally_try_boost(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, _effect: Option<&Effect>) -> EventResult {
    use crate::battle::Arg;

    // if ((source && target === source) || !target.hasType('Grass')) return;
    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (source && target === source) return;
    if let Some(source) = source_pos {
        if target_pos == source {
            return EventResult::Continue;
        }
    }

    // if (!target.hasType('Grass')) return;
    let is_grass_type = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target.has_type(battle, "Grass")
    };

    if !is_grass_type {
        return EventResult::Continue;
    }

    // let showMsg = false;
    // for (i in boost) {
    //     if (boost[i]! < 0) {
    //         delete boost[i];
    //         showMsg = true;
    //     }
    // }
    let mut show_msg = false;
    if let Some(ref mut event) = battle.event {
        if let Some(crate::event::EventResult::Boost(ref mut boosts)) = event.relay_var {
            if boosts.atk < 0 { boosts.atk = 0; show_msg = true; }
            if boosts.def < 0 { boosts.def = 0; show_msg = true; }
            if boosts.spa < 0 { boosts.spa = 0; show_msg = true; }
            if boosts.spd < 0 { boosts.spd = 0; show_msg = true; }
            if boosts.spe < 0 { boosts.spe = 0; show_msg = true; }
            if boosts.accuracy < 0 { boosts.accuracy = 0; show_msg = true; }
            if boosts.evasion < 0 { boosts.evasion = 0; show_msg = true; }
        }
    }

    // if (showMsg && !(effect as ActiveMove).secondaries) {
    if show_msg {
        let has_secondaries = battle.active_move.as_ref()
            .map(|m| !m.borrow().secondaries.is_empty())
            .unwrap_or(false);

        if !has_secondaries {
            // const effectHolder = this.effectState.target;
            // this.add('-block', target, 'ability: Flower Veil', `[of] ${effectHolder}`);
            let (target_slot, effect_holder_slot) = {
                let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                let target_slot = target.get_slot();

                let effect_holder_slot = match battle.effect_state.borrow().target {
                    Some(pos) => {
                        let holder = match battle.pokemon_at(pos.0, pos.1) {
                            Some(p) => p,
                            None => return EventResult::Continue,
                        };
                        holder.get_slot()
                    }
                    None => return EventResult::Continue,
                };

                (target_slot, effect_holder_slot)
            };

            battle.add("-block", &[
                Arg::String(target_slot),
                Arg::Str("ability: Flower Veil"),
                Arg::String(format!("[of] {}", effect_holder_slot)),
            ]);
        }
    }

    EventResult::Continue
}

/// onAllySetStatus(status, target, source, effect) {
///     if (target.hasType('Grass') && source && target !== source && effect && effect.id !== 'yawn') {
///         this.debug('interrupting setStatus with Flower Veil');
///         if (effect.name === 'Synchronize' || (effect.effectType === 'Move' && !effect.secondaries)) {
///             const effectHolder = this.effectState.target;
///             this.add('-block', target, 'ability: Flower Veil', `[of] ${effectHolder}`);
///         }
///         return null;
///     }
/// }
pub fn on_ally_set_status(battle: &mut Battle, _status_id: &str, target_pos: (usize, usize), source_pos: Option<(usize, usize)>, effect: Option<&Effect>) -> EventResult {
    let effect_id = effect.map(|e| e.id.as_str());
    use crate::battle::Arg;

    // if (target.hasType('Grass') && source && target !== source && effect && effect.id !== 'yawn') {
    let is_grass_type = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target.has_type(battle, "Grass")
    };

    if !is_grass_type {
        return EventResult::Continue;
    }

    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    if target_pos == source {
        return EventResult::Continue;
    }

    let effect_id = match effect_id {
        Some(id) => id,
        None => return EventResult::Continue,
    };

    if effect_id == "yawn" {
        return EventResult::Continue;
    }

    // this.debug('interrupting setStatus with Flower Veil');
    // if (effect.name === 'Synchronize' || (effect.effectType === 'Move' && !effect.secondaries)) {
    let should_show_block = {
        if effect_id == "synchronize" {
            true
        } else {
            battle.active_move.as_ref()
                .map(|m| m.borrow().secondaries.is_empty())
                .unwrap_or(false)
        }
    };

    if should_show_block {
        // const effectHolder = this.effectState.target;
        // this.add('-block', target, 'ability: Flower Veil', `[of] ${effectHolder}`);
        let (target_slot, effect_holder_slot) = {
            let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            let target_slot = target.get_slot();

            let effect_holder_slot = match battle.effect_state.borrow().target {
                Some(pos) => {
                    let holder = match battle.pokemon_at(pos.0, pos.1) {
                        Some(p) => p,
                        None => return EventResult::Continue,
                    };
                    holder.get_slot()
                }
                None => return EventResult::Continue,
            };

            (target_slot, effect_holder_slot)
        };

        battle.add("-block", &[
            Arg::String(target_slot),
            Arg::Str("ability: Flower Veil"),
            Arg::String(format!("[of] {}", effect_holder_slot)),
        ]);
    }

    // return null;
    EventResult::Null
}

/// onAllyTryAddVolatile(status, target) {
///     if (target.hasType('Grass') && status.id === 'yawn') {
///         this.debug('Flower Veil blocking yawn');
///         const effectHolder = this.effectState.target;
///         this.add('-block', target, 'ability: Flower Veil', `[of] ${effectHolder}`);
///         return null;
///     }
/// }
pub fn on_ally_try_add_volatile(battle: &mut Battle, status: Option<&str>, target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, _effect: Option<&Effect>) -> EventResult {
    use crate::battle::Arg;

    // if (target.hasType('Grass') && status.id === 'yawn') {
    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let is_grass_type = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target.has_type(battle, "Grass")
    };

    if !is_grass_type {
        return EventResult::Continue;
    }

    let status_id = match status {
        Some(id) => id,
        None => return EventResult::Continue,
    };

    if status_id != "yawn" {
        return EventResult::Continue;
    }

    // this.debug('Flower Veil blocking yawn');
    // const effectHolder = this.effectState.target;
    // this.add('-block', target, 'ability: Flower Veil', `[of] ${effectHolder}`);
    let (target_slot, effect_holder_slot) = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let target_slot = target.get_slot();

        let effect_holder_slot = match battle.effect_state.borrow().target {
            Some(pos) => {
                let holder = match battle.pokemon_at(pos.0, pos.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                holder.get_slot()
            }
            None => return EventResult::Continue,
        };

        (target_slot, effect_holder_slot)
    };

    battle.add("-block", &[
        Arg::String(target_slot),
        Arg::Str("ability: Flower Veil"),
        Arg::String(format!("[of] {}", effect_holder_slot)),
    ]);

    // return null;
    EventResult::Null
}

