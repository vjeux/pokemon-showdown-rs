//! Expert Belt Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyDamage(damage, source, target, move) {
///     if (move && target.getMoveHitData(move).typeMod > 0) {
///         return this.chainModify([4915, 4096]);
///     }
/// }
pub fn on_modify_damage(battle: &mut Battle, _damage: i32, _pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // if (move && target.getMoveHitData(move).typeMod > 0) {
    //     return this.chainModify([4915, 4096]);
    // }

    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // Clone active_move for run_effectiveness (need owned copy to avoid borrow issues)
    let active_move_clone = match &battle.active_move {
        Some(active_move) => active_move.borrow().clone(),
        None => return EventResult::Continue,
    };

    // Check type effectiveness against target
    // target.getMoveHitData(move).typeMod > 0 means super effective
    let type_effectiveness = crate::Pokemon::run_effectiveness(battle, target_pos, &active_move_clone);

    // typeMod > 0 in JavaScript means super effective
    if type_effectiveness > 0 {
        // return this.chainModify([4915, 4096]);
        battle.chain_modify_fraction(4915, 4096);
    }

    EventResult::Continue
}
