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
        // TODO: implement the same logic as JavaScript
        self.last_move = Some(move_id.clone());
        self.last_move_used = Some(move_id.clone());
        self.last_move_target_loc = target_loc;
        self.move_this_turn = Some(move_id);
    }
}
