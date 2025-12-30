use crate::*;

impl<'a> BattleActions<'a> {

    /// Try spread move hit - checks all targets
    /// Equivalent to trySpreadMoveHit in battle-actions.ts
    pub fn try_spread_move_hit_check(target_count: usize, move_target_type: &str) -> bool {
        // Check if this is a spread move
        let is_spread = matches!(
            move_target_type,
            "allAdjacent" | "allAdjacentFoes" | "all" | "foeSide" | "allySide"
        );

        is_spread && target_count > 0
    }
}
