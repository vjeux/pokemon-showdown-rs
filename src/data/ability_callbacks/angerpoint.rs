//! Anger Point Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target, source, move) {
///     if (!target.hp) return;
///     if (move?.effectType === 'Move' && target.getMoveHitData(move).crit) {
///         this.boost({ atk: 12 }, target, target);
///     }
/// }
pub fn on_hit(battle: &mut Battle, target_pos: (usize, usize), _source_pos: (usize, usize), active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult { let move_id = active_move.map(|m| m.id.as_str()).unwrap_or("");
    // if (!target.hp) return;
    let has_hp = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target.hp > 0
    };

    if !has_hp {
        return EventResult::Continue;
    }

    // if (move?.effectType === 'Move' && target.getMoveHitData(move).crit)
    // Check if move exists in moves dex (effectType === 'Move')
    if battle.dex.moves().get(move_id).is_some() {
        if let Some(hit_data) = battle.get_move_hit_data(target_pos) {
            if hit_data.crit {
                // this.boost({ atk: 12 }, target, target);
                battle.boost(
                    &[("atk", 12)],
                    target_pos,
                    Some(target_pos),
                    None,
                    false,
                    false,
                );
            }
        }
    }

    EventResult::Continue
}

