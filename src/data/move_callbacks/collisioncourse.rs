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
pub fn on_base_power(battle: &mut Battle, base_power: i32, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let move_id = match &battle.active_move {
        Some(id) => id,
        None => return EventResult::Continue,
    };

    // Run effectiveness to check if move is super-effective
    let effectiveness = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        target.run_effectiveness(move_id.as_str())
    };

    if effectiveness > 0 {
        battle.debug("collision course super effective buff");
        return EventResult::Number(battle.chain_modify_fraction(5461, 4096));
    }

    EventResult::Continue
}

