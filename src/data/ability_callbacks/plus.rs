//! Plus Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onModifySpA(spa, pokemon) {
///     for (const allyActive of pokemon.allies()) {
///         if (allyActive.hasAbility(['minus', 'plus'])) {
///             return this.chainModify(1.5);
///         }
///     }
/// }
pub fn on_modify_sp_a(battle: &mut Battle, _spa: i32, attacker_pos: (usize, usize), _defender_pos: (usize, usize), _active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // for (const allyActive of pokemon.allies()) {
    //     if (allyActive.hasAbility(['minus', 'plus'])) {
    //         return this.chainModify(1.5);
    //     }
    // }

    // Check if any ally has Plus or Minus ability
    let has_synergy = {
        let pokemon = match battle.pokemon_at(attacker_pos.0, attacker_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let allies = pokemon.allies(battle, false);

        allies.iter().any(|&ally_pos| {
            if let Some(ally) = battle.pokemon_at(ally_pos.0, ally_pos.1) {
                ally.has_ability(battle, &["minus", "plus"])
            } else {
                false
            }
        })
    };

    if has_synergy {
        battle.chain_modify(1.5); return EventResult::Continue;
    }

    EventResult::Continue
}

