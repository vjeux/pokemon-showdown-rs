//! G-Max Steelsurge Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;


pub mod condition {
    use super::*;

    /// onSideStart(side) {
    ///     this.add('-sidestart', side, 'move: G-Max Steelsurge');
    /// }
    pub fn on_side_start(battle: &mut Battle) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onSwitchIn(pokemon) {
    ///     if (pokemon.hasItem('heavydutyboots')) return;
    ///     // Ice Face and Disguise correctly get typed damage from Stealth Rock
    ///     // because Stealth Rock bypasses Substitute.
    ///     // They don't get typed damage from Steelsurge because Steelsurge doesn't,
    ///     // so we're going to test the damage of a Steel-type Stealth Rock instead.
    ///     const steelHazard = this.dex.getActiveMove('Stealth Rock');
    ///     steelHazard.type = 'Steel';
    ///     const typeMod = this.clampIntRange(pokemon.runEffectiveness(steelHazard), -6, 6);
    ///     this.damage(pokemon.maxhp * (2 ** typeMod) / 8);
    /// }
    pub fn on_switch_in(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

}
