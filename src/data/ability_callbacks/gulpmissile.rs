//! Gulp Missile Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	gulpmissile: {
//! 		onDamagingHit(damage, target, source, move) {
//! 			if (!source.hp || !source.isActive || target.isSemiInvulnerable()) return;
//! 			if (['cramorantgulping', 'cramorantgorging'].includes(target.species.id)) {
//! 				this.damage(source.baseMaxhp / 4, source, target);
//! 				if (target.species.id === 'cramorantgulping') {
//! 					this.boost({ def: -1 }, source, target, null, true);
//! 				} else {
//! 					source.trySetStatus('par', target, move);
//! 				}
//! 				target.formeChange('cramorant', move);
//! 			}
//! 		},
//! 		// The Dive part of this mechanic is implemented in Dive's `onTryMove` in moves.ts
//! 		onSourceTryPrimaryHit(target, source, effect) {
//! 			if (effect?.id === 'surf' && source.hasAbility('gulpmissile') && source.species.name === 'Cramorant') {
//! 				const forme = source.hp <= source.maxhp / 2 ? 'cramorantgorging' : 'cramorantgulping';
//! 				source.formeChange(forme, effect);
//! 			}
//! 		},
//! 		flags: { cantsuppress: 1, notransform: 1 },
//! 		name: "Gulp Missile",
//! 		rating: 2.5,
//! 		num: 241,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onDamagingHit(...)
pub fn on_damaging_hit(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

/// onSourceTryPrimaryHit(...)
pub fn on_source_try_primary_hit(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

