use crate::*;

impl Battle {

    /// Execute a move
    pub fn run_move(&mut self, side_idx: usize, poke_idx: usize, move_id: &ID, target_loc: i8) {
        if side_idx >= self.sides.len() {
            return;
        }

        // Check if Pokemon can still move
        let can_act = {
            let pokemon = &self.sides[side_idx].pokemon[poke_idx];
            !pokemon.is_fainted() && pokemon.is_active
        };

        if !can_act {
            return;
        }

        // Get target side and Pokemon
        let (target_side_idx, target_poke_idx) = self.get_move_target(side_idx, target_loc);

        // Log the move use
        let (attacker_name, move_name) = {
            let side_id = self.sides[side_idx].id_str();
            let pokemon = &self.sides[side_idx].pokemon[poke_idx];
            (
                format!("{}: {}", side_id, pokemon.name),
                move_id.as_str().to_string(),
            )
        };
        self.add_log("move", &[&attacker_name, &move_name]);

        // Deduct PP
        self.sides[side_idx].pokemon[poke_idx].deduct_pp(move_id, 1);

        // Execute the move using battle_actions module
        // This calls the line-by-line port of battle-actions.ts useMove()
        crate::battle_actions::use_move(
            self,
            move_id,
            (side_idx, poke_idx),
            Some((target_side_idx, target_poke_idx)),
            None, // source_effect
            None, // z_move
            None, // max_move
        );
    }
}
