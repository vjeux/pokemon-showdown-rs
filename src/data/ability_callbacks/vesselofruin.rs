//! Vessel of Ruin Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	vesselofruin: {
//! 		onStart(pokemon) {
//! 			if (this.suppressingAbility(pokemon)) return;
//! 			this.add('-ability', pokemon, 'Vessel of Ruin');
//! 		},
//! 		onAnyModifySpA(spa, source, target, move) {
//! 			const abilityHolder = this.effectState.target;
//! 			if (source.hasAbility('Vessel of Ruin')) return;
//! 			if (!move.ruinedSpA) move.ruinedSpA = abilityHolder;
//! 			if (move.ruinedSpA !== abilityHolder) return;
//! 			this.debug('Vessel of Ruin SpA drop');
//! 			return this.chainModify(0.75);
//! 		},
//! 		flags: {},
//! 		name: "Vessel of Ruin",
//! 		rating: 4.5,
//! 		num: 284,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onStart(pokemon)
/// Announces Vessel of Ruin when sent out
pub fn on_start(battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult {
    // if (this.suppressingAbility(pokemon)) return;
    if battle.suppressing_ability(Some((pokemon.side_index, pokemon.position))) {
        return AbilityHandlerResult::Undefined;
    }

    // this.add('-ability', pokemon, 'Vessel of Ruin');
    battle.add("-ability", &[Arg::Pokemon(pokemon), Arg::Str("Vessel of Ruin")]);
    AbilityHandlerResult::Undefined
}

/// onAnyModifySpA(spa, source, target, move)
/// Reduces all Pokemon's Special Attack by 25% except those with Vessel of Ruin
/// Note: onAnyModifySpA handler needs to be wired into battle engine
pub fn on_any_modify_sp_a(_battle: &mut Battle, _spa: u32, source: &Pokemon, _target: Option<&Pokemon>, _move: Option<&MoveDef>, _ability_holder: &Pokemon) -> AbilityHandlerResult {
    // const abilityHolder = this.effectState.target;
    // if (source.hasAbility('Vessel of Ruin')) return;
    if source.ability.as_str() == "vesselofruin" {
        return AbilityHandlerResult::Undefined;
    }

    // if (!move.ruinedSpA) move.ruinedSpA = abilityHolder;
    // if (move.ruinedSpA !== abilityHolder) return;
    // TODO: Track ruinedSpA on move to ensure only one Vessel of Ruin applies

    // this.debug('Vessel of Ruin SpA drop');
    // return this.chainModify(0.75);
    // 0.75 = 3/4
    AbilityHandlerResult::ChainModify(3, 4)
}

