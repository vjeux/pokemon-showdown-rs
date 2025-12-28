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
pub fn on_base_power(battle: &mut Battle, _base_power: i32, _pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // Get the target
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // Get the active move
    let move_id = match &battle.active_move {
        Some(active_move) => active_move.id.clone(),
        None => return EventResult::Continue,
    };

    // if (target.runEffectiveness(move) > 0) {
    let effectiveness = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.run_effectiveness(move_id.as_str())
    };

    if effectiveness > 0.0 {
        // this.debug(`collision course super effective buff`);
        // TODO: debug not yet implemented

        // return this.chainModify([5461, 4096]);
        let result = battle.chain_modify_fraction(5461, 4096);
        return EventResult::Number(result);
    }

    EventResult::Continue
}
