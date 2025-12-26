//! Suction Cups Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	suctioncups: {
//! 		onDragOutPriority: 1,
//! 		onDragOut(pokemon) {
//! 			this.add('-activate', pokemon, 'ability: Suction Cups');
//! 			return null;
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Suction Cups",
//! 		rating: 1,
//! 		num: 21,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_DRAG_OUT_PRIORITY: i32 = 1;

/// onDragOut(pokemon)
/// Prevents forced switching (Dragon Tail, Roar, Whirlwind, etc.)
pub fn on_drag_out(battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult {
    // this.add('-activate', pokemon, 'ability: Suction Cups');
    battle.add("-activate", &[Arg::Pokemon(pokemon), Arg::Str("ability: Suction Cups")]);
    // return null;
    AbilityHandlerResult::Null
}

