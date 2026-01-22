//! Friend Guard Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onAnyModifyDamage(damage, source, target, move) {
///     if (target !== this.effectState.target && target.isAlly(this.effectState.target)) {
///         this.debug('Friend Guard weaken');
///         return this.chainModify(0.75);
///     }
/// }
pub fn on_any_modify_damage(battle: &mut Battle, _damage: i32, _source_pos: Option<(usize, usize)>, target_pos: Option<(usize, usize)>, _active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    let ability_holder = match battle.effect_state.target {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (target !== this.effectState.target && target.isAlly(this.effectState.target))
    if let Some(tpos) = target_pos {
        if tpos != ability_holder && battle.is_ally(tpos, ability_holder) {
            debug_elog!("Friend Guard weaken");
            battle.chain_modify(0.75); return EventResult::Continue;
        }
    }

    EventResult::Continue
}

