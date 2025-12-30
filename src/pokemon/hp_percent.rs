use crate::*;

impl Pokemon {

    /// Get current HP percentage
    pub fn hp_percent(&self) -> f64 {
        if self.maxhp == 0 {
            return 0.0;
        }
        (self.hp as f64 / self.maxhp as f64) * 100.0
    }
}
