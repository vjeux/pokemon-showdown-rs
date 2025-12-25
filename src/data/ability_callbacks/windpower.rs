//! Wind Power Ability - Gets charged when hit by wind moves

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_DAMAGING_HIT_ORDER: i32 = 1;

/// onDamagingHit, onSideConditionStart
/// Gets Charge status when hit by wind moves or Tailwind is set
///
/// TODO: Needs volatile status system (addVolatile charge)
/// TODO: Needs side condition system for Tailwind detection
pub fn on_damaging_hit(_battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    AbilityHandlerResult::Undefined
}

pub fn on_side_condition_start(_battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    AbilityHandlerResult::Undefined
}
