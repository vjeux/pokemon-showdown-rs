//! Heal Block Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

pub mod condition {
    use super::*;

    /// durationCallback(target, source, effect) {
    ///     if (effect?.name === "Psychic Noise") {
    ///         return 2;
    ///     }
    ///     if (source?.hasAbility('persistent')) {
    ///         this.add('-activate', source, 'ability: Persistent', '[move] Heal Block');
    ///         return 7;
    ///     }
    ///     return 5;
    /// }
    pub fn duration_callback(
        battle: &mut Battle,
        _target_pos: Option<(usize, usize)>,
        source_pos: Option<(usize, usize)>,
        effect_id: Option<&str>,
    ) -> EventResult {
        // if (effect?.name === "Psychic Noise") {
        //     return 2;
        // }
        if let Some(eff_id) = effect_id {
            use crate::dex_data::ID;
            let effect_id_obj = ID::from(eff_id);

            // Check if it's the Psychic Noise move
            if let Some(move_data) = battle.dex.moves().get_by_id(&effect_id_obj) {
                if move_data.name == "Psychic Noise" {
                    return EventResult::Number(2);
                }
            }
        }

        // if (source?.hasAbility('persistent')) {
        //     this.add('-activate', source, 'ability: Persistent', '[move] Heal Block');
        //     return 7;
        // }
        if let Some(source) = source_pos {
            let has_persistent = {
                let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                    Some(p) => p,
                    None => return EventResult::Number(5),
                };
                source_pokemon.has_ability(battle, &["persistent"])
            };

            if has_persistent {
                // this.add('-activate', source, 'ability: Persistent', '[move] Heal Block');
                let source_slot = {
                    let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                        Some(p) => p,
                        None => return EventResult::Number(5),
                    };
                    source_pokemon.get_slot()
                };

                battle.add(
                    "-activate",
                    &[
                        crate::battle::Arg::from(source_slot),
                        crate::battle::Arg::from("ability: Persistent"),
                        crate::battle::Arg::from("[move] Heal Block"),
                    ],
                );

                // return 7;
                return EventResult::Number(7);
            }
        }

        // return 5;
        EventResult::Number(5)
    }

    /// onStart(pokemon, source) {
    ///     this.add('-start', pokemon, 'move: Heal Block');
    ///     source.moveThisTurnResult = true;
    /// }
    pub fn on_start(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        source_pos: Option<(usize, usize)>,
    ) -> EventResult {
        let pokemon = pokemon_pos;

        // this.add('-start', pokemon, 'move: Heal Block');
        let pokemon_slot = {
            let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_ref.get_slot()
        };

        battle.add(
            "-start",
            &[
                crate::battle::Arg::from(pokemon_slot),
                crate::battle::Arg::from("move: Heal Block"),
            ],
        );

        // source.moveThisTurnResult = true;
        if let Some(source) = source_pos {
            let source_pokemon = match battle.pokemon_at_mut(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            source_pokemon.move_this_turn_result = Some(true);
        }

        EventResult::Continue
    }

    /// onDisableMove(pokemon) {
    ///     for (const moveSlot of pokemon.moveSlots) {
    ///         if (this.dex.moves.get(moveSlot.id).flags['heal']) {
    ///             pokemon.disableMove(moveSlot.id);
    ///         }
    ///     }
    /// }
    pub fn on_disable_move(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        let pokemon = pokemon_pos;

        // for (const moveSlot of pokemon.moveSlots)
        // Collect move IDs to disable (to avoid borrow checker issues)
        let moves_to_disable: Vec<crate::dex_data::ID> = {
            let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };

            pokemon_ref
                .move_slots
                .iter()
                .filter(|slot| {
                    // if (this.dex.moves.get(moveSlot.id).flags['heal'])
                    battle
                        .dex
                        .moves
                        .get(&slot.id)
                        .map(|move_data| move_data.flags.get("heal").unwrap_or(false))
                        .unwrap_or(false)
                })
                .map(|slot| slot.id.clone())
                .collect()
        };

        // pokemon.disableMove(moveSlot.id);
        let pokemon_mut = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let effect = crate::battle::Effect::move_("healblock");
        for move_id in moves_to_disable {
            pokemon_mut.disable_move(move_id.as_str(), false, Some(&effect));
        }

        EventResult::Continue
    }

    /// onBeforeMove(pokemon, target, move) {
    ///     if (move.flags['heal'] && !move.isZ && !move.isMax) {
    ///         this.add('cant', pokemon, 'move: Heal Block', move);
    ///         return false;
    ///     }
    /// }
    pub fn on_before_move(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        _target_pos: Option<(usize, usize)>,
        active_move: Option<&crate::battle_actions::ActiveMove>,
    ) -> EventResult {
        let move_id = active_move.map(|m| m.id.as_str()).unwrap_or("");
        use crate::dex_data::ID;

        let pokemon = pokemon_pos;

        // Check if move has heal flag
        let has_heal_flag = {
            let move_id_obj = ID::from(move_id);
            battle
                .dex
                .moves
                .get(&move_id_obj)
                .and_then(|move_data| move_data.flags.get("heal"))
                .is_some()
        };

        // Check if move is Z or Max move
        let (is_z, is_max) = if let Some(ref active_move) = battle.active_move {
            (active_move.is_z.is_some(), active_move.is_max.is_some())
        } else {
            (false, false)
        };

        // if (move.flags['heal'] && !move.isZ && !move.isMax)
        if has_heal_flag && !is_z && !is_max {
            // this.add('cant', pokemon, 'move: Heal Block', move);
            let pokemon_slot = {
                let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                pokemon_ref.get_slot()
            };

            // Get move name for display
            let move_name = {
                let move_id_obj = ID::from(move_id);
                battle
                    .dex
                    .moves().get_by_id(&move_id_obj)
                    .map(|m| m.name.clone())
                    .unwrap_or_else(|| move_id.to_string())
            };

            battle.add(
                "cant",
                &[
                    crate::battle::Arg::from(pokemon_slot),
                    crate::battle::Arg::from("move: Heal Block"),
                    crate::battle::Arg::from(move_name),
                ],
            );

            // return false;
            return EventResult::Boolean(false);
        }

        EventResult::Continue
    }

    /// onModifyMove(move, pokemon, target) {
    ///     if (move.flags['heal'] && !move.isZ && !move.isMax) {
    ///         this.add('cant', pokemon, 'move: Heal Block', move);
    ///         return false;
    ///     }
    /// }
    pub fn on_modify_move(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        _target_pos: Option<(usize, usize)>,
    ) -> EventResult {

        let pokemon = pokemon_pos;

        // Get the active move ID
        let move_id = if let Some(ref active_move) = battle.active_move {
            active_move.id.clone()
        } else {
            return EventResult::Continue;
        };

        // Check if move has heal flag
        let has_heal_flag = {
            battle
                .dex
                .moves
                .get(&move_id)
                .and_then(|move_data| move_data.flags.get("heal"))
                .is_some()
        };

        // Check if move is Z or Max move
        let (is_z, is_max) = if let Some(ref active_move) = battle.active_move {
            (active_move.is_z.is_some(), active_move.is_max.is_some())
        } else {
            (false, false)
        };

        // if (move.flags['heal'] && !move.isZ && !move.isMax)
        if has_heal_flag && !is_z && !is_max {
            // this.add('cant', pokemon, 'move: Heal Block', move);
            let pokemon_slot = {
                let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                pokemon_ref.get_slot()
            };

            // Get move name for display
            let move_name = {
                battle
                    .dex
                    .moves().get_by_id(&move_id)
                    .map(|m| m.name.clone())
                    .unwrap_or_else(|| move_id.to_string())
            };

            battle.add(
                "cant",
                &[
                    crate::battle::Arg::from(pokemon_slot),
                    crate::battle::Arg::from("move: Heal Block"),
                    crate::battle::Arg::from(move_name),
                ],
            );

            // return false;
            return EventResult::Boolean(false);
        }

        EventResult::Continue
    }

    /// onEnd(pokemon) {
    ///     this.add('-end', pokemon, 'move: Heal Block');
    /// }
    pub fn on_end(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        let pokemon = pokemon_pos;

        // this.add('-end', pokemon, 'move: Heal Block');
        let pokemon_slot = {
            let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_ref.get_slot()
        };

        battle.add(
            "-end",
            &[
                crate::battle::Arg::from(pokemon_slot),
                crate::battle::Arg::from("move: Heal Block"),
            ],
        );

        EventResult::Continue
    }

    /// onTryHeal(damage, target, source, effect) {
    ///     if (effect && (effect.id === 'zpower' || (effect as Move).isZ)) return damage;
    ///     if (source && target !== source && target.hp !== target.maxhp && effect.name === "Pollen Puff") {
    ///         this.attrLastMove('[still]');
    ///         // FIXME: Wrong error message, correct one not supported yet
    ///         this.add('cant', source, 'move: Heal Block', effect);
    ///         return null;
    ///     }
    ///     return false;
    /// }
    pub fn on_try_heal(
        battle: &mut Battle,
        damage: i32,
        target_pos: Option<(usize, usize)>,
        source_pos: Option<(usize, usize)>,
        effect_id: Option<&str>,
    ) -> EventResult {
        use crate::dex_data::ID;

        // if (effect && (effect.id === 'zpower' || (effect as Move).isZ)) return damage;
        if let Some(eff_id) = effect_id {
            // Check if effect is zpower
            if eff_id == "zpower" {
                return EventResult::Number(damage);
            }

            // Check if the active move is a Z-move
            if let Some(ref active_move) = battle.active_move {
                if active_move.is_z.is_some() {
                    return EventResult::Number(damage);
                }
            }
        }

        // if (source && target !== source && target.hp !== target.maxhp && effect.name === "Pollen Puff")
        if let (Some(source), Some(target)) = (source_pos, target_pos) {
            // target !== source
            if target != source {
                // target.hp !== target.maxhp
                let (hp, maxhp) = {
                    let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                        Some(p) => p,
                        None => return EventResult::Boolean(false),
                    };
                    (target_pokemon.hp, target_pokemon.maxhp)
                };

                if hp != maxhp {
                    // effect.name === "Pollen Puff"
                    if let Some(eff_id) = effect_id {
                        let effect_id_obj = ID::from(eff_id);
                        if let Some(move_data) = battle.dex.moves().get_by_id(&effect_id_obj) {
                            if move_data.name == "Pollen Puff" {
                                // this.attrLastMove('[still]');
                                // NOTE: attrLastMove not implemented yet (see FIXME comment in TypeScript)

                                // this.add('cant', source, 'move: Heal Block', effect);
                                let source_slot = {
                                    let source_pokemon = match battle.pokemon_at(source.0, source.1)
                                    {
                                        Some(p) => p,
                                        None => return EventResult::Boolean(false),
                                    };
                                    source_pokemon.get_slot()
                                };

                                let effect_name = move_data.name.clone();

                                battle.add(
                                    "cant",
                                    &[
                                        crate::battle::Arg::from(source_slot),
                                        crate::battle::Arg::from("move: Heal Block"),
                                        crate::battle::Arg::from(effect_name),
                                    ],
                                );

                                // return null;
                                return EventResult::Null;
                            }
                        }
                    }
                }
            }
        }

        // return false;
        EventResult::Boolean(false)
    }

    /// ```ignore
    /// onRestart(target, source, effect) {
    ///     if (effect?.name === 'Psychic Noise') return;
    ///
    ///     this.add('-fail', target, 'move: Heal Block'); // Succeeds to suppress downstream messages
    ///     if (!source.moveThisTurnResult) {
    ///         source.moveThisTurnResult = false;
    ///     }
    /// }
    /// ```
    pub fn on_restart(
        battle: &mut Battle,
        target_pos: Option<(usize, usize)>,
        source_pos: Option<(usize, usize)>,
        effect_id: Option<&str>,
    ) -> EventResult {
        // if (effect?.name === 'Psychic Noise') return;
        if let Some(eff_id) = effect_id {
            use crate::dex_data::ID;
            let effect_id_obj = ID::from(eff_id);

            if let Some(move_data) = battle.dex.moves().get_by_id(&effect_id_obj) {
                if move_data.name == "Psychic Noise" {
                    return EventResult::Continue;
                }
            }
        }

        // this.add('-fail', target, 'move: Heal Block');
        if let Some(target) = target_pos {
            let target_slot = {
                let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                target_pokemon.get_slot()
            };

            battle.add(
                "-fail",
                &[
                    crate::battle::Arg::from(target_slot),
                    crate::battle::Arg::from("move: Heal Block"),
                ],
            );
        }

        // if (!source.moveThisTurnResult) {
        //     source.moveThisTurnResult = false;
        // }
        if let Some(source) = source_pos {
            let source_pokemon = match battle.pokemon_at_mut(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };

            if source_pokemon.move_this_turn_result.is_none() {
                source_pokemon.move_this_turn_result = Some(false);
            }
        }

        EventResult::Continue
    }
}
