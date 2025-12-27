//! Electromorphosis Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	electromorphosis: {
//! 		onDamagingHitOrder: 1,
//! 		onDamagingHit(damage, target, source, move) {
//! 			target.addVolatile('charge');
//! 		},
//! 		flags: {},
//! 		name: "Electromorphosis",
//! 		rating: 3,
//! 		num: 280,
//! 	},
//! ```


use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_DAMAGING_HIT_ORDER: i32 = 1;

    /// onDamagingHit(damage, target, source, move)
    pub fn on_damaging_hit(_battle: &mut Battle, _damage: i32, target: &mut Pokemon, _source: &Pokemon, _move_: &MoveDef) -> AbilityHandlerResult {
        target.add_volatile(ID::new("charge"));
        AbilityHandlerResult::Undefined
    }
