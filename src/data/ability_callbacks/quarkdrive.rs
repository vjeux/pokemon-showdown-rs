//! Quark Drive Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	quarkdrive: {
//! 		onSwitchInPriority: -2,
//! 		onStart(pokemon) {
//! 			this.singleEvent('TerrainChange', this.effect, this.effectState, pokemon);
//! 		},
//! 		onTerrainChange(pokemon) {
//! 			if (this.field.isTerrain('electricterrain')) {
//! 				pokemon.addVolatile('quarkdrive');
//! 			} else if (!pokemon.volatiles['quarkdrive']?.fromBooster) {
//! 				pokemon.removeVolatile('quarkdrive');
//! 			}
//! 		},
//! 		onEnd(pokemon) {
//! 			delete pokemon.volatiles['quarkdrive'];
//! 			this.add('-end', pokemon, 'Quark Drive', '[silent]');
//! 		},
//! 		condition: {
//! 			noCopy: true,
//! 			onStart(pokemon, source, effect) {
//! 				if (effect?.name === 'Booster Energy') {
//! 					this.effectState.fromBooster = true;
//! 					this.add('-activate', pokemon, 'ability: Quark Drive', '[fromitem]');
//! 				} else {
//! 					this.add('-activate', pokemon, 'ability: Quark Drive');
//! 				}
//! 				this.effectState.bestStat = pokemon.getBestStat(false, true);
//! 				this.add('-start', pokemon, 'quarkdrive' + this.effectState.bestStat);
//! 			},
//! 			onModifyAtkPriority: 5,
//! 			onModifyAtk(atk, pokemon) {
//! 				if (this.effectState.bestStat !== 'atk' || pokemon.ignoringAbility()) return;
//! 				this.debug('Quark Drive atk boost');
//! 				return this.chainModify([5325, 4096]);
//! 			},
//! 			onModifyDefPriority: 6,
//! 			onModifyDef(def, pokemon) {
//! 				if (this.effectState.bestStat !== 'def' || pokemon.ignoringAbility()) return;
//! 				this.debug('Quark Drive def boost');
//! 				return this.chainModify([5325, 4096]);
//! 			},
//! 			onModifySpAPriority: 5,
//! 			onModifySpA(spa, pokemon) {
//! 				if (this.effectState.bestStat !== 'spa' || pokemon.ignoringAbility()) return;
//! 				this.debug('Quark Drive spa boost');
//! 				return this.chainModify([5325, 4096]);
//! 			},
//! 			onModifySpDPriority: 6,
//! 			onModifySpD(spd, pokemon) {
//! 				if (this.effectState.bestStat !== 'spd' || pokemon.ignoringAbility()) return;
//! 				this.debug('Quark Drive spd boost');
//! 				return this.chainModify([5325, 4096]);
//! 			},
//! 			onModifySpe(spe, pokemon) {
//! 				if (this.effectState.bestStat !== 'spe' || pokemon.ignoringAbility()) return;
//! 				this.debug('Quark Drive spe boost');
//! 				return this.chainModify(1.5);
//! 			},
//! 			onEnd(pokemon) {
//! 				this.add('-end', pokemon, 'Quark Drive');
//! 			},
//! 		},
//! 		flags: { failroleplay: 1, noreceiver: 1, noentrain: 1, notrace: 1, failskillswap: 1, notransform: 1 },
//! 		name: "Quark Drive",
//! 		rating: 3,
//! 		num: 282,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_SWITCH_IN_PRIORITY: i32 = -2;

/// Complex ability - boosts highest stat by 1.3x on Electric Terrain or with Booster Energy
/// TODO: Needs terrain system, volatile conditions, effectState tracking, stat boosting
/// TODO: Multiple handlers: onStart, onTerrainChange, onEnd
/// TODO: Volatile condition with handlers: onStart, onModifyAtk/Def/SpA/SpD/Spe (with priorities), onEnd
/// TODO: This ability requires extensive infrastructure for volatile conditions with nested handlers
pub const ABILITY_DESCRIPTION: &str = "Quark Drive: Boosts highest stat on Electric Terrain or with Booster Energy";
