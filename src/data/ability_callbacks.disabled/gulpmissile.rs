//! Gulp Missile Ability - Cramorant catches prey with Surf/Dive

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// TODO: Complex - requires forme change, onSourceTryPrimaryHit, onDamagingHit
pub fn on_source_try_primary_hit(_battle: &mut Battle, /* TODO */) -> AbilityHandlerResult {
    AbilityHandlerResult::Undefined
}
pub fn on_damaging_hit(_battle: &mut Battle, /* TODO */) -> AbilityHandlerResult {
    AbilityHandlerResult::Undefined
}
