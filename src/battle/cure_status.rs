use crate::*;

impl Battle {

    /// Cure status condition of a pokemon
    /// Matches JavaScript pokemon.ts cureStatus(silent = false)
    ///
    pub fn cure_status(&mut self, target: (usize, usize)) -> bool {
        let (side_idx, poke_idx) = target;

        // JS: if (!this.hp || !this.status) return false;
        let (has_hp, has_status, status, name) = if let Some(side) = self.sides.get(side_idx) {
            if let Some(pokemon) = side.pokemon.get(poke_idx) {
                (
                    pokemon.hp > 0,
                    !pokemon.status.is_empty(),
                    pokemon.status.as_str().to_string(),
                    pokemon.name.clone(),
                )
            } else {
                return false;
            }
        } else {
            return false;
        };

        if !has_hp || !has_status {
            return false;
        }

        // JS: this.battle.add('-curestatus', this, this.status, silent ? '[silent]' : '[msg]');
        let side_id = self.sides[side_idx].id_str();
        let full_name = format!("{}: {}", side_id, name);
        self.add("-curestatus", &[full_name.as_str().into(), status.as_str().into(), "[msg]".into()]);

        // JS: if (this.status === 'slp' && this.removeVolatile('nightmare')) { ... }
        if status == "slp" {
            let removed_nightmare = if let Some(side) = self.sides.get_mut(side_idx) {
                if let Some(pokemon) = side.pokemon.get_mut(poke_idx) {
                    pokemon.volatiles.remove(&ID::new("nightmare")).is_some()
                } else {
                    false
                }
            } else {
                false
            };

            if removed_nightmare {
                self.add("-end", &[full_name.as_str().into(), "Nightmare".into(), "[silent]".into()]);
            }
        }

        // JS: this.setStatus('');
        if let Some(side) = self.sides.get_mut(side_idx) {
            if let Some(pokemon) = side.pokemon.get_mut(poke_idx) {
                pokemon.set_status(ID::new(""));
                pokemon.status_state.duration = None;
            }
        }

        // JS: return true;
        true
    }
}
