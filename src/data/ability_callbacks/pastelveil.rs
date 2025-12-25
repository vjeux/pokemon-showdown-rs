//! Pastel Veil Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	pastelveil: {
//! 		onStart(pokemon) {
//! 			for (const ally of pokemon.alliesAndSelf()) {
//! 				if (['psn', 'tox'].includes(ally.status)) {
//! 					this.add('-activate', pokemon, 'ability: Pastel Veil');
//! 					ally.cureStatus();
//! 				}
//! 			}
//! 		},
//! 		onUpdate(pokemon) {
//! 			if (['psn', 'tox'].includes(pokemon.status)) {
//! 				this.add('-activate', pokemon, 'ability: Pastel Veil');
//! 				pokemon.cureStatus();
//! 			}
//! 		},
//! 		onAnySwitchIn() {
//! 			((this.effect as any).onStart as (p: Pokemon) => void).call(this, this.effectState.target);
//! 		},
//! 		onSetStatus(status, target, source, effect) {
//! 			if (!['psn', 'tox'].includes(status.id)) return;
//! 			if ((effect as Move)?.status) {
//! 				this.add('-immune', target, '[from] ability: Pastel Veil');
//! 			}
//! 			return false;
//! 		},
//! 		onAllySetStatus(status, target, source, effect) {
//! 			if (!['psn', 'tox'].includes(status.id)) return;
//! 			if ((effect as Move)?.status) {
//! 				const effectHolder = this.effectState.target;
//! 				this.add('-block', target, 'ability: Pastel Veil', `[of] ${effectHolder}`);
//! 			}
//! 			return false;
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Pastel Veil",
//! 		rating: 2,
//! 		num: 257,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onStart(pokemon)
/// Cures poison/toxic on self and allies on switch-in
pub fn on_start(battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult {
    // for (const ally of pokemon.alliesAndSelf())
    let side_index = pokemon.side_index;
    let mut allies_to_cure: Vec<(usize, String)> = Vec::new();
    let mut activated = false;

    // Collect allies (including self) with psn/tox status
    if let Some(side) = battle.sides.get(side_index) {
        for ally in side.pokemon.iter().filter(|p| p.is_active && !p.fainted) {
            // if (['psn', 'tox'].includes(ally.status))
            if ally.status.as_str() == "psn" || ally.status.as_str() == "tox" {
                if !activated {
                    activated = true;
                }
                allies_to_cure.push((ally.position, ally.name.clone()));
            }
        }
    }

    if activated {
        // this.add('-activate', pokemon, 'ability: Pastel Veil');
        battle.add("-activate", &[
            Arg::Pokemon(pokemon),
            Arg::Str("ability: Pastel Veil")
        ]);
    }

    // ally.cureStatus();
    for (position, _ally_name) in allies_to_cure {
        battle.sides[side_index].pokemon[position].cure_status();
    }

    AbilityHandlerResult::Undefined
}

/// onUpdate(pokemon)
/// TODO: onUpdate handler not yet implemented in battle engine
/// When implemented, should cure poison/toxic on self
pub fn on_update(_battle: &mut Battle, /* TODO: Add parameters when handler exists */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS when handler infrastructure exists
    AbilityHandlerResult::Undefined
}

/// onAnySwitchIn()
/// TODO: onAnySwitchIn handler not yet implemented in battle engine
/// When implemented, should call onStart with effectState.target
pub fn on_any_switch_in(_battle: &mut Battle, /* TODO: Add parameters when handler exists */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS when handler infrastructure exists
    AbilityHandlerResult::Undefined
}

/// onSetStatus(status, target, source, effect)
/// Prevents poison/toxic on self
pub fn on_set_status(battle: &mut Battle, status: &Status, target: &Pokemon, _source: Option<&Pokemon>, effect: Option<&Effect>) -> AbilityHandlerResult {
    // if (!['psn', 'tox'].includes(status.id)) return;
    if status.id != "psn" && status.id != "tox" {
        return AbilityHandlerResult::Undefined;
    }

    // if ((effect as Move)?.status)
    if let Some(eff) = effect {
        if eff.effect_type == "Move" && eff.status.is_some() {
            // this.add('-immune', target, '[from] ability: Pastel Veil');
            battle.add("-immune", &[
                Arg::Pokemon(target),
                Arg::Str("[from] ability: Pastel Veil")
            ]);
        }
    }

    // return false;
    AbilityHandlerResult::False
}

/// onAllySetStatus(status, target, source, effect)
/// TODO: onAllySetStatus handler not yet implemented in battle engine
/// When implemented, should prevent poison/toxic on allies
pub fn on_ally_set_status(_battle: &mut Battle, /* TODO: Add parameters when handler exists */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS when handler infrastructure exists
    AbilityHandlerResult::Undefined
}

