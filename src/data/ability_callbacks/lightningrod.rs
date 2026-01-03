//! Lightning Rod Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(target, source, move) {
///     if (target !== source && move.type === 'Electric') {
///         if (!this.boost({ spa: 1 })) {
///             this.add('-immune', target, '[from] ability: Lightning Rod');
///         }
///         return null;
///     }
/// }
pub fn on_try_hit(battle: &mut Battle, target_pos: (usize, usize), source_pos: (usize, usize), move_id: &str) -> EventResult {
    // Immune to Electric-type moves and boost Special Attack by 1
    if target_pos != source_pos {
        // Check if the move is Electric-type
        let is_electric = {
            let move_data = match battle.dex.moves().get(move_id) {
                Some(m) => m,
                None => return EventResult::Continue,
            };
            move_data.move_type == "Electric"
        };

        if is_electric {
            // Boost Special Attack by 1
            battle.boost(&[("spa", 1)], target_pos, None, None, false, false);
            // Return Null to prevent the move from hitting
            return EventResult::Null;
        }
    }
    EventResult::Continue
}

/// onAnyRedirectTarget(target, source, source2, move) {
///     if (move.type !== 'Electric' || move.flags['pledgecombo']) return;
///     const redirectTarget = ['randomNormal', 'adjacentFoe'].includes(move.target) ? 'normal' : move.target;
///     if (this.validTarget(this.effectState.target, source, redirectTarget)) {
///         if (move.smartTarget) move.smartTarget = false;
///         if (this.effectState.target !== target) {
///             this.add('-activate', this.effectState.target, 'ability: Lightning Rod');
///         }
///         return this.effectState.target;
///     }
/// }
pub fn on_any_redirect_target(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    // onAnyRedirectTarget(target, source, source2, move) {
    //     if (move.type !== 'Electric' || move.flags['pledgecombo']) return;
    //     const redirectTarget = ['randomNormal', 'adjacentFoe'].includes(move.target) ? 'normal' : move.target;
    //     if (this.validTarget(this.effectState.target, source, redirectTarget)) {
        //         if (move.smartTarget) move.smartTarget = false;
        //         if (this.effectState.target !== target) {
        //             this.add('-activate', this.effectState.target, 'ability: Lightning Rod');
        //         }
        //         return this.effectState.target;
    //     }
    // }

    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (move.type !== 'Electric' || move.flags['pledgecombo']) return;
    let (is_electric, has_pledgecombo) = {
        let active_move = match &battle.active_move {
            Some(m) => m,
            None => return EventResult::Continue,
        };
        (active_move.move_type == "Electric", active_move.flags.pledgecombo)
    };

    if !is_electric || has_pledgecombo {
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

    // Get the Pokemon with Lightning Rod from effect_state.target
    // if (this.validTarget(this.effectState.target, source, redirectTarget)) {
    let lightning_rod_holder = match battle.effect_state.target {
        Some(target) => target,
        None => return EventResult::Continue,
    };

    if battle.valid_target(lightning_rod_holder, source, redirect_target) {
        // if (move.smartTarget) move.smartTarget = false;
        if let Some(ref mut active_move) = battle.active_move {
            if active_move.smart_target == Some(true) {
                active_move.smart_target = Some(false);
            }
        }

        // if (this.effectState.target !== target) {
        //     this.add('-activate', this.effectState.target, 'ability: Lightning Rod');
        // }
        if Some(lightning_rod_holder) != target_pos {
            let holder_slot = {
                let pokemon = match battle.pokemon_at(lightning_rod_holder.0, lightning_rod_holder.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                pokemon.get_slot()
            };
            battle.add("-activate", &[holder_slot.into(), "ability: Lightning Rod".into()]);
        }

        // return this.effectState.target;
        return EventResult::Position(lightning_rod_holder);
    }

    EventResult::Continue
}

