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
    pub fn on_start(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        _source_pos: Option<(usize, usize)>,
        _effect: Option<&crate::battle::Effect>,
    ) -> EventResult {
        use crate::dex_data::ID;

        let target = pokemon_pos;

        // let move: Move | ActiveMove | null = target.lastMove;
        // if (!move || target.volatiles['dynamax']) return false;
        // JavaScript uses target.lastMove which is the full ActiveMove with runtime flags
        // We use last_move_used for the full ActiveMove, last_move for the ID
        let (last_move_used, last_move_id, has_dynamax) = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            (
                target_pokemon.last_move_used.clone(),
                target_pokemon.last_move.clone(),
                target_pokemon.volatiles.contains_key(&ID::from("dynamax")),
            )
        };

        let last_move_used = match last_move_used {
            Some(m) => m,
            None => return EventResult::Boolean(false),
        };
        let move_id = last_move_id.unwrap_or_else(|| last_move_used.id.clone());

        if has_dynamax {
            return EventResult::Boolean(false);
        }

        // Encore only works on Max Moves if the base move is not itself a Max Move
        // if (move.isMax && move.baseMove) move = this.dex.moves.get(move.baseMove);
        // Check runtime isMax from last_move_used
        let actual_move_id = {
            if last_move_used.is_max.is_some() {
                if let Some(ref base_move) = last_move_used.base_move {
                    base_move.clone()
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
        // Check isZ and isMax from runtime ActiveMove (last_move_used)
        let (move_slot_valid, move_has_fail_encore, move_is_z_or_max) = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };

            // Check if move has failencore flag from dex data
            let move_data = battle.dex.moves().get_by_id(&actual_move_id);
            let has_fail_encore = move_data.map(|m| m.flags.contains_key("failencore")).unwrap_or(false);

            // isZ and isMax are runtime flags from the ActiveMove (last_move_used)
            let is_z_or_max = last_move_used.is_z.is_some() || last_move_used.is_max.is_some();

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
        battle.with_effect_state(|state| {
            state.move_id = Some(actual_move_id.to_string());
        });

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
            battle.with_effect_state(|state| {
                if let Some(duration) = state.duration {
                    state.duration = Some(duration + 1);
                }
            });
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
        active_move: Option<&crate::battle_actions::ActiveMove>,
    ) -> EventResult {
        let move_id = active_move.map(|m| m.id.to_string()).unwrap_or_default();
        use crate::dex_data::ID;

        // if (move.id !== this.effectState.move) return this.effectState.move;
        let encore_move_id = battle.with_effect_state_ref(|state| {
            state.move_id.clone().map(ID::from)
        }).flatten();

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
        let encore_move_id = battle.with_effect_state_ref(|state| {
            state.move_id.clone().map(ID::from)
        }).flatten();

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
        let encore_move_id = battle.with_effect_state_ref(|state| {
            state.move_id.clone().map(ID::from)
        }).flatten();

        let encore_id = match encore_move_id {
            Some(id) => id,
            None => return EventResult::Continue,
        };

        let has_move = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            // Create ActiveMove from ID to call has_move
            battle.dex.get_active_move(encore_id.as_str())
                .map(|am| pokemon_pokemon.has_move(&am))
                .unwrap_or(false)
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
