//! Costar Ability - Copies ally's stat boosts on switch-in

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_SWITCH_IN_PRIORITY: i32 = -2;

/// onStart(pokemon)
/// Copies ally's stat boosts and critical hit volatiles
///
/// TODO: Requires:
/// - pokemon.allies() to get partner in doubles
/// - Direct boost copying (pokemon.boosts = ally.boosts)
/// - Volatile status system (addVolatile/removeVolatile)
/// - Specific volatile layers copying (gmaxchistrike, dragoncheer)
/// When implemented: Copies all stat boosts and crit-related volatiles from ally
pub fn on_start(_battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    AbilityHandlerResult::Undefined
}
