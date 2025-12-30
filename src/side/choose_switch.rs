use crate::side::*;
use crate::*;

impl Side {

    /// Choose switch action
    //
    // 	chooseSwitch(slotText?: string) {
    // 		if (this.requestState !== 'move' && this.requestState !== 'switch') {
    // 			return this.emitChoiceError(`Can't switch: You need a ${this.requestState} response`);
    // 		}
    // 		const index = this.getChoiceIndex();
    // 		if (index >= this.active.length) {
    // 			if (this.requestState === 'switch') {
    // 				return this.emitChoiceError(`Can't switch: You sent more switches than Pokémon that need to switch`);
    // 			}
    // 			return this.emitChoiceError(`Can't switch: You sent more choices than unfainted Pokémon`);
    // 		}
    // 		const pokemon = this.active[index];
    // 		let slot;
    // 		if (!slotText) {
    // 			if (this.requestState !== 'switch') {
    // 				return this.emitChoiceError(`Can't switch: You need to select a Pokémon to switch in`);
    // 			}
    // 			if (this.slotConditions[pokemon.position]['revivalblessing']) {
    // 				slot = 0;
    // 				while (!this.pokemon[slot].fainted) slot++;
    // 			} else {
    // 				if (!this.choice.forcedSwitchesLeft) return this.choosePass();
    // 				slot = this.active.length;
    // 				while (this.choice.switchIns.has(slot) || this.pokemon[slot].fainted) slot++;
    // 			}
    // 		} else {
    // 			slot = parseInt(slotText) - 1;
    // 		}
    // 		if (isNaN(slot) || slot < 0) {
    // 			// maybe it's a name/species id!
    // 			slot = -1;
    // 			for (const [i, mon] of this.pokemon.entries()) {
    // 				if (slotText!.toLowerCase() === mon.name.toLowerCase() || toID(slotText) === mon.species.id) {
    // 					slot = i;
    // 					break;
    // 				}
    // 			}
    // 			if (slot < 0) {
    // 				return this.emitChoiceError(`Can't switch: You do not have a Pokémon named "${slotText}" to switch to`);
    // 			}
    // 		}
    // 		if (slot >= this.pokemon.length) {
    // 			return this.emitChoiceError(`Can't switch: You do not have a Pokémon in slot ${slot + 1} to switch to`);
    // 		} else if (slot < this.active.length && !this.slotConditions[pokemon.position]['revivalblessing']) {
    // 			return this.emitChoiceError(`Can't switch: You can't switch to an active Pokémon`);
    // 		} else if (this.choice.switchIns.has(slot)) {
    // 			return this.emitChoiceError(`Can't switch: The Pokémon in slot ${slot + 1} can only switch in once`);
    // 		}
    // 		const targetPokemon = this.pokemon[slot];
    //
    // 		if (this.slotConditions[pokemon.position]['revivalblessing']) {
    // 			if (!targetPokemon.fainted) {
    // 				return this.emitChoiceError(`Can't switch: You have to pass to a fainted Pokémon`);
    // 			}
    // 			// Should always subtract, but stop at 0 to prevent errors.
    // 			this.choice.forcedSwitchesLeft = this.battle.clampIntRange(this.choice.forcedSwitchesLeft - 1, 0);
    // 			pokemon.switchFlag = false;
    // 			this.choice.actions.push({
    // 				choice: 'revivalblessing',
    // 				pokemon,
    // 				target: targetPokemon,
    // 			} as ChosenAction);
    // 			return true;
    // 		}
    //
    // 		if (targetPokemon.fainted) {
    // 			return this.emitChoiceError(`Can't switch: You can't switch to a fainted Pokémon`);
    // 		}
    //
    // 		if (this.requestState === 'move') {
    // 			if (pokemon.trapped) {
    // 				return this.emitChoiceError(`Can't switch: The active Pokémon is trapped`, { pokemon, update: req => {
    // 					let updated = false;
    // 					if (req.maybeTrapped) {
    // 						delete req.maybeTrapped;
    // 						updated = true;
    // 					}
    // 					if (!req.trapped) {
    // 						req.trapped = true;
    // 						updated = true;
    // 					}
    // 					return updated;
    // 				} });
    // 			} else if (pokemon.maybeTrapped) {
    // 				this.choice.cantUndo = true;
    // 			}
    // 		} else if (this.requestState === 'switch') {
    // 			if (!this.choice.forcedSwitchesLeft) {
    // 				throw new Error(`Player somehow switched too many Pokemon`);
    // 			}
    // 			this.choice.forcedSwitchesLeft--;
    // 		}
    //
    // 		this.choice.switchIns.add(slot);
    //
    // 		this.choice.actions.push({
    // 			choice: (this.requestState === 'switch' ? 'instaswitch' : 'switch'),
    // 			pokemon,
    // 			target: targetPokemon,
    // 		} as ChosenAction);
    //
    // 		return true;
    // 	}
    //
    pub fn choose_switch(&mut self, slot: usize) -> Result<(), String> {
        let index = self.get_choice_index(false);
        if index >= self.active.len() {
            return Err("You sent more switches than needed".to_string());
        }

        if slot >= self.pokemon.len() {
            return Err(format!("You don't have a Pokemon in slot {}", slot + 1));
        }

        if slot < self.active.len() {
            return Err("Can't switch to an active Pokemon".to_string());
        }

        if self.choice.switch_ins.contains(&slot) {
            return Err(format!("Pokemon in slot {} already switching in", slot + 1));
        }

        let target = self.pokemon.get(slot).ok_or("Invalid slot")?;
        if target.is_fainted() {
            return Err("Can't switch to a fainted Pokemon".to_string());
        }

        self.choice.switch_ins.push(slot);

        let choice_type = if self.request_state == RequestState::Switch {
            if self.choice.forced_switches_left == 0 {
                return Err("No more forced switches".to_string());
            }
            self.choice.forced_switches_left -= 1;
            ChoiceType::InstaSwitch
        } else {
            ChoiceType::Switch
        };

        self.choice.actions.push(ChosenAction {
            choice: choice_type,
            pokemon_index: index,
            target_loc: None,
            move_id: None,
            switch_index: Some(slot),
            mega: false,
            zmove: None,
            max_move: None,
            terastallize: None,
        });

        Ok(())
    }
}
