//! Heal Bell Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onHit(target, source) {
/// this.add('-activate', source, 'move: Heal Bell');
/// let success = false;
/// const allies = [...target.side.pokemon, ...target.side.allySide?.pokemon || []];
/// for (const ally of allies) {
///     if (ally !== source && !this.suppressingAbility(ally)) {
///         if (ally.hasAbility('soundproof')) {
///             this.add('-immune', ally, '[from] ability: Soundproof');
///             continue;
///         }
///         if (ally.hasAbility('goodasgold')) {
///             this.add('-immune', ally, '[from] ability: Good as Gold');
///             continue;
///         }
///     }
///     if (ally.cureStatus()) success = true;
/// }
/// return success;
/// }
pub fn on_hit(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

