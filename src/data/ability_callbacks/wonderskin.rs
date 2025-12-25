//! Wonder Skin Ability - Status moves have 50% accuracy

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_MODIFY_ACCURACY_PRIORITY: i32 = 10;

/// onModifyAccuracy(accuracy, target, source, move)
/// Reduces accuracy of status moves to 50%
pub fn on_modify_accuracy(_battle: &mut Battle, _accuracy: u32, _target: &Pokemon, _source: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    // if (move.category === 'Status' && typeof accuracy === 'number')
    if move_.category == MoveCategory::Status {
        // this.debug('Wonder Skin - setting accuracy to 50');
        // return 50;
        return AbilityHandlerResult::Number(50);
    }
    AbilityHandlerResult::Undefined
}

