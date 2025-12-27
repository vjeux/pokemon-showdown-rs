//! Endeavor Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// damageCallback(pokemon, target) {
///     return target.getUndynamaxedHP() - pokemon.hp;
/// }
pub fn damage_callback(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

/// onTryImmunity(target, pokemon) {
///     return pokemon.hp < target.hp;
/// }
pub fn on_try_immunity(battle: &mut Battle, target_pos: Option<(usize, usize)>, pokemon_pos: (usize, usize)) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

