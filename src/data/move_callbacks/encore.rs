//! Encore Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

pub mod condition {
    use super::*;

    /// ```ignore
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
    /// ```
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
            (
                target_pokemon.last_move.clone(),
                target_pokemon.volatiles.contains_key(&ID::from("dynamax")),
            )
        };

        let move_id = match last_move_id {
            Some(id) => id,
            None => return EventResult::Boolean(false),
        };

        if has_dynamax {
            return EventResult::Boolean(false);
        }

        // Encore only works on Max Moves if the base move is not itself a Max Move
        // if (move.isMax && move.baseMove) move = this.dex.moves.get(move.baseMove);
        let actual_move_id = {
            let move_data = battle.dex.moves().get_by_id(&move_id);
            if let Some(m) = move_data {
                if m.is_max.is_some() {
                    if let Some(ref base_move) = m.base_move {
                        base_move.clone()
                    } else {
                        move_id.clone()
                    }
                } else {
                    move_id.clone()
                }
            } else {
                move_id.clone()
            }
        };

        // const moveSlot = target.getMoveData(move.id);
        // if (move.isZ || move.isMax || move.flags['failencore'] || !moveSlot || moveSlot.pp <= 0) {
        //     return false;
        // }
        let (move_slot_valid, move_has_fail_encore, move_is_z_or_max) = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };

            // Check if move has failencore flag and if it's Z or Max
            let move_data = battle.dex.moves().get_by_id(&actual_move_id);
            let (has_fail_encore, is_z_or_max) = if let Some(m) = move_data {
                (
                    m.flags.contains_key("failencore"),
                    m.is_z.is_some() || m.is_max.is_some(),
                )
            } else {
                (false, false)
            };

            // Find the move slot
            let move_slot = target_pokemon
                .move_slots
                .iter()
                .find(|s| s.id == actual_move_id);
            let slot_valid = move_slot.map(|s| s.pp > 0).unwrap_or(false);

            (slot_valid, has_fail_encore, is_z_or_max)
        };

        if move_has_fail_encore || !move_slot_valid || move_is_z_or_max {
            // it failed
            return EventResult::Boolean(false);
        }

        // this.effectState.move = move.id;
        if let Some(ref mut effect_state) = battle.current_effect_state {
            effect_state.data.insert(
                "move".to_string(),
                serde_json::to_value(actual_move_id.to_string()).unwrap(),
            );
        }

        // this.add('-start', target, 'Encore');
        let target_ident = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.get_slot()
        };

        battle.add("-start", &[target_ident.as_str().into(), "Encore".into()]);

        // if (!this.queue.willMove(target)) {
        //     this.effectState.duration!++;
        // }
        let will_move = battle.queue.will_move(target.0, target.1);
        if will_move.is_none() {
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
    pub fn on_override_action(
        battle: &mut Battle,
        _pokemon_pos: (usize, usize),
        _target_pos: Option<(usize, usize)>,
        move_id: &str,
    ) -> EventResult {
        use crate::dex_data::ID;

        // if (move.id !== this.effectState.move) return this.effectState.move;
        let encore_move_id = if let Some(ref effect_state) = battle.current_effect_state {
            effect_state
                .data
                .get("move")
                .and_then(|v| v.as_str())
                .map(ID::from)
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
            effect_state
                .data
                .get("move")
                .and_then(|v| v.as_str())
                .map(ID::from)
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
                Pokemon::remove_volatile(battle, target, &ID::from("encore"));
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
        let target_ident = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.get_slot()
        };

        battle.add("-end", &[target_ident.as_str().into(), "Encore".into()]);

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
            effect_state
                .data
                .get("move")
                .and_then(|v| v.as_str())
                .map(ID::from)
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
            pokemon_pokemon.has_move(encore_id.as_str())
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

            pokemon_pokemon
                .move_slots
                .iter()
                .filter(|slot| slot.id != encore_id)
                .map(|slot| slot.id.clone())
                .collect()
        };

        let pokemon_pokemon = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        for move_id in move_ids_to_disable {
            pokemon_pokemon.disable_move(move_id.as_str(), false, None);
        }

        EventResult::Continue
    }
}
