//! Safeguard Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;


pub mod condition {
    use super::*;

    /// durationCallback(target, source, effect) {
    ///     if (source?.hasAbility('persistent')) {
    ///         this.add('-activate', source, 'ability: Persistent', '[move] Safeguard');
    ///         return 7;
    ///     }
    ///     return 5;
    /// }
    pub fn duration_callback(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
        use crate::dex_data::ID;

        // if (source?.hasAbility('persistent')) {
        //     this.add('-activate', source, 'ability: Persistent', '[move] Safeguard');
        //     return 7;
        // }
        // return 5;
        if let Some(source) = source_pos {
            let has_persistent = {
                let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                source_pokemon.has_ability(&["persistent"])
            };

            if has_persistent {
                let source_arg = {
                    let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                        Some(p) => p,
                        None => return EventResult::Continue,
                    };
                    crate::battle::Arg::from(source_pokemon)
                };

                battle.add("-activate", &[
                    source_arg,
                    "ability: Persistent".into(),
                    "[move] Safeguard".into(),
                ]);
                return EventResult::Number(7);
            }
        }

        EventResult::Number(5)
    }

    /// onSetStatus(status, target, source, effect) {
    ///     if (!effect || !source) return;
    ///     if (effect.id === 'yawn') return;
    ///     if (effect.effectType === 'Move' && effect.infiltrates && !target.isAlly(source)) return;
    ///     if (target !== source) {
    ///         this.debug('interrupting setStatus');
    ///         if (effect.name === 'Synchronize' || (effect.effectType === 'Move' && !effect.secondaries)) {
    ///             this.add('-activate', target, 'move: Safeguard');
    ///         }
    ///         return null;
    ///     }
    /// }
    pub fn on_set_status(battle: &mut Battle, status: Option<&str>, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
        use crate::dex_data::ID;

        // if (!effect || !source) return;
        if effect_id.is_none() || source_pos.is_none() {
            return EventResult::Continue;
        }

        let effect_id = effect_id.unwrap();
        let source = source_pos.unwrap();
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // if (effect.id === 'yawn') return;
        if effect_id == "yawn" {
            return EventResult::Continue;
        }

        // if (effect.effectType === 'Move' && effect.infiltrates && !target.isAlly(source)) return;
        let effect_data = battle.dex.get_effect_by_id(&ID::from(effect_id));
        if let Some(ref effect) = effect_data {
            if effect.effect_type == "Move" {
                // Check if the active move has infiltrates property (set by Infiltrator ability)
                let infiltrates = {
                    if let Some(ref active_move) = battle.active_move {
                        if active_move.id == ID::from(effect_id) {
                            active_move.infiltrates
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                };

                if infiltrates {
                    let is_ally = battle.is_ally(target, source);
                    if !is_ally {
                        return EventResult::Continue;
                    }
                }
            }
        }

        // if (target !== source) {
        if target != source {
            // this.debug('interrupting setStatus');
            battle.debug("interrupting setStatus");

            // if (effect.name === 'Synchronize' || (effect.effectType === 'Move' && !effect.secondaries)) {
            //     this.add('-activate', target, 'move: Safeguard');
            // }
            let should_activate = if let Some(ref effect) = effect_data {
                effect.name == "Synchronize" || (effect.effect_type == "Move" && {
                    if let Some(move_data) = battle.dex.get_move_by_id(&ID::from(effect_id)) {
                        move_data.secondaries.is_none() || move_data.secondaries.as_ref().map(|s| s.is_empty()).unwrap_or(true)
                    } else {
                        false
                    }
                })
            } else {
                false
            };

            if should_activate {
                let target_arg = {
                    let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                        Some(p) => p,
                        None => return EventResult::Continue,
                    };
                    crate::battle::Arg::from(target_pokemon)
                };

                battle.add("-activate", &[
                    target_arg,
                    "move: Safeguard".into(),
                ]);
            }

            // return null;
            return EventResult::Stop;
        }

        EventResult::Continue
    }

    /// onTryAddVolatile(status, target, source, effect) {
    ///     if (!effect || !source) return;
    ///     if (effect.effectType === 'Move' && effect.infiltrates && !target.isAlly(source)) return;
    ///     if ((status.id === 'confusion' || status.id === 'yawn') && target !== source) {
    ///         if (effect.effectType === 'Move' && !effect.secondaries) this.add('-activate', target, 'move: Safeguard');
    ///         return null;
    ///     }
    /// }
    pub fn on_try_add_volatile(battle: &mut Battle, status: Option<&str>, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
        use crate::dex_data::ID;

        // if (!effect || !source) return;
        if effect_id.is_none() || source_pos.is_none() {
            return EventResult::Continue;
        }

        let effect_id = effect_id.unwrap();
        let source = source_pos.unwrap();
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };
        let status = match status {
            Some(s) => s,
            None => return EventResult::Continue,
        };

        // if (effect.effectType === 'Move' && effect.infiltrates && !target.isAlly(source)) return;
        let effect_data = battle.dex.get_effect_by_id(&ID::from(effect_id));
        if let Some(ref effect) = effect_data {
            if effect.effect_type == "Move" {
                // Check if the active move has infiltrates property (set by Infiltrator ability)
                let infiltrates = {
                    if let Some(ref active_move) = battle.active_move {
                        if active_move.id == ID::from(effect_id) {
                            active_move.infiltrates
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                };

                if infiltrates {
                    let is_ally = battle.is_ally(target, source);
                    if !is_ally {
                        return EventResult::Continue;
                    }
                }
            }
        }

        // if ((status.id === 'confusion' || status.id === 'yawn') && target !== source) {
        if (status == "confusion" || status == "yawn") && target != source {
            // if (effect.effectType === 'Move' && !effect.secondaries) this.add('-activate', target, 'move: Safeguard');
            let should_activate = if let Some(ref effect) = effect_data {
                effect.effect_type == "Move" && {
                    if let Some(move_data) = battle.dex.get_move_by_id(&ID::from(effect_id)) {
                        move_data.secondaries.is_none() || move_data.secondaries.as_ref().map(|s| s.is_empty()).unwrap_or(true)
                    } else {
                        false
                    }
                }
            } else {
                false
            };

            if should_activate {
                let target_arg = {
                    let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                        Some(p) => p,
                        None => return EventResult::Continue,
                    };
                    crate::battle::Arg::from(target_pokemon)
                };

                battle.add("-activate", &[
                    target_arg,
                    "move: Safeguard".into(),
                ]);
            }

            // return null;
            return EventResult::Stop;
        }

        EventResult::Continue
    }

    /// onSideStart(side, source) {
    ///     if (source?.hasAbility('persistent')) {
    ///         this.add('-sidestart', side, 'Safeguard', '[persistent]');
    ///     } else {
    ///         this.add('-sidestart', side, 'Safeguard');
    ///     }
    /// }
    pub fn on_side_start(battle: &mut Battle, source_pos: Option<(usize, usize)>) -> EventResult {
        use crate::dex_data::ID;

        // onSideStart(side, source) {
        //     if (source?.hasAbility('persistent')) {
        //         this.add('-sidestart', side, 'Safeguard', '[persistent]');
        //     } else {
        //         this.add('-sidestart', side, 'Safeguard');
        //     }
        // }
        let side = {
            let effect_state = match &battle.current_effect_state {
                Some(es) => es,
                None => return EventResult::Continue,
            };
            match effect_state.side {
                Some(s) => s,
                None => return EventResult::Continue,
            }
        };

        let side_arg = crate::battle::Arg::Str(if side == 0 { "p1" } else { "p2" });

        if let Some(source) = source_pos {
            let has_persistent = {
                let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                source_pokemon.has_ability(&["persistent"])
            };

            if has_persistent {
                battle.add("-sidestart", &[
                    side_arg,
                    "Safeguard".into(),
                    "[persistent]".into(),
                ]);
            } else {
                battle.add("-sidestart", &[
                    side_arg,
                    "Safeguard".into(),
                ]);
            }
        } else {
            battle.add("-sidestart", &[
                side_arg,
                "Safeguard".into(),
            ]);
        }

        EventResult::Continue
    }

    /// onSideEnd(side) {
    ///     this.add('-sideend', side, 'Safeguard');
    /// }
    pub fn on_side_end(battle: &mut Battle) -> EventResult {
        // this.add('-sideend', side, 'Safeguard');
        let side = {
            let effect_state = match &battle.current_effect_state {
                Some(es) => es,
                None => return EventResult::Continue,
            };
            match effect_state.side {
                Some(s) => s,
                None => return EventResult::Continue,
            }
        };

        let side_arg = crate::battle::Arg::Str(if side == 0 { "p1" } else { "p2" });

        battle.add("-sideend", &[
            side_arg,
            "Safeguard".into(),
        ]);

        EventResult::Continue
    }
}
