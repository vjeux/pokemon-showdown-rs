//! Bide Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// beforeMoveCallback(pokemon) {
///     if (pokemon.volatiles['bide']) return true;
/// }
pub fn before_move_callback(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onStart(pokemon) {
    ///     this.effectState.totalDamage = 0;
    ///     this.add('-start', pokemon, 'move: Bide');
    /// }
    pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onDamage(damage, target, source, move) {
    ///     if (!move || move.effectType !== 'Move' || !source) return;
    ///     this.effectState.totalDamage += damage;
    ///     this.effectState.lastDamageSource = source;
    /// }
    pub fn on_damage(battle: &mut Battle, damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onBeforeMove(pokemon, target, move) {
    ///     if (this.effectState.duration === 1) {
    ///         this.add('-end', pokemon, 'move: Bide');
    ///         target = this.effectState.lastDamageSource;
    ///         if (!target || !this.effectState.totalDamage) {
    ///             this.attrLastMove('[still]');
    ///             this.add('-fail', pokemon);
    ///             return false;
    ///         }
    ///         if (!target.isActive) {
    ///             const possibleTarget = this.getRandomTarget(pokemon, this.dex.moves.get('pound'));
    ///             if (!possibleTarget) {
    ///                 this.add('-miss', pokemon);
    ///                 return false;
    ///             }
    ///             target = possibleTarget;
    ///         }
    ///         const moveData: Partial<ActiveMove> = {
    ///             id: 'bide' as ID,
    ///             name: "Bide",
    ///             accuracy: true,
    ///             damage: this.effectState.totalDamage * 2,
    ///             category: "Physical",
    ///             priority: 1,
    ///             flags: { contact: 1, protect: 1 },
    ///             effectType: 'Move',
    ///             type: 'Normal',
    ///         };
    ///         this.actions.tryMoveHit(target, pokemon, moveData as ActiveMove);
    ///         pokemon.removeVolatile('bide');
    ///         return false;
    ///     }
    ///     this.add('-activate', pokemon, 'move: Bide');
    /// }
    pub fn on_before_move(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onMoveAborted(pokemon) {
    ///     pokemon.removeVolatile('bide');
    /// }
    pub fn on_move_aborted(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onEnd(pokemon) {
    ///     this.add('-end', pokemon, 'move: Bide', '[silent]');
    /// }
    pub fn on_end(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

}
