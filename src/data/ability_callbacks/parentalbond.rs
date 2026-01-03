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
pub fn on_prepare_hit(battle: &mut Battle, _source_pos: Option<(usize, usize)>, _target_pos: Option<(usize, usize)>, _move_id: &str) -> EventResult {
    // if (move.category === 'Status' || move.multihit || move.flags['noparentalbond'] || move.flags['charge'] ||
    //     move.flags['futuremove'] || move.spreadHit || move.isZ || move.isMax) return;
    let should_return = if let Some(ref active_move) = battle.active_move {
        active_move.category == "Status"
            || active_move.multi_hit.is_some()
            || active_move.flags.noparentalbond
            || active_move.flags.charge
            || active_move.flags.future_move
            || active_move.spread_hit
            || active_move.is_z
            || active_move.is_max
    } else {
        return EventResult::Continue;
    };

    if should_return {
        return EventResult::Continue;
    }

    // move.multihit = 2;
    // move.multihitType = 'parentalbond';
    if let Some(ref mut active_move) = battle.active_move {
        active_move.multi_hit = Some(2);
        active_move.multi_hit_type = Some("parentalbond".to_string());
    }

    EventResult::Continue
}

/// onSourceModifySecondaries(secondaries, target, source, move) {
///     if (move.multihitType === 'parentalbond' && move.id === 'secretpower' && move.hit < 2) {
///         // hack to prevent accidentally suppressing King's Rock/Razor Fang
///         return secondaries.filter(effect => effect.volatileStatus === 'flinch');
///     }
/// }
pub fn on_source_modify_secondaries(_battle: &mut Battle, _target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, _move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

