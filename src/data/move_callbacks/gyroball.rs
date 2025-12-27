//! Gyro Ball Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// basePowerCallback(pokemon, target) {
///     let power = Math.floor(25 * target.getStat('spe') / pokemon.getStat('spe')) + 1;
///     if (!isFinite(power)) power = 1;
///     if (power > 150) power = 150;
///     this.debug(`BP: ${power}`);
///     return power;
/// }
pub fn base_power_callback(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

