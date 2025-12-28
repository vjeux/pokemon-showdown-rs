//! Nature's Madness Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// damageCallback(pokemon, target) {
///     return this.clampIntRange(Math.floor(target.getUndynamaxedHP() / 2), 1);
/// }
pub fn damage_callback(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // return this.clampIntRange(Math.floor(target.getUndynamaxedHP() / 2), 1);
    let damage = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let undynamaxed_hp = target_pokemon.max_hp_undynamaxed;
        // Math.floor(target.getUndynamaxedHP() / 2)
        let damage = undynamaxed_hp / 2;
        // this.clampIntRange(..., 1)
        std::cmp::max(damage, 1)
    };

    EventResult::Number(damage)
}

