use crate::*;
use crate::battle::BattleRequestState;

impl Battle {

    /// Parse a choice string and store the actions
    /// Rust helper - JavaScript has side.choose() which handles parsing
    /// This is split out in Rust to work around borrow checker constraints
    pub fn parse_choice(&mut self, side_id: SideID, choice: &str) {
        let side_idx = side_id.index();
        if side_idx >= self.sides.len() {
            return;
        }

        // Clear previous choice
        self.sides[side_idx].choice.clear();

        // Parse the choice string
        let parts: Vec<&str> = choice.split(',').map(|s| s.trim()).collect();

        for (slot, part) in parts.iter().enumerate() {
            if slot >= self.active_per_half {
                break;
            }

            let words: Vec<&str> = part.split_whitespace().collect();
            if words.is_empty() {
                continue;
            }

            let action = match words[0] {
                "move" => {
                    let move_id = if words.len() > 1 {
                        // Could be a move name or number
                        if let Ok(num) = words[1].parse::<usize>() {
                            // Move by number (1-indexed)
                            if let Some(Some(poke_idx)) = self.sides[side_idx].active.get(slot) {
                                if let Some(pokemon) = self.sides[side_idx].pokemon.get(*poke_idx) {
                                    if num > 0 && num <= pokemon.move_slots.len() {
                                        let selected_move = pokemon.move_slots[num - 1].id.clone();
                                        Some(selected_move)
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        } else {
                            Some(ID::new(words[1]))
                        }
                    } else {
                        None
                    };

                    // Check Choice item lock - if locked, override with locked move
                    let move_id =
                        if let Some(Some(poke_idx)) = self.sides[side_idx].active.get(slot) {
                            if let Some(pokemon) = self.sides[side_idx].pokemon.get(*poke_idx) {
                                if let Some(ref locked) = pokemon.locked_move {
                                    Some(locked.clone())
                                } else {
                                    move_id
                                }
                            } else {
                                move_id
                            }
                        } else {
                            move_id
                        };

                    // Parse target if present
                    let target_loc = if words.len() > 2 {
                        words[2].parse::<i8>().ok()
                    } else {
                        None
                    };

                    // Check for mega/zmove flags
                    let mega = words.contains(&"mega");
                    let terastallize = if words.contains(&"terastallize") {
                        self.sides[side_idx]
                            .get_active(slot)
                            .and_then(|p| p.tera_type.clone())
                    } else {
                        None
                    };

                    crate::side::ChosenAction {
                        choice: crate::side::ChoiceType::Move,
                        pokemon_index: slot,
                        target_loc,
                        move_id,
                        switch_index: None,
                        mega,
                        zmove: None,
                        max_move: None,
                        terastallize,
                    }
                }
                "switch" => {
                    let switch_idx = if words.len() > 1 {
                        words[1].parse::<usize>().ok().map(|n| n.saturating_sub(1))
                    } else {
                        None
                    };

                    crate::side::ChosenAction {
                        choice: crate::side::ChoiceType::Switch,
                        pokemon_index: slot,
                        target_loc: None,
                        move_id: None,
                        switch_index: switch_idx,
                        mega: false,
                        zmove: None,
                        max_move: None,
                        terastallize: None,
                    }
                }
                "default" | "auto" => {
                    // JS: case "auto": case "default": this.autoChoose(); break;
                    // autoChoose() calls chooseTeam() if requestState === "teampreview"
                    // or chooseMove() if requestState === "move"
                    // or chooseSwitch() if requestState === "switch"
                    match self.request_state {
                        BattleRequestState::TeamPreview => {
                            // JS: if (this.requestState === "teampreview") { if (!this.isChoiceDone()) this.chooseTeam(); }
                            crate::side::ChosenAction {
                                choice: crate::side::ChoiceType::Team,
                                pokemon_index: slot,
                                target_loc: None,
                                move_id: None,
                                switch_index: None,
                                mega: false,
                                zmove: None,
                                max_move: None,
                                terastallize: None,
                            }
                        }
                        BattleRequestState::Move => {
                            // JS: } else if (this.requestState === "move") {
                            //       while (!this.isChoiceDone()) {
                            //         if (!this.chooseMove()) throw new Error(`autoChoose crashed: ${this.choice.error}`);
                            // chooseMove() with no arguments chooses the first available move

                            // Get the first available move
                            let move_id: Option<ID> = {
                                let side = &self.sides[side_idx];
                                // side.active[slot] is Option<usize> - index into side.pokemon
                                if let Some(&Some(pokemon_idx)) = side.active.get(slot) {
                                    if let Some(pokemon) = side.pokemon.get(pokemon_idx) {
                                        if let Some(first_move_slot) = pokemon.move_slots.first() {
                                            Some(first_move_slot.id.clone())
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            };

                            if let Some(move_id) = move_id {
                                crate::side::ChosenAction {
                                    choice: crate::side::ChoiceType::Move,
                                    pokemon_index: slot,
                                    target_loc: None,
                                    move_id: Some(move_id),
                                    switch_index: None,
                                    mega: false,
                                    zmove: None,
                                    max_move: None,
                                    terastallize: None,
                                }
                            } else {
                                crate::side::ChosenAction {
                                    choice: crate::side::ChoiceType::Pass,
                                    pokemon_index: slot,
                                    target_loc: None,
                                    move_id: None,
                                    switch_index: None,
                                    mega: false,
                                    zmove: None,
                                    max_move: None,
                                    terastallize: None,
                                }
                            }
                        }
                        BattleRequestState::Switch => {
                            // JS: } else if (this.requestState === "switch") {
                            //       while (!this.isChoiceDone()) {
                            //         if (!this.chooseSwitch()) throw new Error(`autoChoose switch crashed: ${this.choice.error}`);
                            // TODO: Implement auto-switch
                            crate::side::ChosenAction {
                                choice: crate::side::ChoiceType::Pass,
                                pokemon_index: slot,
                                target_loc: None,
                                move_id: None,
                                switch_index: None,
                                mega: false,
                                zmove: None,
                                max_move: None,
                                terastallize: None,
                            }
                        }
                        _ => {
                            crate::side::ChosenAction {
                                choice: crate::side::ChoiceType::Pass,
                                pokemon_index: slot,
                                target_loc: None,
                                move_id: None,
                                switch_index: None,
                                mega: false,
                                zmove: None,
                                max_move: None,
                                terastallize: None,
                            }
                        }
                    }
                }
                _ => crate::side::ChosenAction {
                    choice: crate::side::ChoiceType::Pass,
                    pokemon_index: slot,
                    target_loc: None,
                    move_id: None,
                    switch_index: None,
                    mega: false,
                    zmove: None,
                    max_move: None,
                    terastallize: None,
                },
            };

            self.sides[side_idx].choice.actions.push(action);
        }
    }
}
