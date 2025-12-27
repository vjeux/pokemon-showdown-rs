//! Substitute Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(source) {
///     if (source.volatiles['substitute']) {
///         this.add('-fail', source, 'move: Substitute');
///         return this.NOT_FAIL;
///     }
///     if (source.hp <= source.maxhp / 4 || source.maxhp === 1) { // Shedinja clause
///         this.add('-fail', source, 'move: Substitute', '[weak]');
///         return this.NOT_FAIL;
///     }
/// }
pub fn on_try_hit(battle: &mut Battle, source_pos: (usize, usize), target_pos: (usize, usize), move_id: Option<&str>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onHit(target) {
///     this.directDamage(target.maxhp / 4);
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: Option<&str>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onStart(target, source, effect) {
    ///     if (effect?.id === 'shedtail') {
    ///         this.add('-start', target, 'Substitute', '[from] move: Shed Tail');
    ///     } else {
    ///         this.add('-start', target, 'Substitute');
    ///     }
    ///     this.effectState.hp = Math.floor(target.maxhp / 4);
    ///     if (target.volatiles['partiallytrapped']) {
    ///         this.add('-end', target, target.volatiles['partiallytrapped'].sourceEffect, '[partiallytrapped]', '[silent]');
    ///         delete target.volatiles['partiallytrapped'];
    ///     }
    /// }
    pub fn on_start(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onTryPrimaryHit(target, source, move) {
    ///     if (target === source || move.flags['bypasssub'] || move.infiltrates) {
    ///         return;
    ///     }
    ///     let damage = this.actions.getDamage(source, target, move);
    ///     if (!damage && damage !== 0) {
    ///         this.add('-fail', source);
    ///         this.attrLastMove('[still]');
    ///         return null;
    ///     }
    ///     if (damage > target.volatiles['substitute'].hp) {
    ///         damage = target.volatiles['substitute'].hp as number;
    ///     }
    ///     target.volatiles['substitute'].hp -= damage;
    ///     source.lastDamage = damage;
    ///     if (target.volatiles['substitute'].hp <= 0) {
    ///         if (move.ohko) this.add('-ohko');
    ///         target.removeVolatile('substitute');
    ///     } else {
    ///         this.add('-activate', target, 'move: Substitute', '[damage]');
    ///     }
    ///     if (move.recoil || move.id === 'chloroblast') {
    ///         this.damage(this.actions.calcRecoilDamage(damage, move, source), source, target, 'recoil');
    ///     }
    ///     if (move.drain) {
    ///         this.heal(Math.ceil(damage * move.drain[0] / move.drain[1]), source, target, 'drain');
    ///     }
    ///     this.singleEvent('AfterSubDamage', move, null, target, source, move, damage);
    ///     this.runEvent('AfterSubDamage', target, source, move, damage);
    ///     return this.HIT_SUBSTITUTE;
    /// }
    pub fn on_try_primary_hit(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onEnd(target) {
    ///     this.add('-end', target, 'Substitute');
    /// }
    pub fn on_end(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }
}
