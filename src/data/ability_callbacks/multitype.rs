//! Multitype Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	multitype: {
//! 		// Multitype's type-changing itself is implemented in statuses.js
//! 		flags: { failroleplay: 1, noreceiver: 1, noentrain: 1, notrace: 1, failskillswap: 1, cantsuppress: 1 },
//! 		name: "Multitype",
//! 		rating: 4,
//! 		num: 121,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

// Multitype has no callback handlers in the JS source
// Type-changing is implemented in statuses.js
// The ability changes Arceus's type based on held Plate
