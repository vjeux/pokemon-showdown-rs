use crate::*;

impl Pokemon {

    /// Record that this Pokemon was attacked
    //
    // 	gotAttacked(move: string | Move, damage: number | false | undefined, source: Pokemon) {
    // 		const damageNumber = (typeof damage === 'number') ? damage : 0;
    // 		move = this.battle.dex.moves.get(move);
    // 		this.attackedBy.push({
    // 			source,
    // 			damage: damageNumber,
    // 			move: move.id,
    // 			thisTurn: true,
    // 			slot: source.getSlot(),
    // 			damageValue: damage,
    // 		});
    // 	}
    //
    pub fn got_attacked(
        &mut self,
        move_id: ID,
        damage: i32,
        _source_side: usize,
        _source_pos: usize,
    ) {
        self.last_damage = damage;
        self.times_attacked += 1;
        // Would store in attackedBy array in full implementation
        let _ = move_id; // Use to avoid warning
    }
}
