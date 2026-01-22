//! Float Stone Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onModifyWeight(weighthg) {
///     return this.trunc(weighthg / 2);
/// }
pub fn on_modify_weight(_battle: &mut Battle, weighthg: i32) -> EventResult {
    // return this.trunc(weighthg / 2);
    EventResult::Number(weighthg / 2)
}
