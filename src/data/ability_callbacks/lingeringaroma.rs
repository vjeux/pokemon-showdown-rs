//! Lingering Aroma Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	lingeringaroma: {
//! 		onDamagingHit(damage, target, source, move) {
//! 			const sourceAbility = source.getAbility();
//! 			if (sourceAbility.flags['cantsuppress'] || sourceAbility.id === 'lingeringaroma') {
//! 				return;
//! 			}
//! 			if (this.checkMoveMakesContact(move, source, target, !source.isAlly(target))) {
//! 				const oldAbility = source.setAbility('lingeringaroma', target);
//! 				if (oldAbility) {
//! 					this.add('-activate', target, 'ability: Lingering Aroma', this.dex.abilities.get(oldAbility).name, `[of] ${source}`);
//! 				}
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Lingering Aroma",
//! 		rating: 2,
//! 		num: 268,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onDamagingHit(damage, target, source, move)
/// Changes attacker's ability to Lingering Aroma on contact
pub fn on_damaging_hit(battle: &mut Battle, _damage: u32, target: &Pokemon, source: &mut Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    // const sourceAbility = source.getAbility();
    let source_ability_id = source.get_ability();

    // if (sourceAbility.flags['cantsuppress'] || sourceAbility.id === 'lingeringaroma')
    if let Some(source_ability_def) = crate::data::abilities::get_ability(source_ability_id) {
        if source_ability_def.flags.cannot_suppress || source_ability_id.as_str() == "lingeringaroma" {
            return AbilityHandlerResult::Undefined;
        }
    }

    // if (this.checkMoveMakesContact(move, source, target, !source.isAlly(target)))
    let source_ref = (source.side_index, source.position);
    if battle.check_move_makes_contact(&move_.id, source_ref) {
        // const oldAbility = source.setAbility('lingeringaroma', target);
        let old_ability = source.set_ability(ID::new("lingeringaroma"));

        // if (oldAbility)
        if !old_ability.is_empty() {
            // this.add('-activate', target, 'ability: Lingering Aroma', this.dex.abilities.get(oldAbility).name, `[of] ${source}`);
            battle.add("-activate", &[
                Arg::Pokemon(target),
                Arg::Str("ability: Lingering Aroma"),
                Arg::String(old_ability.as_str().to_string()),
                Arg::Str(&format!("[of] {}", source.name)),
            ]);
        }
    }
    AbilityHandlerResult::Undefined
}

