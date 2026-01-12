//! Shell Bell Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onAfterMoveSecondarySelf(pokemon, target, move) {
///     if (move.totalDamage && !pokemon.forceSwitchFlag) {
///         this.heal(move.totalDamage / 8, pokemon);
///     }
/// }
pub fn on_after_move_secondary_self(battle: &mut Battle, source_pos: (usize, usize), _target_pos: Option<(usize, usize)>, active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // if (move.totalDamage && !pokemon.forceSwitchFlag) {
    //     this.heal(move.totalDamage / 8, pokemon);
    // }

    // Get active_move from parameter
    let active_move_ref = match active_move {
        Some(m) => m,
        None => return EventResult::Continue,
    };

    // Check if active move has totalDamage
    let total_damage = active_move_ref.total_damage;

    if total_damage == 0 {
        return EventResult::Continue;
    }

    // Check if pokemon has forceSwitchFlag
    let has_force_switch = {
        let pokemon = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.force_switch_flag
    };

    if has_force_switch {
        return EventResult::Continue;
    }

    // this.heal(move.totalDamage / 8, pokemon);
    // In JS, division produces a float (e.g., 7/8 = 0.875).
    // The heal() function then applies: if (damage && damage <= 1) damage = 1;
    // So any non-zero totalDamage results in at least 1 HP heal.
    // In Rust, we need to match this behavior by using max(1) for non-zero damage.
    let heal_amount = (total_damage / 8).max(1);
    battle.heal(heal_amount, Some(source_pos), None, None);

    EventResult::Continue
}
