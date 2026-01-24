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
pub fn on_try_hit(battle: &mut Battle, target_pos: (usize, usize), source_pos: (usize, usize), active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // Immune to Electric-type moves and boost Special Attack by 1
    // if (target !== source && move.type === 'Electric') {
    if target_pos != source_pos {
        // Check if the move is Electric-type
        // JavaScript checks move.type (the active move's type, not the dex type)
        // This is important for moves like Electrify which change the move type at runtime
        let is_electric = active_move.map(|m| m.move_type == "Electric").unwrap_or(false);

        if is_electric {
            // Boost Special Attack by 1
            // if (!this.boost({ spa: 1 })) {
            //     this.add('-immune', target, '[from] ability: Lightning Rod');
            // }
            battle.boost(&[("spa", 1)], target_pos, None, None, false, false);
            // Return Null to prevent the move from hitting
            // return null;
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
pub fn on_any_redirect_target(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, _source2_pos: Option<(usize, usize)>, active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult { let _move_id = active_move.map(|m| m.id.as_str()).unwrap_or("");
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

    // Get the active_move from parameter
    let active_move_ref = match active_move {
        Some(m) => m,
        None => return EventResult::Continue,
    };

    // if (move.type !== 'Electric' || move.flags['pledgecombo']) return;
    if active_move_ref.move_type != "Electric" || active_move_ref.flags.pledgecombo {
        return EventResult::Continue;
    }

    // const redirectTarget = ['randomNormal', 'adjacentFoe'].includes(move.target) ? 'normal' : move.target;
    let redirect_target = if active_move_ref.target == "randomNormal" || active_move_ref.target == "adjacentFoe" {
        "normal"
    } else {
        active_move_ref.target.as_str()
    };

    // Get the Pokemon with Lightning Rod from effect_state.target
    // if (this.validTarget(this.effectState.target, source, redirectTarget)) {
    let lightning_rod_holder = match battle.effect_state.borrow().target {
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

