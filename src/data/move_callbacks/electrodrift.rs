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
pub fn on_base_power(battle: &mut Battle, base_power: i32, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
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
    let effectiveness = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.run_effectiveness(&move_id, battle)
    };

    if effectiveness > 0.0 {
        // this.debug(`electro drift super effective buff`);
        battle.debug("electro drift super effective buff");

        // return this.chainModify([5461, 4096]);
        return EventResult::Number(battle.chain_modify_fraction(5461, 4096));
    }

    EventResult::Continue
}

