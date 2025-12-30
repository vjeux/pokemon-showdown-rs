use crate::side::*;

impl Side {

    /// Process a choice command
    /// Equivalent to side.ts choose()
    //
    // 	choose(input: string) {
    // 		if (!this.requestState) {
    // 			return this.emitChoiceError(
    // 				this.battle.ended ? `Can't do anything: The game is over` : `Can't do anything: It's not your turn`
    // 			);
    // 		}
    //
    // 		if (this.choice.cantUndo) {
    // 			return this.emitChoiceError(`Can't undo: A trapping/disabling effect would cause undo to leak information`);
    // 		}
    //
    // 		this.clearChoice();
    //
    // 		const choiceStrings = (input.startsWith('team ') ? [input] : input.split(','));
    //
    // 		if (choiceStrings.length > this.active.length) {
    // 			return this.emitChoiceError(
    // 				`Can't make choices: You sent choices for ${choiceStrings.length} Pokémon, but this is a ${this.battle.gameType} game!`
    // 			);
    // 		}
    //
    // 		for (const choiceString of choiceStrings) {
    // 			let [choiceType, data] = Utils.splitFirst(choiceString.trim(), ' ');
    // 			data = data.trim();
    // 			if (choiceType === 'testfight') {
    // 				choiceType = 'move';
    // 				data = 'testfight';
    // 			}
    //
    // 			switch (choiceType) {
    // 			case 'move':
    // 				const original = data;
    // 				const error = () => this.emitChoiceError(`Conflicting arguments for "move": ${original}`);
    // 				let targetLoc: number | undefined;
    // 				let event: 'mega' | 'megax' | 'megay' | 'zmove' | 'ultra' | 'dynamax' | 'terastallize' | '' = '';
    // 				while (true) {
    // 					// If data ends with a number, treat it as a target location.
    // 					// We need to special case 'Conversion 2' so it doesn't get
    // 					// confused with 'Conversion' erroneously sent with the target
    // 					// '2' (since Conversion targets 'self', targetLoc can't be 2).
    // 					if (/\s(?:-|\+)?[1-3]$/.test(data) && toID(data) !== 'conversion2') {
    // 						if (targetLoc !== undefined) return error();
    // 						targetLoc = parseInt(data.slice(-2));
    // 						data = data.slice(0, -2).trim();
    // 					} else if (data.endsWith(' mega')) {
    // 						if (event) return error();
    // 						event = 'mega';
    // 						data = data.slice(0, -5);
    // 					} else if (data.endsWith(' megax')) {
    // 						if (event) return error();
    // 						event = 'megax';
    // 						data = data.slice(0, -6);
    // 					} else if (data.endsWith(' megay')) {
    // 						if (event) return error();
    // 						event = 'megay';
    // 						data = data.slice(0, -6);
    // 					} else if (data.endsWith(' zmove')) {
    // 						if (event) return error();
    // 						event = 'zmove';
    // 						data = data.slice(0, -6);
    // 					} else if (data.endsWith(' ultra')) {
    // 						if (event) return error();
    // 						event = 'ultra';
    // 						data = data.slice(0, -6);
    // 					} else if (data.endsWith(' dynamax')) {
    // 						if (event) return error();
    // 						event = 'dynamax';
    // 						data = data.slice(0, -8);
    // 					} else if (data.endsWith(' gigantamax')) {
    // 						if (event) return error();
    // 						event = 'dynamax';
    // 						data = data.slice(0, -11);
    // 					} else if (data.endsWith(' max')) {
    // 						if (event) return error();
    // 						event = 'dynamax';
    // 						data = data.slice(0, -4);
    // 					} else if (data.endsWith(' terastal')) {
    // 						if (event) return error();
    // 						event = 'terastallize';
    // 						data = data.slice(0, -9);
    // 					} else if (data.endsWith(' terastallize')) {
    // 						if (event) return error();
    // 						event = 'terastallize';
    // 						data = data.slice(0, -13);
    // 					} else {
    // 						break;
    // 					}
    // 				}
    // 				if (!this.chooseMove(data, targetLoc, event)) return false;
    // 				break;
    // 			case 'switch':
    // 				this.chooseSwitch(data);
    // 				break;
    // 			case 'shift':
    // 				if (data) return this.emitChoiceError(`Unrecognized data after "shift": ${data}`);
    // 				if (!this.chooseShift()) return false;
    // 				break;
    // 			case 'team':
    // 				if (!this.chooseTeam(data)) return false;
    // 				break;
    // 			case 'pass':
    // 			case 'skip':
    // 				if (data) return this.emitChoiceError(`Unrecognized data after "pass": ${data}`);
    // 				if (!this.choosePass()) return false;
    // 				break;
    // 			case 'auto':
    // 			case 'default':
    // 				this.autoChoose();
    // 				break;
    // 			default:
    // 				this.emitChoiceError(`Unrecognized choice: ${choiceString}`);
    // 				break;
    // 			}
    // 		}
    //
    // 		return !this.choice.error;
    // 	}
    //
    pub fn choose(&mut self, input: &str) -> Result<bool, String> {
        // Parse and execute choice commands
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            return Err("Empty choice".to_string());
        }

