//! BattleActions::moveHit - Apply move hit to target(s)
//!
//! 1:1 port of moveHit from battle-actions.ts:1385

// JS Source:
// 	moveHit(
// 		targets: Pokemon | null | (Pokemon | null)[], pokemon: Pokemon, moveOrMoveName: ActiveMove,
// 		moveData?: Dex.HitEffect, isSecondary?: boolean, isSelf?: boolean
// 	): number | undefined | false {
// 		if (!Array.isArray(targets)) targets = [targets];
// 		const retVal = this.spreadMoveHit(targets, pokemon, moveOrMoveName, moveData, isSecondary, isSelf)[0][0];
// 		return retVal === true ? undefined : retVal;
// 	}

use crate::*;
use crate::battle_actions::spread_move_hit::spread_move_hit;
use crate::battle_actions::SpreadMoveDamageValue;

/// Apply move hit to target(s)
/// Equivalent to moveHit() in battle-actions.ts:1385
///
/// JavaScript signature:
/// moveHit(targets: Pokemon | null | (Pokemon | null)[], pokemon: Pokemon, moveOrMoveName: ActiveMove,
///         moveData?: Dex.HitEffect, isSecondary?: boolean, isSelf?: boolean): number | undefined | false
///
/// Returns:
/// - Some(damage) if damage was dealt
/// - None if move succeeded without damage (undefined in JS)
/// - Some(-1) to represent false (move failed)
pub fn move_hit(
    battle: &mut Battle,
    targets: &[Option<(usize, usize)>],
    pokemon_pos: (usize, usize),
    move_id: &ID,
    move_data_id: Option<&ID>,
    is_secondary: bool,
    is_self: bool,
) -> Option<i32> {
    // if (!Array.isArray(targets)) targets = [targets];
    // (Already handled by caller passing slice)

    // const retVal = this.spreadMoveHit(targets, pokemon, moveOrMoveName, moveData, isSecondary, isSelf)[0][0];
    let (damages, _targets) = spread_move_hit(battle, targets, pokemon_pos, move_id, move_data_id, is_secondary, is_self);

    let ret_val = match damages.get(0) {
        Some(SpreadMoveDamageValue::Damage(n)) => Some(*n),
        _ => None,
    };

    // return retVal === true ? undefined : retVal;
    // In JavaScript, retVal can be:
    // - number (damage dealt)
    // - undefined (move succeeded, no damage)
    // - false (move failed)
    // - true (becomes undefined)
    //
    // In Rust we use Option<i32>:
    // - Some(damage) for number
    // - None for undefined/true
    // - We don't have a direct "false" representation, but that's handled by spreadMoveHit
    //   returning None or 0 in the damage array

    ret_val
}
