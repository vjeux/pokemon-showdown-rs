//! Soul-Heart Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onAnyFaint() {
///     this.boost({ spa: 1 }, this.effectState.target);
/// }
pub fn on_any_faint(battle: &mut Battle) -> EventResult {
    let ability_holder = match battle.effect_state.target {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // this.boost({ spa: 1 }, this.effectState.target);
    battle.boost(
        &[("spa", 1)],
        ability_holder,
        Some(ability_holder),
        None,
        false,
        false,
    );

    EventResult::Continue
}

