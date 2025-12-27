//! Grudge Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Arg, Battle};
use crate::event::EventResult;
use crate::dex_data::ID;


pub mod condition {
    use super::*;

    /// onStart(pokemon) {
    ///     this.add('-singlemove', pokemon, 'Grudge');
    /// }
    pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        battle.add("-singlemove", &[Arg::Pokemon(pokemon), Arg::Str("Grudge")]);
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
        let source_pos = match source_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // Check if source fainted
        let source_fainted = if let Some(source) = battle.pokemon_at(source_pos.0, source_pos.1) {
            source.fainted
        } else {
            return EventResult::Continue;
        };

        if source_fainted {
            return EventResult::Continue;
        }

        // Check if effect exists
        if effect_id.is_none() {
            return EventResult::Continue;
        }

        // TODO: Need effect.effectType and effect.flags['futuremove']
        // For now, implement the core logic assuming it's a Move effect

        let last_move_id = if let Some(source) = battle.pokemon_at(source_pos.0, source_pos.1) {
            source.last_move.clone()
        } else {
            return EventResult::Continue;
        };

        let move_id = match last_move_id {
            Some(id) => id,
            None => return EventResult::Continue,
        };

        // Get move name for the message
        let move_name = battle.dex.moves.get(move_id.as_str())
            .map(|m| m.name.clone())
            .unwrap_or_else(|| move_id.to_string());

        // Set PP to 0 for the move
        if let Some(source) = battle.pokemon_at_mut(source_pos.0, source_pos.1) {
            source.set_pp(&move_id, 0);
        }

        let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        battle.add("-activate", &[
            Arg::Pokemon(source),
            Arg::Str("move: Grudge"),
            Arg::String(move_name),
        ]);

        EventResult::Continue
    }

    /// onBeforeMove(pokemon) {
    ///     this.debug('removing Grudge before attack');
    ///     pokemon.removeVolatile('grudge');
    /// }
    pub fn on_before_move(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        battle.debug("removing Grudge before attack");

        if let Some(pokemon) = battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            pokemon.remove_volatile(&ID::new("grudge"));
        }

        EventResult::Continue
    }
}

