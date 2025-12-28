//! Power Anklet Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifySpe(spe) {
///     return this.chainModify(0.5);
/// }
pub fn on_modify_spe(battle: &mut Battle) -> EventResult {
    // return this.chainModify(0.5);
    battle.chain_modify(0.5);
    EventResult::Continue
}
