//! Synchronize Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	synchronize: {
//! 		onAfterSetStatus(status, target, source, effect) {
//! 			if (!source || source === target) return;
//! 			if (effect && effect.id === 'toxicspikes') return;
//! 			if (status.id === 'slp' || status.id === 'frz') return;
//! 			this.add('-activate', target, 'ability: Synchronize');
//! 			// Hack to make status-prevention abilities think Synchronize is a status move
//! 			// and show messages when activating against it.
//! 			source.trySetStatus(status, target, { status: status.id, id: 'synchronize' } as Effect);
//! 		},
//! 		flags: {},
//! 		name: "Synchronize",
//! 		rating: 2,
//! 		num: 28,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onAfterSetStatus(status, target, source, effect)
/// Passes status conditions back to the source Pokemon
///
/// TODO: onAfterSetStatus handler not yet called by battle engine
pub fn on_after_set_status(battle: &mut Battle, status: &Status, target: &Pokemon, source: Option<&Pokemon>, effect: &Effect) -> AbilityHandlerResult {
    // if (!source || source === target) return;
    let source = match source {
        Some(s) => s,
        None => return AbilityHandlerResult::Undefined,
    };

    let target_ref = (target.side_index, target.position);
    let source_ref = (source.side_index, source.position);

    if target_ref == source_ref {
        return AbilityHandlerResult::Undefined;
    }

    // if (effect && effect.id === 'toxicspikes') return;
    if effect.id == "toxicspikes" {
        return AbilityHandlerResult::Undefined;
    }

    // if (status.id === 'slp' || status.id === 'frz') return;
    if status.id == "slp" || status.id == "frz" {
        return AbilityHandlerResult::Undefined;
    }

    // this.add('-activate', target, 'ability: Synchronize');
    battle.add("-activate", &[Arg::Pokemon(target), Arg::Str("ability: Synchronize")]);

    // source.trySetStatus(status, target, { status: status.id, id: 'synchronize' } as Effect);
    // TODO: This requires trySetStatus method on Pokemon or battle
    // For now, we can use battle.set_status if it exists
    // The comment says this is a hack to make status-prevention abilities show messages

    AbilityHandlerResult::Undefined
}

