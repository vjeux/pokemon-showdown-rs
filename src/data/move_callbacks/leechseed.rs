//! Leech Seed Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryImmunity(target) {
///     return !target.hasType('Grass');
/// }
pub fn on_try_immunity(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    // Grass-types are immune to Leech Seed
    EventResult::Bool(!target.has_type("Grass"))
}

pub mod condition {
    use super::*;

    /// onStart(target) {
    ///     this.add('-start', target, 'move: Leech Seed');
    /// }
    pub fn on_start(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onResidual(pokemon) {
    ///     const target = this.getAtSlot(pokemon.volatiles['leechseed'].sourceSlot);
    ///     if (!target || target.fainted || target.hp <= 0) {
    ///         this.debug('Nothing to leech into');
    ///         return;
    ///     }
    ///     const damage = this.damage(pokemon.baseMaxhp / 8, pokemon, target);
    ///     if (damage) {
    ///         this.heal(damage, target, pokemon);
    ///     }
    /// }
    pub fn on_residual(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }
}
