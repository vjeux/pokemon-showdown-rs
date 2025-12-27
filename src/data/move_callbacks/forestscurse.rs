//! Forest's Curse Move
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

/// onHit(target) {
///     if (target.hasType('Grass')) return false;
///     if (!target.addType('Grass')) return false;
///     this.add('-start', target, 'typeadd', 'Grass', '[from] move: Forest\'s Curse');
/// }
pub fn on_hit(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

