use crate::*;
use crate::battle_actions::SpreadMoveDamageValue;

impl<'a> BattleActions<'a> {

    /// Try move hit - wrapper for single/multi target moves
    /// Equivalent to battle-actions.ts tryMoveHit()
    pub fn try_move_hit_stub(
        battle: &mut crate::battle::Battle,
        target_or_targets: &[usize],
        pokemon_index: usize,
        move_id: &ID,
    ) -> Option<i32> {
        // This calls hitStepMoveHitLoop and returns damage
        let damage = Self::hit_step_move_hit_loop_stub(
            battle,
            target_or_targets,
            pokemon_index,
            move_id,
            None,
            9, // Default to gen 9
        );

        // Sum up damage
        let mut total = 0i32;
        for d in damage {
            if let SpreadMoveDamageValue::Damage(dmg) = d {
                total += dmg;
            }
        }

        if total > 0 {
            Some(total)
        } else {
            None
        }
    }
}
