//! Shields Down Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	shieldsdown: {
//! 		onSwitchInPriority: -1,
//! 		onStart(pokemon) {
//! 			if (pokemon.baseSpecies.baseSpecies !== 'Minior' || pokemon.transformed) return;
//! 			if (pokemon.hp > pokemon.maxhp / 2) {
//! 				if (pokemon.species.forme !== 'Meteor') {
//! 					pokemon.formeChange('Minior-Meteor');
//! 				}
//! 			} else {
//! 				if (pokemon.species.forme === 'Meteor') {
//! 					pokemon.formeChange(pokemon.set.species);
//! 				}
//! 			}
//! 		},
//! 		onResidualOrder: 29,
//! 		onResidual(pokemon) {
//! 			if (pokemon.baseSpecies.baseSpecies !== 'Minior' || pokemon.transformed || !pokemon.hp) return;
//! 			if (pokemon.hp > pokemon.maxhp / 2) {
//! 				if (pokemon.species.forme !== 'Meteor') {
//! 					pokemon.formeChange('Minior-Meteor');
//! 				}
//! 			} else {
//! 				if (pokemon.species.forme === 'Meteor') {
//! 					pokemon.formeChange(pokemon.set.species);
//! 				}
//! 			}
//! 		},
//! 		onSetStatus(status, target, source, effect) {
//! 			if (target.species.id !== 'miniormeteor' || target.transformed) return;
//! 			if ((effect as Move)?.status) {
//! 				this.add('-immune', target, '[from] ability: Shields Down');
//! 			}
//! 			return false;
//! 		},
//! 		onTryAddVolatile(status, target) {
//! 			if (target.species.id !== 'miniormeteor' || target.transformed) return;
//! 			if (status.id !== 'yawn') return;
//! 			this.add('-immune', target, '[from] ability: Shields Down');
//! 			return null;
//! 		},
//! 		flags: { failroleplay: 1, noreceiver: 1, noentrain: 1, notrace: 1, failskillswap: 1, cantsuppress: 1 },
//! 		name: "Shields Down",
//! 		rating: 3,
//! 		num: 197,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// Minior forme change ability - switches between Meteor (shielded) and Core (unshielded) formes
/// TODO: Complex ability with multiple handlers
/// TODO: Needs pokemon.baseSpecies, pokemon.transformed, pokemon.hp, pokemon.maxhp, pokemon.species.forme, pokemon.formeChange(), pokemon.set.species
/// TODO: onStart and onResidual: Change to Meteor forme if HP > 50%, change to Core forme if HP <= 50%
/// TODO: onSetStatus: While in Meteor forme, immune to status conditions
/// TODO: onTryAddVolatile: While in Meteor forme, immune to Yawn
pub const ON_SWITCH_IN_PRIORITY: i32 = -1;
pub const ON_RESIDUAL_ORDER: i32 = 29;

