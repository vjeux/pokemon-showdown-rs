use crate::*;
use crate::dex_data::StatsTable;

impl Dex {

    /// Get base stats for a species
    pub fn get_base_stats(&self, species_name: &str) -> Option<StatsTable> {
        self.species().get(species_name)
            .map(|s| s.base_stats.clone().into())
    }
}
