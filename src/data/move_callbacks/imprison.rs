//! Imprison Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;


pub mod condition {
    use super::*;

    /// onStart(target) {
    ///     this.add('-start', target, 'move: Imprison');
    /// }
    pub fn on_start(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // this.add('-start', target, 'move: Imprison');
        let target_arg = crate::battle::Arg::Pos(target.0, target.1);
        battle.add("-start", &[target_arg, "move: Imprison".into()]);

        EventResult::Continue
    }

    /// onFoeDisableMove(pokemon) {
    ///     for (const moveSlot of this.effectState.source.moveSlots) {
    ///         if (moveSlot.id === 'struggle') continue;
    ///         pokemon.disableMove(moveSlot.id, true);
    ///     }
    ///     pokemon.maybeDisabled = true;
    /// }
    pub fn on_foe_disable_move(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        use crate::dex_data::ID;

        let pokemon = pokemon_pos;

        // for (const moveSlot of this.effectState.source.moveSlots) {
        //     if (moveSlot.id === 'struggle') continue;
        //     pokemon.disableMove(moveSlot.id, true);
        // }
        let source_move_ids: Vec<ID> = {
            let source_pos = match &battle.current_effect_state {
                Some(state) => match state.source {
                    Some(pos) => pos,
                    None => return EventResult::Continue,
                },
                None => return EventResult::Continue,
            };

            let source_pokemon = match battle.pokemon_at(source_pos.0, source_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };

            source_pokemon.move_slots.iter()
                .map(|slot| slot.id.clone())
                .filter(|id| id != &ID::from("struggle"))
                .collect()
        };

        for move_id in source_move_ids {
            battle.disable_move(pokemon, &move_id, true);
        }

        // pokemon.maybeDisabled = true;
        {
            let pokemon_pokemon = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_pokemon.maybe_disabled = true;
        }

        EventResult::Continue
    }

    /// onFoeBeforeMove(attacker, defender, move) {
    ///     if (move.id !== 'struggle' && this.effectState.source.hasMove(move.id) && !move.isZOrMaxPowered) {
    ///         this.add('cant', attacker, 'move: Imprison', move);
    ///         return false;
    ///     }
    /// }
    pub fn on_foe_before_move(battle: &mut Battle, move_id: &str) -> EventResult {
        use crate::dex_data::ID;

        // if (move.id !== 'struggle' && this.effectState.source.hasMove(move.id) && !move.isZOrMaxPowered) {
        if move_id == "struggle" {
            return EventResult::Continue;
        }

        let move_id_obj = ID::from(move_id);

        // Check if move is Z or Max powered
        let is_z_or_max_powered = {
            battle.active_move.as_ref().map(|m| m.is_z_or_max_powered).unwrap_or(false)
        };

        if is_z_or_max_powered {
            return EventResult::Continue;
        }

        // Check if source has this move
        let source_has_move = {
            let source_pos = match &battle.current_effect_state {
                Some(state) => match state.source {
                    Some(pos) => pos,
                    None => return EventResult::Continue,
                },
                None => return EventResult::Continue,
            };

            let source_pokemon = match battle.pokemon_at(source_pos.0, source_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };

            source_pokemon.has_move(move_id_obj.as_str())
        };

        if source_has_move {
            // this.add('cant', attacker, 'move: Imprison', move);
            // We need to get the attacker position from current_effect_state.target
            let attacker_pos = match &battle.current_effect_state {
                Some(state) => match state.target {
                    Some(pos) => pos,
                    None => return EventResult::Continue,
                },
                None => return EventResult::Continue,
            };

            let attacker_arg = crate::battle::Arg::Pos(attacker_pos.0, attacker_pos.1);
            let move_arg = crate::battle::Arg::Move(move_id_obj);
            battle.add("cant", &[attacker_arg, "move: Imprison".into(), move_arg]);

            // return false;
            return EventResult::Boolean(false);
        }

        EventResult::Continue
    }
}
