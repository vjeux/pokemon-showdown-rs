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
        // JavaScript only skips `false` targets, NOT `null` targets!
        // When a substitute is hit, target becomes null but we still process secondaries
        // (especially self-targeting ones like Steel Wing's defense boost)
        let target_pos = match target {
            SpreadMoveTarget::Target(pos) => Some(*pos),
            SpreadMoveTarget::None => None,  // null - still process secondaries (for self-targeting)
            SpreadMoveTarget::Failed => continue,  // false - skip secondaries entirely
        };

        // const secondaries: Dex.SecondaryEffect[] =
        //     this.battle.runEvent('ModifySecondaries', target, source, moveData, moveData.secondaries.slice());
        // Pass secondaries as the relay value and get the (potentially modified) result back
        // When target is null, JavaScript still calls runEvent with null target
        let modify_result = if let Some(tpos) = target_pos {
            battle.run_event(
                "ModifySecondaries",
                Some(crate::event::EventTarget::Pokemon(tpos)),
                Some(source_pos),
                Some(&crate::battle::Effect::move_(move_data.id.clone())),
                EventResult::Secondaries(move_data.secondaries.clone()),
                false,
                false,
            )
        } else {
            // Target is null - JavaScript passes null to runEvent
            // We simulate this by not passing a target
            battle.run_event(
                "ModifySecondaries",
                None,  // null target
                Some(source_pos),
                Some(&crate::battle::Effect::move_(move_data.id.clone())),
                EventResult::Secondaries(move_data.secondaries.clone()),
                false,
                false,
            )
        };

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
            debug_elog!("[SECONDARIES] Processing secondary: chance={:?}, volatile_status={:?}, target={:?}",
                secondary.chance, secondary.volatile_status, target_pos);

            // const secondaryRoll = this.battle.random(100);
            let secondary_roll = battle.random(100);

            // const secondaryOverflow = (secondary.boosts || secondary.self) && this.battle.gen <= 8;
            let has_boosts = secondary.boosts.is_some();
            let has_self = secondary.self_secondary.is_some();
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
                // When target is null (hit substitute), only self-targeting secondaries apply
                let effect_target = if secondary.self_secondary.is_some() {
                    Some(source_pos)
                } else if let Some(tpos) = target_pos {
                    Some(tpos)
                } else {
                    // Target is null and secondary doesn't target self - JavaScript still calls moveHit
                    // with null target, which would fail to apply the effect
                    // For now, skip non-self effects when target is null
                    None
                };

                if let Some(et) = effect_target {
                    crate::battle_actions::move_hit(
                        battle,
                        &[Some(et)],
                        source_pos,
                        move_,
                        Some(HitEffect::Secondary(&secondary)),
                        true,  // isSecondary
                        is_self,
                    );
                }
            }
        }
    }
}
