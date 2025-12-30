use crate::*;
use crate::dex::NatureData;

impl Dex {

    /// Get nature data by name or ID
    pub fn get_nature(&self, name: &str) -> Option<&NatureData> {
        let id = ID::new(name);
        self.natures.get(&id)
    }
}
