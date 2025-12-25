//! Battle Armor Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	battlearmor: {
//! 		onCriticalHit: false,
//! 		flags: { breakable: 1 },
//! 		name: "Battle Armor",
//! 		rating: 1,
//! 		num: 4,
//! 	},
//! ```


use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onCriticalHit: false
    /// Prevents critical hits against this Pokemon
    pub fn on_critical_hit() -> AbilityHandlerResult {
        // onCriticalHit: false,
        AbilityHandlerResult::False
    }
