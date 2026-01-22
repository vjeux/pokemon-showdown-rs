//! Contrary Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onChangeBoost(boost, target, source, effect) {
///     if (effect && effect.id === 'zpower') return;
///     let i: BoostID;
///     for (i in boost) {
///         boost[i]! *= -1;
///     }
/// }
pub fn on_change_boost(battle: &mut Battle, _target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, effect: Option<&Effect>) -> EventResult {
    let effect_id = effect.map(|e| e.id.as_str());
    // if (effect && effect.id === 'zpower') return;
    if let Some(eff_id) = effect_id {
        if eff_id == "zpower" {
            return EventResult::Continue;
        }
    }

    // for (i in boost) { boost[i]! *= -1; }
    // Extract boosts from event, invert them, and return as EventResult::Boost
    if let Some(ref event) = battle.event {
        if let Some(crate::event::EventResult::Boost(ref boosts)) = event.relay_var {
            let mut inverted = boosts.clone();
            inverted.atk *= -1;
            inverted.def *= -1;
            inverted.spa *= -1;
            inverted.spd *= -1;
            inverted.spe *= -1;
            inverted.accuracy *= -1;
            inverted.evasion *= -1;
            return EventResult::Boost(inverted);
        }
    }

    EventResult::Continue
}

