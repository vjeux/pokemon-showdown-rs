//! Throat Chop Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// Secondary onHit callback
/// JavaScript: secondary: { chance: 100, onHit(target) { target.addVolatile('throatchop'); } }
pub fn on_hit(
    battle: &mut Battle,
    target_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
) -> EventResult {
    // target.addVolatile('throatchop');
    crate::pokemon::Pokemon::add_volatile(
        battle,
        target_pos,
        crate::dex_data::ID::from("throatchop"),
        source_pos,
        Some(&crate::battle::Effect::move_("throatchop")),
        None, // linked_status
        None, // embedded_condition - throatchop's condition data comes from dex.conditions
    );

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onStart(target) {
    ///     this.add('-start', target, 'Throat Chop', '[silent]');
    /// }
    pub fn on_start(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        _source_pos: Option<(usize, usize)>,
        _effect: Option<&crate::battle::Effect>,
    ) -> EventResult {
        let target = pokemon_pos;

        // this.add('-start', target, 'Throat Chop', '[silent]');
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
                crate::battle::Arg::from("Throat Chop"),
                crate::battle::Arg::from("[silent]"),
            ],
        );

        EventResult::Continue
    }

    /// onDisableMove(pokemon) {
    ///     for (const moveSlot of pokemon.moveSlots) {
    ///         if (this.dex.moves.get(moveSlot.id).flags['sound']) {
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
                    // if (this.dex.moves.get(moveSlot.id).flags['sound'])
                    battle
                        .dex
                        .moves
                        .get(&slot.id)
                        .map(|move_data| move_data.flags.get("sound").unwrap_or(false))
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

        let effect = crate::battle::Effect::move_("throatchop");
        for move_id in moves_to_disable {
            pokemon_mut.disable_move(move_id.as_str(), false, Some(&effect));
        }

        EventResult::Continue
    }

    /// onBeforeMove(pokemon, target, move) {
    ///     if (!move.isZOrMaxPowered && move.flags['sound']) {
    ///         this.add('cant', pokemon, 'move: Throat Chop');
    ///         return false;
    ///     }
    /// }
    pub fn on_before_move(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        _target_pos: Option<(usize, usize)>,
        active_move: Option<&crate::battle_actions::ActiveMove>,
    ) -> EventResult {
        let pokemon = pokemon_pos;

        // Get active_move from parameter
        let active_move_ref = match active_move {
            Some(m) => m,
            None => return EventResult::Continue,
        };

        // if (!move.isZOrMaxPowered && move.flags['sound'])
        let should_block = !active_move_ref.is_z_or_max_powered && active_move_ref.flags.sound;

        if should_block {
            // this.add('cant', pokemon, 'move: Throat Chop');
            let pokemon_slot = {
                let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                pokemon_ref.get_slot()
            };

            battle.add(
                "cant",
                &[
                    crate::battle::Arg::from(pokemon_slot),
                    crate::battle::Arg::from("move: Throat Chop"),
                ],
            );

            // return false; - prevents the move from executing
            return EventResult::Boolean(false);
        }

        EventResult::Continue
    }

    /// onModifyMove(move, pokemon, target) {
    ///     if (!move.isZOrMaxPowered && move.flags['sound']) {
    ///         this.add('cant', pokemon, 'move: Throat Chop');
    ///         return false;
    ///     }
    /// }
    pub fn on_modify_move(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        _target_pos: Option<(usize, usize)>,
    ) -> EventResult {
        let pokemon = pokemon_pos;

        // if (!move.isZOrMaxPowered && move.flags['sound'])
        let should_block = {
            let active_move = match &battle.active_move {
                Some(m) => m,
                None => return EventResult::Continue,
            };

            !active_move.borrow().is_z_or_max_powered && active_move.borrow().flags.sound
        };

        if should_block {
            // this.add('cant', pokemon, 'move: Throat Chop');
            let pokemon_slot = {
                let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                pokemon_ref.get_slot()
            };

            battle.add(
                "cant",
                &[
                    crate::battle::Arg::from(pokemon_slot),
                    crate::battle::Arg::from("move: Throat Chop"),
                ],
            );

            // return false; - prevents the move from executing
            return EventResult::Boolean(false);
        }

        EventResult::Continue
    }

    /// onEnd(target) {
    ///     this.add('-end', target, 'Throat Chop', '[silent]');
    /// }
    pub fn on_end(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // this.add('-end', target, 'Throat Chop', '[silent]');
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
                crate::battle::Arg::from("Throat Chop"),
                crate::battle::Arg::from("[silent]"),
            ],
        );

        EventResult::Continue
    }
}
