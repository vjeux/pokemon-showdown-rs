use crate::*;

impl Battle {
    #[allow(dead_code)]
    /// Apply confusion volatile to a Pokemon
    pub fn apply_confusion(&mut self, side_idx: usize, poke_idx: usize) {
        if side_idx >= self.sides.len() || poke_idx >= self.sides[side_idx].pokemon.len() {
            return;
        }

        let confusion_id = ID::new("confusion");
        if self.sides[side_idx].pokemon[poke_idx].has_volatile(&confusion_id) {
            return; // Already confused
        }

        // Confusion lasts 2-5 turns
        let duration = 2 + self.random(4);
        self.sides[side_idx].pokemon[poke_idx].add_volatile(confusion_id.clone());
        if let Some(state) = self.sides[side_idx].pokemon[poke_idx].get_volatile_mut(&confusion_id)
        {
            state.duration = Some(duration);
        }

        let name = {
            let side_id = self.sides[side_idx].id_str();
            let pokemon = &self.sides[side_idx].pokemon[poke_idx];
            format!("{}: {}", side_id, pokemon.name)
        };
        self.add("-start", &[name.into(), "confusion".into()]);
    }
}
