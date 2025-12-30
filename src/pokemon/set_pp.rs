use crate::*;

impl Pokemon {

    /// Set PP for a specific move (used by moves like Grudge)
    pub fn set_pp(&mut self, move_id: &ID, pp: u8) -> bool {
        if let Some(slot) = self.move_slots.iter_mut().find(|s| &s.id == move_id) {
            slot.pp = pp;
            true
        } else {
            false
        }
    }
}
