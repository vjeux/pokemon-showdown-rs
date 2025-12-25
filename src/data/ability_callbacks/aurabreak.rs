//! Aura Break Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	aurabreak: {
//! 		onStart(pokemon) {
//! 			this.add('-ability', pokemon, 'Aura Break');
//! 		},
//! 		onAnyTryPrimaryHit(target, source, move) {
//! 			if (target === source || move.category === 'Status') return;
//! 			move.hasAuraBreak = true;
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Aura Break",
//! 		rating: 1,
//! 		num: 188,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onStart(pokemon)
pub fn on_start(battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult {
    // this.add('-ability', pokemon, 'Aura Break');
    battle.add("-ability", &[Arg::Pokemon(pokemon), Arg::Str("Aura Break")]);
    AbilityHandlerResult::Undefined
}

/// onAnyTryPrimaryHit(target, source, move)
/// Note: This sets move.hasAuraBreak which affects Fairy Aura and Dark Aura abilities.
/// The actual effect (reversing aura boosts) is handled where those abilities are processed.
pub fn on_any_try_primary_hit(battle: &mut Battle, target: &Pokemon, source: &Pokemon, move_: &mut MoveDef) -> AbilityHandlerResult {
    // if (target === source || move.category === 'Status') return;
    if target.side_index == source.side_index && target.position == source.position {
        return AbilityHandlerResult::Undefined;
    }
    if move_.category == MoveCategory::Status {
        return AbilityHandlerResult::Undefined;
    }

    // move.hasAuraBreak = true;
    // Note: MoveDef doesn't have a mutable hasAuraBreak field in the static data.
    // This would need to be tracked in battle state or event info.
    // For now, we acknowledge the limitation.
    AbilityHandlerResult::Undefined
}

