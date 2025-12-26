//! Hunger Switch Ability - Morpeko forme changes each turn

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_RESIDUAL_ORDER: i32 = 29;

/// TODO: Requires forme change system
pub fn on_residual(_battle: &mut Battle, /* TODO */) -> AbilityHandlerResult {
    AbilityHandlerResult::Undefined
}
