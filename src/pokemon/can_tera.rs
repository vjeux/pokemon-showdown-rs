use crate::*;

impl Pokemon {

    /// Check if Pokemon can terastallize
    pub fn can_tera(&self) -> bool {
        self.terastallized.is_none() && self.can_terastallize.is_some()
    }
}
