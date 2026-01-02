//! BattleActions::secondaries - Apply secondary effects of a move
//!
//! 1:1 port of secondaries from battle-actions.ts

// JS Source:
// 	secondaries(targets: SpreadMoveTargets, source: Pokemon, move: ActiveMove, moveData: ActiveMove, isSelf?: boolean) {
// 		if (!moveData.secondaries) return;
// 		for (const target of targets) {
// 			if (target === false) continue;
// 			const secondaries: Dex.SecondaryEffect[] =
// 				this.battle.runEvent('ModifySecondaries', target, source, moveData, moveData.secondaries.slice());
// 			for (const secondary of secondaries) {
// 				const secondaryRoll = this.battle.random(100);
// 				// User stat boosts or target stat drops can possibly overflow if it goes beyond 256 in Gen 8 or prior
// 				const secondaryOverflow = (secondary.boosts || secondary.self) && this.battle.gen <= 8;
// 				if (typeof secondary.chance === 'undefined' ||
// 					secondaryRoll < (secondaryOverflow ? secondary.chance % 256 : secondary.chance)) {
// 					this.moveHit(target, source, move, secondary, true, isSelf);
// 				}
// 			}
// 		}
// 	}

use crate::*;
use crate::battle_actions::{SpreadMoveTargets, SpreadMoveTarget};

/// Apply secondary effects of a move
/// Equivalent to secondaries() in battle-actions.ts
///
/// JavaScript signature:
/// secondaries(targets: SpreadMoveTargets, source: Pokemon, move: ActiveMove, moveData: ActiveMove, isSelf?: boolean)
pub fn secondaries(
    battle: &mut Battle,
    targets: &SpreadMoveTargets,
    source_pos: (usize, usize),
    move_id: &ID,
    _is_self: bool,
) {
    // if (!moveData.secondaries) return;
    let has_secondaries = {
        if let Some(ref active_move) = battle.active_move {
            active_move.secondaries.is_some() && !active_move.secondaries.as_ref().unwrap().is_empty()
        } else {
            false
        }
    };

    if !has_secondaries {
        return;
    }

    // for (const target of targets) {
    for target in targets {
        // if (target === false) continue;
        let target_pos = match target {
            SpreadMoveTarget::Target(pos) => *pos,
            _ => continue,
        };

        // const secondaries: Dex.SecondaryEffect[] =
        //     this.battle.runEvent('ModifySecondaries', target, source, moveData, moveData.secondaries.slice());
        // TODO: Implement ModifySecondaries event
        // For now, just use the secondaries from active_move directly
        let secondaries = if let Some(ref active_move) = battle.active_move {
            active_move.secondaries.clone().unwrap_or_default()
        } else {
            continue;
        };

        // for (const secondary of secondaries) {
        for secondary in secondaries {
            // const secondaryRoll = this.battle.random(100);
            let secondary_roll = battle.random(100);

            // const secondaryOverflow = (secondary.boosts || secondary.self) && this.battle.gen <= 8;
            let has_boosts = secondary.boosts.is_some();
            let has_self = secondary.self_data.is_some();
            let secondary_overflow = (has_boosts || has_self) && battle.gen <= 8;

            // if (typeof secondary.chance === 'undefined' ||
            //     secondaryRoll < (secondaryOverflow ? secondary.chance % 256 : secondary.chance)) {
            let should_apply = match secondary.chance {
                None => true, // chance is undefined, always apply
                Some(chance) => {
                    let effective_chance = if secondary_overflow {
                        chance % 256
                    } else {
                        chance
                    };
                    secondary_roll < effective_chance
                }
            };

            if should_apply {
                // this.moveHit(target, source, move, secondary, true, isSelf);
                // TODO: Implement full moveHit function
                // For now, apply boosts if present
                if let Some(ref boosts) = secondary.boosts {
                    let mut boost_array = Vec::new();
                    if boosts.atk != 0 {
                        boost_array.push(("atk", boosts.atk));
                    }
                    if boosts.def != 0 {
                        boost_array.push(("def", boosts.def));
                    }
                    if boosts.spa != 0 {
                        boost_array.push(("spa", boosts.spa));
                    }
                    if boosts.spd != 0 {
                        boost_array.push(("spd", boosts.spd));
                    }
                    if boosts.spe != 0 {
                        boost_array.push(("spe", boosts.spe));
                    }
                    if boosts.accuracy != 0 {
                        boost_array.push(("accuracy", boosts.accuracy));
                    }
                    if boosts.evasion != 0 {
                        boost_array.push(("evasion", boosts.evasion));
                    }

                    battle.boost(&boost_array, target_pos, Some(source_pos), None, false, true);
                }

                // TODO: Handle other secondary effects (status, volatiles, side conditions, etc.)
            }
        }
    }
}
