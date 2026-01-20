//! Sucker Punch Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTry(source, target) {
///     const action = this.queue.willMove(target);
///     const move = action?.choice === 'move' ? action.move : null;
///     if (!move || (move.category === 'Status' && move.id !== 'mefirst') || target.volatiles['mustrecharge']) {
///         return false;
///     }
/// }
pub fn on_try(
    battle: &mut Battle,
    _source_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    use crate::dex_data::ID;

    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // const action = this.queue.willMove(target);
    let action = battle.queue.will_move(target.0, target.1);

    // const move = action?.choice === 'move' ? action.move : null;
    let move_id = match action {
        Some(move_action) => &move_action.move_id,
        // !move -> return false
        None => return EventResult::Boolean(false),
    };

    // Get move data to check category
    let move_data = match battle.dex.moves().get_by_id(move_id) {
        Some(m) => m,
        // !move -> return false
        None => return EventResult::Boolean(false),
    };

    // Check if target has mustrecharge volatile
    let has_must_recharge = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.has_volatile(&ID::from("mustrecharge"))
    };

    // if (!move || (move.category === 'Status' && move.id !== 'mefirst') || target.volatiles['mustrecharge'])
    // JavaScript returns false here, not NOT_FAIL
    if (move_data.category.as_str() == "Status" && move_id.as_str() != "mefirst")
        || has_must_recharge
    {
        return EventResult::Boolean(false);
    }

    EventResult::Continue
}
