//! Fairy Aura Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	fairyaura: {
//! 		onStart(pokemon) {
//! 			if (this.suppressingAbility(pokemon)) return;
//! 			this.add('-ability', pokemon, 'Fairy Aura');
//! 		},
//! 		onAnyBasePowerPriority: 20,
//! 		onAnyBasePower(basePower, source, target, move) {
//! 			if (target === source || move.category === 'Status' || move.type !== 'Fairy') return;
//! 			if (!move.auraBooster?.hasAbility('Fairy Aura')) move.auraBooster = this.effectState.target;
//! 			if (move.auraBooster !== this.effectState.target) return;
//! 			return this.chainModify([move.hasAuraBreak ? 3072 : 5448, 4096]);
//! 		},
//! 		flags: {},
//! 		name: "Fairy Aura",
//! 		rating: 3,
//! 		num: 187,
//! 	},
//! ```


use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onStart(pokemon)
    pub fn on_start(battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult {
        let pokemon_ref = Some((pokemon.side_index, pokemon.position));
        if battle.suppressing_ability(pokemon_ref) {
            return AbilityHandlerResult::Undefined;
        }
        battle.add("-ability", &[Arg::Pokemon(pokemon), Arg::Str("Fairy Aura")]);
        AbilityHandlerResult::Undefined
    }

    pub const ON_ANY_BASE_POWER_PRIORITY: i32 = 20;

    /// onAnyBasePower(basePower, source, target, move)
    pub fn on_any_base_power(_base_power: u32, source: &Pokemon, target: &Pokemon, move_: &MoveDef, aura_booster: Option<&Pokemon>, has_aura_break: bool) -> AbilityHandlerResult {
        let source_ref = (source.side_index, source.position);
        let target_ref = (target.side_index, target.position);
        if target_ref == source_ref || move_.category == MoveCategory::Status || move_.move_type != "Fairy" {
            return AbilityHandlerResult::Undefined;
        }
        // Simplified: apply boost if this is the aura booster
        if aura_booster.is_some() {
            if has_aura_break {
                return AbilityHandlerResult::ChainModify(3072, 4096); // 0.75x
            } else {
                return AbilityHandlerResult::ChainModify(5448, 4096); // ~1.33x
            }
        }
        AbilityHandlerResult::Undefined
    }
