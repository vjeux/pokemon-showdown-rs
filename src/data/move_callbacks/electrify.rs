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
pub fn on_try_hit(battle: &mut Battle, source_pos: (usize, usize), target_pos: (usize, usize)) -> EventResult {
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
    pub fn on_start(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // this.add('-singleturn', target, 'move: Electrify');
        let target_arg = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            crate::battle::Arg::from(target_pokemon)
        };

        battle.add("-singleturn", &[target_arg, "move: Electrify".into()]);

        EventResult::Continue
    }

    /// onModifyType(move) {
    ///     if (move.id !== 'struggle') {
    ///         this.debug('Electrify making move type electric');
    ///         move.type = 'Electric';
    ///     }
    /// }
    pub fn on_modify_type(battle: &mut Battle, move_id: &str) -> EventResult {
        use crate::dex_data::ID;

        // if (move.id !== 'struggle') {
        //     this.debug('Electrify making move type electric');
        //     move.type = 'Electric';
        // }
        if move_id != "struggle" {
            // this.debug('Electrify making move type electric');
            battle.debug("Electrify making move type electric");

            // move.type = 'Electric';
            // Return the Electric type as the modified type
            return EventResult::ID(ID::from("electric"));
        }

        EventResult::Continue
    }
}
