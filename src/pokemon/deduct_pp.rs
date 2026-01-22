// JS Source:
//
// 	deductPP(move: string | Move, amount?: number | null, target?: Pokemon | null | false) {
// 		const gen = this.battle.gen;
// 		move = this.battle.dex.moves.get(move);
// 		const ppData = this.getMoveData(move);
// 		if (!ppData) return 0;
// 		ppData.used = true;
// 		if (!ppData.pp && gen > 1) return 0;
//
// 		if (!amount) amount = 1;
// 		ppData.pp -= amount;
// 		if (ppData.pp < 0 && gen > 1) {
// 			amount += ppData.pp;
// 			ppData.pp = 0;
// 		}
// 		return amount;
// 	}
//
// Note: In Rust, we pass gen as a parameter instead of Battle to avoid borrow conflicts.
// Target parameter is unused in JS so we omit it.

use crate::*;
use crate::battle_actions::ActiveMove;

impl Pokemon {
    /// Deduct PP for a move
    /// Equivalent to pokemon.ts deductPP()
    /// Returns the actual amount deducted
    pub fn deduct_pp(&mut self, gen: u8, active_move: &ActiveMove, amount: Option<u8>) -> u8 {
        let move_id = &active_move.id;

        // JS: const ppData = this.getMoveData(move);
        // JS: if (!ppData) return 0;
        let slot_index = match self.move_slots.iter().position(|s| &s.id == move_id) {
            Some(idx) => idx,
            None => return 0,
        };

        // Debug logging
        let _old_pp = self.move_slots[slot_index].pp;
        debug_elog!("[DEDUCT_PP] Pokemon={}, move={}, old_pp={}, amount={:?}",
            self.species_id.as_str(), move_id.as_str(), _old_pp, amount);

        // JS: ppData.used = true;
        self.move_slots[slot_index].used = true;

        // JS: if (!ppData.pp && gen > 1) return 0;
        if self.move_slots[slot_index].pp == 0 && gen > 1 {
            debug_elog!("[DEDUCT_PP] PP already 0, not deducting");
            return 0;
        }

        // JS: if (!amount) amount = 1;
        let mut amount = amount.unwrap_or(1);

        // JS: ppData.pp -= amount;
        let current_pp = self.move_slots[slot_index].pp;
        let new_pp = current_pp.wrapping_sub(amount);
        self.move_slots[slot_index].pp = new_pp;

        // JS: In JavaScript, baseMoveSlots and moveSlots share the same MoveSlot objects
        // (until Transform is used), so PP changes affect both arrays.
        // In Rust, they're separate clones, so we need to sync manually.
        // Find the same move in base_move_slots and update its PP too.
        if let Some(base_slot_idx) = self.base_move_slots.iter().position(|s| &s.id == move_id) {
            self.base_move_slots[base_slot_idx].pp = new_pp;
            self.base_move_slots[base_slot_idx].used = true;
        }

        // JS: if (ppData.pp < 0 && gen > 1) {
        //         amount += ppData.pp;
        //         ppData.pp = 0;
        //     }
        // Note: In Rust, pp is u8 so we need to check if subtraction would go negative
        if gen > 1 && new_pp > current_pp {
            // Wraparound occurred (went negative)
            amount = current_pp; // Actual amount deducted is just current_pp
            self.move_slots[slot_index].pp = 0;
            // Also sync to base_move_slots
            if let Some(base_slot_idx) = self.base_move_slots.iter().position(|s| &s.id == move_id) {
                self.base_move_slots[base_slot_idx].pp = 0;
            }
        }

        debug_elog!("[DEDUCT_PP] new_pp={}, actual_amount={}", self.move_slots[slot_index].pp, amount);

        // JS: return amount;
        amount
    }
}
