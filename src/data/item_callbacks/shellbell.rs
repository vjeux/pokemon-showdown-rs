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
pub fn on_after_move_secondary_self(battle: &mut Battle, source_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    // if (move.totalDamage && !pokemon.forceSwitchFlag) {
    //     this.heal(move.totalDamage / 8, pokemon);
    // }

    // Check if active move has totalDamage
    let total_damage = battle.active_move.as_ref()
        .map(|m| m.total_damage)
        .unwrap_or(0);

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
    battle.heal(total_damage / 8, Some(source_pos), None, None);

    EventResult::Continue
}
