use crate::*;

impl Dex {

    /// Get ability description
    pub fn get_ability_desc(&self, ability_name: &str) -> Option<&str> {
        self.get_ability(ability_name)
            .and_then(|a| a.desc.as_deref())
    }
}
