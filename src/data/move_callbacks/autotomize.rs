//! Autotomize Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts
//!
//! ```text
//! JS Source (data/moves.ts):
//! 	autotomize: {
//! 		num: 475,
//! 		accuracy: true,
//! 		basePower: 0,
//! 		category: "Status",
//! 		isNonstandard: "Past",
//! 		name: "Autotomize",
//! 		pp: 15,
//! 		priority: 0,
//! 		flags: { snatch: 1, metronome: 1 },
//! 		onTryHit(pokemon) {
//! 			const hasContrary = pokemon.hasAbility('contrary');
//! 			if ((!hasContrary && pokemon.boosts.spe === 6) || (hasContrary && pokemon.boosts.spe === -6)) {
//! 				return false;
//! 			}
//! 		},
//! 		boosts: {
//! 			spe: 2,
//! 		},
//! 		onHit(pokemon) {
//! 			if (pokemon.weighthg > 1) {
//! 				pokemon.weighthg = Math.max(1, pokemon.weighthg - 1000);
//! 				this.add('-start', pokemon, 'Autotomize');
//! 			}
//! 		},
//! 		secondary: null,
//! 		target: "self",
//! 		type: "Steel",
//! 		zMove: { effect: 'clearnegativeboost' },
//! 		contestType: "Beautiful",
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onTryHit callback for Autotomize
/// Checks if Speed is already maxed (+6 normally, -6 with Contrary)
/// JS: onTryHit(pokemon) { ... }
pub fn on_try_hit(
    battle: &mut Battle,
    target: (usize, usize),
    source: (usize, usize),
    move_id: &ID,
) -> MoveHandlerResult {
    // JavaScript: const hasContrary = pokemon.hasAbility('contrary');
    let (side_idx, poke_idx) = target;

    let (has_contrary, current_spe_boost) = if let Some(side) = battle.sides.get(side_idx) {
        if let Some(pokemon) = side.pokemon.get(poke_idx) {
            let has_contrary = pokemon.ability.as_str() == "contrary";
            let spe_boost = pokemon.boosts.spe;
            (has_contrary, spe_boost)
        } else {
            return MoveHandlerResult::Undefined;
        }
    } else {
        return MoveHandlerResult::Undefined;
    };

    // JavaScript: if ((!hasContrary && pokemon.boosts.spe === 6) || (hasContrary && pokemon.boosts.spe === -6)) { return false; }
    if (!has_contrary && current_spe_boost == 6) || (has_contrary && current_spe_boost == -6) {
        // Speed is already maxed, move fails
        return MoveHandlerResult::False;
    }

    MoveHandlerResult::Undefined
}

/// onHit callback for Autotomize
/// Reduces the Pokemon's weight by 100 kg (minimum 0.1 kg)
/// JS: onHit(pokemon) { ... }
pub fn on_hit(
    battle: &mut Battle,
    target: (usize, usize),
    source: (usize, usize),
    move_id: &ID,
) -> MoveHandlerResult {
    // JavaScript: if (pokemon.weighthg > 1) { pokemon.weighthg = Math.max(1, pokemon.weighthg - 1000); this.add('-start', pokemon, 'Autotomize'); }
    let (side_idx, poke_idx) = target;

    let current_weight = if let Some(side) = battle.sides.get(side_idx) {
        if let Some(pokemon) = side.pokemon.get(poke_idx) {
            pokemon.weight_hg
        } else {
            return MoveHandlerResult::Undefined;
        }
    } else {
        return MoveHandlerResult::Undefined;
    };

    if current_weight > 1 {
        // Reduce weight by 100 kg (1000 hectograms), minimum 0.1 kg (1 hectogram)
        let new_weight = (current_weight - 1000).max(1);

        if let Some(side) = battle.sides.get_mut(side_idx) {
            if let Some(pokemon) = side.pokemon.get_mut(poke_idx) {
                pokemon.weight_hg = new_weight;
            }
        }

        // JavaScript: this.add('-start', pokemon, 'Autotomize');
        let pokemon_id = if let Some(side) = battle.sides.get(side_idx) {
            if let Some(pokemon) = side.pokemon.get(poke_idx) {
                format!("{}: {}", side.id_str(), pokemon.name)
            } else {
                String::from("unknown")
            }
        } else {
            String::from("unknown")
        };

        battle.add_log("-start", &[&pokemon_id, "Autotomize"]);
    }

    MoveHandlerResult::Undefined
}

