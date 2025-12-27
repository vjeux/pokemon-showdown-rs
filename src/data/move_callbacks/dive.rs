//! Dive Move
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
///     if (attacker.hasAbility('gulpmissile') && attacker.species.name === 'Cramorant' && !attacker.transformed) {
///         const forme = attacker.hp <= attacker.maxhp / 2 ? 'cramorantgorging' : 'cramorantgulping';
///         attacker.formeChange(forme, move);
///     }
///     this.add('-prepare', attacker, move.name);
///     if (!this.runEvent('ChargeMove', attacker, defender, move)) {
///         return;
///     }
///     attacker.addVolatile('twoturnmove', defender);
///     return null;
/// }
pub fn on_try_move(battle: &mut Battle, source_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onImmunity(type, pokemon) {
    ///     if (type === 'sandstorm' || type === 'hail') return false;
    /// }
    pub fn on_immunity(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onInvulnerability(target, source, move) {
    ///     if (['surf', 'whirlpool'].includes(move.id)) {
    ///         return;
    ///     }
    ///     return false;
    /// }
    pub fn on_invulnerability(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
        // Dive makes the user invulnerable, except to Surf and Whirlpool
        if move_id == "surf" || move_id == "whirlpool" {
            return EventResult::Continue;
        }
        EventResult::Bool(false)
    }

    /// onSourceModifyDamage(damage, source, target, move) {
    ///     if (move.id === 'surf' || move.id === 'whirlpool') {
    ///         return this.chainModify(2);
    ///     }
    /// }
    pub fn on_source_modify_damage(battle: &mut Battle, damage: i32, source_pos: Option<(usize, usize)>, target_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
        // Surf and Whirlpool deal double damage to diving Pokemon
        if move_id == "surf" || move_id == "whirlpool" {
            EventResult::Number(damage * 2)
        } else {
            EventResult::Continue
        }
    }
}
