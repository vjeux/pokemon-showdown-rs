use crate::*;
use crate::battle_actions::ActiveMove;

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
    pub fn move_used(&mut self, battle: &Battle, active_move: &ActiveMove, target_loc: Option<i8>) {
        let move_id = active_move.id.clone();

        // JS: this.lastMove = move;
        // Note: JS stores the entire ActiveMove object, Rust stores just the ID
        self.last_move = Some(move_id.clone());

        // JS: if (this.battle.gen === 2) this.lastMoveEncore = move;
        // ✅ NOW IMPLEMENTED: Gen 2 Encore tracking
        if battle.gen == 2 {
            self.last_move_encore = Some(move_id.clone());
        }

        // JS: this.lastMoveUsed = move;
        // Store the full ActiveMove with runtime type modifications
        self.last_move_used = Some(Box::new(active_move.clone()));

        // JS: this.lastMoveTargetLoc = targetLoc;
        self.last_move_target_loc = target_loc;

        // JS: this.moveThisTurn = move.id;
        self.move_this_turn = Some(move_id);
    }
}
