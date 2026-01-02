// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Pokemon {

    /// Terastallize the Pokemon
    pub fn terastallize(&mut self) {
        if let Some(ref tera_type) = self.can_terastallize {
            self.terastallized = Some(tera_type.clone());
            self.can_terastallize = None;
        }
    }
}
