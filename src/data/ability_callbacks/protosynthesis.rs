//! Protosynthesis Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	protosynthesis: {
//! 		onSwitchInPriority: -2,
//! 		onStart(pokemon) {
//! 			this.singleEvent('WeatherChange', this.effect, this.effectState, pokemon);
//! 		},
//! 		onWeatherChange(pokemon) {
//! 			// Protosynthesis is not affected by Utility Umbrella
//! 			if (this.field.isWeather('sunnyday')) {
//! 				pokemon.addVolatile('protosynthesis');
//! 			} else if (!pokemon.volatiles['protosynthesis']?.fromBooster && !this.field.isWeather('sunnyday')) {
//! 				pokemon.removeVolatile('protosynthesis');
//! 			}
//! 		},
//! 		onEnd(pokemon) {
//! 			delete pokemon.volatiles['protosynthesis'];
//! 			this.add('-end', pokemon, 'Protosynthesis', '[silent]');
//! 		},
//! 		condition: {
//! 			noCopy: true,
//! 			onStart(pokemon, source, effect) {
//! 				if (effect?.name === 'Booster Energy') {
//! 					this.effectState.fromBooster = true;
//! 					this.add('-activate', pokemon, 'ability: Protosynthesis', '[fromitem]');
//! 				} else {
//! 					this.add('-activate', pokemon, 'ability: Protosynthesis');
//! 				}
//! 				this.effectState.bestStat = pokemon.getBestStat(false, true);
//! 				this.add('-start', pokemon, 'protosynthesis' + this.effectState.bestStat);
//! 			},
//! 			onModifyAtkPriority: 5,
//! 			onModifyAtk(atk, pokemon) {
//! 				if (this.effectState.bestStat !== 'atk' || pokemon.ignoringAbility()) return;
//! 				this.debug('Protosynthesis atk boost');
//! 				return this.chainModify([5325, 4096]);
//! 			},
//! 			onModifyDefPriority: 6,
//! 			onModifyDef(def, pokemon) {
//! 				if (this.effectState.bestStat !== 'def' || pokemon.ignoringAbility()) return;
//! 				this.debug('Protosynthesis def boost');
//! 				return this.chainModify([5325, 4096]);
//! 			},
//! 			onModifySpAPriority: 5,
//! 			onModifySpA(spa, pokemon) {
//! 				if (this.effectState.bestStat !== 'spa' || pokemon.ignoringAbility()) return;
//! 				this.debug('Protosynthesis spa boost');
//! 				return this.chainModify([5325, 4096]);
//! 			},
//! 			onModifySpDPriority: 6,
//! 			onModifySpD(spd, pokemon) {
//! 				if (this.effectState.bestStat !== 'spd' || pokemon.ignoringAbility()) return;
//! 				this.debug('Protosynthesis spd boost');
//! 				return this.chainModify([5325, 4096]);
//! 			},
//! 			onModifySpe(spe, pokemon) {
//! 				if (this.effectState.bestStat !== 'spe' || pokemon.ignoringAbility()) return;
//! 				this.debug('Protosynthesis spe boost');
//! 				return this.chainModify(1.5);
//! 			},
//! 			onEnd(pokemon) {
//! 				this.add('-end', pokemon, 'Protosynthesis');
//! 			},
//! 		},
//! 		flags: { failroleplay: 1, noreceiver: 1, noentrain: 1, notrace: 1, failskillswap: 1, notransform: 1 },
//! 		name: "Protosynthesis",
//! 		rating: 3,
//! 		num: 281,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_SWITCH_IN_PRIORITY: i32 = -2;

/// Complex ability - boosts highest stat by 1.3x in sun or with Booster Energy
/// TODO: Needs weather system, volatile conditions, effectState tracking, stat boosting
/// TODO: Multiple handlers: onStart, onWeatherChange, onEnd
/// TODO: Volatile condition with handlers: onStart, onModifyAtk/Def/SpA/SpD/Spe (with priorities), onEnd
/// TODO: This ability requires extensive infrastructure for volatile conditions with nested handlers
pub const ABILITY_DESCRIPTION: &str = "Protosynthesis: Boosts highest stat in sun or with Booster Energy";
