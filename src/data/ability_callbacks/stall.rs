//! Stall Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	stall: {
//! 		onFractionalPriority: -0.1,
//! 		flags: {},
//! 		name: "Stall",
//! 		rating: -1,
//! 		num: 100,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_FRACTIONAL_PRIORITY: f32 = -0.1;

