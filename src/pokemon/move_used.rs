use crate::*;

impl Pokemon {

    /// Record that a move was used
    //
    // 	moveUsed(move: ActiveMove, targetLoc?: number) {
    // 		this.lastMove = move;
    // 		if (this.battle.gen === 2) this.lastMoveEncore = move;
    // 		this.lastMoveTargetLoc = targetLoc;
    // 		this.moveThisTurn = move.id;
    // 	}
    //
    pub fn move_used(&mut self, move_id: ID, target_loc: Option<i8>) {
        // JS: this.lastMove = move;
        // Note: JS stores the entire ActiveMove object, Rust stores just the ID
        self.last_move = Some(move_id.clone());

        // Note: last_move_used field not in JavaScript - Rust-specific tracking
        self.last_move_used = Some(move_id.clone());

        // JS: if (this.battle.gen === 2) this.lastMoveEncore = move;
        // Note: lastMoveEncore tracking not implemented - would need gen check

        // JS: this.lastMoveTargetLoc = targetLoc;
        self.last_move_target_loc = target_loc;

        // JS: this.moveThisTurn = move.id;
        self.move_this_turn = Some(move_id);
    }
}
