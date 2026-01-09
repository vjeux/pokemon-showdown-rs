//! Parental Bond Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::dex::Multihit;

/// onPrepareHit(source, target, move) {
///     if (move.category === 'Status' || move.multihit || move.flags['noparentalbond'] || move.flags['charge'] ||
///         move.flags['futuremove'] || move.spreadHit || move.isZ || move.isMax) return;
///     move.multihit = 2;
///     move.multihitType = 'parentalbond';
/// }
pub fn on_prepare_hit(battle: &mut Battle, _source_pos: Option<(usize, usize)>, _target_pos: Option<(usize, usize)>, active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult { let move_id = active_move.map(|m| m.id.as_str()).unwrap_or("");
    // if (move.category === 'Status' || move.multihit || move.flags['noparentalbond'] || move.flags['charge'] ||
    //     move.flags['futuremove'] || move.spreadHit || move.isZ || move.isMax) return;
    let should_return = if let Some(ref active_move) = battle.active_move {
        active_move.category == "Status"
            || active_move.multi_hit.is_some()
            || active_move.flags.noparentalbond
            || active_move.flags.charge
            || active_move.flags.future_move
            || active_move.spread_hit
            || active_move.is_z.is_some()
            || active_move.is_max.is_some()
    } else {
        return EventResult::Continue;
    };

    if should_return {
        return EventResult::Continue;
    }

    // move.multihit = 2;
    // move.multihitType = 'parentalbond';
    if let Some(ref mut active_move) = battle.active_move {
        active_move.multi_hit = Some(Multihit::Fixed(2));
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
pub fn on_source_modify_secondaries(
    battle: &mut Battle,
    secondaries: &Vec<crate::battle_actions::SecondaryEffect>,
    _target_pos: Option<(usize, usize)>,
    _source_pos: Option<(usize, usize)>,
    active_move: Option<&crate::battle_actions::ActiveMove>,
) -> EventResult {
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or("");
    // if (move.multihitType === 'parentalbond' && move.id === 'secretpower' && move.hit < 2)
    let should_filter = if let Some(ref active_move) = battle.active_move {
        active_move.multi_hit_type.as_deref() == Some("parentalbond")
            && move_id == "secretpower"
            && active_move.hit < 2
    } else {
        return EventResult::Continue;
    };

    if !should_filter {
        return EventResult::Continue;
    }

    // hack to prevent accidentally suppressing King's Rock/Razor Fang
    // return secondaries.filter(effect => effect.volatileStatus === 'flinch');
    let filtered: Vec<_> = secondaries
        .iter()
        .filter(|effect| effect.volatile_status.as_deref() == Some("flinch"))
        .cloned()
        .collect();

    EventResult::Secondaries(filtered)
}

