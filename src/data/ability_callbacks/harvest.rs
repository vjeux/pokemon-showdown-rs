//! Harvest Ability - Restores Berry at end of turn

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// TODO: Requires onResidualOrder, item restoration, weather checking
pub const ON_RESIDUAL_ORDER: i32 = 28;
pub fn on_residual(_battle: &mut Battle, /* TODO */) -> AbilityHandlerResult {
    AbilityHandlerResult::Undefined
}
