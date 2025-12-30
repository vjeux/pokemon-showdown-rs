use crate::*;

impl Battle {

    /// Helper to add direct damage log messages
    /// Rust helper method - JavaScript has this logic inline in directDamage() method (battle.ts:2217-2226)
    /// Extracted for borrow checker compatibility
    pub fn add_direct_damage_log(&mut self, target: (usize, usize), effect: Option<&ID>) {
        let (side_idx, poke_idx) = target;

        // Get target health string
        let health_str = if let Some(side) = self.sides.get(side_idx) {
            if let Some(pokemon) = side.pokemon.get(poke_idx) {
                format!("{}/{}", pokemon.hp, pokemon.maxhp)
            } else {
                return;
            }
        } else {
            return;
        };

        let target_str = format!("p{}a", side_idx + 1);
        let effect_id = effect.map(|e| e.as_str()).unwrap_or("");

        // Special case handling
        match effect_id {
            "strugglerecoil" => {
                self.add_log("-damage", &[&target_str, &health_str, "[from] recoil"]);
            }
            "confusion" => {
                self.add_log("-damage", &[&target_str, &health_str, "[from] confusion"]);
            }
            _ => {
                self.add_log("-damage", &[&target_str, &health_str]);
            }
        }
    }
}
