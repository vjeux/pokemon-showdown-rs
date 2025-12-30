//! Supreme Overlord Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(pokemon) {
///     if (pokemon.side.totalFainted) {
///         this.add('-activate', pokemon, 'ability: Supreme Overlord');
///         const fallen = Math.min(pokemon.side.totalFainted, 5);
///         this.add('-start', pokemon, `fallen${fallen}`, '[silent]');
///         this.effectState.fallen = fallen;
///     }
/// }
pub fn on_start(_battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onEnd(pokemon) {
///     this.add('-end', pokemon, `fallen${this.effectState.fallen}`, '[silent]');
/// }
pub fn on_end(_battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onBasePower(basePower, attacker, defender, move) {
///     if (this.effectState.fallen) {
///         const powMod = [4096, 4506, 4915, 5325, 5734, 6144];
///         this.debug(`Supreme Overlord boost: ${powMod[this.effectState.fallen]}/4096`);
///         return this.chainModify([powMod[this.effectState.fallen], 4096]);
///     }
/// }
pub fn on_base_power(_battle: &mut Battle, _base_power: i32, _attacker_pos: (usize, usize), _defender_pos: (usize, usize), _move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

