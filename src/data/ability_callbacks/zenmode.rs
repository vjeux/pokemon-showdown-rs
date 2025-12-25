//! Zen Mode Ability - Darmanitan changes forme at low HP

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_RESIDUAL_ORDER: i32 = 29;

/// onResidual, onEnd
/// Changes Darmanitan forme based on HP
///
/// TODO: Needs volatile status system
/// TODO: Needs forme change system (pokemon.formeChange)
/// TODO: Needs pokemon.transformed tracking
pub fn on_residual(_battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    AbilityHandlerResult::Undefined
}

pub fn on_end(_battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    AbilityHandlerResult::Undefined
}
