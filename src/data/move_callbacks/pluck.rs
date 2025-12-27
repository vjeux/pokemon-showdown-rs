//! Pluck Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use crate::event::EventResult;
use super::{Status, Effect};

/// onHit(target, source, move) {
///     const item = target.getItem();
///     if (source.hp && item.isBerry && target.takeItem(source)) {
///         this.add('-enditem', target, item.name, '[from] stealeat', '[move] Pluck', `[of] ${source}`);
///         if (this.singleEvent('Eat', item, target.itemState, source, source, move)) {
///             this.runEvent('EatItem', source, source, move, item);
///             if (item.id === 'leppaberry') target.staleness = 'external';
///         }
///         if (item.onEat) source.ateBerry = true;
///     }
/// }
pub fn on_hit(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

