//! Sap Sipper Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(target, source, move) {
///     if (target !== source && move.type === 'Grass') {
///         if (!this.boost({ atk: 1 })) {
///             this.add('-immune', target, '[from] ability: Sap Sipper');
///         }
///         return null;
///     }
/// }
pub fn on_try_hit(battle: &mut Battle, target_pos: (usize, usize), source_pos: (usize, usize), active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult { let move_id = active_move.map(|m| m.id.as_str()).unwrap_or("");
    // Immune to Grass-type moves and boost Attack by 1
    if target_pos != source_pos {
        // Check if the move is Grass-type
        let is_grass = {
            let move_data = match battle.dex.moves().get(move_id) {
                Some(m) => m,
                None => return EventResult::Continue,
            };
            move_data.move_type == "Grass"
        };

        if is_grass {
            // Try to boost Attack
            battle.boost(&[("atk", 1)], target_pos, None, None, false, false);
            // Return Null to prevent the move from hitting
            return EventResult::Null;
        }
    }
    EventResult::Continue
}

/// onAllyTryHitSide(target, source, move) {
///     if (source === this.effectState.target || !target.isAlly(source)) return;
///     if (move.type === 'Grass') {
///         this.boost({ atk: 1 }, this.effectState.target);
///     }
/// }
pub fn on_ally_try_hit_side(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult { let _move_id = active_move.map(|m| m.id.as_str()).unwrap_or("");
    

    // if (source === this.effectState.target || !target.isAlly(source)) return;
    let ability_holder = match battle.effect_state.target {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let source_pos = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // Skip if source is the ability holder
    if source_pos == ability_holder {
        return EventResult::Continue;
    }

    // Skip if target is not an ally of source
    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    if !battle.is_ally(target_pos, source_pos) {
        return EventResult::Continue;
    }

    // if (move.type === 'Grass')
    let is_grass = {
        let active_move = match active_move {
            Some(m) => m,
            None => return EventResult::Continue,
        };
        active_move.move_type == "Grass"
    };

    if is_grass {
        // this.boost({ atk: 1 }, this.effectState.target);
        battle.boost(&[("atk", 1)], ability_holder, None, None, false, false);
    }

    EventResult::Continue
}

