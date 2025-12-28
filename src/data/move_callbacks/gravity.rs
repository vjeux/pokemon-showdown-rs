//! Gravity Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;


pub mod condition {
    use super::*;

    /// durationCallback(source, effect) {
    ///     if (source?.hasAbility('persistent')) {
    ///         this.add('-activate', source, 'ability: Persistent', '[move] Gravity');
    ///         return 7;
    ///     }
    ///     return 5;
    /// }
    pub fn duration_callback(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
        use crate::dex_data::ID;

        // if (source?.hasAbility('persistent')) {
        if let Some(source) = source_pos {
            let has_persistent = {
                let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                    Some(p) => p,
                    None => return EventResult::Number(5),
                };
                source_pokemon.has_ability(&["persistent"])
            };

            if has_persistent {
                // this.add('-activate', source, 'ability: Persistent', '[move] Gravity');
                let source_arg = {
                    let pokemon = match battle.pokemon_at(source.0, source.1) {
                        Some(p) => p,
                        None => return EventResult::Continue,
                    };
                    crate::battle::Arg::from(pokemon)
                };
                battle.add("-activate", &[source_arg, "ability: Persistent".into(), "[move] Gravity".into()]);

                // return 7;
                return EventResult::Number(7);
            }
        }

        // return 5;
        EventResult::Number(5)
    }

    /// onFieldStart(target, source) {
    ///     if (source?.hasAbility('persistent')) {
    ///         this.add('-fieldstart', 'move: Gravity', '[persistent]');
    ///     } else {
    ///         this.add('-fieldstart', 'move: Gravity');
    ///     }
    ///     for (const pokemon of this.getAllActive()) {
    ///         let applies = false;
    ///         if (pokemon.removeVolatile('bounce') || pokemon.removeVolatile('fly')) {
    ///             applies = true;
    ///             this.queue.cancelMove(pokemon);
    ///             pokemon.removeVolatile('twoturnmove');
    ///         }
    ///         if (pokemon.volatiles['skydrop']) {
    ///             applies = true;
    ///             this.queue.cancelMove(pokemon);
    /// 
    ///             if (pokemon.volatiles['skydrop'].source) {
    ///                 this.add('-end', pokemon.volatiles['twoturnmove'].source, 'Sky Drop', '[interrupt]');
    ///             }
    ///             pokemon.removeVolatile('skydrop');
    ///             pokemon.removeVolatile('twoturnmove');
    ///         }
    ///         if (pokemon.volatiles['magnetrise']) {
    ///             applies = true;
    ///             delete pokemon.volatiles['magnetrise'];
    ///         }
    ///         if (pokemon.volatiles['telekinesis']) {
    ///             applies = true;
    ///             delete pokemon.volatiles['telekinesis'];
    ///         }
    ///         if (applies) this.add('-activate', pokemon, 'move: Gravity');
    ///     }
    /// }
    pub fn on_field_start(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>) -> EventResult {
        use crate::dex_data::ID;

        // if (source?.hasAbility('persistent')) {
        let has_persistent = if let Some(source) = source_pos {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            source_pokemon.has_ability(&["persistent"])
        } else {
            false
        };

        if has_persistent {
            // this.add('-fieldstart', 'move: Gravity', '[persistent]');
            battle.add("-fieldstart", &["move: Gravity".into(), "[persistent]".into()]);
        } else {
            // this.add('-fieldstart', 'move: Gravity');
            battle.add("-fieldstart", &["move: Gravity".into()]);
        }

        // for (const pokemon of this.getAllActive()) {
        let all_active = battle.get_all_active(false);

        for pokemon_pos in all_active {
            let mut applies = false;

            // if (pokemon.removeVolatile('bounce') || pokemon.removeVolatile('fly')) {
            let removed_bounce = {
                let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
                    Some(p) => p,
                    None => continue,
                };
                pokemon.remove_volatile(&ID::from("bounce"))
            };

            let removed_fly = {
                let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
                    Some(p) => p,
                    None => continue,
                };
                pokemon.remove_volatile(&ID::from("fly"))
            };

            if removed_bounce || removed_fly {
                // applies = true;
                applies = true;

                // this.queue.cancelMove(pokemon);
                battle.queue.cancel_move(pokemon_pos.0, pokemon_pos.1);

                // pokemon.removeVolatile('twoturnmove');
                let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
                    Some(p) => p,
                    None => continue,
                };
                pokemon.remove_volatile(&ID::from("twoturnmove"));
            }

            // if (pokemon.volatiles['skydrop']) {
            let has_skydrop = {
                let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                    Some(p) => p,
                    None => continue,
                };
                pokemon.volatiles.contains_key(&ID::from("skydrop"))
            };

            if has_skydrop {
                // applies = true;
                applies = true;

                // this.queue.cancelMove(pokemon);
                battle.queue.cancel_move(pokemon_pos.0, pokemon_pos.1);

                // if (pokemon.volatiles['skydrop'].source) {
                //     this.add('-end', pokemon.volatiles['twoturnmove'].source, 'Sky Drop', '[interrupt]');
                // }
                // Note: The TypeScript accesses skydrop.source but shows twoturnmove.source in the message
                // We'll check for the source in the skydrop volatile
                let skydrop_source = {
                    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                        Some(p) => p,
                        None => continue,
                    };
                    pokemon.volatiles.get(&ID::from("skydrop"))
                        .and_then(|v| v.source)
                };

                if let Some(source) = skydrop_source {
                    let source_arg = {
                        let pokemon = match battle.pokemon_at(source.0, source.1) {
                            Some(p) => p,
                            None => continue,
                        };
                        crate::battle::Arg::from(pokemon)
                    };
                    battle.add("-end", &[source_arg, "Sky Drop".into(), "[interrupt]".into()]);
                }

                // pokemon.removeVolatile('skydrop');
                // pokemon.removeVolatile('twoturnmove');
                let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
                    Some(p) => p,
                    None => continue,
                };
                pokemon.remove_volatile(&ID::from("skydrop"));
                pokemon.remove_volatile(&ID::from("twoturnmove"));
            }

            // if (pokemon.volatiles['magnetrise']) {
            let has_magnetrise = {
                let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                    Some(p) => p,
                    None => continue,
                };
                pokemon.volatiles.contains_key(&ID::from("magnetrise"))
            };

            if has_magnetrise {
                // applies = true;
                applies = true;

                // delete pokemon.volatiles['magnetrise'];
                let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
                    Some(p) => p,
                    None => continue,
                };
                pokemon.volatiles.remove(&ID::from("magnetrise"));
            }

            // if (pokemon.volatiles['telekinesis']) {
            let has_telekinesis = {
                let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                    Some(p) => p,
                    None => continue,
                };
                pokemon.volatiles.contains_key(&ID::from("telekinesis"))
            };

            if has_telekinesis {
                // applies = true;
                applies = true;

                // delete pokemon.volatiles['telekinesis'];
                let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
                    Some(p) => p,
                    None => continue,
                };
                pokemon.volatiles.remove(&ID::from("telekinesis"));
            }

            // if (applies) this.add('-activate', pokemon, 'move: Gravity');
            if applies {
                let pokemon_arg = {
                    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                        Some(p) => p,
                        None => continue,
                    };
                    crate::battle::Arg::from(pokemon)
                };
                battle.add("-activate", &[pokemon_arg, "move: Gravity".into()]);
            }
        }

        EventResult::Continue
    }

    /// onModifyAccuracy(accuracy) {
    ///     if (typeof accuracy !== 'number') return;
    ///     return this.chainModify([6840, 4096]);
    /// }
    pub fn on_modify_accuracy(battle: &mut Battle, accuracy: i32) -> EventResult {
        // if (typeof accuracy !== 'number') return;
        // Already guaranteed to be a number by the type system

        // return this.chainModify([6840, 4096]);
        EventResult::Number(battle.chain_modify(6840.0 / 4096.0 as f32))
    }

    /// onDisableMove(pokemon) {
    ///     for (const moveSlot of pokemon.moveSlots) {
    ///         if (this.dex.moves.get(moveSlot.id).flags['gravity']) {
    ///             pokemon.disableMove(moveSlot.id);
    ///         }
    ///     }
    /// }
    pub fn on_disable_move(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        use crate::dex_data::ID;

        let pokemon = pokemon_pos;

        // for (const moveSlot of pokemon.moveSlots) {
        let move_slots = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_pokemon.move_slots.clone()
        };

        for move_slot in move_slots {
            // if (this.dex.moves.get(moveSlot.id).flags['gravity']) {
            let has_gravity_flag = if let Some(move_data) = battle.dex.get_move_by_id(&move_slot.id) {
                move_data.flags.contains_key("gravity")
            } else {
                false
            };

            if has_gravity_flag {
                // pokemon.disableMove(moveSlot.id);
                let pokemon_pokemon = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                pokemon_pokemon.disable_move(&move_slot.id, None);
            }
        }

        EventResult::Continue
    }

    /// onBeforeMove(pokemon, target, move) {
    ///     if (move.flags['gravity'] && !move.isZ) {
    ///         this.add('cant', pokemon, 'move: Gravity', move);
    ///         return false;
    ///     }
    /// }
    pub fn on_before_move(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
        let pokemon = pokemon_pos;

        // if (move.flags['gravity'] && !move.isZ) {
        let has_gravity_flag = if let Some(move_data) = battle.active_move.as_ref() {
            move_data.flags.contains_key("gravity")
        } else {
            false
        };

        let is_z_move = if let Some(move_data) = battle.active_move.as_ref() {
            move_data.is_z
        } else {
            false
        };

        if has_gravity_flag && !is_z_move {
            // this.add('cant', pokemon, 'move: Gravity', move);
            let pokemon_arg = {
                let poke = match battle.pokemon_at(pokemon.0, pokemon.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                crate::battle::Arg::from(poke)
            };
            let move_arg = crate::battle::Arg::from(move_id);
            battle.add("cant", &[pokemon_arg, "move: Gravity".into(), move_arg]);

            // return false;
            return EventResult::Boolean(false);
        }

        EventResult::Continue
    }

    /// onModifyMove(move, pokemon, target) {
    ///     if (move.flags['gravity'] && !move.isZ) {
    ///         this.add('cant', pokemon, 'move: Gravity', move);
    ///         return false;
    ///     }
    /// }
    pub fn on_modify_move(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
        let pokemon = pokemon_pos;

        // if (move.flags['gravity'] && !move.isZ) {
        let has_gravity_flag = if let Some(move_data) = battle.active_move.as_ref() {
            move_data.flags.contains_key("gravity")
        } else {
            false
        };

        let is_z_move = if let Some(move_data) = battle.active_move.as_ref() {
            move_data.is_z
        } else {
            false
        };

        let move_id = battle.active_move.as_ref().map(|m| m.to_string()).unwrap_or_default();

        if has_gravity_flag && !is_z_move {
            // this.add('cant', pokemon, 'move: Gravity', move);
            let pokemon_arg = {
                let poke = match battle.pokemon_at(pokemon.0, pokemon.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                crate::battle::Arg::from(poke)
            };
            let move_arg = crate::battle::Arg::from(move_id.as_str());
            battle.add("cant", &[pokemon_arg, "move: Gravity".into(), move_arg]);

            // return false;
            return EventResult::Boolean(false);
        }

        EventResult::Continue
    }

    /// onFieldEnd() {
    ///     this.add('-fieldend', 'move: Gravity');
    /// }
    pub fn on_field_end(battle: &mut Battle) -> EventResult {
        // this.add('-fieldend', 'move: Gravity');
        battle.add("-fieldend", &["move: Gravity".into()]);

        EventResult::Continue
    }
}
