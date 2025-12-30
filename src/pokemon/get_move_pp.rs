use crate::*;

impl Pokemon {

    /// Get move PP for a move
    pub fn get_move_pp(&self, move_id: &ID) -> Option<u8> {
        self.move_slots
            .iter()
            .find(|slot| &slot.id == move_id)
            .map(|slot| slot.pp)
    }
}
