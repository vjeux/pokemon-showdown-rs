//! Dragon Cheer Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};



// Condition handlers
pub mod condition {
    use super::*;

    /// onStart(target, source, effect) {
    ///     if (target.volatiles['focusenergy']) return false;
    ///     if (effect && (['costar', 'imposter', 'psychup', 'transform'].includes(effect.id))) {
    ///         this.add('-start', target, 'move: Dragon Cheer', '[silent]');
    ///     } else {
    ///         this.add('-start', target, 'move: Dragon Cheer');
    ///     }
    ///     // Store at the start because the boost doesn't change if a Pokemon
    ///     // Terastallizes into Dragon while having this volatile
    ///     // Found by DarkFE:
    ///     // https://www.smogon.com/forums/threads/scarlet-violet-battle-mechanics-research.3709545/post-9894139
    ///     this.effectState.hasDragonType = target.hasType("Dragon");
    /// }
    pub fn on_start(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> MoveHandlerResult {
        // TODO: Implement 1-to-1 from JS
        MoveHandlerResult::Undefined
    }

    /// onModifyCritRatio(critRatio, source) {
    ///     return critRatio + (this.effectState.hasDragonType ? 2 : 1);
    /// }
    pub fn on_modify_crit_ratio(battle: &mut Battle, source_pos: Option<(usize, usize)>) -> MoveHandlerResult {
        // TODO: Implement 1-to-1 from JS
        MoveHandlerResult::Undefined
    }

}
