//! Armor Tail Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onFoeTryMove(target, source, move) {
///     const targetAllExceptions = ['perishsong', 'flowershield', 'rototiller'];
///     if (move.target === 'foeSide' || (move.target === 'all' && !targetAllExceptions.includes(move.id))) {
///         return;
///     }
/// 
///     const armorTailHolder = this.effectState.target;
///     if ((source.isAlly(armorTailHolder) || move.target === 'all') && move.priority > 0.1) {
///         this.attrLastMove('[still]');
///         this.add('cant', armorTailHolder, 'ability: Armor Tail', move, `[of] ${target}`);
///         return false;
///     }
/// }
pub fn on_foe_try_move(_battle: &mut Battle, _target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, _move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

