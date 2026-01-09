//! BattleActions::tryPrimaryHitEvent - Fire TryPrimaryHit event for all targets
//!
//! 1:1 port of tryPrimaryHitEvent from battle-actions.ts

// JS Source:
// 	tryPrimaryHitEvent(
// 		damage: SpreadMoveDamage, targets: SpreadMoveTargets, pokemon: Pokemon,
// 		move: ActiveMove, moveData: ActiveMove, isSecondary?: boolean
// 	): SpreadMoveDamage {
// 		for (const [i, target] of targets.entries()) {
// 			if (!target) continue;
// 			damage[i] = this.battle.runEvent('TryPrimaryHit', target, pokemon, moveData);
// 		}
// 		return damage;
// 	}

use crate::*;
use crate::event::EventResult;
use crate::battle_actions::{SpreadMoveDamage, DamageResult, SpreadMoveTargets, SpreadMoveTarget};

/// Fire TryPrimaryHit event for all targets
/// Equivalent to tryPrimaryHitEvent() in battle-actions.ts
///
/// JavaScript signature:
/// tryPrimaryHitEvent(damage: SpreadMoveDamage, targets: SpreadMoveTargets, pokemon: Pokemon,
///                    move: ActiveMove, moveData: ActiveMove, isSecondary?: boolean): SpreadMoveDamage
// TODO: Verify move parameter type matches JavaScript's ActiveMove usage
pub fn try_primary_hit_event(
    battle: &mut Battle,
    mut damage: SpreadMoveDamage,
    targets: &SpreadMoveTargets,
    pokemon_pos: (usize, usize),
    move_id: &ID,
    _is_secondary: bool,
) -> SpreadMoveDamage {
    // for (const [i, target] of targets.entries()) {
    for (i, target) in targets.iter().enumerate() {
        // if (!target) continue;
        let target_pos = match target {
            SpreadMoveTarget::Target(pos) => *pos,
            _ => continue,
        };

        // damage[i] = this.battle.runEvent('TryPrimaryHit', target, pokemon, moveData);
        let result = battle.run_event("TryPrimaryHit", Some(crate::event::EventTarget::Pokemon(target_pos)), Some(pokemon_pos), Some(&crate::battle::Effect::move_(move_id.clone())), EventResult::Continue, false, false);

        // JavaScript: damage[i] = result
        // Direct assignment - runEvent returns number | boolean | undefined in JS
        // In Rust: EventResult maps to number | boolean | null
        // IMPORTANT: EventResult::Continue means "no handler interfered", which is success (true) in damage terms
        // IMPORTANT: EventResult::HitSubstitute means the substitute took the hit, not the target
        damage[i] = match result {
            EventResult::Number(val) => DamageResult::Damage(val),
            EventResult::Boolean(false) => DamageResult::Failed,
            EventResult::Null => DamageResult::Undefined,
            EventResult::HitSubstitute => DamageResult::HitSubstitute,
            // Continue means no handler stopped the move → success
            EventResult::Continue | EventResult::Boolean(true) => DamageResult::Success,
            _ => DamageResult::Success,
        };
    }

    // return damage;
    damage
}
