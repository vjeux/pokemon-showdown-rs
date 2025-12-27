//! Haze Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHitField() {
///     this.add('-clearallboost');
///     for (const pokemon of this.getAllActive()) {
///         pokemon.clearBoosts();
///     }
/// }
pub fn on_hit_field(battle: &mut Battle) -> EventResult {
    // TODO: battle.add('-clearallboost');

    // Get all active pokemon positions
    let active_positions = battle.get_all_active();

    // Clear boosts for all active pokemon
    for pos in active_positions {
        if let Some(pokemon) = battle.pokemon_at_mut(pos.0, pos.1) {
            pokemon.clear_boosts();
        }
    }

    EventResult::Continue
}

