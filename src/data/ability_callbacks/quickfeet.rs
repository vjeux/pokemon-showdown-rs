//! Quick Feet Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onModifySpe(spe, pokemon) {
///     if (pokemon.status) {
///         return this.chainModify(1.5);
///     }
/// }
pub fn on_modify_spe(battle: &mut Battle, _spe: i32, pokemon_pos: (usize, usize)) -> EventResult {
    if let Some(pokemon) = battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
        if !pokemon.status.is_empty() {
            battle.chain_modify(1.5);
            return EventResult::Continue;
        }
    }
    EventResult::Continue
}

