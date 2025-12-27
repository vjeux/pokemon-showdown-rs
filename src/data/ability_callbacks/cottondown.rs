//! Cotton Down Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	cottondown: {
//! 		onDamagingHit(damage, target, source, move) {
//! 			let activated = false;
//! 			for (const pokemon of this.getAllActive()) {
//! 				if (pokemon === target || pokemon.fainted) continue;
//! 				if (!activated) {
//! 					this.add('-ability', target, 'Cotton Down');
//! 					activated = true;
//! 				}
//! 				this.boost({ spe: -1 }, pokemon, target, null, true);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Cotton Down",
//! 		rating: 2,
//! 		num: 238,
//! 	},
//! ```


use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onDamagingHit(damage, target, source, move)
    /// Lowers Speed of all other active Pokemon by 1 stage when hit
    pub fn on_damaging_hit(battle: &mut Battle, _damage: i32, target: &Pokemon, _source: &Pokemon, _move_: &MoveDef) -> AbilityHandlerResult {
        // let activated = false;
        let mut activated = false;
        // for (const pokemon of this.getAllActive())
        // Collect Pokemon indices first to avoid borrow checker issues
        let pokemon_to_boost: Vec<(usize, usize)> = battle.get_all_active(false)
            .iter()
            .filter(|pokemon| !pokemon.is_same(target))
            .map(|pokemon| (pokemon.side_index, pokemon.position))
            .collect();

        for (side_idx, poke_idx) in pokemon_to_boost {
            // if (!activated)
            if !activated {
                // this.add('-ability', target, 'Cotton Down');
                battle.add("-ability", &[Arg::Pokemon(target), Arg::Str("Cotton Down")]);
                activated = true;
            }
            // this.boost({ spe: -1 }, pokemon, target, null, true);
            battle.boost(&[("spe", -1)], (side_idx, poke_idx), Some((target.side_index, target.position)), None);
        }
        AbilityHandlerResult::Undefined
    }
