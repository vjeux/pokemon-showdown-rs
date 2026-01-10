//! Collision Course Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onBasePower(basePower, source, target, move) {
///     if (target.runEffectiveness(move) > 0) {
///         // Placeholder
///         this.debug(`collision course super effective buff`);
///         return this.chainModify([5461, 4096]);
///     }
/// }
pub fn on_base_power(
    battle: &mut Battle,
    _base_power: i32,
    _pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    // Get the target
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // Get the active move (clone to avoid borrow issues)
    let active_move_clone = match &battle.active_move {
        Some(active_move) => active_move.clone(),
        None => return EventResult::Continue,
    };

    // if (target.runEffectiveness(move) > 0) {
    let effectiveness = crate::Pokemon::run_effectiveness(battle, target, &active_move_clone);

    if effectiveness > 0 {
        // this.debug(`collision course super effective buff`);
        battle.debug("collision course super effective buff");

        // return this.chainModify([5461, 4096]);
        let result = battle.chain_modify_fraction(5461, 4096);
        return EventResult::Number(result);
    }

    EventResult::Continue
}
