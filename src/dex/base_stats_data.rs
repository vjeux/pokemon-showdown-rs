//! Base stats as stored in JSON

use serde::{Deserialize, Serialize};

use crate::dex_data::StatsTable;

/// Base stats as stored in JSON
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// JavaScript equivalent: StatsTable (sim/global-types.ts)
/// 6 fields in JavaScript
pub struct BaseStatsData {
    /// Hit Points
    /// JavaScript: hp: number
    pub hp: i32,
    /// Attack
    /// JavaScript: atk: number
    pub atk: i32,
    /// Defense
    /// JavaScript: def: number
    pub def: i32,
    /// Special Attack
    /// JavaScript: spa: number
    pub spa: i32,
    /// Special Defense
    /// JavaScript: spd: number
    pub spd: i32,
    /// Speed
    /// JavaScript: spe: number
    pub spe: i32,
}

impl From<BaseStatsData> for StatsTable {
    fn from(data: BaseStatsData) -> Self {
        StatsTable {
            hp: data.hp,
            atk: data.atk,
            def: data.def,
            spa: data.spa,
            spd: data.spd,
            spe: data.spe,
        }
    }
}
