use crate::side::*;
use crate::*;

impl Side {

    /// Check if side has lost
    pub fn has_lost(&self) -> bool {
        self.count_unfainted() == 0
    }
}
