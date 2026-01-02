use crate::*;

impl Pokemon {

    /// Deduct PP for a move
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
    pub fn deduct_pp(&mut self, move_id: &ID, amount: u8) -> bool {
        // TODO: implement the same logic as JavaScript
        if let Some(slot) = self.move_slots.iter_mut().find(|s| &s.id == move_id) {
            slot.pp = slot.pp.saturating_sub(amount);
            slot.used = true;
            true
        } else {
            false
        }
    }
}
