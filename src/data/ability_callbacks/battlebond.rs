//! Battle Bond Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	battlebond: {
//! 		onSourceAfterFaint(length, target, source, effect) {
//! 			if (source.bondTriggered) return;
//! 			if (effect?.effectType !== 'Move') return;
//! 			if (source.species.id === 'greninjabond' && source.hp && !source.transformed && source.side.foePokemonLeft()) {
//! 				this.boost({ atk: 1, spa: 1, spe: 1 }, source, source, this.effect);
//! 				this.add('-activate', source, 'ability: Battle Bond');
//! 				source.bondTriggered = true;
//! 			}
//! 		},
//! 		onModifyMovePriority: -1,
//! 		onModifyMove(move, attacker) {
//! 			if (move.id === 'watershuriken' && attacker.species.name === 'Greninja-Ash' &&
//! 				!attacker.transformed) {
//! 				move.multihit = 3;
//! 			}
//! 		},
//! 		flags: { failroleplay: 1, noreceiver: 1, noentrain: 1, notrace: 1, failskillswap: 1, cantsuppress: 1 },
//! 		name: "Battle Bond",
//! 		rating: 3.5,
//! 		num: 210,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onSourceAfterFaint(length, target, source, effect)
pub fn on_source_after_faint(battle: &mut Battle, _length: i32, _target: &Pokemon, source: &mut Pokemon, effect: &Effect) -> AbilityHandlerResult {
    // if (source.bondTriggered) return;
    if source.ability_state.data.get("bondTriggered").and_then(|v| v.as_bool()).unwrap_or(false) {
        return AbilityHandlerResult::Undefined;
    }

    // if (effect?.effectType !== 'Move') return;
    if effect.effect_type != "Move" {
        return AbilityHandlerResult::Undefined;
    }

    // if (source.species.id === 'greninjabond' && source.hp && !source.transformed && source.side.foePokemonLeft())
    if source.species_id.as_str() == "greninjabond" &&
       source.hp > 0 &&
       !source.transformed {
        // Check if foes are left
        let foe_side_index = 1 - source.side_index;
        let foe_pokemon_left = battle.sides[foe_side_index].foe_pokemon_left();

        if foe_pokemon_left > 0 {
            // this.boost({ atk: 1, spa: 1, spe: 1 }, source, source, this.effect);
            battle.boost(
                &[("atk", 1), ("spa", 1), ("spe", 1)],
                (source.side_index, source.position),
                Some((source.side_index, source.position)),
                None
            );

            // this.add('-activate', source, 'ability: Battle Bond');
            battle.add("-activate", &[
                Arg::Pokemon(source),
                Arg::Str("ability: Battle Bond")
            ]);

            // source.bondTriggered = true;
            source.ability_state.data.insert("bondTriggered".to_string(), serde_json::Value::Bool(true));
        }
    }

    AbilityHandlerResult::Undefined
}

/// onModifyMovePriority(...)
pub fn on_modify_move_priority(_battle: &mut Battle) -> AbilityHandlerResult {
    // onModifyMovePriority: -1,
    AbilityHandlerResult::Number(-1)
}

/// onModifyMove(move, attacker)
pub fn on_modify_move(move_: &mut MoveDef, attacker: &Pokemon) -> AbilityHandlerResult {
    // if (move.id === 'watershuriken' && attacker.species.name === 'Greninja-Ash' && !attacker.transformed)
    if move_.id.as_str() == "watershuriken" &&
       attacker.species_id.as_str() == "greninjaash" &&
       !attacker.transformed {
        // move.multihit = 3;
        move_.multi_hit = Some((3, 3)); // Exactly 3 hits
    }

    AbilityHandlerResult::Undefined
}

