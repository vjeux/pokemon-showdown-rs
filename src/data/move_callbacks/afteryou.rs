//! After You Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::event::EventResult;

/// onHit(target) {
///     if (this.activePerHalf === 1) return false; // fails in singles
///     const action = this.queue.willMove(target);
///     if (action) {
///         this.queue.prioritizeAction(action);
///         this.add('-activate', target, 'move: After You');
///     } else {
///         return false;
///     }
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // if (this.activePerHalf === 1) return false; // fails in singles
    if battle.active_per_half == 1 {
        return EventResult::Boolean(false);
    }

    // Get target position
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // const action = this.queue.willMove(target);
    let action = battle.queue.will_move(target.0, target.1);

    // if (action) {
    if action.is_some() {
        // this.queue.prioritizeAction(action);
        battle.queue.prioritize_action(target.0, target.1);

        // this.add('-activate', target, 'move: After You');
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        battle.add("-activate", &[target_pokemon.into(), "move: After You".into()]);

        EventResult::Continue
    } else {
        // return false;
        EventResult::Boolean(false)
    }
}

