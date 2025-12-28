//! Electric Terrain Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;


pub mod condition {
    use super::*;

    /// durationCallback(source, effect) {
    ///     if (source?.hasItem('terrainextender')) {
    ///         return 8;
    ///     }
    ///     return 5;
    /// }
    pub fn duration_callback(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
        use crate::dex_data::ID;

        // if (source?.hasItem('terrainextender')) {
        //     return 8;
        // }
        // return 5;
        if let Some(source) = source_pos {
            let has_terrain_extender = {
                let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                    Some(p) => p,
                    None => return EventResult::Number(5),
                };
                source_pokemon.has_item(&["terrainextender"])
            };

            if has_terrain_extender {
                return EventResult::Number(8);
            }
        }

        EventResult::Number(5)
    }

    /// onSetStatus(status, target, source, effect) {
    ///     if (status.id === 'slp' && target.isGrounded() && !target.isSemiInvulnerable()) {
    ///         if (effect.id === 'yawn' || (effect.effectType === 'Move' && !effect.secondaries)) {
    ///             this.add('-activate', target, 'move: Electric Terrain');
    ///         }
    ///         return false;
    ///     }
    /// }
    pub fn on_set_status(battle: &mut Battle, status: Option<&str>, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // if (status.id === 'slp' && target.isGrounded() && !target.isSemiInvulnerable()) {
        if status == Some("slp") {
            let (is_grounded, is_semi_invulnerable) = {
                let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                (target_pokemon.is_grounded(), target_pokemon.is_semi_invulnerable())
            };

            if is_grounded && !is_semi_invulnerable {
                // if (effect.id === 'yawn' || (effect.effectType === 'Move' && !effect.secondaries)) {
                //     this.add('-activate', target, 'move: Electric Terrain');
                // }
                if let Some(effect) = effect_id {
                    let should_activate = if effect == "yawn" {
                        true
                    } else {
                        // Check if effect is a Move without secondaries
                        battle.dex.get_move_by_id(&crate::dex_data::ID::from(effect))
                            .map(|move_data| move_data.secondaries.is_none() || move_data.secondaries.as_ref().map(|s| s.is_empty()).unwrap_or(true))
                            .unwrap_or(false)
                    };

                    if should_activate {
                        let target_arg = {
                            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                                Some(p) => p,
                                None => return EventResult::Continue,
                            };
                            crate::battle::Arg::from(target_pokemon)
                        };

                        battle.add("-activate", &[target_arg, "move: Electric Terrain".into()]);
                    }
                }

                // return false;
                return EventResult::Boolean(false);
            }
        }

        EventResult::Continue
    }

    /// onTryAddVolatile(status, target) {
    ///     if (!target.isGrounded() || target.isSemiInvulnerable()) return;
    ///     if (status.id === 'yawn') {
    ///         this.add('-activate', target, 'move: Electric Terrain');
    ///         return null;
    ///     }
    /// }
    pub fn on_try_add_volatile(battle: &mut Battle, status: Option<&str>, target_pos: Option<(usize, usize)>) -> EventResult {
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // if (!target.isGrounded() || target.isSemiInvulnerable()) return;
        let (is_grounded, is_semi_invulnerable) = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            (target_pokemon.is_grounded(), target_pokemon.is_semi_invulnerable())
        };

        if !is_grounded || is_semi_invulnerable {
            // return;
            return EventResult::Continue;
        }

        // if (status.id === 'yawn') {
        //     this.add('-activate', target, 'move: Electric Terrain');
        //     return null;
        // }
        if status == Some("yawn") {
            let target_arg = {
                let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                crate::battle::Arg::from(target_pokemon)
            };

            battle.add("-activate", &[target_arg, "move: Electric Terrain".into()]);

            // return null;
            return EventResult::Stop;
        }

        EventResult::Continue
    }

    /// onBasePower(basePower, attacker, defender, move) {
    ///     if (move.type === 'Electric' && attacker.isGrounded() && !attacker.isSemiInvulnerable()) {
    ///         this.debug('electric terrain boost');
    ///         return this.chainModify([5325, 4096]);
    ///     }
    /// }
    pub fn on_base_power(battle: &mut Battle, base_power: i32, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
        use crate::dex_data::ID;

        let attacker = pokemon_pos;

        // Get move type
        let move_type = match &battle.active_move {
            Some(active_move) => active_move.move_type.clone(),
            None => return EventResult::Continue,
        };

        // if (move.type === 'Electric' && attacker.isGrounded() && !attacker.isSemiInvulnerable()) {
        if move_type == "electric" {
            let (is_grounded, is_semi_invulnerable) = {
                let attacker_pokemon = match battle.pokemon_at(attacker.0, attacker.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                (attacker_pokemon.is_grounded(), attacker_pokemon.is_semi_invulnerable())
            };

            if is_grounded && !is_semi_invulnerable {
                // this.debug('electric terrain boost');
                battle.debug("electric terrain boost");

                // return this.chainModify([5325, 4096]);
                return EventResult::Number(battle.chain_modify_fraction(5325, 4096));
            }
        }

        EventResult::Continue
    }

    /// onFieldStart(field, source, effect) {
    ///     if (effect?.effectType === 'Ability') {
    ///         this.add('-fieldstart', 'move: Electric Terrain', '[from] ability: ' + effect.name, `[of] ${source}`);
    ///     } else {
    ///         this.add('-fieldstart', 'move: Electric Terrain');
    ///     }
    /// }
    pub fn on_field_start(battle: &mut Battle, field_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
        use crate::dex_data::ID;

        // if (effect?.effectType === 'Ability') {
        //     this.add('-fieldstart', 'move: Electric Terrain', '[from] ability: ' + effect.name, `[of] ${source}`);
        // } else {
        //     this.add('-fieldstart', 'move: Electric Terrain');
        // }
        if let Some(effect) = effect_id {
            // Check if effect is an ability
            let is_ability = battle.dex.get_ability_by_id(&ID::from(effect)).is_some();

            if is_ability {
                if let Some(source) = source_pos {
                    let (source_arg, ability_name) = {
                        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                            Some(p) => p,
                            None => return EventResult::Continue,
                        };
                        let ability_name = battle.dex.get_ability_by_id(&ID::from(effect))
                            .map(|a| a.name.clone())
                            .unwrap_or_else(|| effect.to_string());
                        (crate::battle::Arg::from(source_pokemon), ability_name)
                    };

                    battle.add("-fieldstart", &[
                        "move: Electric Terrain".into(),
                        format!("[from] ability: {}", ability_name).into(),
                        format!("[of] {}", source_arg).into(),
                    ]);
                } else {
                    battle.add("-fieldstart", &["move: Electric Terrain".into()]);
                }
            } else {
                battle.add("-fieldstart", &["move: Electric Terrain".into()]);
            }
        } else {
            battle.add("-fieldstart", &["move: Electric Terrain".into()]);
        }

        EventResult::Continue
    }

    /// onFieldEnd() {
    ///     this.add('-fieldend', 'move: Electric Terrain');
    /// }
    pub fn on_field_end(battle: &mut Battle) -> EventResult {
        // this.add('-fieldend', 'move: Electric Terrain');
        battle.add("-fieldend", &["move: Electric Terrain".into()]);

        EventResult::Continue
    }
}
