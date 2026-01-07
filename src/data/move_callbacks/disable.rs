//! Disable Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(target) {
///     if (!target.lastMove || target.lastMove.isZOrMaxPowered || target.lastMove.isMax || target.lastMove.id === 'struggle') {
///         return false;
///     }
/// }
pub fn on_try_hit(
    battle: &mut Battle,
    _source_pos: (usize, usize),
    target_pos: (usize, usize),
) -> EventResult {
    // if (!target.lastMove || target.lastMove.isZOrMaxPowered || target.lastMove.isMax || target.lastMove.id === 'struggle') {
    //     return false;
    // }
    let target = target_pos;

    let target_pokemon = match battle.pokemon_at(target.0, target.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    // Check if target has a last move
    let last_move_id = match &target_pokemon.last_move {
        Some(id) => id.clone(),
        None => {
            // !target.lastMove
            return EventResult::Boolean(false);
        }
    };

    // target.lastMove.id === 'struggle'
    if last_move_id.as_str() == "struggle" {
        return EventResult::Boolean(false);
    }

    // target.lastMove.isZOrMaxPowered || target.lastMove.isMax
    let move_data = match battle.dex.moves().get_by_id(&last_move_id) {
        Some(m) => m,
        None => return EventResult::Continue,
    };

    if move_data.is_z_or_max_powered || move_data.is_max.is_some() {
        return EventResult::Boolean(false);
    }

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onStart(pokemon, source, effect) {
    ///     // The target hasn't taken its turn, or Cursed Body activated and the move was not used through Dancer or Instruct
    ///     if (
    ///         this.queue.willMove(pokemon) ||
    ///         (pokemon === this.activePokemon && this.activeMove && !this.activeMove.isExternal)
    ///     ) {
    ///         this.effectState.duration!--;
    ///     }
    ///     if (!pokemon.lastMove) {
    ///         this.debug(`Pokemon hasn't moved yet`);
    ///         return false;
    ///     }
    ///     for (const moveSlot of pokemon.moveSlots) {
    ///         if (moveSlot.id === pokemon.lastMove.id) {
    ///             if (!moveSlot.pp) {
    ///                 this.debug('Move out of PP');
    ///                 return false;
    ///             }
    ///         }
    ///     }
    ///     if (effect.effectType === 'Ability') {
    ///         this.add('-start', pokemon, 'Disable', pokemon.lastMove.name, '[from] ability: ' + effect.name, `[of] ${source}`);
    ///     } else {
    ///         this.add('-start', pokemon, 'Disable', pokemon.lastMove.name);
    ///     }
    ///     this.effectState.move = pokemon.lastMove.id;
    /// }
    pub fn on_start(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        source_pos: Option<(usize, usize)>,
        effect_id: Option<&str>,
    ) -> EventResult {
        // // The target hasn't taken its turn, or Cursed Body activated and the move was not used through Dancer or Instruct
        // if (
        //     this.queue.willMove(pokemon) ||
        //     (pokemon === this.activePokemon && this.activeMove && !this.activeMove.isExternal)
        // ) {
        //     this.effectState.duration!--;
        // }
        let pokemon = pokemon_pos;

        let will_move = battle.queue.will_move(pokemon.0, pokemon.1);

        // Check if pokemon === this.activePokemon && this.activeMove && !this.activeMove.isExternal
        let is_active_pokemon_with_internal_move = if let Some(active_pos) = battle.active_pokemon {
            active_pos == pokemon && battle.active_move.as_ref().map(|m| !m.is_external).unwrap_or(false)
        } else {
            false
        };

        if will_move.is_some() || is_active_pokemon_with_internal_move {
            // this.effectState.duration!--;
            battle.with_effect_state(|state| {
                if let Some(duration) = state.duration {
                    state.duration = Some(duration - 1);
                }
            });
        }

        // if (!pokemon.lastMove) {
        //     this.debug(`Pokemon hasn't moved yet`);
        //     return false;
        // }
        let last_move_id = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            match &pokemon_pokemon.last_move {
                Some(id) => id.clone(),
                None => {
                    battle.debug("Pokemon hasn't moved yet");
                    return EventResult::Boolean(false);
                }
            }
        };

        // for (const moveSlot of pokemon.moveSlots) {
        //     if (moveSlot.id === pokemon.lastMove.id) {
        //         if (!moveSlot.pp) {
        //             this.debug('Move out of PP');
        //             return false;
        //         }
        //     }
        // }
        let _has_pp = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };

            let mut found = false;
            for move_slot in &pokemon_pokemon.move_slots {
                if move_slot.id == last_move_id {
                    if move_slot.pp == 0 {
                        battle.debug("Move out of PP");
                        return EventResult::Boolean(false);
                    }
                    found = true;
                    break;
                }
            }
            found
        };

        // if (effect.effectType === 'Ability') {
        //     this.add('-start', pokemon, 'Disable', pokemon.lastMove.name, '[from] ability: ' + effect.name, `[of] ${source}`);
        // } else {
        //     this.add('-start', pokemon, 'Disable', pokemon.lastMove.name);
        // }
        let (pokemon_ident, last_move_name) = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            let move_data = battle.dex.moves().get_by_id(&last_move_id);
            let move_name = move_data
                .map(|m| m.name.clone())
                .unwrap_or_else(|| last_move_id.to_string());

            (pokemon_pokemon.get_slot(), move_name)
        };

        // Check if effect is an Ability
        if let Some(eff_id) = effect_id {
            use crate::dex_data::ID;
            let effect_id_obj = ID::from(eff_id);
            let is_ability = battle.get_effect_type(&effect_id_obj) == "Ability";

            if is_ability {
                // Get effect name and source
                let effect_name = battle
                    .dex
                    .abilities().get_by_id(&effect_id_obj)
                    .map(|a| a.name.clone())
                    .unwrap_or_else(|| eff_id.to_string());

                if let Some(src_pos) = source_pos {
                    let source_ident = {
                        let source_pokemon = match battle.pokemon_at(src_pos.0, src_pos.1) {
                            Some(p) => p,
                            None => return EventResult::Continue,
                        };
                        source_pokemon.get_slot()
                    };

                    battle.add(
                        "-start",
                        &[
                            pokemon_ident.as_str().into(),
                            "Disable".into(),
                            last_move_name.into(),
                            format!("[from] ability: {}", effect_name).into(),
                            format!("[of] {}", source_ident).into(),
                        ],
                    );
                } else {
                    battle.add(
                        "-start",
                        &[
                            pokemon_ident.as_str().into(),
                            "Disable".into(),
                            last_move_name.into(),
                            format!("[from] ability: {}", effect_name).into(),
                        ],
                    );
                }
            } else {
                battle.add(
                    "-start",
                    &[
                        pokemon_ident.as_str().into(),
                        "Disable".into(),
                        last_move_name.into(),
                    ],
                );
            }
        } else {
            battle.add(
                "-start",
                &[
                    pokemon_ident.as_str().into(),
                    "Disable".into(),
                    last_move_name.into(),
                ],
            );
        }

        // this.effectState.move = pokemon.lastMove.id;
        battle.with_effect_state(|state| {
            state.data.insert(
                "move".to_string(),
                serde_json::to_value(last_move_id.to_string()).unwrap(),
            );
        });

        EventResult::Continue
    }

    /// onEnd(pokemon) {
    ///     this.add('-end', pokemon, 'Disable');
    /// }
    pub fn on_end(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // this.add('-end', pokemon, 'Disable');
        let pokemon = pokemon_pos;

        let pokemon_ident = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_pokemon.get_slot()
        };

        battle.add("-end", &[pokemon_ident.as_str().into(), "Disable".into()]);

        EventResult::Continue
    }

    /// onBeforeMove(attacker, defender, move) {
    ///     if (!(move.isZ && move.isZOrMaxPowered) && move.id === this.effectState.move) {
    ///         this.add('cant', attacker, 'Disable', move);
    ///         return false;
    ///     }
    /// }
    pub fn on_before_move(battle: &mut Battle, move_id: &str) -> EventResult {
        // if (!(move.isZ && move.isZOrMaxPowered) && move.id === this.effectState.move) {
        //     this.add('cant', attacker, 'Disable', move);
        //     return false;
        // }

        // Get the disabled move from effect state
        let disabled_move_id = battle.with_effect_state_ref(|state| {
            state
                .data
                .get("move")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
        }).flatten();

        if let Some(disabled_id) = disabled_move_id {
            // move.id === this.effectState.move
            if move_id == disabled_id {
                // Check !(move.isZ && move.isZOrMaxPowered)
                let is_z_or_max_powered_z = battle
                    .active_move
                    .as_ref()
                    .map(|m| m.is_z && m.is_z_or_max_powered)
                    .unwrap_or(false);

                // If it's a Z-move with max power, don't disable it
                if is_z_or_max_powered_z {
                    return EventResult::Continue;
                }

                // this.add('cant', attacker, 'Disable', move);
                // Note: This callback doesn't have access to attacker position in the signature
                // We would need to get it from battle context
                // For now, we'll just return false to indicate the move is blocked
                return EventResult::Boolean(false);
            }
        }

        EventResult::Continue
    }

    /// onDisableMove(pokemon) {
    ///     for (const moveSlot of pokemon.moveSlots) {
    ///         if (moveSlot.id === this.effectState.move) {
    ///             pokemon.disableMove(moveSlot.id);
    ///         }
    ///     }
    /// }
    pub fn on_disable_move(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        use crate::dex_data::ID;

        // for (const moveSlot of pokemon.moveSlots) {
        //     if (moveSlot.id === this.effectState.move) {
        //         pokemon.disableMove(moveSlot.id);
        //     }
        // }
        let pokemon = pokemon_pos;

        // Get the disabled move from effect state
        let disabled_move_id = battle.with_effect_state_ref(|state| {
            state
                .data
                .get("move")
                .and_then(|v| v.as_str())
                .map(ID::from)
        }).flatten();

        if let Some(disabled_id) = disabled_move_id {
            let pokemon_pokemon = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };

            // for (const moveSlot of pokemon.moveSlots)
            // Find the move to disable, then disable it
            let move_id_to_disable = pokemon_pokemon
                .move_slots
                .iter()
                .find(|move_slot| move_slot.id == disabled_id)
                .map(|move_slot| move_slot.id.clone());

            if let Some(move_id) = move_id_to_disable {
                pokemon_pokemon.disable_move(move_id.as_str(), false, None);
            }
        }

        EventResult::Continue
    }
}
