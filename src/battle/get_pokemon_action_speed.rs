use crate::*;

impl Battle {

    /// Get a Pokemon's action speed (called by pokemon.getActionSpeed() in JS)
    /// This is the helper method for getting base Pokemon speed
    pub fn get_pokemon_action_speed(&self, side_idx: usize, poke_idx: usize) -> i32 {
        if let Some(side) = self.sides.get(side_idx) {
            if let Some(pokemon) = side.pokemon.get(poke_idx) {
                // Apply speed boosts
                let base_speed = pokemon.stored_stats.spe;
                let stage = pokemon.boosts.spe;
                let multiplier = if stage >= 0 {
                    (2 + stage as i32) as f64 / 2.0
                } else {
                    2.0 / (2 + (-stage) as i32) as f64
                };
                return (base_speed as f64 * multiplier) as i32;
            }
        }
        0
    }
}
