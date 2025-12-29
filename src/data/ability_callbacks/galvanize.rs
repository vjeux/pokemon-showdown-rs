//! Galvanize Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyType(move, pokemon) {
///     const noModifyType = [
///         'judgment', 'multiattack', 'naturalgift', 'revelationdance', 'technoblast', 'terrainpulse', 'weatherball',
///     ];
///     if (move.type === 'Normal' && (!noModifyType.includes(move.id) || this.activeMove?.isMax) &&
///         !(move.isZ && move.category !== 'Status') && !(move.name === 'Tera Blast' && pokemon.terastallized)) {
///         move.type = 'Electric';
///         move.typeChangerBoosted = this.effect;
///     }
/// }
pub fn on_modify_type(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onBasePower(basePower, pokemon, target, move) {
///     if (move.typeChangerBoosted === this.effect) return this.chainModify([4915, 4096]);
/// }
pub fn on_base_power(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

