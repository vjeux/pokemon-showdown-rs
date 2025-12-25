//! Frisk Ability - Reveals foe's held items

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// TODO: Requires foes() iterator, target.item and getItem() access
pub fn on_start(_battle: &mut Battle, /* TODO */) -> AbilityHandlerResult {
    AbilityHandlerResult::Undefined
}
