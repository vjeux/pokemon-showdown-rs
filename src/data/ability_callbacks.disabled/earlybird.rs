//! Early Bird Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	earlybird: {
//! 		flags: {},
//! 		name: "Early Bird",
//! 		// Implemented in statuses.js
//! 		rating: 1.5,
//! 		num: 48,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

// Early Bird has no callback handlers in the JS source
// It's implemented in statuses.js
// The ability makes sleep last half as long
