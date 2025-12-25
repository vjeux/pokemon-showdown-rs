//! Levitate Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	levitate: {
//! 		// airborneness implemented in sim/pokemon.js:Pokemon#isGrounded
//! 		flags: { breakable: 1 },
//! 		name: "Levitate",
//! 		rating: 3.5,
//! 		num: 26,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

// Levitate has no callback handlers in the JS source
// Airborne status is implemented in sim/pokemon.js:Pokemon#isGrounded
// The ability grants immunity to Ground-type moves
