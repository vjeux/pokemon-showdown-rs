//! BattleActions::getSpreadDamage - Get damage for each target in a spread move
//!
//! 1:1 port of getSpreadDamage from battle-actions.ts

use crate::*;
use crate::battle_actions::{SpreadMoveDamage, DamageResult, SpreadMoveTargets, SpreadMoveTarget, HitEffect};

/// Get damage for each target in a spread move
/// Equivalent to getSpreadDamage() in battle-actions.ts:1163
///
/// JavaScript (battle-actions.ts):
///   getSpreadDamage(
///     damage: SpreadMoveDamage, targets: SpreadMoveTargets, source: Pokemon,
///     move: ActiveMove, moveData: ActiveMove, isSecondary?: boolean, isSelf?: boolean
///   ): SpreadMoveDamage {
///     for (const [i, target] of targets.entries()) {
///       if (!target) continue;
///       this.battle.activeTarget = target;
///       damage[i] = undefined;
///       const curDamage = this.getDamage(source, target, moveData);
///       // getDamage has several possible return values:
///       //
///       //   a number:
///       //     means that much damage is dealt (0 damage still counts as dealing
///       //     damage for the purposes of things like Static)
///       //   false:
///       //     gives error message: "But it failed!" and move ends
///       //   null:
///       //     the move ends, with no message (usually, a custom fail message
///       //     was already output by an event handler)
///       //   undefined:
///       //     means no damage is dealt and the move continues
///       //
///       // basically, these values have the same meanings as they do for event
///       // handlers.
///
///       if (curDamage === false || curDamage === null) {
///         if (damage[i] === false && !isSecondary && !isSelf) {
///           this.battle.add('-fail', source);
///           this.battle.attrLastMove('[still]');
///         }
///         this.battle.debug('damage calculation interrupted');
///         damage[i] = false;
///         continue;
///       }
///       damage[i] = curDamage;
///     }
///     return damage;
///   }
// TODO: Verify move parameter type matches JavaScript's ActiveMove usage
pub fn get_spread_damage<'a>(
    battle: &mut Battle,
    damages: SpreadMoveDamage,
    targets: &SpreadMoveTargets,
    source_pos: (usize, usize),
    move_id: &ID,
    hit_effect: Option<HitEffect<'a>>,
    _is_secondary: bool,
    _is_self: bool,
) -> SpreadMoveDamage {
    let mut result_damages = damages;

    // for (const [i, target] of targets.entries()) {
    for (i, target) in targets.iter().enumerate() {
        // if (!target) continue;
        let target_pos = match target {
            SpreadMoveTarget::Target(pos) => *pos,
            _ => continue,
        };

        // this.battle.activeTarget = target;
        battle.active_target = Some(target_pos);

        // damage[i] = undefined;
        result_damages[i] = DamageResult::Undefined;

        // In JavaScript, getDamage is called with moveData, which can be a SecondaryEffect.
        // SecondaryEffect has no basePower, so getDamage returns undefined early:
        //   if (!basePower) return basePower === 0 ? undefined : basePower;
        // We need to match this behavior: if hit_effect is a Secondary, skip damage calculation.
        if matches!(hit_effect, Some(HitEffect::Secondary(_))) {
            // Secondary effects have no basePower, so getDamage would return undefined
            // in JavaScript. Keep damage[i] as Undefined and continue.
            continue;
        }

        // const curDamage = this.getDamage(source, target, moveData);
        let cur_damage = crate::battle_actions::get_damage(battle, source_pos, target_pos, move_id);

        // getDamage has several possible return values:
        //
        //   a number (Some(i32)):
        //     means that much damage is dealt (0 damage still counts as dealing
        //     damage for the purposes of things like Static)
        //   None (undefined):
        //     Status moves return undefined - no damage calculation performed
        //     Move succeeds but doesn't trigger DamagingHit
        //
        // JavaScript: if (curDamage === false || curDamage === null) { damage[i] = false; }
        // JavaScript: damage[i] = curDamage;  // This can be undefined or a number
        match cur_damage {
            None => {
                // undefined - Status move, no damage dealt but move succeeds
                result_damages[i] = DamageResult::Undefined;
            }
            Some(dmg) => {
                // Numeric damage value
                result_damages[i] = DamageResult::Damage(dmg);
            }
        }
    }

    // return damage;
    result_damages
}
