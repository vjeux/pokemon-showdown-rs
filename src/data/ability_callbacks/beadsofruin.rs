//! Beads of Ruin Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	beadsofruin: {
//! 		onStart(pokemon) {
//! 			if (this.suppressingAbility(pokemon)) return;
//! 			this.add('-ability', pokemon, 'Beads of Ruin');
//! 		},
//! 		onAnyModifySpD(spd, target, source, move) {
//! 			const abilityHolder = this.effectState.target;
//! 			if (target.hasAbility('Beads of Ruin')) return;
//! 			if (!move.ruinedSpD?.hasAbility('Beads of Ruin')) move.ruinedSpD = abilityHolder;
//! 			if (move.ruinedSpD !== abilityHolder) return;
//! 			this.debug('Beads of Ruin SpD drop');
//! 			return this.chainModify(0.75);
//! 		},
//! 		flags: {},
//! 		name: "Beads of Ruin",
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
pub fn on_start(battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult {
    // if (this.suppressingAbility(pokemon)) return;
    if battle.suppressing_ability(Some((pokemon.side_index, pokemon.position))) {
        return AbilityHandlerResult::Undefined;
    }

    // this.add('-ability', pokemon, 'Beads of Ruin');
    battle.add("-ability", &[Arg::Pokemon(pokemon), Arg::Str("Beads of Ruin")]);
    AbilityHandlerResult::Undefined
}

/// onAnyModifySpD(spd, target, source, move)
/// Note: This reduces Special Defense of all Pokemon without Beads of Ruin by 25%.
/// The implementation requires tracking ruinedSpD on the move, which is complex.
/// For now, we return a modifier value.
pub fn on_any_modify_sp_d(battle: &mut Battle, spd: u32, target: &Pokemon, _source: Option<&Pokemon>, _move: Option<&MoveDef>, ability_holder: &Pokemon) -> AbilityHandlerResult {
    // const abilityHolder = this.effectState.target;
    // if (target.hasAbility('Beads of Ruin')) return;
    if target.ability.as_str() == "beadsofruin" {
        return AbilityHandlerResult::Undefined;
    }

    // if (!move.ruinedSpD?.hasAbility('Beads of Ruin')) move.ruinedSpD = abilityHolder;
    // if (move.ruinedSpD !== abilityHolder) return;
    // TODO: Track ruinedSpD on move to ensure only one Beads of Ruin applies

    // this.debug('Beads of Ruin SpD drop');
    // return this.chainModify(0.75);
    // 0.75 = 3/4
    AbilityHandlerResult::ChainModify(3, 4)
}

