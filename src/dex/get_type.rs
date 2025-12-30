use crate::*;
use crate::dex::TypeData;

impl Dex {

    /// Get type data by name
    pub fn get_type(&self, name: &str) -> Option<&TypeData> {
        // Types use capitalized names as keys
        self.types.get(name)
    }
}
