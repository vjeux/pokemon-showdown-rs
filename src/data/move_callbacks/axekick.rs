//! Axe Kick Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts
//!
//! ```text
//! JS Source (data/moves.ts):
//! 	axekick: {
//! 		num: 853,
//! 		accuracy: 90,
//! 		basePower: 120,
//! 		category: "Physical",
//! 		name: "Axe Kick",
//! 		pp: 10,
//! 		priority: 0,
//! 		flags: { contact: 1, protect: 1, mirror: 1, metronome: 1 },
//! 		hasCrashDamage: true,
//! 		onMoveFail(target, source, move) {
//! 			this.damage(source.baseMaxhp / 2, source, source, this.dex.conditions.get('High Jump Kick'));
//! 		},
//! 		secondary: {
//! 			chance: 30,
//! 			volatileStatus: 'confusion',
//! 		},
//! 		target: "normal",
//! 		type: "Fighting",
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onMoveFail callback for Axe Kick
/// When the move misses, the user takes 50% of their max HP as crash damage
/// JS: onMoveFail(target, source, move) { this.damage(source.baseMaxhp / 2, source, source, this.dex.conditions.get('High Jump Kick')); }
///
/// TODO: This callback is implemented but not yet wired into the battle flow
/// When accuracy checking infrastructure is built (likely for moves like Thunder, Blizzard, etc.),
/// the MoveFail event needs to be triggered when:
/// - A move misses due to accuracy check
/// - A move is blocked by protection moves (Protect, Detect, etc.)
/// - A move fails due to other reasons (target already fainted, etc.)
/// See pokemon-showdown-js/sim/battle-actions.ts around line 2000+ for reference
pub fn on_move_fail(
    battle: &mut Battle,
    target: (usize, usize),
    source: (usize, usize),
    move_id: &ID,
) -> MoveHandlerResult {
    // JavaScript: this.damage(source.baseMaxhp / 2, source, source, this.dex.conditions.get('High Jump Kick'));
    let (side_idx, poke_idx) = source;

    let base_maxhp = match battle.pokemon_at(side_idx, poke_idx) {
        Some(pokemon) => pokemon.base_maxhp,
        None => return MoveHandlerResult::Undefined,
    };

    // Calculate crash damage: 50% of max HP
    let crash_damage = base_maxhp / 2;

    // Apply damage to the source Pokemon
    // The JavaScript uses this.dex.conditions.get('High Jump Kick') as the effect
    // This is for logging purposes - shows "High Jump Kick" as the source of crash damage
    battle.damage(
        crash_damage as i32,
        Some((side_idx, poke_idx)),
        Some((side_idx, poke_idx)),
        Some(&ID::new("highjumpkick")),
        false, // instafaint
    );

    MoveHandlerResult::Undefined
}

