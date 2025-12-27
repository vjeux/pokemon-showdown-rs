//! Conversion 2 Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target, source) {
///     if (!target.lastMoveUsed) {
///         return false;
///     }
///     const possibleTypes = [];
///     const attackType = target.lastMoveUsed.type;
///     for (const typeName of this.dex.types.names()) {
///         if (source.hasType(typeName)) continue;
///         const typeCheck = this.dex.types.get(typeName).damageTaken[attackType];
///         if (typeCheck === 2 || typeCheck === 3) {
///             possibleTypes.push(typeName);
///         }
///     }
///     if (!possibleTypes.length) {
///         return false;
///     }
///     const randomType = this.sample(possibleTypes);
/// 
///     if (!source.setType(randomType)) return false;
///     this.add('-start', source, 'typechange', randomType);
/// }
pub fn on_hit(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

