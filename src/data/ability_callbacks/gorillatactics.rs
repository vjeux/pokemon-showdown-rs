//! Gorilla Tactics Ability - Boosts Attack, locks into first move

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_MODIFY_ATK_PRIORITY: i32 = 5;

/// onModifyAtk - boost exists, can implement
pub fn on_modify_atk(_atk: u32, _attacker: &Pokemon, _defender: &Pokemon, _move: &MoveDef) -> AbilityHandlerResult {
    AbilityHandlerResult::ChainModify(6144, 4096) // 1.5x
}

/// TODO: onDisableMove needs implementation for choice locking
pub fn on_disable_move(_battle: &mut Battle, /* TODO */) -> AbilityHandlerResult {
    AbilityHandlerResult::Undefined
}
