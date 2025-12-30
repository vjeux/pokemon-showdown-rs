use crate::*;

impl Dex {

    /// Calculate Base Stat Total for a species
    pub fn get_bst(&self, species_name: &str) -> Option<i32> {
        self.species().get(species_name).map(|s| {
            s.base_stats.hp
                + s.base_stats.atk
                + s.base_stats.def
                + s.base_stats.spa
                + s.base_stats.spd
                + s.base_stats.spe
        })
    }
}
