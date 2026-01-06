//! Contrary Ability
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
///         boost[i]! *= -1;
///     }
/// }
pub fn on_change_boost(battle: &mut Battle, _target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
    // if (effect && effect.id === 'zpower') return;
    if let Some(eff_id) = effect_id {
        if eff_id == "zpower" {
            return EventResult::Continue;
        }
    }

    // for (i in boost) { boost[i]! *= -1; }
    if let Some(ref mut event) = battle.current_event {
        if let Some(crate::event::EventResult::Boost(ref mut boosts)) = event.relay_var {
            boosts.atk *= -1;
            boosts.def *= -1;
            boosts.spa *= -1;
            boosts.spd *= -1;
            boosts.spe *= -1;
            boosts.accuracy *= -1;
            boosts.evasion *= -1;
        }
    }

    EventResult::Continue
}

