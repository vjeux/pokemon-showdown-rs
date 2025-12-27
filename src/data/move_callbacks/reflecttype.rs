//! Reflect Type Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onHit(target, source) {            if (source.species && (source.species.num === 493 || source.species.num === 773)) return false;
///             if (source.terastallized) return false;
///             const oldApparentType = source.apparentType;
///             let newBaseTypes = target.getTypes(true).filter(type => type !== '???');
///             if (!newBaseTypes.length) {
///                 if (target.addedType) {
///                     newBaseTypes = ['Normal'];
///                 } else {
///                     return false;
///                 }
///             }
///             this.add('-start', source, 'typechange', '[from] move: Reflect Type', `[of] ${target}`);
///             source.setType(newBaseTypes);
///             source.addedType = target.addedType;
///             source.knownType = target.isAlly(source) && target.knownType;
///             if (!source.knownType) source.apparentType = oldApparentType;
///         }
pub fn on_hit(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

