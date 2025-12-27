//! Punishment Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// basePowerCallback(pokemon, target) {
///     let power = 60 + 20 * target.positiveBoosts();
///     if (power > 200) power = 200;
///     this.debug(`BP: ${power}`);
///     return power;
/// }
pub fn base_power_callback(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    // Count positive boosts
    let positive_boosts =
        std::cmp::max(0, target.boosts.atk) +
        std::cmp::max(0, target.boosts.def) +
        std::cmp::max(0, target.boosts.spa) +
        std::cmp::max(0, target.boosts.spd) +
        std::cmp::max(0, target.boosts.spe) +
        std::cmp::max(0, target.boosts.accuracy) +
        std::cmp::max(0, target.boosts.evasion);

    let mut power = 60 + (20 * positive_boosts);
    if power > 200 {
        power = 200;
    }

    // TODO: battle.debug(`BP: ${power}`);
    EventResult::Number(power)
}

