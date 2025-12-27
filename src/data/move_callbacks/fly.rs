//! Fly Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryMove(attacker, defender, move) {
///     if (attacker.removeVolatile(move.id)) {
///         return;
///     }
///     this.add('-prepare', attacker, move.name);
///     if (!this.runEvent('ChargeMove', attacker, defender, move)) {
///         return;
///     }
///     attacker.addVolatile('twoturnmove', defender);
///     return null;
/// }
pub fn on_try_move(battle: &mut Battle, move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onInvulnerability(target, source, move) {
    ///     if (['gust', 'twister', 'skyuppercut', 'thunder', 'hurricane', 'smackdown', 'thousandarrows'].includes(move.id)) {
    ///         return;
    ///     }
    ///     return false;
    /// }
    pub fn on_invulnerability(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onSourceModifyDamage(damage, source, target, move) {
    ///     if (move.id === 'gust' || move.id === 'twister') {
    ///         return this.chainModify(2);
    ///     }
    /// }
    pub fn on_source_modify_damage(battle: &mut Battle, damage: i32, source_pos: Option<(usize, usize)>, target_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }
}
