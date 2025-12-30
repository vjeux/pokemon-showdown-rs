use crate::*;
use crate::battle_actions::SpreadMoveDamageValue;

impl<'a> BattleActions<'a> {

    /// Move hit - single target wrapper for spreadMoveHit
    /// Equivalent to battle-actions.ts moveHit()
    pub fn move_hit_stub(
        target_index: Option<usize>,
        pokemon_index: usize,
        move_id: &ID,
        is_secondary: bool,
        is_self: bool,
    ) -> Option<i32> {
        let targets = match target_index {
            Some(idx) => vec![idx],
            None => vec![],
        };

        if targets.is_empty() {
            return None;
        }

        let (damage, _) =
            Self::spread_move_hit_stub(&targets, pokemon_index, move_id, is_secondary, is_self);

        match damage.first() {
            Some(SpreadMoveDamageValue::Damage(d)) => Some(*d),
            Some(SpreadMoveDamageValue::Success) => None,
            _ => None,
        }
    }
}
