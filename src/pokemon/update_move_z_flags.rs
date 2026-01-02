// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Pokemon {
    /// Update is_z flags for all move slots based on move data from Dex
    pub fn update_move_z_flags(&mut self, dex: &Dex) {
        for slot in &mut self.move_slots {
            if let Some(move_data) = dex.moves().get(slot.id.as_str()) {
                // is_z in MoveData is Option<String> indicating the Z-move type
                slot.is_z = move_data.is_z.is_some();
            }
        }
        // Also update base_move_slots
        for slot in &mut self.base_move_slots {
            if let Some(move_data) = dex.moves().get(slot.id.as_str()) {
                slot.is_z = move_data.is_z.is_some();
            }
        }
    }
}
