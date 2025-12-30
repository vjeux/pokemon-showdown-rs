use crate::*;

impl Battle {

    /// Get the target for a move based on target_loc
    pub fn get_move_target(&self, side_idx: usize, target_loc: i8) -> (usize, usize) {
        // In singles, target_loc is typically 0 (auto-target foe) or specific
        // Positive = foe position, Negative = ally position
        let foe_idx = if side_idx == 0 { 1 } else { 0 };

        if target_loc <= 0 {
            // Default to first active foe
            let target_poke_idx = self
                .sides
                .get(foe_idx)
                .and_then(|s| s.active.first())
                .and_then(|opt| *opt)
                .unwrap_or(0);
            (foe_idx, target_poke_idx)
        } else {
            // Specific target position
            let slot = (target_loc.abs() - 1) as usize;
            let target_poke_idx = self
                .sides
                .get(foe_idx)
                .and_then(|s| s.active.get(slot))
                .and_then(|opt| *opt)
                .unwrap_or(0);
            (foe_idx, target_poke_idx)
        }
    }
}
