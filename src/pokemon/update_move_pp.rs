// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Pokemon {
    /// Update PP values for all move slots based on move data from Dex
    pub fn update_move_pp(&mut self, dex: &Dex, gen: u8) {
        for slot in &mut self.move_slots {
            if let Some(move_data) = dex.moves().get(slot.id.as_str()) {
                // Calculate base PP
                // JavaScript: let basepp = move.noPPBoosts ? move.pp : move.pp * 8 / 5;
                let base_pp = if move_data.no_pp_boosts {
                    move_data.pp
                } else {
                    (move_data.pp * 8) / 5
                };

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
                // JavaScript: let basepp = move.noPPBoosts ? move.pp : move.pp * 8 / 5;
                let base_pp = if move_data.no_pp_boosts {
                    move_data.pp
                } else {
                    (move_data.pp * 8) / 5
                };

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
