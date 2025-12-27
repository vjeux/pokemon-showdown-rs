//! Encore Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;


pub mod condition {
    use super::*;

    /// onStart(target) {
    ///     let move: Move | ActiveMove | null = target.lastMove;
    ///     if (!move || target.volatiles['dynamax']) return false;
    ///
    ///     // Encore only works on Max Moves if the base move is not itself a Max Move
    ///     if (move.isMax && move.baseMove) move = this.dex.moves.get(move.baseMove);
    ///     const moveSlot = target.getMoveData(move.id);
    ///     if (move.isZ || move.isMax || move.flags['failencore'] || !moveSlot || moveSlot.pp <= 0) {
    ///         // it failed
    ///         return false;
    ///     }
    ///     this.effectState.move = move.id;
    ///     this.add('-start', target, 'Encore');
    ///     if (!this.queue.willMove(target)) {
    ///         this.effectState.duration!++;
    ///     }
    /// }
    pub fn on_start(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        use crate::dex_data::ID;

        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // let move: Move | ActiveMove | null = target.lastMove;
        // if (!move || target.volatiles['dynamax']) return false;
        let (last_move_id, has_dynamax) = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            (target_pokemon.last_move.clone(), target_pokemon.volatiles.contains_key(&ID::from("dynamax")))
        };

        let move_id = match last_move_id {
            Some(id) => id,
            None => return EventResult::Bool(false),
        };

        if has_dynamax {
            return EventResult::Bool(false);
        }

        // Encore only works on Max Moves if the base move is not itself a Max Move
        // if (move.isMax && move.baseMove) move = this.dex.moves.get(move.baseMove);
        // For now, just use the move as-is since we don't have isMax/baseMove info readily available

        // const moveSlot = target.getMoveData(move.id);
        // if (move.isZ || move.isMax || move.flags['failencore'] || !moveSlot || moveSlot.pp <= 0) {
        //     return false;
        // }
        let (move_slot_valid, move_has_fail_encore) = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };

            // Check if move has failencore flag
            let move_data = battle.dex.get_move_by_id(&move_id);
            let has_fail_encore = move_data
                .map(|m| m.flags.contains_key("failencore"))
                .unwrap_or(false);

            // Find the move slot
            let move_slot = target_pokemon.move_slots.iter().find(|s| s.id == move_id);
            let slot_valid = move_slot.map(|s| s.pp > 0).unwrap_or(false);

            (slot_valid, has_fail_encore)
        };

        if move_has_fail_encore || !move_slot_valid {
            // it failed
            return EventResult::Bool(false);
        }

        // this.effectState.move = move.id;
        if let Some(ref mut effect_state) = battle.current_effect_state {
            effect_state.data.insert("move".to_string(), serde_json::to_value(move_id.to_string()).unwrap());
        }

        // this.add('-start', target, 'Encore');
        let target_arg = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            crate::battle::Arg::from(target_pokemon)
        };

        battle.add("-start", &[target_arg, "Encore".into()]);

        // if (!this.queue.willMove(target)) {
        //     this.effectState.duration!++;
        // }
        let will_move = battle.queue.will_move(target);
        if !will_move {
            if let Some(ref mut effect_state) = battle.current_effect_state {
                if let Some(duration) = effect_state.duration {
                    effect_state.duration = Some(duration + 1);
                }
            }
        }

        EventResult::Continue
    }

    /// onOverrideAction(pokemon, target, move) {
    ///     if (move.id !== this.effectState.move) return this.effectState.move;
    /// }
    pub fn on_override_action(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
        use crate::dex_data::ID;

        // if (move.id !== this.effectState.move) return this.effectState.move;
        let encore_move_id = if let Some(ref effect_state) = battle.current_effect_state {
            effect_state.data.get("move")
                .and_then(|v| v.as_str())
                .map(|s| ID::from(s))
        } else {
            None
        };

        if let Some(encore_id) = encore_move_id {
            if move_id != encore_id.as_str() {
                // return this.effectState.move;
                return EventResult::ID(encore_id);
            }
        }

        EventResult::Continue
    }

    /// onResidual(target) {
    ///     const moveSlot = target.getMoveData(this.effectState.move);
    ///     if (!moveSlot || moveSlot.pp <= 0) {
    ///         // early termination if you run out of PP
    ///         target.removeVolatile('encore');
    ///     }
    /// }
    pub fn on_residual(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        use crate::dex_data::ID;

        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // const moveSlot = target.getMoveData(this.effectState.move);
        // if (!moveSlot || moveSlot.pp <= 0) {
        //     target.removeVolatile('encore');
        // }
        let encore_move_id = if let Some(ref effect_state) = battle.current_effect_state {
            effect_state.data.get("move")
                .and_then(|v| v.as_str())
                .map(|s| ID::from(s))
        } else {
            None
        };

        if let Some(encore_id) = encore_move_id {
            let should_remove = {
                let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };

                // Find the move slot
                let move_slot = target_pokemon.move_slots.iter().find(|s| s.id == encore_id);
                !move_slot.map(|s| s.pp > 0).unwrap_or(false)
            };

            if should_remove {
                // early termination if you run out of PP
                // target.removeVolatile('encore');
                let target_pokemon = match battle.pokemon_at_mut(target.0, target.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                target_pokemon.remove_volatile(&ID::from("encore"));
            }
        }

        EventResult::Continue
    }

    /// onEnd(target) {
    ///     this.add('-end', target, 'Encore');
    /// }
    pub fn on_end(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // this.add('-end', target, 'Encore');
        let target_arg = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            crate::battle::Arg::from(target_pokemon)
        };

        battle.add("-end", &[target_arg, "Encore".into()]);

        EventResult::Continue
    }

    /// onDisableMove(pokemon) {
    ///     if (!this.effectState.move || !pokemon.hasMove(this.effectState.move)) {
    ///         return;
    ///     }
    ///     for (const moveSlot of pokemon.moveSlots) {
    ///         if (moveSlot.id !== this.effectState.move) {
    ///             pokemon.disableMove(moveSlot.id);
    ///         }
    ///     }
    /// }
    pub fn on_disable_move(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        use crate::dex_data::ID;

        let pokemon = pokemon_pos;

        // if (!this.effectState.move || !pokemon.hasMove(this.effectState.move)) {
        //     return;
        // }
        let encore_move_id = if let Some(ref effect_state) = battle.current_effect_state {
            effect_state.data.get("move")
                .and_then(|v| v.as_str())
                .map(|s| ID::from(s))
        } else {
            None
        };

        let encore_id = match encore_move_id {
            Some(id) => id,
            None => return EventResult::Continue,
        };

        let has_move = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_pokemon.has_move(&encore_id)
        };

        if !has_move {
            // return;
            return EventResult::Continue;
        }

        // for (const moveSlot of pokemon.moveSlots) {
        //     if (moveSlot.id !== this.effectState.move) {
        //         pokemon.disableMove(moveSlot.id);
        //     }
        // }
        let move_ids_to_disable: Vec<ID> = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };

            pokemon_pokemon.move_slots.iter()
                .filter(|slot| slot.id != encore_id)
                .map(|slot| slot.id.clone())
                .collect()
        };

        let pokemon_pokemon = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        for move_id in move_ids_to_disable {
            pokemon_pokemon.disable_move(&move_id);
        }

        EventResult::Continue
    }
}
