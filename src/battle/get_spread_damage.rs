use crate::*;

impl Battle {

    /// Get damage for each target in a spread move
    /// Equivalent to getSpreadDamage() in battle-actions.ts:1163
    pub fn get_spread_damage(
        &mut self,
        damages: &[Option<i32>],
        targets: &[Option<(usize, usize)>],
        source_pos: (usize, usize),
        move_id: &ID,
        _is_secondary: bool,
        _is_self: bool,
    ) -> Vec<Option<i32>> {
        let mut result_damages = damages.to_vec();

        for (i, &target) in targets.iter().enumerate() {
            if let Some(target_pos) = target {
                // Calculate damage using getDamage
                let cur_damage = crate::battle_actions::get_damage(self, source_pos, target_pos, move_id);
                result_damages[i] = cur_damage;
            } else {
                result_damages[i] = None;
            }
        }

        result_damages
    }
}
