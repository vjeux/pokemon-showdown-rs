//! As One (Spectrier) Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	asonespectrier: {
//! 		onSwitchInPriority: 1,
//! 		onStart(pokemon) {
//! 			if (this.effectState.unnerved) return;
//! 			this.add('-ability', pokemon, 'As One');
//! 			this.add('-ability', pokemon, 'Unnerve');
//! 			this.effectState.unnerved = true;
//! 		},
//! 		onEnd() {
//! 			this.effectState.unnerved = false;
//! 		},
//! 		onFoeTryEatItem() {
//! 			return !this.effectState.unnerved;
//! 		},
//! 		onSourceAfterFaint(length, target, source, effect) {
//! 			if (effect && effect.effectType === 'Move') {
//! 				this.boost({ spa: length }, source, source, this.dex.abilities.get('grimneigh'));
//! 			}
//! 		},
//! 		flags: { failroleplay: 1, noreceiver: 1, noentrain: 1, notrace: 1, failskillswap: 1, cantsuppress: 1 },
//! 		name: "As One (Spectrier)",
//! 		rating: 3.5,
//! 		num: 267,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onSwitchInPriority: 1
pub const ON_SWITCH_IN_PRIORITY: i32 = 1;

/// onStart(pokemon)
pub fn on_start(battle: &mut Battle, pokemon: &mut Pokemon) -> AbilityHandlerResult {
    // if (this.effectState.unnerved) return;
    let unnerved = pokemon.ability_state.data.get("unnerved")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    if unnerved {
        return AbilityHandlerResult::Undefined;
    }

    // this.add('-ability', pokemon, 'As One');
    battle.add("-ability", &[Arg::Pokemon(pokemon), Arg::Str("As One")]);
    // this.add('-ability', pokemon, 'Unnerve');
    battle.add("-ability", &[Arg::Pokemon(pokemon), Arg::Str("Unnerve")]);
    // this.effectState.unnerved = true;
    pokemon.ability_state.data.insert("unnerved".to_string(), serde_json::Value::Bool(true));

    AbilityHandlerResult::Undefined
}

/// onEnd()
pub fn on_end(_battle: &mut Battle, pokemon: &mut Pokemon) -> AbilityHandlerResult {
    // this.effectState.unnerved = false;
    pokemon.ability_state.data.insert("unnerved".to_string(), serde_json::Value::Bool(false));
    AbilityHandlerResult::Undefined
}

/// onFoeTryEatItem()
pub fn on_foe_try_eat_item(pokemon: &Pokemon) -> AbilityHandlerResult {
    // return !this.effectState.unnerved;
    let unnerved = pokemon.ability_state.data.get("unnerved")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    if !unnerved {
        AbilityHandlerResult::True
    } else {
        AbilityHandlerResult::False
    }
}

/// onSourceAfterFaint(length, target, source, effect)
pub fn on_source_after_faint(battle: &mut Battle, length: i32, _target: &Pokemon, source: &Pokemon, effect: &Effect) -> AbilityHandlerResult {
    // if (effect && effect.effectType === 'Move')
    if effect.effect_type == "Move" {
        // this.boost({ spa: length }, source, source, this.dex.abilities.get('grimneigh'));
        let source_ref = (source.side_index, source.position);
        battle.boost(&[("spa", length as i8)], source_ref, Some(source_ref), None);
    }
    AbilityHandlerResult::Undefined
}
