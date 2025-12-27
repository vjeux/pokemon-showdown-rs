//! Grudge Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;


pub mod condition {
    use super::*;

    /// onStart(pokemon) {
    ///     this.add('-singlemove', pokemon, 'Grudge');
    /// }
    pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        let pokemon = pokemon_pos;

        // this.add('-singlemove', pokemon, 'Grudge');
        let pokemon_arg = crate::battle::Arg::Pos(pokemon.0, pokemon.1);
        battle.add("-singlemove", &[pokemon_arg, "Grudge".into()]);

        EventResult::Continue
    }

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
    pub fn on_faint(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
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
        let is_move_effect = if let Some(move_data) = battle.dex.get_move_by_id(&ID::from(effect_id_str)) {
            !move_data.flags.contains_key("futuremove")
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

        if is_move_effect && source_last_move.is_some() {
            // let move: Move = source.lastMove;
            let mut move_id = source_last_move.unwrap();

            // if (move.isMax && move.baseMove) move = this.dex.moves.get(move.baseMove);
            let move_data = battle.dex.get_move_by_id(&move_id);
            if let Some(move_data) = move_data {
                if move_data.is_max && move_data.base_move.is_some() {
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
                        }
                    }

                    // this.add('-activate', source, 'move: Grudge', move.name);
                    let move_name = battle.dex.get_move_by_id(&move_id)
                        .map(|m| m.name.clone())
                        .unwrap_or_else(|| move_id.to_string());
                    let source_arg = crate::battle::Arg::Pos(source.0, source.1);
                    battle.add("-activate", &[source_arg, "move: Grudge".into(), move_name.into()]);
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
        let pokemon_pokemon = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_pokemon.remove_volatile(&ID::from("grudge"));

        EventResult::Continue
    }
}
