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
use crate::event::EventResult;
use crate::battle_actions::{SpreadMoveTargets, SpreadMoveTarget, ActiveMove, HitEffect};

/// Apply secondary effects of a move
/// Equivalent to secondaries() in battle-actions.ts
///
/// JavaScript signature:
/// secondaries(targets: SpreadMoveTargets, source: Pokemon, move: ActiveMove, moveData: ActiveMove, isSelf?: boolean)
///
/// IMPORTANT: move_data must be a clone of the ActiveMove, passed to preserve it across nested
/// move calls (like Metronome calling another move which would overwrite battle.active_move).
/// JavaScript doesn't have this problem because it passes moveData as a parameter.
pub fn secondaries(
    battle: &mut Battle,
    targets: &SpreadMoveTargets,
    source_pos: (usize, usize),
    move_: &ActiveMove,
    move_data: &ActiveMove,
    is_self: bool,
) {
    // if (!moveData.secondaries) return;
    if move_data.secondaries.is_empty() {
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
        // Pass secondaries as the relay value and get the (potentially modified) result back
        let modify_result = battle.run_event(
            "ModifySecondaries",
            Some(crate::event::EventTarget::Pokemon(target_pos)),
            Some(source_pos),
            Some(&crate::battle::Effect::move_(move_data.id.clone())),
            EventResult::Secondaries(move_data.secondaries.clone()),
            false,
            false,
        );

        // Extract secondaries from result, or use empty if event blocked them
        let secondaries = match modify_result {
            EventResult::Secondaries(secs) => secs,
            EventResult::Boolean(false) | EventResult::Null | EventResult::Number(0) => {
                // Event returned falsy, block all secondaries
                Vec::new()
            }
            _ => {
                // Fallback to original if event didn't return secondaries
                move_data.secondaries.clone()
            }
        };

        // for (const secondary of secondaries) {
        for secondary in secondaries {
            eprintln!("[SECONDARIES] Processing secondary: chance={:?}, volatile_status={:?}, target={:?}",
                secondary.chance, secondary.volatile_status, target_pos);

            // const secondaryRoll = this.battle.random(100);
            let secondary_roll = battle.random(100);

            // const secondaryOverflow = (secondary.boosts || secondary.self) && this.battle.gen <= 8;
            let has_boosts = secondary.boosts.is_some();
            let has_self = secondary.self_effect;
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
                // Determine the actual target based on secondary.self
                let effect_target = if secondary.self_effect {
                    source_pos
                } else {
                    target_pos
                };

                crate::battle_actions::move_hit(
                    battle,
                    &[Some(effect_target)],
                    source_pos,
                    &move_.id,
                    Some(HitEffect::Secondary(&secondary)),
                    true,  // isSecondary
                    is_self,
                );
            }
        }
    }
}
