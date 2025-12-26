//! Baneful Bunker Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts
//!
//! ```text
//! JS Source (data/moves.ts):
//! 	banefulbunker: {
//! 		num: 661,
//! 		accuracy: true,
//! 		basePower: 0,
//! 		category: "Status",
//! 		name: "Baneful Bunker",
//! 		pp: 10,
//! 		priority: 4,
//! 		flags: { noassist: 1, failcopycat: 1 },
//! 		stallingMove: true,
//! 		volatileStatus: 'banefulbunker',
//! 		onPrepareHit(pokemon) {
//! 			return !!this.queue.willAct() && this.runEvent('StallMove', pokemon);
//! 		},
//! 		onHit(pokemon) {
//! 			pokemon.addVolatile('stall');
//! 		},
//! 		condition: {
//! 			duration: 1,
//! 			onStart(target) {
//! 				this.add('-singleturn', target, 'move: Protect');
//! 			},
//! 			onTryHitPriority: 3,
//! 			onTryHit(target, source, move) {
//! 				if (!move.flags['protect']) {
//! 					if (['gmaxoneblow', 'gmaxrapidflow'].includes(move.id)) return;
//! 					if (move.isZ || move.isMax) target.getMoveHitData(move).zBrokeProtect = true;
//! 					return;
//! 				}
//! 				if (move.smartTarget) {
//! 					move.smartTarget = false;
//! 				} else {
//! 					this.add('-activate', target, 'move: Protect');
//! 				}
//! 				const lockedmove = source.getVolatile('lockedmove');
//! 				if (lockedmove) {
//! 					// Outrage counter is reset
//! 					if (source.volatiles['lockedmove'].duration === 2) {
//! 						delete source.volatiles['lockedmove'];
//! 					}
//! 				}
//! 				if (this.checkMoveMakesContact(move, source, target)) {
//! 					source.trySetStatus('psn', target);
//! 				}
//! 				return this.NOT_FAIL;
//! 			},
//! 			onHit(target, source, move) {
//! 				if (move.isZOrMaxPowered && this.checkMoveMakesContact(move, source, target)) {
//! 					source.trySetStatus('psn', target);
//! 				}
//! 			},
//! 		},
//! 		secondary: null,
//! 		target: "self",
//! 		type: "Poison",
//! 		zMove: { boost: { def: 1 } },
//! 		contestType: "Tough",
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::MoveDef;
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::MoveHandlerResult;

/// onPrepareHit(pokemon)
/// Checks if any Pokemon will act and runs StallMove event
pub fn on_prepare_hit(battle: &mut Battle, pokemon: (usize, usize)) -> MoveHandlerResult {
    // return !!this.queue.willAct() && this.runEvent('StallMove', pokemon);
    if battle.queue.will_act() {
        // TODO: Implement StallMove event
        // For now, just check will_act()
        return MoveHandlerResult::True;
    }
    MoveHandlerResult::False
}

/// onHit(pokemon)
/// Adds 'stall' volatile status to the user
pub fn on_hit(battle: &mut Battle, pokemon: (usize, usize)) -> MoveHandlerResult {
    // pokemon.addVolatile('stall');
    battle.add_volatile_to_pokemon(
        pokemon,
        &ID::from("stall"),
        None,
        None,
    );
    MoveHandlerResult::Undefined
}

// Condition callbacks (for the 'banefulbunker' volatile status)

/// onStart(target) - condition callback
/// Displays the Protect message
pub fn condition_on_start(battle: &mut Battle, target: (usize, usize)) -> MoveHandlerResult {
    // this.add('-singleturn', target, 'move: Protect');
    // Build the argument string first to avoid borrow checker issues
    let pokemon_str = if let Some(pokemon) = battle.pokemon_at(target.0, target.1) {
        format!("p{}a: {}", pokemon.side_index + 1, pokemon.name)
    } else {
        return MoveHandlerResult::Undefined;
    };

    battle.add(
        "-singleturn",
        &[
            Arg::Str(&pokemon_str),
            Arg::Str("move: Protect"),
        ],
    );
    MoveHandlerResult::Undefined
}

/// onTryHitPriority: 3
pub const ON_TRY_HIT_PRIORITY: i32 = 3;

/// onTryHit(target, source, move) - condition callback
/// Protects from moves with the 'protect' flag and poisons attackers that make contact
///
/// TODO: This condition callback needs to be wired up in the event system.
/// It should be called when any move tries to hit a Pokemon with the banefulbunker volatile status.
/// This requires implementing a general volatile condition TryHit event system.
pub fn condition_on_try_hit(
    battle: &mut Battle,
    target: &Pokemon,
    source: &Pokemon,
    move_: &MoveDef,
) -> MoveHandlerResult {
    // TODO: Full implementation requires:
    // 1. Check move.flags['protect']
    // 2. Handle smartTarget
    // 3. Check lockedmove volatile
    // 4. Check move makes contact and poison the attacker
    // 5. Return NOT_FAIL to prevent the move

    // For now, just add a placeholder message
    battle.add(
        "-activate",
        &[
            Arg::Pokemon(target),
            Arg::Str("move: Protect"),
        ],
    );

    // Return Undefined for now (full implementation needs NOT_FAIL constant)
    MoveHandlerResult::Undefined
}

/// onHit(target, source, move) - condition callback
/// Poisons Z-move/Max move attackers that make contact
///
/// TODO: This requires Z-move/Max move detection to be implemented
pub fn condition_on_hit(
    battle: &mut Battle,
    _target: &Pokemon,
    _source: &Pokemon,
    _move_: &MoveDef,
) -> MoveHandlerResult {
    // TODO: Check for Z-move or Max move and poison if makes contact
    MoveHandlerResult::Undefined
}
