use crate::*;
use std::collections::HashMap;

impl Dex {

    /// Convert HashMap boosts to BoostsTable struct
    pub fn convert_boosts_hash_to_table(boosts: &HashMap<String, i32>) -> crate::dex_data::BoostsTable {
        crate::dex_data::BoostsTable {
            atk: boosts.get("atk").copied().unwrap_or(0) as i8,
            def: boosts.get("def").copied().unwrap_or(0) as i8,
            spa: boosts.get("spa").copied().unwrap_or(0) as i8,
            spd: boosts.get("spd").copied().unwrap_or(0) as i8,
            spe: boosts.get("spe").copied().unwrap_or(0) as i8,
            accuracy: boosts.get("accuracy").copied().unwrap_or(0) as i8,
            evasion: boosts.get("evasion").copied().unwrap_or(0) as i8,
        }
    }
}
