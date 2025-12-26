//! Persistent Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	persistent: {
//! 		isNonstandard: "CAP",
//! 		// implemented in the corresponding move
//! 		flags: {},
//! 		name: "Persistent",
//! 		rating: 3,
//! 		num: -3,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

// Persistent has no callback handlers in the JS source
// It's implemented in the corresponding move
// This is a CAP (Create-A-Pokemon) ability
