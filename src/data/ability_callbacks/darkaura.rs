//! Dark Aura Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	darkaura: {
//! 		onStart(pokemon) {
//! 			if (this.suppressingAbility(pokemon)) return;
//! 			this.add('-ability', pokemon, 'Dark Aura');
//! 		},
//! 		onAnyBasePowerPriority: 20,
//! 		onAnyBasePower(basePower, source, target, move) {
//! 			if (target === source || move.category === 'Status' || move.type !== 'Dark') return;
//! 			if (!move.auraBooster?.hasAbility('Dark Aura')) move.auraBooster = this.effectState.target;
//! 			if (move.auraBooster !== this.effectState.target) return;
//! 			return this.chainModify([move.hasAuraBreak ? 3072 : 5448, 4096]);
//! 		},
//! 		flags: {},
//! 		name: "Dark Aura",
//! 		rating: 3,
//! 		num: 186,
//! 	},
//! ```


use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onAnyBasePowerPriority: 20
    pub const ON_ANY_BASE_POWER_PRIORITY: i32 = 20;

    /// onStart(pokemon)
    pub fn on_start(battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult {
        // if (this.suppressingAbility(pokemon)) return;
        // this.add('-ability', pokemon, 'Dark Aura');
        battle.add("-ability", &[Arg::Pokemon(pokemon), Arg::Str("Dark Aura")]);
        AbilityHandlerResult::Undefined
    }

    /// onAnyBasePower(basePower, source, target, move)
    /// Boosts Dark-type moves for all Pokemon
    pub fn on_any_base_power(_base_power: u32, source: &Pokemon, target: &Pokemon, move_: &MoveDef, has_aura_break: bool) -> AbilityHandlerResult {
        // if (target === source || move.category === 'Status' || move.type !== 'Dark') return;
        let source_ref = (source.side_index, source.position);
        let target_ref = (target.side_index, target.position);
        if source_ref == target_ref || move_.category == MoveCategory::Status || move_.move_type != "Dark" {
            return AbilityHandlerResult::Undefined;
        }
        // return this.chainModify([move.hasAuraBreak ? 3072 : 5448, 4096]);
        // 5448/4096 = ~1.33x (boost), 3072/4096 = 0.75x (with Aura Break)
        if has_aura_break {
            AbilityHandlerResult::ChainModify(3072, 4096)
        } else {
            AbilityHandlerResult::ChainModify(5448, 4096)
        }
    }
