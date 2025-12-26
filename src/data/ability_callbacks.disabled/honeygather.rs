//! Honey Gather Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	honeygather: {
//! 		flags: {},
//! 		name: "Honey Gather",
//! 		rating: 0,
//! 		num: 118,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

// Honey Gather has no callback handlers in the JS source
// It's a passive ability that only works outside of battle
