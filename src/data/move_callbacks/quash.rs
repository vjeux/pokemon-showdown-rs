//! Quash Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// ```ignore
/// onHit(target) {
///     if (this.activePerHalf === 1) return false; // fails in singles
///     const action = this.queue.willMove(target);
///     if (!action) return false;
///
///     action.order = 201;
///     this.add('-activate', target, 'move: Quash');
/// }
/// ```
pub fn on_hit(
    battle: &mut Battle,
    target_pos: (usize, usize),
    _source_pos: Option<(usize, usize)>,
) -> EventResult {
    // onHit(target) - target is the first param (the opponent being delayed)
    let target = target_pos;

    // if (this.activePerHalf === 1) return false; // fails in singles
    if battle.active_per_half == 1 {
        return EventResult::Boolean(false);
    }

    // const action = this.queue.willMove(target);
    // if (!action) return false;
    let has_action = battle.queue.will_move(target.0, target.1).is_some();

    if !has_action {
        return EventResult::Boolean(false);
    }

    // action.order = 201;
    if let Some(action) = battle.queue.will_move_mut(target.0, target.1) {
        action.order = 201;
    }

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
