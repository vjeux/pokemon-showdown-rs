//! Curious Medicine Ability - Clears ally stat changes on switch-in

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onStart(pokemon)
/// Clears all stat boosts from adjacent allies
///
/// TODO: Requires:
/// - pokemon.adjacentAllies() to get adjacent partners in doubles
/// - ally.clearBoosts() to reset all stat changes
/// When implemented: Loops through adjacentAllies and clears their boosts
pub fn on_start(_battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    AbilityHandlerResult::Undefined
}
