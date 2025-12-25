//! Bulletproof Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	bulletproof: {
//! 		onTryHit(pokemon, target, move) {
//! 			if (move.flags['bullet']) {
//! 				this.add('-immune', pokemon, '[from] ability: Bulletproof');
//! 				return null;
//! 			}
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Bulletproof",
//! 		rating: 3,
//! 		num: 171,
//! 	},
//! ```


use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onTryHit(pokemon, target, move)
    /// Immune to ball/bomb moves
    pub fn on_try_hit(battle: &mut Battle, pokemon: &Pokemon, _target: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
        if move_.flags.bullet {
            battle.add("-immune", &[Arg::Pokemon(pokemon), Arg::Str("[from] ability: Bulletproof")]);
            return AbilityHandlerResult::Null;
        }
        AbilityHandlerResult::Undefined
    }
