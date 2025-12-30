use crate::*;

impl Pokemon {

    /// Reset for switching out
    pub fn clear_switch_state(&mut self) {
        self.is_active = false;
        self.is_started = false;
        self.clear_volatiles();
        self.clear_boosts();
        self.last_move = None;
        self.switch_flag = false;
        self.force_switch_flag = false;
        self.trapped = false;
        self.maybe_trapped = false;
        self.newly_switched = false;
        self.being_called_back = false;
        self.active_turns = 0;
        self.active_move_actions = 0;
        self.locked_move = None; // Clear Choice item lock
    }
}
