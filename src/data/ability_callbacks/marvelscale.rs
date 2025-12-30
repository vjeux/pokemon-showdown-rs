//! Marvel Scale Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyDef(def, pokemon) {
///     if (pokemon.status) {
///         return this.chainModify(1.5);
///     }
/// }
pub fn on_modify_def(battle: &mut Battle, _def: i32, defender_pos: (usize, usize), _attacker_pos: (usize, usize), _move_id: &str) -> EventResult {
    if let Some(pokemon) = battle.pokemon_at(defender_pos.0, defender_pos.1) {
        if !pokemon.status.is_empty() {
            let modified = battle.chain_modify(1.5);
            return EventResult::Number(modified);
        }
    }
    EventResult::Continue
}

