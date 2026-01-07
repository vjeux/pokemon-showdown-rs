//! Merciless Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyCritRatio(critRatio, source, target) {
///     if (target && ['psn', 'tox'].includes(target.status)) return 5;
/// }
pub fn on_modify_crit_ratio(battle: &mut Battle, _crit_ratio: i32, _source_pos: (usize, usize), target_pos: Option<(usize, usize)>, _move_id: &str) -> EventResult {
    if let Some(target_pos) = target_pos {
        if let Some(target) = battle.pokemon_at(target_pos.0, target_pos.1) {
            if target.status == "psn".into() || target.status == "tox".into() {
                return EventResult::Number(5);
            }
        }
    }
    EventResult::Continue
}

