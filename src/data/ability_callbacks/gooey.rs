//! Gooey Ability - Lowers Speed on contact

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onDamagingHit - already has infrastructure, implement it
pub fn on_damaging_hit(battle: &mut Battle, _damage: u32, target: &Pokemon, source: &mut Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    let source_ref = (source.side_index, source.position);
    if battle.check_move_makes_contact(&move_.id, source_ref) {
        battle.add("-ability", &[Arg::Pokemon(target), Arg::Str("Gooey")]);
        let target_ref = (target.side_index, target.position);
        battle.boost(&[("spe", -1)], source_ref, Some(target_ref), None);
    }
    AbilityHandlerResult::Undefined
}
