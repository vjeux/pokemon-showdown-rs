//! Tera Shift Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	terashift: {
//! 		onSwitchInPriority: 2,
//! 		onSwitchIn(pokemon) {
//! 			if (pokemon.baseSpecies.baseSpecies !== 'Terapagos') return;
//! 			if (pokemon.species.forme !== 'Terastal') {
//! 				this.add('-activate', pokemon, 'ability: Tera Shift');
//! 				pokemon.formeChange('Terapagos-Terastal', this.effect, true);
//! 			}
//! 		},
//! 		flags: { failroleplay: 1, noreceiver: 1, noentrain: 1, notrace: 1, failskillswap: 1, cantsuppress: 1, notransform: 1 },
//! 		name: "Tera Shift",
//! 		rating: 3,
//! 		num: 307,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onSwitchInPriority: 2
pub const ON_SWITCH_IN_PRIORITY: i32 = 2;

/// onSwitchIn(pokemon)
/// Transforms Terapagos into Terastal forme on switch in
///
/// TODO: onSwitchIn handler not yet implemented in battle system
/// TODO: Forme change system (pokemon.formeChange) not yet implemented
/// When implemented, should:
/// 1. Check if pokemon.baseSpecies.baseSpecies !== 'Terapagos', return if not
/// 2. Check if pokemon.species.forme !== 'Terastal'
/// 3. Add activate message: this.add('-activate', pokemon, 'ability: Tera Shift')
/// 4. Call pokemon.formeChange('Terapagos-Terastal', this.effect, true)
pub fn on_switch_in(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

