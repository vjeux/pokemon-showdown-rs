//! Sheer Force Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onModifyMove(move, pokemon) {
///     if (move.secondaries) {
///         delete move.secondaries;
///         // Technically not a secondary effect, but it is negated
///         delete move.self;
///         if (move.id === 'clangoroussoulblaze') delete move.selfBoost;
///         // Actual negation of `AfterMoveSecondary` effects implemented in scripts.js
///         move.hasSheerForce = true;
///     }
/// }
pub fn on_modify_move(_battle: &mut Battle, active_move: Option<&mut crate::battle_actions::ActiveMove>, _pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    // if (move.secondaries)
    if let Some(active_move) = active_move {
        let move_id = active_move.id.as_str();
        // Check if secondaries exist (not empty)
        if !active_move.secondaries.is_empty() {
            // delete move.secondaries;
            active_move.secondaries.clear();

            // delete move.self;
            // Technically not a secondary effect, but it is negated
            active_move.self_effect = None;

            // if (move.id === 'clangoroussoulblaze') delete move.selfBoost;
            if move_id == "clangoroussoulblaze" {
                active_move.self_boost = None;
            }

            // move.hasSheerForce = true;
            // Actual negation of `AfterMoveSecondary` effects implemented in scripts.js
            active_move.has_sheer_force = true;
        }
    }

    EventResult::Continue
}

/// onBasePower(basePower, pokemon, target, move) {
///     if (move.hasSheerForce) return this.chainModify([5325, 4096]);
/// }
pub fn on_base_power(battle: &mut Battle, _base_power: i32, _attacker_pos: (usize, usize), _defender_pos: (usize, usize), active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // if (move.hasSheerForce) return this.chainModify([5325, 4096]);
    let has_sheer_force = if let Some(active_move) = active_move {
        active_move.has_sheer_force
    } else {
        return EventResult::Continue;
    };

    if has_sheer_force {
        // 5325/4096 = ~1.3x power boost
        battle.chain_modify_fraction(5325, 4096); return EventResult::Continue;
    }

    EventResult::Continue
}

