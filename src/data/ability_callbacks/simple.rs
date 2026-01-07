//! Simple Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onChangeBoost(boost, target, source, effect) {
///     if (effect && effect.id === 'zpower') return;
///     let i: BoostID;
///     for (i in boost) {
///         boost[i]! *= 2;
///     }
/// }
pub fn on_change_boost(battle: &mut Battle, _target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
    // if (effect && effect.id === 'zpower') return;
    if let Some(eff_id) = effect_id {
        if eff_id == "zpower" {
            return EventResult::Continue;
        }
    }

    // for (i in boost) { boost[i]! *= 2; }
    if let Some(ref mut event) = battle.event {
        if let Some(EventResult::Boost(ref mut boosts)) = event.relay_var {
            boosts.atk *= 2;
            boosts.def *= 2;
            boosts.spa *= 2;
            boosts.spd *= 2;
            boosts.spe *= 2;
            boosts.accuracy *= 2;
            boosts.evasion *= 2;
        }
    }

    EventResult::Continue
}

