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
use crate::battle_actions::DamageResult;

/// Apply move hit to target(s)
/// Equivalent to moveHit() in battle-actions.ts:1385
///
/// JavaScript signature:
/// moveHit(targets: Pokemon | null | (Pokemon | null)[], pokemon: Pokemon, moveOrMoveName: ActiveMove,
///         moveData?: Dex.HitEffect, isSecondary?: boolean, isSelf?: boolean): number | undefined | false
///
/// Returns DamageResult:
/// - Damage(n) if damage was dealt (number in JS)
/// - Undefined if move succeeded without damage (undefined in JS)
/// - Failed if move failed (false in JS)
pub fn move_hit(
    battle: &mut Battle,
    targets: &[Option<(usize, usize)>],
    pokemon_pos: (usize, usize),
    move_id: &ID,
    move_data_id: Option<&ID>,
    is_secondary: bool,
    is_self: bool,
) -> DamageResult {
    // if (!Array.isArray(targets)) targets = [targets];
    // (Already handled by caller passing slice)

    // const retVal = this.spreadMoveHit(targets, pokemon, moveOrMoveName, moveData, isSecondary, isSelf)[0][0];
    // Convert slice to SpreadMoveTargets
    use crate::battle_actions::SpreadMoveTarget;
    let spread_targets: crate::battle_actions::SpreadMoveTargets = targets.iter().map(|&t| match t {
        Some(pos) => SpreadMoveTarget::Target(pos),
        None => SpreadMoveTarget::None,
    }).collect();

    let (damages, _targets) = spread_move_hit(battle, &spread_targets, pokemon_pos, move_id, move_data_id, is_secondary, is_self);

    let ret_val = match damages.get(0) {
        Some(DamageResult::Damage(n)) => DamageResult::Damage(*n),
        Some(DamageResult::Failed) => DamageResult::Failed,
        Some(DamageResult::Success) | None => DamageResult::Undefined,
        Some(DamageResult::NotFail) => DamageResult::NotFail,
        Some(DamageResult::Undefined) => DamageResult::Undefined,
        Some(DamageResult::HitSubstitute) => DamageResult::Damage(0),
    };

    // return retVal === true ? undefined : retVal;
    // JavaScript converts true (Success) to undefined
    match ret_val {
        DamageResult::Success => DamageResult::Undefined,
        other => other,
    }
}

