//! Tablets of Ruin Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	tabletsofruin: {
//! 		onStart(pokemon) {
//! 			if (this.suppressingAbility(pokemon)) return;
//! 			this.add('-ability', pokemon, 'Tablets of Ruin');
//! 		},
//! 		onAnyModifyAtk(atk, source, target, move) {
//! 			const abilityHolder = this.effectState.target;
//! 			if (source.hasAbility('Tablets of Ruin')) return;
//! 			if (!move.ruinedAtk) move.ruinedAtk = abilityHolder;
//! 			if (move.ruinedAtk !== abilityHolder) return;
//! 			this.debug('Tablets of Ruin Atk drop');
//! 			return this.chainModify(0.75);
//! 		},
//! 		flags: {},
//! 		name: "Tablets of Ruin",
//! 		rating: 4.5,
//! 		num: 284,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onStart(pokemon)
/// Announces Tablets of Ruin when sent out
pub fn on_start(battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult {
    // if (this.suppressingAbility(pokemon)) return;
    if battle.suppressing_ability(Some((pokemon.side_index, pokemon.position))) {
        return AbilityHandlerResult::Undefined;
    }

    // this.add('-ability', pokemon, 'Tablets of Ruin');
    battle.add("-ability", &[Arg::Pokemon(pokemon), Arg::Str("Tablets of Ruin")]);
    AbilityHandlerResult::Undefined
}

/// onAnyModifyAtk(atk, source, target, move)
/// Reduces all Pokemon's Attack by 25% except those with Tablets of Ruin
/// Note: onAnyModifyAtk handler needs to be wired into battle engine
pub fn on_any_modify_atk(_battle: &mut Battle, _atk: u32, source: &Pokemon, _target: Option<&Pokemon>, _move: Option<&MoveDef>, _ability_holder: &Pokemon) -> AbilityHandlerResult {
    // const abilityHolder = this.effectState.target;
    // if (source.hasAbility('Tablets of Ruin')) return;
    if source.ability.as_str() == "tabletsofruin" {
        return AbilityHandlerResult::Undefined;
    }

    // if (!move.ruinedAtk) move.ruinedAtk = abilityHolder;
    // if (move.ruinedAtk !== abilityHolder) return;
    // TODO: Track ruinedAtk on move to ensure only one Tablets of Ruin applies

    // this.debug('Tablets of Ruin Atk drop');
    // return this.chainModify(0.75);
    // 0.75 = 3/4
    AbilityHandlerResult::ChainModify(3, 4)
}

