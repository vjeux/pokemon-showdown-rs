//! Ball Fetch Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	ballfetch: {
//! 		flags: {},
//! 		name: "Ball Fetch",
//! 		rating: 0,
//! 		num: 237,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

// Ball Fetch has no callback handlers in the JS source
// It's a passive ability that only works outside of battle
