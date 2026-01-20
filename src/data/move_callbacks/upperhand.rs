//! Upper Hand Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTry(source, target) {
///     const action = this.queue.willMove(target);
///     const move = action?.choice === 'move' ? action.move : null;
///     if (!move || move.priority <= 0.1 || move.category === 'Status') {
///         return false;
///     }
/// }
pub fn on_try(
    battle: &mut Battle,
    _source_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // const action = this.queue.willMove(target);
    let action = battle.queue.will_move(target.0, target.1);

    // const move = action?.choice === 'move' ? action.move : null;
    let (move_id, move_priority) = match action {
        Some(move_action) => (&move_action.move_id, move_action.priority),
        // !move -> return false
        None => return EventResult::Boolean(false),
    };

    // Get move data to check category
    let move_data = match battle.dex.moves().get_by_id(move_id) {
        Some(m) => m,
        // !move -> return false
        None => return EventResult::Boolean(false),
    };

    // if (!move || move.priority <= 0.1 || move.category === 'Status')
    // JavaScript returns false here, not NOT_FAIL
    if move_priority as f64 <= 0.1 || move_data.category.as_str() == "Status" {
        return EventResult::Boolean(false);
    }

    EventResult::Continue
}
