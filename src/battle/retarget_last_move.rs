use crate::*;

impl Battle {

    /// Retarget the last executed move
    /// Equivalent to battle.ts retargetLastMove()
    //
    // 	retargetLastMove(newTarget: Pokemon) {
    // 		if (this.lastMoveLine < 0) return;
    // 		const parts = this.log[this.lastMoveLine].split('|');
    // 		parts[4] = newTarget.toString();
    // 		this.log[this.lastMoveLine] = parts.join('|');
    // 	}
    //
    pub fn retarget_last_move(&mut self, new_target: (usize, usize)) {
        // JS: if (this.lastMoveLine < 0) return;
        if self.last_move_line < 0 {
            return;
        }

        let line_idx = self.last_move_line as usize;
        if line_idx >= self.log.len() {
            return;
        }

        // Get the new target's string representation
        let new_target_str = if let Some(side) = self.sides.get(new_target.0) {
            if let Some(pokemon) = side.pokemon.get(new_target.1) {
                format!("{}: {}", side.id_str(), pokemon.name)
            } else {
                return;
            }
        } else {
            return;
        };

        // Split the log line and update target (part 4)
        // JS: const parts = this.log[this.lastMoveLine].split('|');
        // JS: parts[4] = newTarget.toString();
        let parts: Vec<&str> = self.log[line_idx].split('|').collect();
        let mut new_parts: Vec<String> = parts.iter().map(|s| s.to_string()).collect();
        if new_parts.len() > 4 {
            new_parts[4] = new_target_str;
        }
        self.log[line_idx] = new_parts.join("|");
    }
}
