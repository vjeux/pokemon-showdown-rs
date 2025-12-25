//! Corrosion Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	corrosion: {
//! 		// Implemented in sim/pokemon.js:Pokemon#setStatus
//! 		flags: {},
//! 		name: "Corrosion",
//! 		rating: 2.5,
//! 		num: 212,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

// Corrosion has no callback handlers in the JS source
// It's implemented in sim/pokemon.js:Pokemon#setStatus
// The ability allows poisoning Steel and Poison types
