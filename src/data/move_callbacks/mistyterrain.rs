//! Misty Terrain Move
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
        if let Some(source) = source_pos {
            let has_terrain_extender = {
                let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                    Some(p) => p,
                    None => return EventResult::Number(5),
                };
                source_pokemon.has_item(&ID::from("terrainextender"))
            };

            if has_terrain_extender {
                // return 8;
                return EventResult::Number(8);
            }
        }

        // return 5;
        EventResult::Number(5)
    }

    /// onSetStatus(status, target, source, effect) {
    ///     if (!target.isGrounded() || target.isSemiInvulnerable()) return;
    ///     if (effect && ((effect as Move).status || effect.id === 'yawn')) {
    ///         this.add('-activate', target, 'move: Misty Terrain');
    ///     }
    ///     return false;
    /// }
    pub fn on_set_status(battle: &mut Battle, status: Option<&str>, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
        use crate::dex_data::ID;

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
            (target_pokemon.is_grounded(battle), target_pokemon.is_semi_invulnerable())
        };

        if !is_grounded || is_semi_invulnerable {
            return EventResult::Continue;
        }

        // if (effect && ((effect as Move).status || effect.id === 'yawn')) {
        if let Some(eff_id) = effect_id {
            let should_activate = if eff_id == "yawn" {
                true
            } else {
                // Check if effect is a move with status
                let move_data = battle.dex.get_move_by_id(&ID::from(eff_id));
                move_data.map(|m| m.status.is_some()).unwrap_or(false)
            };

            if should_activate {
                // this.add('-activate', target, 'move: Misty Terrain');
                let target_arg = {
                    let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                        Some(p) => p,
                        None => return EventResult::Boolean(false),
                    };
                    crate::battle::Arg::from(target_pokemon)
                };

                battle.add("-activate", &[target_arg, "move: Misty Terrain".into()]);
            }
        }

        // return false;
        EventResult::Boolean(false)
    }

    /// onTryAddVolatile(status, target, source, effect) {
    ///     if (!target.isGrounded() || target.isSemiInvulnerable()) return;
    ///     if (status.id === 'confusion') {
    ///         if (effect.effectType === 'Move' && !effect.secondaries) this.add('-activate', target, 'move: Misty Terrain');
    ///         return null;
    ///     }
    /// }
    pub fn on_try_add_volatile(battle: &mut Battle, status: Option<&str>, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
        use crate::dex_data::ID;

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
            (target_pokemon.is_grounded(battle), target_pokemon.is_semi_invulnerable())
        };

        if !is_grounded || is_semi_invulnerable {
            return EventResult::Continue;
        }

        // if (status.id === 'confusion') {
        if status == Some("confusion") {
            // if (effect.effectType === 'Move' && !effect.secondaries) this.add('-activate', target, 'move: Misty Terrain');
            if let Some(eff_id) = effect_id {
                let move_data = battle.dex.get_move_by_id(&ID::from(eff_id));
                let is_move_without_secondaries = move_data.map(|m| {
                    m.secondaries.is_none() || m.secondaries.as_ref().map(|s| s.is_empty()).unwrap_or(true)
                }).unwrap_or(false);

                if is_move_without_secondaries {
                    let target_arg = {
                        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                            Some(p) => p,
                            None => return EventResult::Stop,
                        };
                        crate::battle::Arg::from(target_pokemon)
                    };

                    battle.add("-activate", &[target_arg, "move: Misty Terrain".into()]);
                }
            }

            // return null;
            return EventResult::Stop;
        }

        EventResult::Continue
    }

    /// onBasePower(basePower, attacker, defender, move) {
    ///     if (move.type === 'Dragon' && defender.isGrounded() && !defender.isSemiInvulnerable()) {
    ///         this.debug('misty terrain weaken');
    ///         return this.chainModify(0.5);
    ///     }
    /// }
    pub fn on_base_power(battle: &mut Battle, base_power: i32, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
        use crate::dex_data::ID;

        let defender = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // Get the move type
        let move_type = {
            let active_move = match &battle.active_move {
                Some(m) => m,
                None => return EventResult::Continue,
            };
            let move_data = battle.dex.get_move_by_id(active_move);
            move_data.map(|m| m.move_type.clone())
        };

        // if (move.type === 'Dragon' && defender.isGrounded() && !defender.isSemiInvulnerable()) {
        if move_type.as_deref() != Some("Dragon") {
            return EventResult::Continue;
        }

        let (is_grounded, is_semi_invulnerable) = {
            let defender_pokemon = match battle.pokemon_at(defender.0, defender.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            (defender_pokemon.is_grounded(battle), defender_pokemon.is_semi_invulnerable())
        };

        if is_grounded && !is_semi_invulnerable {
            // this.debug('misty terrain weaken');
            // (debug is typically not needed in Rust implementation)

            // return this.chainModify(0.5);
            return EventResult::Number(battle.chain_modify_fraction(1, 2)); // 0.5 = 1/2
        }

        EventResult::Continue
    }

    /// onFieldStart(field, source, effect) {
    ///     if (effect?.effectType === 'Ability') {
    ///         this.add('-fieldstart', 'move: Misty Terrain', '[from] ability: ' + effect.name, `[of] ${source}`);
    ///     } else {
    ///         this.add('-fieldstart', 'move: Misty Terrain');
    ///     }
    /// }
    pub fn on_field_start(battle: &mut Battle, field_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
        use crate::dex_data::ID;

        // if (effect?.effectType === 'Ability') {
        if let Some(eff_id) = effect_id {
            let ability_data = battle.dex.get_ability_by_id(&ID::from(eff_id));

            if let Some(ability) = ability_data {
                // this.add('-fieldstart', 'move: Misty Terrain', '[from] ability: ' + effect.name, `[of] ${source}`);
                if let Some(source) = source_pos {
                    let source_arg = {
                        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                            Some(p) => p,
                            None => return EventResult::Continue,
                        };
                        crate::battle::Arg::from(source_pokemon)
                    };

                    battle.add("-fieldstart", &[
                        "move: Misty Terrain".into(),
                        format!("[from] ability: {}", ability.name).into(),
                        format!("[of] {}", source_arg).into(),
                    ]);
                    return EventResult::Continue;
                }
            }
        }

        // this.add('-fieldstart', 'move: Misty Terrain');
        battle.add("-fieldstart", &["move: Misty Terrain".into()]);

        EventResult::Continue
    }

    /// onFieldEnd() {
    ///     this.add('-fieldend', 'Misty Terrain');
    /// }
    pub fn on_field_end(battle: &mut Battle) -> EventResult {
        // this.add('-fieldend', 'Misty Terrain');
        battle.add("-fieldend", &["Misty Terrain".into()]);

        EventResult::Continue
    }
}
