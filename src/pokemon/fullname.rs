use crate::*;

impl Pokemon {

    /// Get the fullname for protocol messages
    pub fn fullname(&self, side_id: &str) -> String {
        format!("{}: {}", side_id, self.name)
    }
}
