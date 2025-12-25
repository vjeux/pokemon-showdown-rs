//! No Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	noability: {
//! 		isNonstandard: "Past",
//! 		flags: {},
//! 		name: "No Ability",
//! 		rating: 0.1,
//! 		num: 0,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

// No Ability has no callback handlers in the JS source
// This is a placeholder for Pokemon with no ability
