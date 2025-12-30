use crate::*;
use crate::battle::BattleRequestState;

impl Battle {

    /// Validate a single choice (not comma-separated)
    /// pokemon_index: Which active slot this choice is for (0-2 in triples, 0-1 in doubles, 0 in singles)
    pub fn validate_single_choice(
        &mut self,
        side_id: SideID,
        choice: &str,
        pokemon_index: Option<usize>,
    ) -> Result<(), String> {
        // Parse and validate choice
        let parts: Vec<&str> = choice.split_whitespace().collect();
        if parts.is_empty() {
            return Err("Empty choice".to_string());
        }

        let choice_type = parts[0];

        // Validate choice based on current request state
        match self.request_state {
            BattleRequestState::TeamPreview => {
                // During team preview, only 'team' choices are valid
                if choice_type != "team" {
                    return Err(format!(
                        "[Invalid choice] Can't {} for Team Preview: You must select a team order",
                        choice_type
                    ));
                }
            }
            BattleRequestState::Switch => {
                // During switch request, only 'switch' (and 'pass' in non-singles) are valid
                if choice_type == "switch" {
                    // Switch is always valid
                } else if choice_type == "pass" {
                    // Pass is only valid in Doubles/Triples, not in Singles
                    if matches!(self.game_type, GameType::Singles) {
                        return Err("[Invalid choice] Can't pass during switch request in Singles: You must switch a Pokemon".to_string());
                    }
                } else {
                    return Err(format!("[Invalid choice] Can't {} during switch request: You must switch a Pokemon", choice_type));
                }
            }
            _ => {
                // In other states (Move, None), certain choices are not valid
                if choice_type == "team"
                    && !matches!(self.request_state, BattleRequestState::TeamPreview)
                {
                    return Err(
                        "[Invalid choice] Team choices are only valid during Team Preview"
                            .to_string(),
                    );
                }
                // Pass validation:
                // - During Switch requests: always valid (except in Singles)
                // - During Move requests in Doubles/Triples: valid for fainted Pokemon slots
                // - Otherwise: invalid
                if choice_type == "pass" {
                    if matches!(self.request_state, BattleRequestState::Switch) {
                        // Already handled above in the Switch case
                    } else if matches!(self.request_state, BattleRequestState::Move) {
                        // In doubles/triples during Move request, pass is valid for fainted Pokemon
                        if !matches!(self.game_type, GameType::Doubles | GameType::Triples) {
                            return Err("[Invalid choice] Can't pass: You can only pass during switch requests".to_string());
                        }
                        // Check if this slot has a fainted Pokemon
                        let side_idx = side_id.index();
                        if let Some(poke_idx) = pokemon_index {
                            if let Some(active_poke_idx) =
                                self.sides[side_idx].active.get(poke_idx).and_then(|&x| x)
                            {
                                let pokemon = &self.sides[side_idx].pokemon[active_poke_idx];
                                if !pokemon.is_fainted() {
                                    return Err(
                                        "[Invalid choice] Can't pass: Pokemon is not fainted"
                                            .to_string(),
                                    );
                                }
                            }
                        }
                    } else {
                        return Err(
                            "[Invalid choice] Can't pass: You can only pass during switch requests"
                                .to_string(),
                        );
                    }
                }
            }
        }

        match choice_type {
            "move" => {
                // Parse move choice: move <move> [target] [modifier]
                // Examples: "move 1", "move tackle 2", "move 1 +1 mega", "move shadowball zmove 1"
                // Handle multi-word moves like "Conversion 2"
                if parts.len() < 2 {
                    return Err("Move choice requires move name/number".to_string());
                }

                // Try to parse move name - could be multi-word like "Conversion 2"
                // Start with the longest possible name and work backwards
                let mut move_name_parts = 1;
                let mut target_count = 0;
                let mut has_mega = false;
                let mut has_zmove = false;
                let mut has_dynamax = false;
                let mut has_ultra = false;

                // If first part after "move" is a number, it's move slot (1, 2, 3, 4)
                // Otherwise, try to match move names
                if parts[1].parse::<usize>().is_err() {
                    // Not a number - could be multi-word move name
                    // Try matching progressively longer names
                    // e.g. "move Conversion 2 zmove 2" could be:
                    //   - "Conversion" (move) + "2" (target) + "zmove" + "2" (target again - error)
                    //   - "Conversion 2" (move) + "zmove" + "2" (target)

                    // For now, simple heuristic: if next part is numeric and not a modifier, include it in move name
                    // This handles cases like "Conversion 2" where "2" is part of the move name
                    let mut i = 2;
                    while i < parts.len() && move_name_parts < 3 {
                        // Limit to 3 words max
                        let part = parts[i];
                        // If it's a number and we don't have modifiers yet, could be part of move name
                        let is_modifier = matches!(
                            part.to_lowercase().as_str(),
                            "mega" | "zmove" | "dynamax" | "max" | "ultra"
                        );

                        // Targets are: positive numbers, or explicit + relative (not -)
                        let is_explicit_relative = part.starts_with('+')
                            && part.len() > 1
                            && part[1..].parse::<i32>().is_ok();
                        let is_absolute = part.parse::<usize>().is_ok() && !part.starts_with('-');
                        let is_likely_target = is_explicit_relative || (is_absolute && i > 2);

                        if is_modifier || is_likely_target {
                            break; // Stop extending move name
                        }

                        // Check if this could be part of move name (number in position 2 could be like "2" in "Conversion 2")
                        if i == 2 && part.parse::<usize>().is_ok() {
                            // Might be "Conversion 2" - include it
                            move_name_parts += 1;
                            i += 1;
                        } else {
                            break;
                        }
                    }
                }

                // Now scan modifiers and targets starting after move name
                let mut i = 1 + move_name_parts;
                while i < parts.len() {
                    let part = parts[i];

                    // Check if it's a modifier
                    match part.to_lowercase().as_str() {
                        "mega" => {
                            if has_mega || has_zmove || has_ultra {
                                return Err("[Invalid choice] Can't combine multiple evolution/burst modifiers".to_string());
                            }
                            has_mega = true;
                        }
                        "zmove" => {
                            if has_mega || has_zmove || has_dynamax {
                                return Err(
                                    "[Invalid choice] Can't combine multiple move modifiers"
                                        .to_string(),
                                );
                            }
                            has_zmove = true;
                        }
                        "dynamax" | "max" => {
                            if has_zmove || has_dynamax {
                                return Err(
                                    "[Invalid choice] Can't combine multiple move modifiers"
                                        .to_string(),
                                );
                            }
                            has_dynamax = true;
                        }
                        "ultra" => {
                            if has_mega || has_ultra {
                                return Err("[Invalid choice] Can't combine multiple evolution/burst modifiers".to_string());
                            }
                            has_ultra = true;
                        }
                        _ => {
                            // Check if it's a target (positive number or explicit + relative target)
                            // Only + prefix is valid for relative targets, not -
                            let is_explicit_relative = part.starts_with('+')
                                && part.len() > 1
                                && part[1..].parse::<i32>().is_ok();
                            let is_absolute =
                                part.parse::<usize>().is_ok() && !part.starts_with('-');

                            let is_target = is_explicit_relative || is_absolute;

                            if is_target {
                                target_count += 1;
                                if target_count > 1 {
                                    return Err("[Invalid choice] Move can only have one target"
                                        .to_string());
                                }
                            } else {
                                // Check if it looks like an invalid target (negative number)
                                if part.starts_with('-') && part[1..].parse::<i32>().is_ok() {
                                    return Err(
                                        "[Invalid choice] Invalid target format".to_string()
                                    );
                                }
                                // Not a modifier or target - might be part of move name
                                // For now, we'll allow it (multi-word move names)
                            }
                        }
                    }
                    i += 1;
                }

                // In Singles, zmove and mega require a target
                if matches!(self.game_type, GameType::Singles)
                    && (has_zmove || has_mega)
                    && target_count == 0
                {
                    return Err(
                        "[Invalid choice] Z-moves and Mega Evolution require a target in Singles"
                            .to_string(),
                    );
                }

                // Validate that Pokemon has required item for zmove/mega
                if has_zmove || has_mega {
                    let side_idx = side_id.index();
                    if let Some(poke_idx) = pokemon_index {
                        // Get the active Pokemon at this slot
                        if let Some(active_poke_idx) =
                            self.sides[side_idx].active.get(poke_idx).and_then(|&x| x)
                        {
                            let pokemon = &self.sides[side_idx].pokemon[active_poke_idx];
                            let item_id = pokemon.item.as_str();

                            if has_zmove {
                                // Check if Pokemon has a Z-crystal
                                // Z-crystals end with "iumz" (e.g., "normaliumz", "firiumz")
                                if !item_id.ends_with("iumz") {
                                    return Err("[Invalid choice] Can't use Z-Move: Pokemon doesn't have a Z-Crystal".to_string());
                                }
                            }

                            if has_mega {
                                // Check if Pokemon has a mega stone
                                // Mega stones end with "ite" (e.g., "gengarite", "charizarditex")
                                if !item_id.ends_with("ite")
                                    && !item_id.ends_with("itex")
                                    && !item_id.ends_with("itey")
                                {
                                    return Err("[Invalid choice] Can't Mega Evolve: Pokemon doesn't have a Mega Stone".to_string());
                                }
                            }
                        }
                    }
                }

                Ok(())
            }
            "switch" => {
                // Parse switch choice
                if parts.len() < 2 {
                    return Err("Switch choice requires Pokemon number or name".to_string());
                }
                // Validate that the argument is either a number or a valid Pokemon name
                // Accept both numeric (e.g., "switch 2") and name-based (e.g., "switch Bulbasaur")
                let arg = parts[1];
                if arg.parse::<usize>().is_err() {
                    // Not a number - check if it's an obvious invalid word
                    // Reject common English words that are clearly not Pokemon names
                    let lowercase_arg = arg.to_lowercase();
                    let invalid_words = [
                        "first", "second", "third", "fourth", "fifth", "last", "next",
                    ];
                    if invalid_words.contains(&lowercase_arg.as_str()) {
                        return Err("[Invalid choice] Can't switch: You must specify a Pokemon by number or name".to_string());
                    }
                    // Otherwise assume it's a Pokemon name (will be validated during execution)
                }
                // Would validate and add to queue here
                Ok(())
            }
            "team" => {
                // Parse team order choice (for team preview)
                if parts.len() < 2 {
                    return Err("Team choice requires Pokemon order".to_string());
                }
                // Validate that the argument is a number
                match parts[1].parse::<usize>() {
                    Ok(num) => {
                        // Pokemon numbering is 1-based, reject 0
                        if num == 0 {
                            return Err("[Invalid choice] Can't choose for Team Preview: You must select a team order".to_string());
                        }
                        Ok(())
                    }
                    Err(_) => {
                        Err("[Invalid choice] Can't choose for Team Preview: You must select a team order".to_string())
                    }
                }
            }
            "pass" => {
                // Pass should not have any choice details
                if parts.len() > 1 {
                    return Err("[Invalid choice] Pass does not accept any arguments".to_string());
                }
                Ok(())
            }
            "shift" => {
                // Shift is only valid in triples
                if !matches!(self.game_type, GameType::Triples) {
                    return Err(
                        "[Invalid choice] Shift is only valid in Triple Battles".to_string()
                    );
                }
                // Shift cannot be used in the center position (slot 1)
                if let Some(poke_idx) = pokemon_index {
                    if poke_idx == 1 {
                        return Err(
                            "[Invalid choice] The center Pokemon cannot shift position".to_string()
                        );
                    }
                }
                // Shift does not accept any arguments
                if parts.len() > 1 {
                    return Err("[Invalid choice] Shift does not accept any arguments".to_string());
                }
                Ok(())
            }
            _ => Err(format!("Unknown choice type: {}", choice_type)),
        }
    }
}
