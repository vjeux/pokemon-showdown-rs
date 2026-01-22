//! Simple Ability
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
///         boost[i]! *= 2;
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

    // for (i in boost) { boost[i]! *= 2; }
    // Extract boosts from event, double them, and return as EventResult::Boost
    if let Some(ref event) = battle.event {
        if let Some(EventResult::Boost(ref boosts)) = event.relay_var {
            let mut doubled = boosts.clone();
            doubled.atk *= 2;
            doubled.def *= 2;
            doubled.spa *= 2;
            doubled.spd *= 2;
            doubled.spe *= 2;
            doubled.accuracy *= 2;
            doubled.evasion *= 2;
            return EventResult::Boost(doubled);
        }
    }

    EventResult::Continue
}

