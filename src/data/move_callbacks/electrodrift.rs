//! Electro Drift Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onBasePower(basePower, source, target, move) {
///     if (target.runEffectiveness(move) > 0) {
///         // Placeholder
///         this.debug(`electro drift super effective buff`);
///         return this.chainModify([5461, 4096]);
///     }
/// }
pub fn on_base_power(
    battle: &mut Battle,
    _base_power: i32,
    _pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // Get current move
    let move_id = match &battle.active_move {
        Some(active_move) => active_move.id.clone(),
        None => return EventResult::Continue,
    };

    // if (target.runEffectiveness(move) > 0) {
    let effectiveness = crate::Pokemon::run_effectiveness(battle, target, &move_id);

    if effectiveness > 0 {
        // this.debug(`electro drift super effective buff`);
        battle.debug("electro drift super effective buff");

        // return this.chainModify([5461, 4096]);
        battle.chain_modify_fraction(5461, 4096); return EventResult::Continue;
    }

    EventResult::Continue
}
