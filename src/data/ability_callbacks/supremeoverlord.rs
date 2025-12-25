//! Supreme Overlord Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	supremeoverlord: {
//! 		onStart(pokemon) {
//! 			if (pokemon.side.totalFainted) {
//! 				this.add('-activate', pokemon, 'ability: Supreme Overlord');
//! 				const fallen = Math.min(pokemon.side.totalFainted, 5);
//! 				this.add('-start', pokemon, `fallen${fallen}`, '[silent]');
//! 				this.effectState.fallen = fallen;
//! 			}
//! 		},
//! 		onEnd(pokemon) {
//! 			this.add('-end', pokemon, `fallen${this.effectState.fallen}`, '[silent]');
//! 		},
//! 		onBasePowerPriority: 21,
//! 		onBasePower(basePower, attacker, defender, move) {
//! 			if (this.effectState.fallen) {
//! 				const powMod = [4096, 4506, 4915, 5325, 5734, 6144];
//! 				this.debug(`Supreme Overlord boost: ${powMod[this.effectState.fallen]}/4096`);
//! 				return this.chainModify([powMod[this.effectState.fallen], 4096]);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Supreme Overlord",
//! 		rating: 4,
//! 		num: 293,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onStart(pokemon)
/// Activates when Pokemon has fainted allies, boosts damage
pub fn on_start(battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult {
    // if (pokemon.side.totalFainted)
    let total_fainted = battle.sides[pokemon.side_index].total_fainted;
    if total_fainted > 0 {
        // this.add('-activate', pokemon, 'ability: Supreme Overlord');
        battle.add("-activate", &[
            Arg::Pokemon(pokemon),
            Arg::Str("ability: Supreme Overlord")
        ]);

        // const fallen = Math.min(pokemon.side.totalFainted, 5);
        let fallen = total_fainted.min(5);

        // this.add('-start', pokemon, `fallen${fallen}`, '[silent]');
        battle.add("-start", &[
            Arg::Pokemon(pokemon),
            Arg::String(format!("fallen{}", fallen)),
            Arg::Str("[silent]")
        ]);

        // this.effectState.fallen = fallen;
        let holder_ref = (pokemon.side_index, pokemon.position);
        battle.sides[holder_ref.0].pokemon[holder_ref.1].ability_state.data.insert(
            "fallen".to_string(),
            serde_json::json!(fallen)
        );
    }
    AbilityHandlerResult::Undefined
}

/// onEnd(pokemon)
/// Removes the fallen marker when Pokemon switches out
pub fn on_end(battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult {
    // this.add('-end', pokemon, `fallen${this.effectState.fallen}`, '[silent]');
    let holder_ref = (pokemon.side_index, pokemon.position);
    if let Some(fallen_value) = battle.sides[holder_ref.0].pokemon[holder_ref.1].ability_state.data.get("fallen") {
        if let Some(fallen) = fallen_value.as_u64() {
            battle.add("-end", &[
                Arg::Pokemon(pokemon),
                Arg::String(format!("fallen{}", fallen)),
                Arg::Str("[silent]")
            ]);
        }
    }
    AbilityHandlerResult::Undefined
}

/// onBasePowerPriority: 21
pub const ON_BASE_POWER_PRIORITY: i32 = 21;

/// onBasePower(basePower, attacker, defender, move)
/// Boosts base power based on number of fainted allies (up to 5)
/// Multipliers: 0 = 1.0x, 1 = 1.1x, 2 = 1.2x, 3 = 1.3x, 4 = 1.4x, 5 = 1.5x
pub fn on_base_power(_battle: &mut Battle, _base_power: u32, attacker: &Pokemon, _defender: &Pokemon, _move_: &MoveDef) -> AbilityHandlerResult {
    // if (this.effectState.fallen)
    if let Some(fallen_value) = attacker.ability_state.data.get("fallen") {
        if let Some(fallen) = fallen_value.as_u64() {
            // const powMod = [4096, 4506, 4915, 5325, 5734, 6144];
            let pow_mod = [4096, 4506, 4915, 5325, 5734, 6144];

            // this.debug(`Supreme Overlord boost: ${powMod[this.effectState.fallen]}/4096`);
            // return this.chainModify([powMod[this.effectState.fallen], 4096]);
            if (fallen as usize) < pow_mod.len() {
                return AbilityHandlerResult::ChainModify(pow_mod[fallen as usize], 4096);
            }
        }
    }
    AbilityHandlerResult::Undefined
}

