use crate::*;

impl Dex {

    /// Get ability short description
    pub fn get_ability_short_desc(&self, ability_name: &str) -> Option<&str> {
        self.get_ability(ability_name)
            .and_then(|a| a.short_desc.as_deref())
    }
}
