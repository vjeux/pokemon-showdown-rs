//! Red Card Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onAfterMoveSecondary(target, source, move) {
///     if (source && source !== target && source.hp && target.hp && move && move.category !== 'Status') {
///         if (!source.isActive || !this.canSwitch(source.side) || source.forceSwitchFlag || target.forceSwitchFlag) {
///             return;
///         }
///         // The item is used up even against a pokemon with Ingrain or that otherwise can't be forced out
///         if (target.useItem(source)) {
///             if (this.runEvent('DragOut', source, target, move)) {
///                 source.forceSwitchFlag = true;
///             }
///         }
///     }
/// }
pub fn on_after_move_secondary(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    // if (source && source !== target && source.hp && target.hp && move && move.category !== 'Status') {
    //     if (!source.isActive || !this.canSwitch(source.side) || source.forceSwitchFlag || target.forceSwitchFlag) {
    //         return;
    //     }
    //     if (target.useItem(source)) {
    //         if (this.runEvent('DragOut', source, target, move)) {
    //             source.forceSwitchFlag = true;
    //         }
    //     }
    // }
    // TODO: Need move.category, pokemon.isActive, battle.canSwitch(), pokemon.forceSwitchFlag
    // pokemon.useItem(source), battle.runEvent('DragOut', ...) to force opponent switch
    EventResult::Continue
}
