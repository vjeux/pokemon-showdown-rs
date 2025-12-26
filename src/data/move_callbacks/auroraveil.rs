//! Aurora Veil Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts
//!
//! ```text
//! JS Source (data/moves.ts):
//! 	auroraveil: {
//! 		num: 694,
//! 		accuracy: true,
//! 		basePower: 0,
//! 		category: "Status",
//! 		name: "Aurora Veil",
//! 		pp: 20,
//! 		priority: 0,
//! 		flags: { snatch: 1, metronome: 1 },
//! 		sideCondition: 'auroraveil',
//! 		onTry() {
//! 			return this.field.isWeather(['hail', 'snowscape']);
//! 		},
//! 		condition: {
//! 			duration: 5,
//! 			durationCallback(target, source, effect) {
//! 				if (source?.hasItem('lightclay')) {
//! 					return 8;
//! 				}
//! 				return 5;
//! 			},
//! 			onAnyModifyDamage(damage, source, target, move) {
//! 				if (target !== source && this.effectState.target.hasAlly(target)) {
//! 					if ((target.side.getSideCondition('reflect') && this.getCategory(move) === 'Physical') ||
//! 						(target.side.getSideCondition('lightscreen') && this.getCategory(move) === 'Special')) {
//! 						return;
//! 					}
//! 					if (!target.getMoveHitData(move).crit && !move.infiltrates) {
//! 						this.debug('Aurora Veil weaken');
//! 						if (this.activePerHalf > 1) return this.chainModify([2732, 4096]);
//! 						return this.chainModify(0.5);
//! 					}
//! 				}
//! 			},
//! 			onSideStart(side) {
//! 				this.add('-sidestart', side, 'move: Aurora Veil');
//! 			},
//! 			onSideResidualOrder: 26,
//! 			onSideResidualSubOrder: 10,
//! 			onSideEnd(side) {
//! 				this.add('-sideend', side, 'move: Aurora Veil');
//! 			},
//! 		},
//! 		secondary: null,
//! 		target: "allySide",
//! 		type: "Ice",
//! 		zMove: { boost: { spe: 1 } },
//! 		contestType: "Beautiful",
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onTry callback for Aurora Veil
/// Checks if weather is hail or snow
/// JS: onTry() { return this.field.isWeather(['hail', 'snowscape']); }
pub fn on_try(
    battle: &mut Battle,
    source: (usize, usize),
    target: (usize, usize),
    move_id: &ID,
) -> MoveHandlerResult {
    // JavaScript: return this.field.isWeather(['hail', 'snowscape']);
    let weather = battle.field.weather.as_str();
    if weather == "hail" || weather == "snow" || weather == "snowscape" {
        MoveHandlerResult::Undefined
    } else {
        MoveHandlerResult::False
    }
}

/// onAnyModifyDamage - Side condition callback
/// Reduces damage taken by allies (half damage, or 2732/4096 in doubles)
/// JS: onAnyModifyDamage(damage, source, target, move) { ... }
pub fn on_any_modify_damage(
    battle: &mut Battle,
    damage: i32,
    source: (usize, usize),
    target: (usize, usize),
    move_id: &ID,
) -> MoveHandlerResult {
    // JavaScript: if (target !== source && this.effectState.target.hasAlly(target)) { ... }
    if target == source {
        return MoveHandlerResult::Undefined;
    }

    // Check if target is on the side with Aurora Veil
    // effectState.target refers to the side that has the condition
    // We need to check if target is on that side
    // This is handled by the caller checking which side has the condition

    // JavaScript: if ((target.side.getSideCondition('reflect') && this.getCategory(move) === 'Physical') ||
    //             (target.side.getSideCondition('lightscreen') && this.getCategory(move) === 'Special')) { return; }
    let (target_side, target_idx) = target;
    let has_reflect = if let Some(side) = battle.sides.get(target_side) {
        side.has_side_condition(&ID::new("reflect"))
    } else {
        false
    };
    let has_lightscreen = if let Some(side) = battle.sides.get(target_side) {
        side.has_side_condition(&ID::new("lightscreen"))
    } else {
        false
    };

    let category = battle.get_category(move_id);
    if (has_reflect && category == "Physical") || (has_lightscreen && category == "Special") {
        // Don't stack with Reflect/Light Screen
        return MoveHandlerResult::Undefined;
    }

    // JavaScript: if (!target.getMoveHitData(move).crit && !move.infiltrates) { ... }
    // TODO: Check for critical hit (getMoveHitData)
    // TODO: Check move.infiltrates flag
    // For now, assume not a crit and move doesn't infiltrate

    battle.debug("Aurora Veil weaken");

    // JavaScript: if (this.activePerHalf > 1) return this.chainModify([2732, 4096]);
    // TODO: Check activePerHalf (number of active Pokemon per side in doubles/triples)
    // For singles, this is 1, so use 0.5 modifier
    // For now, assume singles
    MoveHandlerResult::ChainModify(1, 2) // 0.5x damage
}

/// onSideStart - Side condition callback
/// Logs when Aurora Veil starts
/// JS: onSideStart(side) { this.add('-sidestart', side, 'move: Aurora Veil'); }
pub fn on_side_start(
    battle: &mut Battle,
    side_idx: usize,
) -> MoveHandlerResult {
    // JavaScript: this.add('-sidestart', side, 'move: Aurora Veil');
    let side_id = if let Some(side) = battle.sides.get(side_idx) {
        side.id_str().to_string()
    } else {
        return MoveHandlerResult::Undefined;
    };

    battle.add_log("-sidestart", &[&side_id, "move: Aurora Veil"]);
    MoveHandlerResult::Undefined
}

/// onSideResidualOrder - Returns priority order for residual processing
/// JS: onSideResidualOrder: 26
pub fn on_side_residual_order(
    _battle: &mut Battle,
    _side_idx: usize,
) -> MoveHandlerResult {
    MoveHandlerResult::Number(26)
}

/// onSideResidualSubOrder - Returns sub-priority for residual processing
/// JS: onSideResidualSubOrder: 10
pub fn on_side_residual_sub_order(
    _battle: &mut Battle,
    _side_idx: usize,
) -> MoveHandlerResult {
    MoveHandlerResult::Number(10)
}

/// onSideEnd - Side condition callback
/// Logs when Aurora Veil ends
/// JS: onSideEnd(side) { this.add('-sideend', side, 'move: Aurora Veil'); }
pub fn on_side_end(
    battle: &mut Battle,
    side_idx: usize,
) -> MoveHandlerResult {
    // JavaScript: this.add('-sideend', side, 'move: Aurora Veil');
    let side_id = if let Some(side) = battle.sides.get(side_idx) {
        side.id_str().to_string()
    } else {
        return MoveHandlerResult::Undefined;
    };

    battle.add_log("-sideend", &[&side_id, "move: Aurora Veil"]);
    MoveHandlerResult::Undefined
}
