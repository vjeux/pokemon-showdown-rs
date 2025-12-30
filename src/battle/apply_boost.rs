use crate::*;

impl Battle {
    #[allow(dead_code)]
    /// Apply a stat boost
    pub fn apply_boost(&mut self, side_idx: usize, poke_idx: usize, stat: &str, amount: i8) {
        if side_idx >= self.sides.len() || poke_idx >= self.sides[side_idx].pokemon.len() {
            return;
        }

        let (name, actual_change) = {
            let pokemon = &mut self.sides[side_idx].pokemon[poke_idx];
            let old_boost = match stat {
                "atk" => pokemon.boosts.atk,
                "def" => pokemon.boosts.def,
                "spa" => pokemon.boosts.spa,
                "spd" => pokemon.boosts.spd,
                "spe" => pokemon.boosts.spe,
                _ => return,
            };

            let new_boost = (old_boost + amount).clamp(-6, 6);
            let actual_change = new_boost - old_boost;

            if actual_change == 0 {
                return;
            }

            match stat {
                "atk" => pokemon.boosts.atk = new_boost,
                "def" => pokemon.boosts.def = new_boost,
                "spa" => pokemon.boosts.spa = new_boost,
                "spd" => pokemon.boosts.spd = new_boost,
                "spe" => pokemon.boosts.spe = new_boost,
                _ => return,
            }

            (pokemon.name.clone(), actual_change)
        };

        let side_id = self.sides[side_idx].id_str();
        let full_name = format!("{}: {}", side_id, name);
        self.add_log("-boost", &[&full_name, stat, &actual_change.to_string()]);
    }
}
