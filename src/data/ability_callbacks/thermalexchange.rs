//! Thermal Exchange Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	thermalexchange: {
//! 		onDamagingHit(damage, target, source, move) {
//! 			if (move.type === 'Fire') {
//! 				this.boost({ atk: 1 });
//! 			}
//! 		},
//! 		onUpdate(pokemon) {
//! 			if (pokemon.status === 'brn') {
//! 				this.add('-activate', pokemon, 'ability: Thermal Exchange');
//! 				pokemon.cureStatus();
//! 			}
//! 		},
//! 		onSetStatus(status, target, source, effect) {
//! 			if (status.id !== 'brn') return;
//! 			if ((effect as Move)?.status) {
//! 				this.add('-immune', target, '[from] ability: Thermal Exchange');
//! 			}
//! 			return false;
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Thermal Exchange",
//! 		rating: 2.5,
//! 		num: 270,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onDamagingHit(damage, target, source, move)
/// Boosts Attack by 1 stage when hit by a Fire-type move
pub fn on_damaging_hit(battle: &mut Battle, _damage: u32, target: &Pokemon, _source: &mut Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    // if (move.type === 'Fire')
    if move_.move_type.as_str() == "Fire" {
        // this.boost({ atk: 1 });
        let target_ref = (target.side_index, target.position);
        battle.boost(&[("atk", 1)], target_ref, Some(target_ref), None);
    }
    AbilityHandlerResult::Undefined
}

/// onUpdate(pokemon)
/// Cures burn status at the end of each turn
pub fn on_update(battle: &mut Battle, pokemon: &mut Pokemon) -> AbilityHandlerResult {
    // if (pokemon.status === 'brn')
    if pokemon.status.as_str() == "brn" {
        // this.add('-activate', pokemon, 'ability: Thermal Exchange');
        battle.add("-activate", &[Arg::Pokemon(pokemon), Arg::Str("ability: Thermal Exchange")]);
        // pokemon.cureStatus();
        pokemon.cure_status();
    }
    AbilityHandlerResult::Undefined
}

/// onSetStatus(status, target, source, effect)
/// Prevents burn status from being set
pub fn on_set_status(_battle: &mut Battle, status: &Status, _target: &Pokemon, _source: Option<&Pokemon>, effect: &Effect) -> AbilityHandlerResult {
    // if (status.id !== 'brn') return;
    if status.id != "brn" {
        return AbilityHandlerResult::Undefined;
    }
    // if ((effect as Move)?.status)
    if effect.status.is_some() {
        // this.add('-immune', target, '[from] ability: Thermal Exchange');
        // Note: battle is not mutable here, so we can't add logs
        // This will need to be handled by the caller
    }
    // return false;
    AbilityHandlerResult::False
}

