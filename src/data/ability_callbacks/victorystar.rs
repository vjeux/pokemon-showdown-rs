//! Victory Star Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onAnyModifyAccuracy(accuracy, target, source) {
///     if (source.isAlly(this.effectState.target) && typeof accuracy === 'number') {
///         return this.chainModify([4506, 4096]);
///     }
/// }
pub fn on_any_modify_accuracy(battle: &mut Battle, _accuracy: i32, _target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>) -> EventResult {
    let ability_holder = match battle.effect_state.borrow().target {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (source.isAlly(this.effectState.target) && typeof accuracy === 'number')
    if let Some(src_pos) = source_pos {
        if battle.is_ally(src_pos, ability_holder) {
            battle.chain_modify_fraction(4506, 4096); return EventResult::Continue;
        }
    }

    EventResult::Continue
}

