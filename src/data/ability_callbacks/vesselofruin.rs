//! Vessel of Ruin Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(pokemon) {
///     if (this.suppressingAbility(pokemon)) return;
///     this.add('-ability', pokemon, 'Vessel of Ruin');
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onAnyModifySpA(spa, source, target, move) {
///     const abilityHolder = this.effectState.target;
///     if (source.hasAbility('Vessel of Ruin')) return;
///     if (!move.ruinedSpA) move.ruinedSpA = abilityHolder;
///     if (move.ruinedSpA !== abilityHolder) return;
///     this.debug('Vessel of Ruin SpA drop');
///     return this.chainModify(0.75);
/// }
pub fn on_any_modify_sp_a(battle: &mut Battle, source_pos: Option<(usize, usize)>, target_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

