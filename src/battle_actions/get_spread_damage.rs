//! BattleActions::getSpreadDamage - Get damage for each target in a spread move
//!
//! 1:1 port of getSpreadDamage from battle-actions.ts

use crate::*;
use crate::battle_actions::{SpreadMoveDamage, SpreadMoveDamageValue, SpreadMoveTargets, SpreadMoveTarget};

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
pub fn get_spread_damage(
    battle: &mut Battle,
    damages: SpreadMoveDamage,
    targets: &SpreadMoveTargets,
    source_pos: (usize, usize),
    move_id: &ID,
    is_secondary: bool,
    is_self: bool,
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
        result_damages[i] = SpreadMoveDamageValue::Undefined;

        // const curDamage = this.getDamage(source, target, moveData);
        let cur_damage = crate::battle_actions::get_damage(battle, source_pos, target_pos, move_id);

        // getDamage has several possible return values:
        //
        //   a number (Some(i32)):
        //     means that much damage is dealt (0 damage still counts as dealing
        //     damage for the purposes of things like Static)
        //   None:
        //     the move fails
        //
        // if (curDamage === false || curDamage === null) {
        if cur_damage.is_none() {
            // if (damage[i] === false && !isSecondary && !isSelf) {
            //   this.battle.add('-fail', source);
            //   this.battle.attrLastMove('[still]');
            // }
            if matches!(result_damages[i], SpreadMoveDamageValue::Failed) && !is_secondary && !is_self {
                let source_slot = if let Some(side) = battle.sides.get(source_pos.0) {
                    if let Some(pokemon) = side.pokemon.get(source_pos.1) {
                        pokemon.get_slot()
                    } else {
                        String::new()
                    }
                } else {
                    String::new()
                };

                battle.add("-fail", &[battle::Arg::String(source_slot)]);
                battle.attr_last_move(&["[still]"]);
            }

            // this.battle.debug('damage calculation interrupted');
            eprintln!("[GET_SPREAD_DAMAGE] damage calculation interrupted for target {}", i);

            // damage[i] = false;
            result_damages[i] = SpreadMoveDamageValue::Failed;

            // continue;
            continue;
        }

        // damage[i] = curDamage;
        result_damages[i] = SpreadMoveDamageValue::Damage(cur_damage.unwrap());
    }

    // return damage;
    result_damages
}
