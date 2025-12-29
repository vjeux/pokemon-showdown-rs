//! Parental Bond Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onPrepareHit(source, target, move) {
///     if (move.category === 'Status' || move.multihit || move.flags['noparentalbond'] || move.flags['charge'] ||
///         move.flags['futuremove'] || move.spreadHit || move.isZ || move.isMax) return;
///     move.multihit = 2;
///     move.multihitType = 'parentalbond';
/// }
pub fn on_prepare_hit(battle: &mut Battle, source_pos: Option<(usize, usize)>, target_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onSourceModifySecondaries(secondaries, target, source, move) {
///     if (move.multihitType === 'parentalbond' && move.id === 'secretpower' && move.hit < 2) {
///         // hack to prevent accidentally suppressing King's Rock/Razor Fang
///         return secondaries.filter(effect => effect.volatileStatus === 'flinch');
///     }
/// }
pub fn on_source_modify_secondaries(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

