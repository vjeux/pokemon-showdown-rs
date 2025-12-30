//! Big Root Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHeal(damage, target, source, effect) {
///     const heals = ['drain', 'leechseed', 'ingrain', 'aquaring', 'strengthsap'];
///     if (heals.includes(effect.id)) {
///         return this.chainModify([5324, 4096]);
///     }
/// }
pub fn on_try_heal(battle: &mut Battle, _damage: i32, _target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
    // const heals = ['drain', 'leechseed', 'ingrain', 'aquaring', 'strengthsap'];
    // if (heals.includes(effect.id)) {
    //     return this.chainModify([5324, 4096]);
    // }

    if let Some(eff_id) = effect_id {
        let heals = ["drain", "leechseed", "ingrain", "aquaring", "strengthsap"];
        if heals.contains(&eff_id) {
            battle.chain_modify_fraction(5324, 4096);
        }
    }

    EventResult::Continue
}
