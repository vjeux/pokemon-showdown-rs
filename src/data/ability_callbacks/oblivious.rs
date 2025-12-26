//! Oblivious Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	oblivious: {
//! 		onUpdate(pokemon) {
//! 			if (pokemon.volatiles['attract']) {
//! 				this.add('-activate', pokemon, 'ability: Oblivious');
//! 				pokemon.removeVolatile('attract');
//! 				this.add('-end', pokemon, 'move: Attract', '[from] ability: Oblivious');
//! 			}
//! 			if (pokemon.volatiles['taunt']) {
//! 				this.add('-activate', pokemon, 'ability: Oblivious');
//! 				pokemon.removeVolatile('taunt');
//! 				// Taunt's volatile already sends the -end message when removed
//! 			}
//! 		},
//! 		onImmunity(type, pokemon) {
//! 			if (type === 'attract') return false;
//! 		},
//! 		onTryHit(pokemon, target, move) {
//! 			if (move.id === 'attract' || move.id === 'captivate' || move.id === 'taunt') {
//! 				this.add('-immune', pokemon, '[from] ability: Oblivious');
//! 				return null;
//! 			}
//! 		},
//! 		onTryBoost(boost, target, source, effect) {
//! 			if (effect.name === 'Intimidate' && boost.atk) {
//! 				delete boost.atk;
//! 				this.add('-fail', target, 'unboost', 'Attack', '[from] ability: Oblivious', `[of] ${target}`);
//! 			}
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Oblivious",
//! 		rating: 1.5,
//! 		num: 12,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onUpdate(pokemon)
/// Cures attract and taunt
pub fn on_update(battle: &mut Battle, pokemon: &mut Pokemon) -> AbilityHandlerResult {
    // if (pokemon.volatiles['attract'])
    if pokemon.has_volatile(&ID::new("attract")) {
        // this.add('-activate', pokemon, 'ability: Oblivious');
        battle.add("-activate", &[Arg::Pokemon(pokemon), Arg::Str("ability: Oblivious")]);
        // pokemon.removeVolatile('attract');
        pokemon.remove_volatile(&ID::new("attract"));
        // this.add('-end', pokemon, 'move: Attract', '[from] ability: Oblivious');
        battle.add("-end", &[
            Arg::Pokemon(pokemon),
            Arg::Str("move: Attract"),
            Arg::Str("[from] ability: Oblivious")
        ]);
    }
    // if (pokemon.volatiles['taunt'])
    if pokemon.has_volatile(&ID::new("taunt")) {
        // this.add('-activate', pokemon, 'ability: Oblivious');
        battle.add("-activate", &[Arg::Pokemon(pokemon), Arg::Str("ability: Oblivious")]);
        // pokemon.removeVolatile('taunt');
        pokemon.remove_volatile(&ID::new("taunt"));
        // Taunt's volatile already sends the -end message when removed
    }
    AbilityHandlerResult::Undefined
}

/// onImmunity(type, pokemon)
/// Prevents attract
pub fn on_immunity(_battle: &mut Battle, immunity_type: &str, _pokemon: &Pokemon) -> AbilityHandlerResult {
    // if (type === 'attract') return false;
    if immunity_type == "attract" {
        return AbilityHandlerResult::False;
    }
    AbilityHandlerResult::Undefined
}

/// onTryHit(pokemon, target, move)
/// Blocks attract, captivate, and taunt moves
pub fn on_try_hit(battle: &mut Battle, pokemon: &mut Pokemon, _target: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    // if (move.id === 'attract' || move.id === 'captivate' || move.id === 'taunt')
    if move_.id.as_str() == "attract" || move_.id.as_str() == "captivate" || move_.id.as_str() == "taunt" {
        // this.add('-immune', pokemon, '[from] ability: Oblivious');
        battle.add("-immune", &[
            Arg::Pokemon(pokemon),
            Arg::Str("[from] ability: Oblivious")
        ]);
        // return null;
        return AbilityHandlerResult::Null;
    }
    AbilityHandlerResult::Undefined
}

/// onTryBoost(boost, target, source, effect)
/// Blocks Intimidate
pub fn on_try_boost(battle: &mut Battle, boost: &mut std::collections::HashMap<String, i8>, target: &Pokemon, _source: Option<&Pokemon>, effect_id: &str, _has_secondaries: bool) -> AbilityHandlerResult {
    // if (effect.name === 'Intimidate' && boost.atk)
    if effect_id == "intimidate" && boost.contains_key("atk") {
        // delete boost.atk;
        boost.remove("atk");
        // this.add('-fail', target, 'unboost', 'Attack', '[from] ability: Oblivious', `[of] ${target}`);
        battle.add("-fail", &[
            Arg::Pokemon(target),
            Arg::Str("unboost"),
            Arg::Str("Attack"),
            Arg::Str("[from] ability: Oblivious"),
            Arg::Str(&format!("[of] {}", target.position))
        ]);
    }
    AbilityHandlerResult::Undefined
}

