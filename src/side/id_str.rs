use crate::side::*;

impl Side {

    /// Get the Side ID as a string
    pub fn id_str(&self) -> &'static str {
        self.id.to_str()
    }
}
