use crate::*;
use crate::dex::AbilityData;

impl Dex {

    /// Get ability by ID (equivalent to DexAbilities.getByID)
    pub fn get_ability_by_id(&self, id: &ID) -> Option<&AbilityData> {
        self.abilities.get(id)
    }
}
