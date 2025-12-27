//! Rest Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onTry(source) {
///     if (source.status === 'slp' || source.hasAbility('comatose')) return false;
/// 
///     if (source.hp === source.maxhp) {
///         this.add('-fail', source, 'heal');
///         return null;
///     }
///     // insomnia and vital spirit checks are separate so that the message is accurate in multi-ability mods
///     if (source.hasAbility('insomnia')) {
///         this.add('-fail', source, '[from] ability: Insomnia', `[of] ${source}`);
///         return null;
///     }
///     if (source.hasAbility('vitalspirit')) {
///         this.add('-fail', source, '[from] ability: Vital Spirit', `[of] ${source}`);
///         return null;
///     }
/// }
pub fn on_try(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

/// onHit(target, source, move) {
///     const result = target.setStatus('slp', source, move);
///     if (!result) return result;
///     target.statusState.time = 3;
///     target.statusState.startTime = 3;
///     this.heal(target.maxhp); // Aesthetic only as the healing happens after you fall asleep in-game
/// }
pub fn on_hit(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

