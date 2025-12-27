//! Gastro Acid Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use crate::event::EventResult;
use super::{Status, Effect};

/// onTryHit(target) {
///     if (target.getAbility().flags['cantsuppress']) {
///         return false;
///     }
///     if (target.hasItem('Ability Shield')) {
///         this.add('-block', target, 'item: Ability Shield');
///         return null;
///     }
/// }
pub fn on_try_hit(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}


// Condition handlers
pub mod condition {
    use super::*;

    /// onStart(pokemon) {
    ///     if (pokemon.hasItem('Ability Shield')) return false;
    ///     this.add('-endability', pokemon);
    ///     this.singleEvent('End', pokemon.getAbility(), pokemon.abilityState, pokemon, pokemon, 'gastroacid');
    /// }
    pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onCopy(pokemon) {
    ///     if (pokemon.getAbility().flags['cantsuppress']) pokemon.removeVolatile('gastroacid');
    /// }
    pub fn on_copy(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

}