        match parts[0] {
            "auto" | "default" => {
                // JS: case 'auto':
                // JS: case 'default':
                // JS:     this.autoChoose();
                // JS:     break;
                self.auto_choose();
                Ok(true)
            }
            "pass" | "skip" => {
                // JS: case 'pass':
                // JS: case 'skip':
                // JS:     if (data) return this.emitChoiceError(`Unrecognized data after "pass": ${data}`);
                // JS:     if (!this.choosePass()) return false;
                // JS:     break;
                if parts.len() > 1 {
                    return Err(format!("Unrecognized data after 'pass': {}", parts[1..].join(" ")));
                }
                self.choose_pass();
                Ok(true)
            }
            "switch" => {
                // JS: case 'switch':
                // JS:     this.chooseSwitch(data);
                // JS:     break;
                if parts.len() < 2 {
                    return Err("Switch requires target".to_string());
                }
                let target: usize = parts[1].parse().map_err(|_| "Invalid switch target")?;
                self.choose_switch(target - 1)?; // 1-indexed in protocol
                Ok(true)
            }
            "move" => {
                // JS: case 'move':
                // JS:     ...lots of parsing logic...
                // JS:     if (!this.chooseMove(data, targetLoc, event)) return false;
                // JS:     break;
                if parts.len() < 2 {
                    return Err("Move requires target".to_string());
                }
                let move_idx: usize = parts[1].parse().map_err(|_| "Invalid move")?;
                let index = self.get_choice_index(false);
                if let Some(Some(poke_idx)) = self.active.get(index) {
                    if let Some(slot) = self.pokemon[*poke_idx].move_slots.get(move_idx - 1) {
                        let move_id = slot.id.clone();
                        self.choose_move(move_id, None, false, None, None, None)?;
                        Ok(true)
                    } else {
                        Err("Invalid move index".to_string())
                    }
                } else {
                    Err("No active Pokemon".to_string())
                }
            }
            "team" => {
                // JS: case 'team':
                // JS:     if (!this.chooseTeam(data)) return false;
                // JS:     break;
                let positions: Result<Vec<usize>, _> = parts[1..]
                    .iter()
                    .map(|s| s.parse::<usize>().map(|n| n - 1))
                    .collect();
                match positions {
                    Ok(pos) => {
                        self.choose_team(pos)?;
                        Ok(true)
                    }
                    Err(_) => Err("Invalid team positions".to_string()),
                }
            }
            "shift" => {
                // JS: case 'shift':
                // JS:     if (data) return this.emitChoiceError(`Unrecognized data after "shift": ${data}`);
                // JS:     if (!this.chooseShift()) return false;
                // JS:     break;
                if parts.len() > 1 {
                    return Err(format!("Unrecognized data after 'shift': {}", parts[1..].join(" ")));
                }
                self.choose_shift()?;
                Ok(true)
            }
            _ => {
                // JS: default:
                // JS:     this.emitChoiceError(`Unrecognized choice: ${choiceString}`);
                // JS:     break;
                self.emit_choice_error(&format!("Unrecognized choice: {}", input));
                Err(format!("Unknown choice: {}", parts[0]))
            }
        }
    }
}
