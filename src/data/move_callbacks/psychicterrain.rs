//! Psychic Terrain Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

pub mod condition {
    use super::*;

    /// durationCallback(source, effect) {
    ///     if (source?.hasItem('terrainextender')) {
    ///         return 8;
    ///     }
    ///     return 5;
    /// }
    pub fn duration_callback(
        battle: &mut Battle,
        _target_pos: Option<(usize, usize)>,
        source_pos: Option<(usize, usize)>,
        _effect_id: Option<&str>,
    ) -> EventResult {
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
                source_pokemon.has_item(battle, &["terrainextender"])
            };

            if has_terrain_extender {
                return EventResult::Number(8);
            }
        }

        EventResult::Number(5)
    }

    /// onTryHit(target, source, effect) {
    ///     if (effect && (effect.priority <= 0.1 || effect.target === 'self')) {
    ///         return;
    ///     }
    ///     if (target.isSemiInvulnerable() || target.isAlly(source)) return;
    ///     if (!target.isGrounded()) {
    ///         const baseMove = this.dex.moves.get(effect.id);
    ///         if (baseMove.priority > 0) {
    ///             this.hint("Psychic Terrain doesn't affect Pokémon immune to Ground.");
    ///         }
    ///         return;
    ///     }
    ///     this.add('-activate', target, 'move: Psychic Terrain');
    ///     return null;
    /// }
    pub fn on_try_hit(
        battle: &mut Battle,
        source_pos: (usize, usize),
        target_pos: (usize, usize),
        _move_id: Option<&str>,
    ) -> EventResult {
        let source = source_pos;
        let target = target_pos;

        // Get the active move (effect)
        let (move_id, move_priority, move_target) = match &battle.active_move {
            Some(active_move) => (
                active_move.id.clone(),
                active_move.priority,
                active_move.target.clone(),
            ),
            None => return EventResult::Continue,
        };

        // if (effect && (effect.priority <= 0.1 || effect.target === 'self')) {
        //     return;
        // }
        if move_priority as f64 <= 0.1 || move_target == "self" {
            return EventResult::Continue;
        }

        // if (target.isSemiInvulnerable() || target.isAlly(source)) return;
        let is_semi_invulnerable = Pokemon::is_semi_invulnerable(battle, target);
        let is_ally = Pokemon::is_ally(battle, target, source.0);

        if is_semi_invulnerable || is_ally {
            return EventResult::Continue;
        }

        // if (!target.isGrounded()) {
        let is_grounded = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.is_grounded(battle, false).unwrap_or(false)
        };

        if !is_grounded {
            // const baseMove = this.dex.moves.get(effect.id);
            // if (baseMove.priority > 0) {
            //     this.hint("Psychic Terrain doesn't affect Pokémon immune to Ground.");
            // }
            let base_move = match battle.dex.moves().get_by_id(&move_id) {
                Some(m) => m,
                None => return EventResult::Continue,
            };

            if base_move.priority > 0 {
                battle.hint("Psychic Terrain doesn't affect Pokémon immune to Ground.", true, None);
            }

            return EventResult::Continue;
        }

        // this.add('-activate', target, 'move: Psychic Terrain');
        let target_ident = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.get_slot()
        };

        battle.add(
            "-activate",
            &[target_ident.as_str().into(), "move: Psychic Terrain".into()],
        );

        // return null;
        EventResult::NotFail
    }

    /// onBasePower(basePower, attacker, defender, move) {
    ///     if (move.type === 'Psychic' && attacker.isGrounded() && !attacker.isSemiInvulnerable()) {
    ///         this.debug('psychic terrain boost');
    ///         return this.chainModify([5325, 4096]);
    ///     }
    /// }
    pub fn on_base_power(
        battle: &mut Battle,
        _base_power: i32,
        pokemon_pos: (usize, usize),
        _target_pos: Option<(usize, usize)>,
    ) -> EventResult {
        let attacker = pokemon_pos;

        // Get move type
        let move_type = match &battle.active_move {
            Some(active_move) => active_move.move_type.clone(),
            None => return EventResult::Continue,
        };

        // if (move.type === 'Psychic' && attacker.isGrounded() && !attacker.isSemiInvulnerable()) {
        if move_type == "psychic" {
            let is_grounded = {
                let attacker_pokemon = match battle.pokemon_at(attacker.0, attacker.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                attacker_pokemon.is_grounded(battle, false).unwrap_or(false)
            };
            let is_semi_invulnerable = Pokemon::is_semi_invulnerable(battle, attacker);

            if is_grounded && !is_semi_invulnerable {
                // this.debug('psychic terrain boost');
                battle.debug("psychic terrain boost");

                // return this.chainModify([5325, 4096]);
                return EventResult::Number(battle.chain_modify_fraction(5325, 4096));
            }
        }

        EventResult::Continue
    }

    /// onFieldStart(field, source, effect) {
    ///     if (effect?.effectType === 'Ability') {
    ///         this.add('-fieldstart', 'move: Psychic Terrain', '[from] ability: ' + effect.name, `[of] ${source}`);
    ///     } else {
    ///         this.add('-fieldstart', 'move: Psychic Terrain');
    ///     }
    /// }
    pub fn on_field_start(
        battle: &mut Battle,
        _field_pos: Option<(usize, usize)>,
        source_pos: Option<(usize, usize)>,
        effect_id: Option<&str>,
    ) -> EventResult {
        use crate::dex_data::ID;

        // if (effect?.effectType === 'Ability') {
        //     this.add('-fieldstart', 'move: Psychic Terrain', '[from] ability: ' + effect.name, `[of] ${source}`);
        // } else {
        //     this.add('-fieldstart', 'move: Psychic Terrain');
        // }
        if let Some(effect) = effect_id {
            // Check if effect is an ability
            let is_ability = battle.dex.abilities().get_by_id(&ID::from(effect)).is_some();

            if is_ability {
                if let Some(source) = source_pos {
                    let (source_ident, ability_name) = {
                        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                            Some(p) => p,
                            None => return EventResult::Continue,
                        };
                        let ability_name = battle
                            .dex
                            .abilities().get_by_id(&ID::from(effect))
                            .map(|a| a.name.clone())
                            .unwrap_or_else(|| effect.to_string());
                        (source_pokemon.get_slot(), ability_name)
                    };

                    battle.add(
                        "-fieldstart",
                        &[
                            "move: Psychic Terrain".into(),
                            format!("[from] ability: {}", ability_name).into(),
                            format!("[of] {}", source_ident).into(),
                        ],
                    );
                } else {
                    battle.add("-fieldstart", &["move: Psychic Terrain".into()]);
                }
            } else {
                battle.add("-fieldstart", &["move: Psychic Terrain".into()]);
            }
        } else {
            battle.add("-fieldstart", &["move: Psychic Terrain".into()]);
        }

        EventResult::Continue
    }

    /// onFieldEnd() {
    ///     this.add('-fieldend', 'move: Psychic Terrain');
    /// }
    pub fn on_field_end(battle: &mut Battle) -> EventResult {
        // this.add('-fieldend', 'move: Psychic Terrain');
        battle.add("-fieldend", &["move: Psychic Terrain".into()]);

        EventResult::Continue
    }
}
