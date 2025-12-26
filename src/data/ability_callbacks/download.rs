//! Download Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	download: {
//! 		onStart(pokemon) {
//! 			let totaldef = 0;
//! 			let totalspd = 0;
//! 			for (const target of pokemon.foes()) {
//! 				totaldef += target.getStat('def', false, true);
//! 				totalspd += target.getStat('spd', false, true);
//! 			}
//! 			if (totaldef && totaldef >= totalspd) {
//! 				this.boost({ spa: 1 });
//! 			} else if (totalspd) {
//! 				this.boost({ atk: 1 });
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Download",
//! 		rating: 3.5,
//! 		num: 88,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::{ID, StatID};
use super::{AbilityHandlerResult, Status, Effect};

/// onStart(pokemon)
pub fn on_start(battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult {
    // let totaldef = 0;
    // let totalspd = 0;
    let mut totaldef = 0;
    let mut totalspd = 0;

    // for (const target of pokemon.foes())
    let foe_side_index = 1 - pokemon.side_index;
    if let Some(foe_side) = battle.sides.get(foe_side_index) {
        for foe in foe_side.pokemon.iter().filter(|p| p.is_active && !p.fainted) {
            // totaldef += target.getStat('def', false, true);
            // totalspd += target.getStat('spd', false, true);
            totaldef += foe.get_stat(StatID::Def, false);
            totalspd += foe.get_stat(StatID::SpD, false);
        }
    }

    // if (totaldef && totaldef >= totalspd) {
    //     this.boost({ spa: 1 });
    // } else if (totalspd) {
    //     this.boost({ atk: 1 });
    // }
    if totaldef > 0 && totaldef >= totalspd {
        battle.boost(&[("spa", 1)], (pokemon.side_index, pokemon.position), Some((pokemon.side_index, pokemon.position)), None);
    } else if totalspd > 0 {
        battle.boost(&[("atk", 1)], (pokemon.side_index, pokemon.position), Some((pokemon.side_index, pokemon.position)), None);
    }

    AbilityHandlerResult::Undefined
}

