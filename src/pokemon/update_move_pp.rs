use crate::*;

impl Pokemon {
    /// Update PP values for all move slots based on move data from Dex
    /// JavaScript source:
    // for (const moveid of this.set.moves) {
    //     let move = this.battle.dex.moves.get(moveid);
    //     let basepp = move.noPPBoosts ? move.pp : move.pp * 8 / 5;
    //     if (this.battle.gen < 3) basepp = Math.min(61, basepp);
    //     this.baseMoveSlots.push({
    //         move: move.name,
    //         id: move.id,
    //         pp: basepp,
    //         maxpp: basepp,
    //         ...
    //     });
    // }
    pub fn update_move_pp(&mut self, dex: &Dex, gen: u8) {
        for slot in &mut self.move_slots {
            if let Some(move_data) = dex.moves().get(slot.id.as_str()) {
                // Calculate base PP: move.pp * 8 / 5
                // TODO: Handle noPPBoosts field when it's added to MoveData
                let base_pp = (move_data.pp * 8) / 5;

                // For gen < 3, cap at 61
                let base_pp = if gen < 3 {
                    base_pp.min(61)
                } else {
                    base_pp
                };

                slot.pp = base_pp as u8;
                slot.maxpp = base_pp as u8;
            }
        }

        // Also update base_move_slots
        for slot in &mut self.base_move_slots {
            if let Some(move_data) = dex.moves().get(slot.id.as_str()) {
                let base_pp = (move_data.pp * 8) / 5;
                let base_pp = if gen < 3 {
                    base_pp.min(61)
                } else {
                    base_pp
                };

                slot.pp = base_pp as u8;
                slot.maxpp = base_pp as u8;
            }
        }
    }
}
