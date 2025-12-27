//! Me First Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(target, pokemon) {
///     const action = this.queue.willMove(target);
///     if (!action) return false;
///     const move = this.dex.getActiveMove(action.move.id);
///     if (action.zmove || move.isZ || move.isMax) return false;
///     if (target.volatiles['mustrecharge']) return false;
///     if (move.category === 'Status' || move.flags['failmefirst']) return false;
/// 
///     pokemon.addVolatile('mefirst');
///     this.actions.useMove(move, pokemon, { target });
///     return null;
/// }
pub fn on_try_hit(battle: &mut Battle, source_pos: (usize, usize), target_pos: (usize, usize), move_id: Option<&str>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onBasePower(basePower) {
    ///     return this.chainModify(1.5);
    /// }
    pub fn on_base_power(battle: &mut Battle, base_power: i32, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: Option<&str>) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }
}
