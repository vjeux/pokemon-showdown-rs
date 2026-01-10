use crate::*;
use crate::pokemon::Attacker;
use crate::battle_actions::ActiveMove;

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
        active_move: &ActiveMove,
        damage: i32,
        source_side: usize,
        source_pos: usize,
    ) {
        let move_id = active_move.id.clone();

        // JS: this.attackedBy.push({ source, damage, move: move.id, thisTurn: true, ... })
        self.attacked_by.push(Attacker {
            source: (source_side, source_pos),
            damage,
            this_turn: true,
            move_id: Some(move_id),
            slot: (source_side, source_pos), // Same as source for tracking
            damage_value: Some(damage), // JavaScript: damageValue: damage
        });

        // NOTE: timesAttacked is NOT incremented here!
        // In JavaScript, gotAttacked() only pushes to attackedBy.
        // The timesAttacked increment happens separately in moveHitLoop.
    }
}
