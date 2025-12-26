//! Dancer Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	dancer: {
//! 		flags: {},
//! 		name: "Dancer",
//! 		// implemented in runMove in scripts.js
//! 		rating: 1.5,
//! 		num: 216,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

// Dancer has no callback handlers in the JS source
// It's implemented in runMove in scripts.js
// The ability copies dance moves used by other Pokemon
