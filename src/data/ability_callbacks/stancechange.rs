//! Stance Change Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyMove(move, attacker, defender) {
///     if (attacker.species.baseSpecies !== 'Aegislash' || attacker.transformed) return;
///     if (move.category === 'Status' && move.id !== 'kingsshield') return;
///     const targetForme = (move.id === 'kingsshield' ? 'Aegislash' : 'Aegislash-Blade');
///     if (attacker.species.name !== targetForme) attacker.formeChange(targetForme);
/// }
pub fn on_modify_move(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

