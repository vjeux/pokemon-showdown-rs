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
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onDamagingHit(damage, target, source, move)
    /// Lowers Speed of all other active Pokemon by 1 stage when hit
    pub fn on_damaging_hit(battle: &mut Battle, _damage: u32, target: &Pokemon, _source: &Pokemon, _move_: &MoveDef) -> AbilityHandlerResult {
        // let activated = false;
        let mut activated = false;
        // for (const pokemon of this.getAllActive())
        let target_ref = (target.side_index, target.position);
        let num_sides = battle.sides.len();
        for side_idx in 0..num_sides {
            let active_len = battle.sides[side_idx].active.len();
            for pos in 0..active_len {
                // if (pokemon === target || pokemon.fainted) continue;
                if (side_idx, pos) == target_ref {
                    continue;
                }
                // active contains Option<usize> - index into pokemon array
                let pokemon_idx = match battle.sides[side_idx].active.get(pos) {
                    Some(Some(idx)) => *idx,
                    _ => continue,
                };
                let is_fainted = battle.sides[side_idx].pokemon.get(pokemon_idx)
                    .map(|p| p.fainted)
                    .unwrap_or(true);
                if is_fainted {
                    continue;
                }
                // if (!activated)
                if !activated {
                    // this.add('-ability', target, 'Cotton Down');
                    battle.add("-ability", &[Arg::Pokemon(target), Arg::Str("Cotton Down")]);
                    activated = true;
                }
                // this.boost({ spe: -1 }, pokemon, target, null, true);
                battle.boost(&[("spe", -1)], (side_idx, pos), Some(target_ref), None);
            }
        }
        AbilityHandlerResult::Undefined
    }
