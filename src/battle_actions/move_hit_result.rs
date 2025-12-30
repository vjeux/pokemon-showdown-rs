use crate::*;
use crate::battle_actions::MoveHitData;

impl<'a> BattleActions<'a> {

    /// Move hit - execute the actual hit
    /// Equivalent to moveHit in battle-actions.ts
    pub fn move_hit_result(damage: i32, type_effectiveness: f64, is_crit: bool) -> MoveHitData {
        MoveHitData {
            crit: is_crit,
            type_mod: (type_effectiveness * 100.0) as i32 - 100,
            damage,
            z_broke_protect: false,
        }
    }
}
