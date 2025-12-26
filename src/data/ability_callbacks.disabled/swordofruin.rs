//! Sword of Ruin Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	swordofruin: {
//! 		onStart(pokemon) {
//! 			if (this.suppressingAbility(pokemon)) return;
//! 			this.add('-ability', pokemon, 'Sword of Ruin');
//! 		},
//! 		onAnyModifyDef(def, target, source, move) {
//! 			const abilityHolder = this.effectState.target;
//! 			if (target.hasAbility('Sword of Ruin')) return;
//! 			if (!move.ruinedDef?.hasAbility('Sword of Ruin')) move.ruinedDef = abilityHolder;
//! 			if (move.ruinedDef !== abilityHolder) return;
//! 			this.debug('Sword of Ruin Def drop');
//! 			return this.chainModify(0.75);
//! 		},
//! 		flags: {},
//! 		name: "Sword of Ruin",
//! 		rating: 4.5,
//! 		num: 285,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onStart(pokemon)
/// Announces Sword of Ruin when sent out
pub fn on_start(battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult {
    // if (this.suppressingAbility(pokemon)) return;
    if battle.suppressing_ability(Some((pokemon.side_index, pokemon.position))) {
        return AbilityHandlerResult::Undefined;
    }

    // this.add('-ability', pokemon, 'Sword of Ruin');
    battle.add("-ability", &[Arg::Pokemon(pokemon), Arg::Str("Sword of Ruin")]);
    AbilityHandlerResult::Undefined
}

/// onAnyModifyDef(def, target, source, move)
/// Reduces all Pokemon's Defense by 25% except those with Sword of Ruin
/// Note: onAnyModifyDef handler needs to be wired into battle engine
pub fn on_any_modify_def(_battle: &mut Battle, _def: u32, target: &Pokemon, _source: Option<&Pokemon>, _move: Option<&MoveDef>, _ability_holder: &Pokemon) -> AbilityHandlerResult {
    // const abilityHolder = this.effectState.target;
    // if (target.hasAbility('Sword of Ruin')) return;
    if target.ability.as_str() == "swordofruin" {
        return AbilityHandlerResult::Undefined;
    }

    // if (!move.ruinedDef?.hasAbility('Sword of Ruin')) move.ruinedDef = abilityHolder;
    // if (move.ruinedDef !== abilityHolder) return;
    // TODO: Track ruinedDef on move to ensure only one Sword of Ruin applies

    // this.debug('Sword of Ruin Def drop');
    // return this.chainModify(0.75);
    // 0.75 = 3/4
    AbilityHandlerResult::ChainModify(3, 4)
}

