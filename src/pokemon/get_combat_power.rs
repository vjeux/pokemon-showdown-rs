use crate::*;

impl Pokemon {

    /// Get combat power (for Pokemon Go style formats)
    /// Equivalent to getCombatPower in pokemon.ts
    pub fn get_combat_power(&self) -> i32 {
        // Simplified formula based on stats
        let atk = self.stored_stats.atk;
        let def = self.stored_stats.def;
        let sta = self.stored_stats.hp.max(10); // Use HP as stamina proxy

        (((atk as f64) * (def as f64).powf(0.5) * (sta as f64).powf(0.5)) / 10.0) as i32
    }
}
