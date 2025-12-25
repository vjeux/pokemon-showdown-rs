//! Flower Veil Ability - Prevents stat/status on Grass allies

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// TODO: Requires onAllyTryBoost, onAllySetStatus handlers
pub fn on_ally_try_boost(_battle: &mut Battle, /* TODO */) -> AbilityHandlerResult {
    AbilityHandlerResult::Undefined
}
pub fn on_ally_set_status(_battle: &mut Battle, /* TODO */) -> AbilityHandlerResult {
    AbilityHandlerResult::Undefined
}
