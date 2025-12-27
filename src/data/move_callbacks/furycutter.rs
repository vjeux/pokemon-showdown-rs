//! Fury Cutter Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// basePowerCallback(pokemon, target, move) {
///     if (!pokemon.volatiles['furycutter'] || move.hit === 1) {
///         pokemon.addVolatile('furycutter');
///     }
///     const bp = this.clampIntRange(move.basePower * pokemon.volatiles['furycutter'].multiplier, 1, 160);
///     this.debug(`BP: ${bp}`);
///     return bp;
/// }
pub fn base_power_callback(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: &str) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}


// Condition handlers
pub mod condition {
    use super::*;

    /// onStart() {
    ///     this.effectState.multiplier = 1;
    /// }
    pub fn on_start(battle: &mut Battle) -> MoveHandlerResult {
        // TODO: Implement 1-to-1 from JS
        MoveHandlerResult::Undefined
    }

    /// onRestart() {
    ///     if (this.effectState.multiplier < 4) {
    ///         this.effectState.multiplier <<= 1;
    ///     }
    ///     this.effectState.duration = 2;
    /// }
    pub fn on_restart(battle: &mut Battle) -> MoveHandlerResult {
        // TODO: Implement 1-to-1 from JS
        MoveHandlerResult::Undefined
    }

}
