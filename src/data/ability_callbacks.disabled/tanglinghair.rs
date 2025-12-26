//! Tangling Hair Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	tanglinghair: {
//! 		onDamagingHit(damage, target, source, move) {
//! 			if (this.checkMoveMakesContact(move, source, target, true)) {
//! 				this.add('-ability', target, 'Tangling Hair');
//! 				this.boost({ spe: -1 }, source, target, null, true);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Tangling Hair",
//! 		rating: 2,
//! 		num: 221,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onDamagingHit(damage, target, source, move)
/// Lowers Speed of attackers that make contact
pub fn on_damaging_hit(battle: &mut Battle, _damage: u32, target: &Pokemon, source: &mut Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    // if (this.checkMoveMakesContact(move, source, target, true))
    let source_ref = (source.side_index, source.position);
    if battle.check_move_makes_contact(&move_.id, source_ref) {
        // this.add('-ability', target, 'Tangling Hair');
        battle.add("-ability", &[Arg::Pokemon(target), Arg::Str("Tangling Hair")]);
        // this.boost({ spe: -1 }, source, target, null, true);
        let target_ref = (target.side_index, target.position);
        battle.boost(&[("spe", -1)], source_ref, Some(target_ref), None);
    }
    AbilityHandlerResult::Undefined
}

