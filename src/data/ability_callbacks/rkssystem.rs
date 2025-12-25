//! RKS System Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	rkssystem: {
//! 		// RKS System's type-changing itself is implemented in statuses.js
//! 		flags: { failroleplay: 1, noreceiver: 1, noentrain: 1, notrace: 1, failskillswap: 1, cantsuppress: 1 },
//! 		name: "RKS System",
//! 		rating: 4,
//! 		num: 225,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

// RKS System has no callback handlers in the JS source
// Type-changing is implemented in statuses.js
// The ability changes Silvally's type based on held Memory
