//! Quash Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target) {
///     if (this.activePerHalf === 1) return false; // fails in singles
///     const action = this.queue.willMove(target);
///     if (!action) return false;
///
///     action.order = 201;
///     this.add('-activate', target, 'move: Quash');
/// }
pub fn on_hit(battle: &mut Battle, _pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (this.activePerHalf === 1) return false; // fails in singles
    if battle.active_per_half == 1 {
        return EventResult::Boolean(false);
    }

    // const action = this.queue.willMove(target);
    // if (!action) return false;
    // TODO: Implement queue_will_move method in Battle
    let has_action = false; // battle.queue_will_move(target);

    if !has_action {
        return EventResult::Boolean(false);
    }

    // action.order = 201;
    // TODO: Implement set_action_order method in Battle
    // battle.set_action_order(target, 201);

    // this.add('-activate', target, 'move: Quash');
    let target_arg = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.get_slot()
    };

    battle.add("-activate", &[target_arg.into(), "move: Quash".into()]);

    EventResult::Continue
}

