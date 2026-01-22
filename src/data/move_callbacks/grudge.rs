//! Grudge Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;
use crate::Pokemon;

pub mod condition {
    use super::*;

    /// onStart(pokemon) {
    ///     this.add('-singlemove', pokemon, 'Grudge');
    /// }
    pub fn on_start(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        _source_pos: Option<(usize, usize)>,
        _effect: Option<&crate::battle::Effect>,
    ) -> EventResult {
        let pokemon = pokemon_pos;

        // this.add('-singlemove', pokemon, 'Grudge');
        let pokemon_ident = {
            let poke = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            poke.get_slot()
        };
        battle.add(
            "-singlemove",
            &[pokemon_ident.as_str().into(), "Grudge".into()],
        );

        EventResult::Continue
    }

    /// ```ignore
    /// onFaint(target, source, effect) {
    ///     if (!source || source.fainted || !effect) return;
    ///     if (effect.effectType === 'Move' && !effect.flags['futuremove'] && source.lastMove) {
    ///         let move: Move = source.lastMove;
    ///         if (move.isMax && move.baseMove) move = this.dex.moves.get(move.baseMove);
    ///
    ///         for (const moveSlot of source.moveSlots) {
    ///             if (moveSlot.id === move.id) {
    ///                 moveSlot.pp = 0;
    ///                 this.add('-activate', source, 'move: Grudge', move.name);
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    pub fn on_faint(
        battle: &mut Battle,
        _target_pos: Option<(usize, usize)>,
        source_pos: Option<(usize, usize)>,
        effect: Option<&Effect>,
    ) -> EventResult {
        let effect_id = effect.map(|e| e.id.as_str());

        use crate::dex_data::ID;

        // if (!source || source.fainted || !effect) return;
        let source = match source_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        let source_fainted = {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            source_pokemon.fainted
        };

        if source_fainted {
            return EventResult::Continue;
        }

        if effect_id.is_none() {
            return EventResult::Continue;
        }

        let effect_id_str = effect_id.unwrap();

        // if (effect.effectType === 'Move' && !effect.flags['futuremove'] && source.lastMove) {
        // Check effectType using battle.event.effect (not battle.effect - that's a different context)
        let is_move_effect = battle.event
            .as_ref()
            .and_then(|e| e.effect.as_ref())
            .map(|e| e.effect_type == crate::battle::EffectType::Move)
            .unwrap_or(false);

        // Also check futuremove flag if it's a move
        let has_futuremove_flag = if is_move_effect {
            if let Some(move_data) = battle.dex.moves().get_by_id(&ID::from(effect_id_str)) {
                move_data.flags.contains_key("futuremove")
            } else {
                false
            }
        } else {
            false
        };

        let source_last_move = {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            source_pokemon.last_move.clone()
        };

        if let (true, Some(mut move_id)) = (is_move_effect && !has_futuremove_flag, source_last_move) {
            // let move: Move = source.lastMove;

            // if (move.isMax && move.baseMove) move = this.dex.moves.get(move.baseMove);
            let move_data = battle.dex.moves().get_by_id(&move_id);
            if let Some(move_data) = move_data {
                if move_data.is_max.is_some() && move_data.base_move.is_some() {
                    move_id = move_data.base_move.clone().unwrap();
                }
            }

            // for (const moveSlot of source.moveSlots) {
            let move_slots = {
                let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                source_pokemon.move_slots.clone()
            };

            for (i, move_slot) in move_slots.iter().enumerate() {
                // if (moveSlot.id === move.id) {
                if move_slot.id == move_id {
                    // moveSlot.pp = 0;
                    {
                        let source_pokemon = match battle.pokemon_at_mut(source.0, source.1) {
                            Some(p) => p,
                            None => return EventResult::Continue,
                        };
                        if i < source_pokemon.move_slots.len() {
                            source_pokemon.move_slots[i].pp = 0;
                            // Also update baseMoveSlots - in JavaScript these share the same MoveSlot objects
                            // because slice() creates a shallow copy. In Rust we clone, so we need to update both.
                            if i < source_pokemon.base_move_slots.len() {
                                source_pokemon.base_move_slots[i].pp = 0;
                            }
                        }
                    }

                    // this.add('-activate', source, 'move: Grudge', move.name);
                    let move_name = battle
                        .dex
                        .moves().get_by_id(&move_id)
                        .map(|m| m.name.clone())
                        .unwrap_or_else(|| move_id.to_string());
                    let source_ident = {
                        let pokemon = match battle.pokemon_at(source.0, source.1) {
                            Some(p) => p,
                            None => return EventResult::Continue,
                        };
                        pokemon.get_slot()
                    };
                    battle.add(
                        "-activate",
                        &[
                            source_ident.as_str().into(),
                            "move: Grudge".into(),
                            move_name.into(),
                        ],
                    );
                }
            }
        }

        EventResult::Continue
    }

    /// onBeforeMove(pokemon) {
    ///     this.debug('removing Grudge before attack');
    ///     pokemon.removeVolatile('grudge');
    /// }
    pub fn on_before_move(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        use crate::dex_data::ID;

        let pokemon = pokemon_pos;

        // this.debug('removing Grudge before attack');
        battle.debug("removing Grudge before attack");

        // pokemon.removeVolatile('grudge');
        Pokemon::remove_volatile(battle, pokemon, &ID::from("grudge"));

        EventResult::Continue
    }
}
