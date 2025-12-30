use crate::*;

impl Pokemon {

    /// Clear all turn state at end of turn
    pub fn clear_turn_state_full(&mut self) {
        self.move_last_turn_result = self.move_this_turn_result;
        self.move_this_turn = None;
        self.move_this_turn_result = None;
        self.hurt_this_turn = None;
        self.used_item_this_turn = false;
        self.stats_raised_this_turn = false;
        self.stats_lowered_this_turn = false;
    }
}
