//! Grudge Move
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

    /// onStart(pokemon) {
    ///     this.add('-singlemove', pokemon, 'Grudge');
    /// }
    pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> MoveHandlerResult {
        // TODO: Implement 1-to-1 from JS
        MoveHandlerResult::Undefined
    }

    /// onFaint(target, source, effect) {
    ///     if (!source || source.fainted || !effect) return;
    ///     if (effect.effectType === 'Move' && !effect.flags['futuremove'] && source.lastMove) {
    ///         let move: Move = source.lastMove;
    ///         if (move.isMax && move.baseMove) move = this.dex.moves.get(move.baseMove);
    /// 
    ///         for (const moveSlot of source.moveSlots) {
    ///             if (moveSlot.id === move.id) {
    ///                 moveSlot.pp = 0;
    ///                 this.add('-activate', source, 'move: Grudge', move.name);
    ///             }
    ///         }
    ///     }
    /// }
    pub fn on_faint(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> MoveHandlerResult {
        // TODO: Implement 1-to-1 from JS
        MoveHandlerResult::Undefined
    }

    /// onBeforeMove(pokemon) {
    ///     this.debug('removing Grudge before attack');
    ///     pokemon.removeVolatile('grudge');
    /// }
    pub fn on_before_move(battle: &mut Battle, pokemon_pos: (usize, usize)) -> MoveHandlerResult {
        // TODO: Implement 1-to-1 from JS
        MoveHandlerResult::Undefined
    }

}
