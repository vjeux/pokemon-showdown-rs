use crate::*;
use crate::dex_data::BoostsTable;

impl Pokemon {

    /// Clear volatile with switch flag handling
    /// Equivalent to clearVolatile in pokemon.ts
    pub fn clear_volatile_full(&mut self, include_switch_flags: bool) {
        self.volatiles.clear();
        self.boosts = BoostsTable::default();

        if include_switch_flags {
            self.switch_flag = false;
            self.force_switch_flag = false;
        }

        self.last_move = None;
        self.last_move_used = None;
        self.move_this_turn = None;
    }
}
