use crate::*;

impl Battle {

    /// Try to hit targets with a spread move
    /// Equivalent to trySpreadMoveHit() in battle-actions.ts:545
    ///
    /// This is the main entry point for move execution with the 7-step pipeline
    pub fn try_spread_move_hit(
        &mut self,
        targets: &[(usize, usize)],
        pokemon_pos: (usize, usize),
        move_id: &ID,
    ) -> bool {
        if targets.is_empty() {
            return false;
        }

        // For now, implement a simplified version that just calls spreadMoveHit
        // The full implementation would have the 7-step pipeline

        let target_list: Vec<Option<(usize, usize)>> = targets.iter().map(|&t| Some(t)).collect();

        let (damages, final_targets) =
            crate::battle_actions::spread_move_hit(self, &target_list, pokemon_pos, move_id, false, false);

        // Check if any target was hit
        for (i, damage) in damages.iter().enumerate() {
            if let Some(dmg) = damage {
                if *dmg != 0 || final_targets.get(i).and_then(|t| *t).is_some() {
                    return true;
                }
            }
        }

        false
    }
}
