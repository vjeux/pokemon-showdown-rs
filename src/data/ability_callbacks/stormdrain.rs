//! Storm Drain Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(target, source, move) {
///     if (target !== source && move.type === 'Water') {
///         if (!this.boost({ spa: 1 })) {
///             this.add('-immune', target, '[from] ability: Storm Drain');
///         }
///         return null;
///     }
/// }
pub fn on_try_hit(battle: &mut Battle, target_pos: (usize, usize), source_pos: (usize, usize), move_id: &str) -> EventResult {
    // Immune to Water-type moves and boost Special Attack by 1
    if target_pos != source_pos {
        // Check if the move is Water-type
        let is_water = {
            let move_data = match battle.dex.moves().get(move_id) {
                Some(m) => m,
                None => return EventResult::Continue,
            };
            move_data.move_type == "Water"
        };

        if is_water {
            // Boost Special Attack by 1
            battle.boost(&[("spa", 1)], target_pos, None, None, false, false);
            // Return Null to prevent the move from hitting
            return EventResult::Null;
        }
    }
    EventResult::Continue
}

/// onAnyRedirectTarget(target, source, source2, move) {
///     if (move.type !== 'Water' || move.flags['pledgecombo']) return;
///     const redirectTarget = ['randomNormal', 'adjacentFoe'].includes(move.target) ? 'normal' : move.target;
///     if (this.validTarget(this.effectState.target, source, redirectTarget)) {
///         if (move.smartTarget) move.smartTarget = false;
///         if (this.effectState.target !== target) {
///             this.add('-activate', this.effectState.target, 'ability: Storm Drain');
///         }
///         return this.effectState.target;
///     }
/// }
pub fn on_any_redirect_target(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, _source2_pos: Option<(usize, usize)>, _move_id: &str) -> EventResult {
    // onAnyRedirectTarget(target, source, source2, move) {
    //     if (move.type !== 'Water' || move.flags['pledgecombo']) return;
    //     const redirectTarget = ['randomNormal', 'adjacentFoe'].includes(move.target) ? 'normal' : move.target;
    //     if (this.validTarget(this.effectState.target, source, redirectTarget)) {
        //         if (move.smartTarget) move.smartTarget = false;
        //         if (this.effectState.target !== target) {
        //             this.add('-activate', this.effectState.target, 'ability: Storm Drain');
        //         }
        //         return this.effectState.target;
    //     }
    // }

    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (move.type !== 'Water' || move.flags['pledgecombo']) return;
    let (is_water, has_pledgecombo) = {
        let active_move = match &battle.active_move {
            Some(m) => m,
            None => return EventResult::Continue,
        };
        (active_move.move_type == "Water", active_move.flags.pledgecombo)
    };

    if !is_water || has_pledgecombo {
        return EventResult::Continue;
    }

    // const redirectTarget = ['randomNormal', 'adjacentFoe'].includes(move.target) ? 'normal' : move.target;
    let redirect_target = {
        let active_move = match &battle.active_move {
            Some(m) => m,
            None => return EventResult::Continue,
        };
        if active_move.target == "randomNormal" || active_move.target == "adjacentFoe" {
            "normal"
        } else {
            active_move.target.as_str()
        }
    };

    // Get the Pokemon with Storm Drain from effect_state.target
    // if (this.validTarget(this.effectState.target, source, redirectTarget)) {
    let storm_drain_holder = match battle.effect_state.target {
        Some(target) => target,
        None => return EventResult::Continue,
    };

    if battle.valid_target(storm_drain_holder, source, redirect_target) {
        // if (move.smartTarget) move.smartTarget = false;
        if let Some(ref mut active_move) = battle.active_move {
            if active_move.smart_target == Some(true) {
                active_move.smart_target = Some(false);
            }
        }

        // if (this.effectState.target !== target) {
        //     this.add('-activate', this.effectState.target, 'ability: Storm Drain');
        // }
        if Some(storm_drain_holder) != target_pos {
            let holder_slot = {
                let pokemon = match battle.pokemon_at(storm_drain_holder.0, storm_drain_holder.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                pokemon.get_slot()
            };
            battle.add("-activate", &[holder_slot.into(), "ability: Storm Drain".into()]);
        }

        // return this.effectState.target;
        return EventResult::Position(storm_drain_holder);
    }

    EventResult::Continue
}

