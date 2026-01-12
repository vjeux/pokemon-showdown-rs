//! Electrify Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(target) {
///     if (!this.queue.willMove(target) && target.activeTurns) return false;
/// }
/// JavaScript signature: onTryHit(target, source, move) - TARGET FIRST
pub fn on_try_hit(
    battle: &mut Battle,
    target_pos: (usize, usize),
    _source_pos: (usize, usize),
) -> EventResult {
    let target = target_pos;

    // if (!this.queue.willMove(target) && target.activeTurns) return false;
    let will_move = battle.queue.will_move(target.0, target.1);

    let active_turns = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.active_turns
    };

    if will_move.is_none() && active_turns > 0 {
        // return false;
        return EventResult::Boolean(false);
    }

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onStart(target) {
    ///     this.add('-singleturn', target, 'move: Electrify');
    /// }
    pub fn on_start(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        _source_pos: Option<(usize, usize)>,
        _effect: Option<&crate::battle::Effect>,
    ) -> EventResult {
        let target = pokemon_pos;

        // this.add('-singleturn', target, 'move: Electrify');
        let target_ident = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.get_slot()
        };

        battle.add(
            "-singleturn",
            &[target_ident.as_str().into(), "move: Electrify".into()],
        );

        EventResult::Continue
    }

    /// onModifyType(move) {
    ///     if (move.id !== 'struggle') {
    ///         this.debug('Electrify making move type electric');
    ///         move.type = 'Electric';
    ///     }
    /// }
    pub fn on_modify_type(battle: &mut Battle, active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
        let move_id = active_move.map(|m| m.id.as_str()).unwrap_or("");

        // if (move.id !== 'struggle') {
        //     this.debug('Electrify making move type electric');
        //     move.type = 'Electric';
        // }
        if move_id != "struggle" {
            // this.debug('Electrify making move type electric');
            battle.debug("Electrify making move type electric");

            // move.type = 'Electric';
            // Modify the active move's type directly
            if let Some(ref mut active_move) = battle.active_move {
                active_move.move_type = "Electric".to_string();
            }
        }

        EventResult::Continue
    }
}
