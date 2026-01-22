//! Rock Head Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onDamage(damage, target, source, effect) {
///     if (effect.id === 'recoil') {
///         if (!this.activeMove) throw new Error("Battle.activeMove is null");
///         if (this.activeMove.id !== 'struggle') return null;
///     }
/// }
pub fn on_damage(battle: &mut Battle, _damage: i32, _target_pos: (usize, usize), _source_pos: Option<(usize, usize)>, effect: Option<&Effect>) -> EventResult {
    let effect_id = effect.map(|e| e.id.as_str());
    // If the effect is recoil damage
    if let Some(eff_id) = effect_id {
        if eff_id == "recoil" {
            // Check if active move exists and is not Struggle
            if let Some(ref active_move) = battle.active_move {
                if active_move.id.as_str() != "struggle" {
                    // Prevent recoil damage for all moves except Struggle
                    return EventResult::Null;
                }
            }
        }
    }
    EventResult::Continue
}

