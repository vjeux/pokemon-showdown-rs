use crate::*;

impl Pokemon {

    /// Apply a boost
    pub fn boost(&mut self, boost_id: crate::dex_data::BoostID, amount: i8) -> i8 {
        self.boosts.boost(boost_id, amount)
    }
}
