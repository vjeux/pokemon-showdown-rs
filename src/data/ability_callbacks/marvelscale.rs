//! Marvel Scale Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onModifyDef(def, pokemon) {
///     if (pokemon.status) {
///         return this.chainModify(1.5);
///     }
/// }
pub fn on_modify_def(battle: &mut Battle, _def: i32, defender_pos: (usize, usize), _attacker_pos: (usize, usize), _active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    if let Some(pokemon) = battle.pokemon_at(defender_pos.0, defender_pos.1) {
        if !pokemon.status.is_empty() {
            battle.chain_modify(1.5);
            return EventResult::Continue;
        }
    }
    EventResult::Continue
}

