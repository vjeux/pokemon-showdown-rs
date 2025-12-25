//! Perish Body Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	perishbody: {
//! 		onDamagingHit(damage, target, source, move) {
//! 			if (!this.checkMoveMakesContact(move, source, target) || source.volatiles['perishsong']) return;
//! 			this.add('-ability', target, 'Perish Body');
//! 			source.addVolatile('perishsong');
//! 			target.addVolatile('perishsong');
//! 		},
//! 		flags: {},
//! 		name: "Perish Body",
//! 		rating: 1,
//! 		num: 253,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onDamagingHit(damage, target, source, move)
/// Inflicts Perish Song on both self and attacker when hit by contact move
pub fn on_damaging_hit(battle: &mut Battle, _damage: u32, target: &Pokemon, source: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    // if (!this.checkMoveMakesContact(move, source, target) || source.volatiles['perishsong']) return;
    if !move_.flags.contact {
        return AbilityHandlerResult::Undefined;
    }

    if source.has_volatile(&ID::new("perishsong")) {
        return AbilityHandlerResult::Undefined;
    }

    // this.add('-ability', target, 'Perish Body');
    battle.add("-ability", &[
        Arg::Pokemon(target),
        Arg::Str("Perish Body")
    ]);

    // source.addVolatile('perishsong');
    let source_ref = (source.side_index, source.position);
    battle.sides[source_ref.0].pokemon[source_ref.1].add_volatile(ID::new("perishsong"));

    // target.addVolatile('perishsong');
    let target_ref = (target.side_index, target.position);
    battle.sides[target_ref.0].pokemon[target_ref.1].add_volatile(ID::new("perishsong"));

    AbilityHandlerResult::Undefined
}

