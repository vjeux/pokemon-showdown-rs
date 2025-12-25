//! Unnerve Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	unnerve: {
//! 		onSwitchInPriority: 1,
//! 		onStart(pokemon) {
//! 			if (this.effectState.unnerved) return;
//! 			this.add('-ability', pokemon, 'Unnerve');
//! 			this.effectState.unnerved = true;
//! 		},
//! 		onEnd() {
//! 			this.effectState.unnerved = false;
//! 		},
//! 		onFoeTryEatItem() {
//! 			return !this.effectState.unnerved;
//! 		},
//! 		flags: {},
//! 		name: "Unnerve",
//! 		rating: 1,
//! 		num: 127,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onSwitchInPriority: 1
pub const ON_SWITCH_IN_PRIORITY: i32 = 1;

/// onStart(pokemon)
/// Announces Unnerve ability (once per switch-in)
pub fn on_start(battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult {
    // if (this.effectState.unnerved) return;
    let holder_ref = (pokemon.side_index, pokemon.position);
    if let Some(unnerved_value) = battle.sides[holder_ref.0].pokemon[holder_ref.1].ability_state.data.get("unnerved") {
        if unnerved_value.as_bool() == Some(true) {
            return AbilityHandlerResult::Undefined;
        }
    }

    // this.add('-ability', pokemon, 'Unnerve');
    battle.add("-ability", &[
        Arg::Pokemon(pokemon),
        Arg::Str("Unnerve")
    ]);

    // this.effectState.unnerved = true;
    battle.sides[holder_ref.0].pokemon[holder_ref.1].ability_state.data.insert(
        "unnerved".to_string(),
        serde_json::json!(true)
    );

    AbilityHandlerResult::Undefined
}

/// onEnd()
/// Clears the unnerved flag when Pokemon switches out
pub fn on_end(battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult {
    // this.effectState.unnerved = false;
    let holder_ref = (pokemon.side_index, pokemon.position);
    battle.sides[holder_ref.0].pokemon[holder_ref.1].ability_state.data.insert(
        "unnerved".to_string(),
        serde_json::json!(false)
    );

    AbilityHandlerResult::Undefined
}

/// onFoeTryEatItem()
/// TODO: onFoeTryEatItem handler not yet implemented in battle engine
/// When implemented, should:
/// - return !this.effectState.unnerved to block berry usage
pub fn on_foe_try_eat_item(_battle: &mut Battle, /* TODO: Add parameters when handler exists */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS when handler infrastructure exists
    // return !this.effectState.unnerved
    AbilityHandlerResult::Undefined
}

