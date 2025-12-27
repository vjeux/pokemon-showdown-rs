//! Pursuit Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onModifyMove(move, source, target) {
///     if (target?.beingCalledBack || target?.switchFlag) move.accuracy = true;
/// }
pub fn on_modify_move(battle: &mut Battle, move_id: &str, source_pos: Option<(usize, usize)>, target_pos: Option<(usize, usize)>) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

/// onTryHit(target, pokemon) {
///     target.side.removeSideCondition('pursuit');
/// }
pub fn on_try_hit(battle: &mut Battle, target_pos: Option<(usize, usize)>, pokemon_pos: (usize, usize)) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}


// Condition handlers
pub mod condition {
    use super::*;

    /// onBeforeSwitchOut(pokemon) {
    ///     this.debug('Pursuit start');
    ///     let alreadyAdded = false;
    ///     pokemon.removeVolatile('destinybond');
    ///     for (const source of this.effectState.sources) {
    ///         if (!source.isAdjacent(pokemon) || !this.queue.cancelMove(source) || !source.hp) continue;
    ///         if (!alreadyAdded) {
    ///             this.add('-activate', pokemon, 'move: Pursuit');
    ///             alreadyAdded = true;
    ///         }
    ///         // Run through each action in queue to check if the Pursuit user is supposed to Mega Evolve this turn.
    ///         // If it is, then Mega Evolve before moving.
    ///         if (source.canMegaEvo || source.canUltraBurst || source.canTerastallize) {
    ///             for (const [actionIndex, action] of this.queue.entries()) {
    ///                 if (action.pokemon === source) {
    ///                     if (action.choice === 'megaEvo') {
    ///                         this.actions.runMegaEvo(source);
    ///                     } else if (action.choice === 'terastallize') {
    ///                         // Also a "forme" change that happens before moves, though only possible in NatDex
    ///                         this.actions.terastallize(source);
    ///                     } else {
    ///                         continue;
    ///                     }
    ///                     this.queue.list.splice(actionIndex, 1);
    ///                     break;
    ///                 }
    ///             }
    ///         }
    ///         this.actions.runMove('pursuit', source, source.getLocOf(pokemon));
    ///     }
    /// }
    pub fn on_before_switch_out(battle: &mut Battle, pokemon_pos: (usize, usize)) -> MoveHandlerResult {
        // TODO: Implement 1-to-1 from JS
        MoveHandlerResult::Undefined
    }

}
