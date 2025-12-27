//! Water Bubble Ability - Halves Fire damage taken, doubles Water damage dealt, prevents burns

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_SOURCE_MODIFY_ATK_PRIORITY: i32 = 5;
pub const ON_SOURCE_MODIFY_SPA_PRIORITY: i32 = 5;
pub const ON_MODIFY_ATK_PRIORITY: i32 = 5;
pub const ON_MODIFY_SP_A_PRIORITY: i32 = 5;

/// Halves Fire-type attack damage taken
pub fn on_source_modify_atk(_atk: i32, _attacker: &Pokemon, _defender: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    if move_.move_type == "Fire" {
        return AbilityHandlerResult::ChainModify(2048, 4096); // 0.5x
    }
    AbilityHandlerResult::Undefined
}

/// Halves Fire-type special attack damage taken
pub fn on_source_modify_sp_a(_spa: i32, _attacker: &Pokemon, _defender: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    if move_.move_type == "Fire" {
        return AbilityHandlerResult::ChainModify(2048, 4096); // 0.5x
    }
    AbilityHandlerResult::Undefined
}

/// Doubles Water-type attack damage dealt
pub fn on_modify_atk(_atk: i32, _attacker: &Pokemon, _defender: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    if move_.move_type == "Water" {
        return AbilityHandlerResult::ChainModify(8192, 4096); // 2x
    }
    AbilityHandlerResult::Undefined
}

/// Doubles Water-type special attack damage dealt
pub fn on_modify_sp_a(_spa: i32, _attacker: &Pokemon, _defender: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    if move_.move_type == "Water" {
        return AbilityHandlerResult::ChainModify(8192, 4096); // 2x
    }
    AbilityHandlerResult::Undefined
}

/// Prevents burns (onUpdate)
pub fn on_update(battle: &mut Battle, pokemon: &mut Pokemon) -> AbilityHandlerResult {
    if pokemon.status.as_str() == "brn" {
        battle.add("-activate", &[Arg::Pokemon(pokemon), Arg::Str("ability: Water Bubble")]);
        pokemon.cure_status();
    }
    AbilityHandlerResult::Undefined
}

/// Prevents burns (onSetStatus)
pub fn on_set_status(_battle: &mut Battle, status: &Status, _target: &Pokemon, _source: Option<&Pokemon>, effect: &Effect) -> AbilityHandlerResult {
    if status.id != "brn" {
        return AbilityHandlerResult::Undefined;
    }
    if effect.status.is_some() {
        // Note: battle is not mutable here, can't add logs
    }
    AbilityHandlerResult::False
}
