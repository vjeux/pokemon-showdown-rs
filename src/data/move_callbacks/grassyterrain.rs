//! Grassy Terrain Move
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
                source_pokemon.has_item(&["terrainextender"])
            };

            if has_terrain_extender {
                // return 8;
                return EventResult::Number(8);
            }
        }

        // return 5;
        EventResult::Number(5)
    }

    /// onBasePower(basePower, attacker, defender, move) {
    ///     const weakenedMoves = ['earthquake', 'bulldoze', 'magnitude'];
    ///     if (weakenedMoves.includes(move.id) && defender.isGrounded() && !defender.isSemiInvulnerable()) {
    ///         this.debug('move weakened by grassy terrain');
    ///         return this.chainModify(0.5);
    ///     }
    ///     if (move.type === 'Grass' && attacker.isGrounded()) {
    ///         this.debug('grassy terrain boost');
    ///         return this.chainModify([5325, 4096]);
    ///     }
    /// }
    pub fn on_base_power(battle: &mut Battle, base_power: i32, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
        use crate::dex_data::ID;

        let attacker = pokemon_pos;
        let defender = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // const weakenedMoves = ['earthquake', 'bulldoze', 'magnitude'];
        // if (weakenedMoves.includes(move.id) && defender.isGrounded() && !defender.isSemiInvulnerable()) {
        let move_id = battle.active_move.as_ref().map(|m| m.clone());

        if let Some(ref id) = move_id {
            let weakened_moves = [
                ID::from("earthquake"),
                ID::from("bulldoze"),
                ID::from("magnitude"),
            ];

            if weakened_moves.contains(&id.id) {
                let defender_grounded = {
                    let defender_pokemon = match battle.pokemon_at(defender.0, defender.1) {
                        Some(p) => p,
                        None => return EventResult::Continue,
                    };
                    defender_pokemon.is_grounded()
                };

                let defender_semi_invuln = {
                    let defender_pokemon = match battle.pokemon_at(defender.0, defender.1) {
                        Some(p) => p,
                        None => return EventResult::Continue,
                    };
                    defender_pokemon.is_semi_invulnerable()
                };

                if defender_grounded && !defender_semi_invuln {
                    // this.debug('move weakened by grassy terrain');
                    battle.debug("move weakened by grassy terrain");

                    // return this.chainModify(0.5);
                    return EventResult::Number(battle.chain_modify(0.5 as f32));
                }
            }
        }

        // if (move.type === 'Grass' && attacker.isGrounded()) {
        let move_type = battle.active_move.as_ref().map(|m| m.move_type.clone());

        if let Some(ref type_id) = move_type {
            if type_id == "grass" {
                let attacker_grounded = {
                    let attacker_pokemon = match battle.pokemon_at(attacker.0, attacker.1) {
                        Some(p) => p,
                        None => return EventResult::Continue,
                    };
                    attacker_pokemon.is_grounded()
                };

                if attacker_grounded {
                    // this.debug('grassy terrain boost');
                    battle.debug("grassy terrain boost");

                    // return this.chainModify([5325, 4096]);
                    return EventResult::Number(battle.chain_modify(5325.0 / 4096.0 as f32));
                }
            }
        }

        EventResult::Continue
    }

    /// onFieldStart(field, source, effect) {
    ///     if (effect?.effectType === 'Ability') {
    ///         this.add('-fieldstart', 'move: Grassy Terrain', '[from] ability: ' + effect.name, `[of] ${source}`);
    ///     } else {
    ///         this.add('-fieldstart', 'move: Grassy Terrain');
    ///     }
    /// }
    pub fn on_field_start(battle: &mut Battle, field_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
        // if (effect?.effectType === 'Ability') {
        let is_ability_effect = if let Some(eid) = effect_id {
            if let Some(ability) = battle.dex.get_ability_by_id(&crate::dex_data::ID::from(eid)) {
                ability.effect_type.as_deref() == Some("Ability")
            } else {
                false
            }
        } else {
            false
        };

        if is_ability_effect {
            // this.add('-fieldstart', 'move: Grassy Terrain', '[from] ability: ' + effect.name, `[of] ${source}`);
            if let (Some(eid), Some(source)) = (effect_id, source_pos) {
                let ability = battle.dex.get_ability_by_id(&crate::dex_data::ID::from(eid));
                let ability_name = ability.map(|a| a.name.clone()).unwrap_or_else(|| eid.to_string());
                let from_msg = format!("[from] ability: {}", ability_name);
                let source_ident = {
                    let pokemon = match battle.pokemon_at(source.0, source.1) {
                        Some(p) => p,
                        None => return EventResult::Continue,
                    };
                    pokemon.get_slot()
                };
                battle.add("-fieldstart", &["move: Grassy Terrain".into(), from_msg.into(), "[of]".into(), source_ident.as_str().into()]);
            }
        } else {
            // this.add('-fieldstart', 'move: Grassy Terrain');
            battle.add("-fieldstart", &["move: Grassy Terrain".into()]);
        }

        EventResult::Continue
    }

    /// onResidual(pokemon) {
    ///     if (pokemon.isGrounded() && !pokemon.isSemiInvulnerable()) {
    ///         this.heal(pokemon.baseMaxhp / 16, pokemon, pokemon);
    ///     } else {
    ///         this.debug(`Pokemon semi-invuln or not grounded; Grassy Terrain skipped`);
    ///     }
    /// }
    pub fn on_residual(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        let pokemon = pokemon_pos;

        // if (pokemon.isGrounded() && !pokemon.isSemiInvulnerable()) {
        let is_grounded = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_pokemon.is_grounded()
        };

        let is_semi_invuln = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_pokemon.is_semi_invulnerable()
        };

        if is_grounded && !is_semi_invuln {
            // this.heal(pokemon.baseMaxhp / 16, pokemon, pokemon);
            let heal_amount = {
                let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                pokemon_pokemon.base_maxhp / 16
            };

            battle.heal(heal_amount, Some(pokemon), Some(pokemon), None);
        } else {
            // this.debug(`Pokemon semi-invuln or not grounded; Grassy Terrain skipped`);
            battle.debug("Pokemon semi-invuln or not grounded; Grassy Terrain skipped");
        }

        EventResult::Continue
    }

    /// onFieldEnd() {
    ///     this.add('-fieldend', 'move: Grassy Terrain');
    /// }
    pub fn on_field_end(battle: &mut Battle) -> EventResult {
        // this.add('-fieldend', 'move: Grassy Terrain');
        battle.add("-fieldend", &["move: Grassy Terrain".into()]);

        EventResult::Continue
    }
}
