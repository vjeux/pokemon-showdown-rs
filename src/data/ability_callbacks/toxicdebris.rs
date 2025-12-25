//! Toxic Debris Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	toxicdebris: {
//! 		onDamagingHit(damage, target, source, move) {
//! 			const side = source.isAlly(target) ? source.side.foe : source.side;
//! 			const toxicSpikes = side.sideConditions['toxicspikes'];
//! 			if (move.category === 'Physical' && (!toxicSpikes || toxicSpikes.layers < 2)) {
//! 				this.add('-activate', target, 'ability: Toxic Debris');
//! 				side.addSideCondition('toxicspikes', target);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Toxic Debris",
//! 		rating: 3.5,
//! 		num: 295,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onDamagingHit(damage, target, source, move)
/// Sets Toxic Spikes on attacker's side when hit by physical move
pub fn on_damaging_hit(battle: &mut Battle, _damage: u32, target: &Pokemon, source: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    // if (move.category === 'Physical' && (!toxicSpikes || toxicSpikes.layers < 2))
    if move_.category != MoveCategory::Physical {
        return AbilityHandlerResult::Undefined;
    }

    // const side = source.isAlly(target) ? source.side.foe : source.side;
    let side_index = if source.is_ally(target.side_index) {
        // Get foe side
        battle.sides[source.side_index].foe_index.unwrap_or(source.side_index)
    } else {
        source.side_index
    };

    // const toxicSpikes = side.sideConditions['toxicspikes'];
    let tspikes_id = ID::new("toxicspikes");
    let can_add = if let Some(tspikes) = battle.sides[side_index].side_conditions.get(&tspikes_id) {
        // if (!toxicSpikes || toxicSpikes.layers < 2)
        tspikes.layers.unwrap_or(0) < 2
    } else {
        true
    };

    if can_add {
        // this.add('-activate', target, 'ability: Toxic Debris');
        battle.add("-activate", &[Arg::Pokemon(target), Arg::Str("ability: Toxic Debris")]);
        // side.addSideCondition('toxicspikes', target);
        battle.sides[side_index].add_side_condition(tspikes_id, None);
    }

    AbilityHandlerResult::Undefined
}
