//! Taunt Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

pub mod condition {
    use super::*;

    /// onStart(target) {
    ///     if (target.activeTurns && !this.queue.willMove(target)) {
    ///         this.effectState.duration!++;
    ///     }
    ///     this.add('-start', target, 'move: Taunt');
    /// }
    pub fn on_start(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // if (target.activeTurns && !this.queue.willMove(target)) {
        //     this.effectState.duration!++;
        // }
        let should_extend_duration = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };

            let active_turns = target_pokemon.active_turns;
            let will_move = battle.queue.will_move(target.0, target.1);

            active_turns > 0 && will_move.is_none()
        };

        if should_extend_duration {
            battle.with_effect_state(|state| {
                if let Some(duration) = state.duration {
                    state.duration = Some(duration + 1);
                }
            });
        }

        // this.add('-start', target, 'move: Taunt');
        let target_slot = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.get_slot()
        };

        battle.add(
            "-start",
            &[
                crate::battle::Arg::from(target_slot),
                crate::battle::Arg::from("move: Taunt"),
            ],
        );

        EventResult::Continue
    }

    /// onEnd(target) {
    ///     this.add('-end', target, 'move: Taunt');
    /// }
    pub fn on_end(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // this.add('-end', target, 'move: Taunt');
        let target_slot = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.get_slot()
        };

        battle.add(
            "-end",
            &[
                crate::battle::Arg::from(target_slot),
                crate::battle::Arg::from("move: Taunt"),
            ],
        );

        EventResult::Continue
    }

    /// onDisableMove(pokemon) {
    ///     for (const moveSlot of pokemon.moveSlots) {
    ///         const move = this.dex.moves.get(moveSlot.id);
    ///         if (move.category === 'Status' && move.id !== 'mefirst') {
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
                    // const move = this.dex.moves.get(moveSlot.id);
                    // if (move.category === 'Status' && move.id !== 'mefirst')
                    battle
                        .dex
                        .moves
                        .get(&slot.id)
                        .map(|move_data| {
                            move_data.category == "Status"
                                && move_data.id.as_str() != "mefirst"
                        })
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

        let effect = crate::battle::Effect::move_("taunt");
        for move_id in moves_to_disable {
            pokemon_mut.disable_move(move_id.as_str(), false, Some(&effect));
        }

        EventResult::Continue
    }

    /// onBeforeMove(attacker, defender, move) {
    ///     if (!(move.isZ && move.isZOrMaxPowered) && move.category === 'Status' && move.id !== 'mefirst') {
    ///         this.add('cant', attacker, 'move: Taunt', move);
    ///         return false;
    ///     }
    /// }
    pub fn on_before_move(
        battle: &mut Battle,
        attacker_pos: (usize, usize),
        _active_move: Option<&crate::battle_actions::ActiveMove>,
    ) -> EventResult {
        // Get active move properties
        let (is_z, is_z_or_max_powered, category, move_id, move_name) = match &battle.active_move {
            Some(m) => {
                let is_z_or_max = m.is_z.is_some() || m.is_max;
                let name = battle.dex.moves().get_by_id(&m.id)
                    .map(|md| md.name.clone())
                    .unwrap_or_else(|| m.id.to_string());
                (m.is_z.is_some(), is_z_or_max, m.category.clone(), m.id.clone(), name)
            }
            None => return EventResult::Continue,
        };

        // if (!(move.isZ && move.isZOrMaxPowered) && move.category === 'Status' && move.id !== 'mefirst')
        let is_z_and_powered = is_z && is_z_or_max_powered;
        if !is_z_and_powered && category == "Status" && move_id.as_str() != "mefirst" {
            // this.add('cant', attacker, 'move: Taunt', move);
            let attacker_slot = {
                let attacker_pokemon = match battle.pokemon_at(attacker_pos.0, attacker_pos.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                attacker_pokemon.get_slot()
            };

            battle.add(
                "cant",
                &[
                    crate::battle::Arg::from(attacker_slot),
                    crate::battle::Arg::from("move: Taunt"),
                    crate::battle::Arg::from(move_name),
                ],
            );

            // return false;
            return EventResult::Boolean(false);
        }

        EventResult::Continue
    }
}
