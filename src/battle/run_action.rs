use crate::*;
use crate::battle::{PriorityItem, BattleRequestState};

impl Battle {

    /// Run a single action from the queue
    /// Equivalent to battle.ts runAction()
    //
    // 	runAction(action: Action) {
    // 		const pokemonOriginalHP = action.pokemon?.hp;
    // 		let residualPokemon: (readonly [Pokemon, number])[] = [];
    // 		// returns whether or not we ended in a callback
    // 		switch (action.choice) {
    // 		case 'start': {
    // 			for (const side of this.sides) {
    // 				if (side.pokemonLeft) side.pokemonLeft = side.pokemon.length;
    // 				this.add('teamsize', side.id, side.pokemon.length);
    // 			}
    //
    // 			this.add('start');
    //
    // 			// Change Zacian/Zamazenta into their Crowned formes
    // 			for (const pokemon of this.getAllPokemon()) {
    // 				let rawSpecies: Species | null = null;
    // 				if (pokemon.species.id === 'zacian' && pokemon.item === 'rustedsword') {
    // 					rawSpecies = this.dex.species.get('Zacian-Crowned');
    // 				} else if (pokemon.species.id === 'zamazenta' && pokemon.item === 'rustedshield') {
    // 					rawSpecies = this.dex.species.get('Zamazenta-Crowned');
    // 				}
    // 				if (!rawSpecies) continue;
    // 				const species = pokemon.setSpecies(rawSpecies);
    // 				if (!species) continue;
    // 				pokemon.baseSpecies = rawSpecies;
    // 				pokemon.details = pokemon.getUpdatedDetails();
    // 				pokemon.setAbility(species.abilities['0'], null, null, true);
    // 				pokemon.baseAbility = pokemon.ability;
    //
    // 				const behemothMove: { [k: string]: string } = {
    // 					'Zacian-Crowned': 'behemothblade', 'Zamazenta-Crowned': 'behemothbash',
    // 				};
    // 				const ironHeadIndex = pokemon.baseMoves.indexOf('ironhead');
    // 				if (ironHeadIndex >= 0) {
    // 					const move = this.dex.moves.get(behemothMove[rawSpecies.name]);
    // 					pokemon.baseMoveSlots[ironHeadIndex] = {
    // 						move: move.name,
    // 						id: move.id,
    // 						pp: move.noPPBoosts ? move.pp : move.pp * 8 / 5,
    // 						maxpp: move.noPPBoosts ? move.pp : move.pp * 8 / 5,
    // 						target: move.target,
    // 						disabled: false,
    // 						disabledSource: '',
    // 						used: false,
    // 					};
    // 					pokemon.moveSlots = pokemon.baseMoveSlots.slice();
    // 				}
    // 			}
    //
    // 			this.format.onBattleStart?.call(this);
    // 			for (const rule of this.ruleTable.keys()) {
    // 				if ('+*-!'.includes(rule.charAt(0))) continue;
    // 				const subFormat = this.dex.formats.get(rule);
    // 				subFormat.onBattleStart?.call(this);
    // 			}
    //
    // 			for (const side of this.sides) {
    // 				for (let i = 0; i < side.active.length; i++) {
    // 					if (!side.pokemonLeft) {
    // 						// forfeited before starting
    // 						side.active[i] = side.pokemon[i];
    // 						side.active[i].fainted = true;
    // 						side.active[i].hp = 0;
    // 					} else {
    // 						this.actions.switchIn(side.pokemon[i], i);
    // 					}
    // 				}
    // 			}
    // 			for (const pokemon of this.getAllPokemon()) {
    // 				this.singleEvent('Start', this.dex.conditions.getByID(pokemon.species.id), pokemon.speciesState, pokemon);
    // 			}
    // 			this.midTurn = true;
    // 			break;
    // 		}
    //
    // 		case 'move':
    // 			if (!action.pokemon.isActive) return false;
    // 			if (action.pokemon.fainted) return false;
    // 			this.actions.runMove(action.move, action.pokemon, action.targetLoc, {
    // 				sourceEffect: action.sourceEffect, zMove: action.zmove,
    // 				maxMove: action.maxMove, originalTarget: action.originalTarget,
    // 			});
    // 			break;
    // 		case 'megaEvo':
    // 			this.actions.runMegaEvo(action.pokemon);
    // 			break;
    // 		case 'megaEvoX':
    // 			this.actions.runMegaEvoX?.(action.pokemon);
    // 			break;
    // 		case 'megaEvoY':
    // 			this.actions.runMegaEvoY?.(action.pokemon);
    // 			break;
    // 		case 'runDynamax':
    // 			action.pokemon.addVolatile('dynamax');
    // 			action.pokemon.side.dynamaxUsed = true;
    // 			if (action.pokemon.side.allySide) action.pokemon.side.allySide.dynamaxUsed = true;
    // 			break;
    // 		case 'terastallize':
    // 			this.actions.terastallize(action.pokemon);
    // 			break;
    // 		case 'beforeTurnMove':
    // 			if (!action.pokemon.isActive) return false;
    // 			if (action.pokemon.fainted) return false;
    // 			this.debug('before turn callback: ' + action.move.id);
    // 			const target = this.getTarget(action.pokemon, action.move, action.targetLoc);
    // 			if (!target) return false;
    // 			if (!action.move.beforeTurnCallback) throw new Error(`beforeTurnMove has no beforeTurnCallback`);
    // 			action.move.beforeTurnCallback.call(this, action.pokemon, target);
    // 			break;
    // 		case 'priorityChargeMove':
    // 			if (!action.pokemon.isActive) return false;
    // 			if (action.pokemon.fainted) return false;
    // 			this.debug('priority charge callback: ' + action.move.id);
    // 			if (!action.move.priorityChargeCallback) throw new Error(`priorityChargeMove has no priorityChargeCallback`);
    // 			action.move.priorityChargeCallback.call(this, action.pokemon);
    // 			break;
    //
    // 		case 'event':
    // 			this.runEvent(action.event!, action.pokemon);
    // 			break;
    // 		case 'team':
    // 			if (action.index === 0) {
    // 				action.pokemon.side.pokemon = [];
    // 			}
    // 			action.pokemon.side.pokemon.push(action.pokemon);
    // 			action.pokemon.position = action.index;
    // 			// we return here because the update event would crash since there are no active pokemon yet
    // 			return;
    //
    // 		case 'pass':
    // 			return;
    // 		case 'instaswitch':
    // 		case 'switch':
    // 			if (action.choice === 'switch' && action.pokemon.status) {
    // 				this.singleEvent('CheckShow', this.dex.abilities.getByID('naturalcure' as ID), null, action.pokemon);
    // 			}
    // 			if (this.actions.switchIn(action.target, action.pokemon.position, action.sourceEffect) === 'pursuitfaint') {
    // 				// a pokemon fainted from Pursuit before it could switch
    // 				if (this.gen <= 4) {
    // 					// in gen 2-4, the switch still happens
    // 					this.hint("Previously chosen switches continue in Gen 2-4 after a Pursuit target faints.");
    // 					action.priority = -101;
    // 					this.queue.unshift(action);
    // 					break;
    // 				} else {
    // 					// in gen 5+, the switch is cancelled
    // 					this.hint("A Pokemon can't switch between when it runs out of HP and when it faints");
    // 					break;
    // 				}
    // 			}
    // 			break;
    // 		case 'revivalblessing':
    // 			action.pokemon.side.pokemonLeft++;
    // 			if (action.target.position < action.pokemon.side.active.length) {
    // 				this.queue.addChoice({
    // 					choice: 'instaswitch',
    // 					pokemon: action.target,
    // 					target: action.target,
    // 				});
    // 			}
    // 			action.target.fainted = false;
    // 			action.target.faintQueued = false;
    // 			action.target.subFainted = false;
    // 			action.target.status = '';
    // 			action.target.hp = 1; // Needed so hp functions works
    // 			action.target.sethp(action.target.maxhp / 2);
    // 			this.add('-heal', action.target, action.target.getHealth, '[from] move: Revival Blessing');
    // 			action.pokemon.side.removeSlotCondition(action.pokemon, 'revivalblessing');
    // 			break;
    // 		case 'runSwitch':
    // 			this.actions.runSwitch(action.pokemon);
    // 			break;
    // 		case 'shift':
    // 			if (!action.pokemon.isActive) return false;
    // 			if (action.pokemon.fainted) return false;
    // 			this.swapPosition(action.pokemon, 1);
    // 			break;
    //
    // 		case 'beforeTurn':
    // 			this.eachEvent('BeforeTurn');
    // 			break;
    // 		case 'residual':
    // 			this.add('');
    // 			this.clearActiveMove(true);
    // 			this.updateSpeed();
    // 			residualPokemon = this.getAllActive().map(pokemon => [pokemon, pokemon.getUndynamaxedHP()] as const);
    // 			this.fieldEvent('Residual');
    // 			if (!this.ended) this.add('upkeep');
    // 			break;
    // 		}
    //
    // 		// phazing (Roar, etc)
    // 		for (const side of this.sides) {
    // 			for (const pokemon of side.active) {
    // 				if (pokemon.forceSwitchFlag) {
    // 					if (pokemon.hp) this.actions.dragIn(pokemon.side, pokemon.position);
    // 					pokemon.forceSwitchFlag = false;
    // 				}
    // 			}
    // 		}
    //
    // 		this.clearActiveMove();
    //
    // 		// fainting
    //
    // 		this.faintMessages();
    // 		if (this.ended) return true;
    //
    // 		// switching (fainted pokemon, U-turn, Baton Pass, etc)
    //
    // 		if (!this.queue.peek() || (this.gen <= 3 && ['move', 'residual'].includes(this.queue.peek()!.choice))) {
    // 			// in gen 3 or earlier, switching in fainted pokemon is done after
    // 			// every move, rather than only at the end of the turn.
    // 			this.checkFainted();
    // 		} else if (['megaEvo', 'megaEvoX', 'megaEvoY'].includes(action.choice) && this.gen === 7) {
    // 			this.eachEvent('Update');
    // 			// In Gen 7, the action order is recalculated for a Pokémon that mega evolves.
    // 			for (const [i, queuedAction] of this.queue.list.entries()) {
    // 				if (queuedAction.pokemon === action.pokemon && queuedAction.choice === 'move') {
    // 					this.queue.list.splice(i, 1);
    // 					queuedAction.mega = 'done';
    // 					this.queue.insertChoice(queuedAction, true);
    // 					break;
    // 				}
    // 			}
    // 			return false;
    // 		} else if (this.queue.peek()?.choice === 'instaswitch') {
    // 			return false;
    // 		}
    //
    // 		if (this.gen >= 5 && action.choice !== 'start') {
    // 			this.eachEvent('Update');
    // 			for (const [pokemon, originalHP] of residualPokemon) {
    // 				const maxhp = pokemon.getUndynamaxedHP(pokemon.maxhp);
    // 				if (pokemon.hp && pokemon.getUndynamaxedHP() <= maxhp / 2 && originalHP > maxhp / 2) {
    // 					this.runEvent('EmergencyExit', pokemon);
    // 				}
    // 			}
    // 		}
    //
    // 		if (action.choice === 'runSwitch') {
    // 			const pokemon = action.pokemon;
    // 			if (pokemon.hp && pokemon.hp <= pokemon.maxhp / 2 && pokemonOriginalHP! > pokemon.maxhp / 2) {
    // 				this.runEvent('EmergencyExit', pokemon);
    // 			}
    // 		}
    //
    // 		const switches = this.sides.map(
    // 			side => side.active.some(pokemon => pokemon && !!pokemon.switchFlag)
    // 		);
    //
    // 		for (let i = 0; i < this.sides.length; i++) {
    // 			let reviveSwitch = false; // Used to ignore the fake switch for Revival Blessing
    // 			if (switches[i] && !this.canSwitch(this.sides[i])) {
    // 				for (const pokemon of this.sides[i].active) {
    // 					if (this.sides[i].slotConditions[pokemon.position]['revivalblessing']) {
    // 						reviveSwitch = true;
    // 						continue;
    // 					}
    // 					pokemon.switchFlag = false;
    // 				}
    // 				if (!reviveSwitch) switches[i] = false;
    // 			} else if (switches[i]) {
    // 				for (const pokemon of this.sides[i].active) {
    // 					if (
    // 						pokemon.hp && pokemon.switchFlag && pokemon.switchFlag !== 'revivalblessing' &&
    // 						!pokemon.skipBeforeSwitchOutEventFlag
    // 					) {
    // 						this.runEvent('BeforeSwitchOut', pokemon);
    // 						pokemon.skipBeforeSwitchOutEventFlag = true;
    // 						this.faintMessages(); // Pokemon may have fainted in BeforeSwitchOut
    // 						if (this.ended) return true;
    // 						if (pokemon.fainted) {
    // 							switches[i] = this.sides[i].active.some(sidePokemon => sidePokemon && !!sidePokemon.switchFlag);
    // 						}
    // 					}
    // 				}
    // 			}
    // 		}
    //
    // 		for (const playerSwitch of switches) {
    // 			if (playerSwitch) {
    // 				this.makeRequest('switch');
    // 				return true;
    // 			}
    // 		}
    //
    // 		if (this.gen < 5) this.eachEvent('Update');
    //
    // 		if (this.gen >= 8 && (this.queue.peek()?.choice === 'move' || this.queue.peek()?.choice === 'runDynamax')) {
    // 			// In gen 8, speed is updated dynamically so update the queue's speed properties and sort it.
    // 			this.updateSpeed();
    // 			for (const queueAction of this.queue.list) {
    // 				if (queueAction.pokemon) this.getActionSpeed(queueAction);
    // 			}
    // 			this.queue.sort();
    // 		}
    //
    // 		return false;
    // 	}
    //
    pub fn run_action(&mut self, action: &crate::battle_queue::Action) {
        use crate::battle_queue::{Action, FieldActionType, PokemonActionType};

        match action {
            Action::Move(move_action) => {
                let side_idx = move_action.side_index;
                let poke_idx = move_action.pokemon_index;
                let move_id = &move_action.move_id;
                let target_loc = move_action.target_loc;

                eprintln!("RUN_ACTION Move: p{}{} uses {}",
                    side_idx + 1,
                    if poke_idx == 0 { "a" } else { "b" },
                    move_id);

                // Check if Pokemon can still act
                if let Some(side) = self.sides.get(side_idx) {
                    if let Some(pokemon) = side.pokemon.get(poke_idx) {
                        if pokemon.is_fainted() {
                            return;
                        }
                    } else {
                        return;
                    }
                } else {
                    return;
                }

                crate::battle_actions::run_move(
                    self,
                    move_id,
                    (side_idx, poke_idx),
                    target_loc,
                    None, // source_effect
                    move_action.zmove.clone(), // z_move
                    false, // external_move
                    move_action.max_move.clone(), // max_move
                    None, // original_target
                );
            }
            Action::Switch(switch_action) => {
                let side_idx = switch_action.side_index;
                let poke_idx = switch_action.pokemon_index;
                let target = switch_action.target_index;

                self.do_switch(side_idx, poke_idx, target);
            }
            Action::Field(field_action) => {
                match field_action.choice {
                    FieldActionType::Residual => {
                        self.run_residual();
                    }
                    FieldActionType::BeforeTurn => {
                        // JS: this.eachEvent('BeforeTurn');
                        self.each_event("BeforeTurn", None, None);
                    }
                    FieldActionType::Start => {
                        // JS: for (const side of this.sides) { if (side.pokemonLeft) side.pokemonLeft = side.pokemon.length; this.add('teamsize', side.id, side.pokemon.length); }
                        for side_idx in 0..self.sides.len() {
                            let team_size = self.sides[side_idx].pokemon.len();
                            if self.sides[side_idx].pokemon_left > 0 {
                                self.sides[side_idx].pokemon_left = team_size;
                            }
                            let side_id = format!("p{}", side_idx + 1);
                            self.add("teamsize", &[side_id.into(), team_size.to_string().into()]);
                        }

                        // JS: this.add('start');
                        self.add("start", &[]);

                        // TODO: Zacian/Zamazenta forme changes (requires species transformation logic)
                        // JS: Change Zacian/Zamazenta into their Crowned formes
                        // This requires:
                        // - Species data lookup (dex.species.get)
                        // - pokemon.setSpecies()
                        // - pokemon.setAbility()
                        // - Move slot modification (behemothblade/behemothbash replacement)

                        // TODO: Format callbacks (requires format infrastructure)
                        // JS: this.format.onBattleStart?.call(this);
                        // JavaScript formats can have onBattleStart callbacks
                        // These cannot be deserialized from JSON - must be registered separately
                        // For now, emit an event that format-specific code can hook into
                        self.run_event("BattleStart", None, None, None, None);

                        // JS: for (const rule of this.ruleTable.keys()) { ... }
                        if let Some(ref rule_table) = self.rule_table {
                            let rule_keys: Vec<String> = rule_table.keys().cloned().collect();

                            for rule in rule_keys {
                                // Skip rules starting with +, *, -, !
                                if let Some(first_char) = rule.chars().next() {
                                    if first_char == '+'
                                        || first_char == '*'
                                        || first_char == '-'
                                        || first_char == '!'
                                    {
                                        continue;
                                    }
                                }

                                // JS: const subFormat = this.dex.formats.get(rule);
                                // JS: subFormat.onBattleStart?.call(this);
                                // Emit event for rule-specific battle start hooks
                                self.run_event(
                                    &format!("RuleBattleStart:{}", rule),
                                    None,
                                    None,
                                    None,
                                    None,
                                );
                            }
                        }

                        // JS: for (const side of this.sides) { for (let i = 0; i < side.active.length; i++) { this.actions.switchIn(side.pokemon[i], i); } }
                        // Switch in all active Pokemon at battle start
                        for side_idx in 0..self.sides.len() {
                            let active_length = self.sides[side_idx].active.len();
                            for i in 0..active_length {
                                // Switch in the ith Pokemon from the team into the ith active position
                                // JS: this.actions.switchIn(side.pokemon[i], i);
                                crate::battle_actions::switch_in(self, side_idx, i, i, None, false);
                            }
                        }

                        // JS: for (const pokemon of this.getAllPokemon()) { this.singleEvent('Start', ...); }
                        // Call Start event for each Pokemon's species
                        for side_idx in 0..self.sides.len() {
                            for poke_idx in 0..self.sides[side_idx].pokemon.len() {
                                let species_id =
                                    self.sides[side_idx].pokemon[poke_idx].species_id.clone();
                                self.single_event(
                                    "Start",
                                    &species_id,
                                    Some((side_idx, poke_idx)),
                                    None,
                                    None,
                                );
                            }
                        }

                        // JS: this.midTurn = true;
                        self.mid_turn = true;
                    }
                    FieldActionType::Pass => {
                        // Pass action - do nothing
                    }
                }
            }
            Action::Team(_) => {
                // Team preview action handled elsewhere
            }
            Action::Pokemon(poke_action) => {
                use crate::battle_queue::PokemonActionType;

                match poke_action.choice {
                    PokemonActionType::RunSwitch => {
                        // JS: const switchersIn = [pokemon];
                        let mut switchers_in = vec![(poke_action.side_index, poke_action.pokemon_index)];

                        // JS: while (this.battle.queue.peek()?.choice === "runSwitch") {
                        //         const nextSwitch = this.battle.queue.shift();
                        //         switchersIn.push(nextSwitch.pokemon);
                        //     }
                        // Collect all consecutive RunSwitch actions
                        loop {
                            let should_extract = if let Some(action) = self.queue.peek() {
                                matches!(action, Action::Pokemon(p) if matches!(p.choice, PokemonActionType::RunSwitch))
                            } else {
                                false
                            };

                            if !should_extract {
                                break;
                            }

                            if let Some(Action::Pokemon(next_poke)) = self.queue.shift() {
                                switchers_in.push((next_poke.side_index, next_poke.pokemon_index));
                            }
                        }

                        // JS: const allActive = this.battle.getAllActive(true);
                        let all_active = self.get_all_active(true);

                        // JS: this.battle.speedSort(allActive);
                        // Extract Pokemon speeds first to avoid borrow checker issues
                        let speeds: Vec<i32> = all_active
                            .iter()
                            .map(|(s_idx, p_idx)| self.sides[*s_idx].pokemon[*p_idx].stored_stats.spe as i32)
                            .collect();

                        // Now sort using the pre-extracted speeds
                        let mut all_active_with_speeds: Vec<_> = all_active
                            .into_iter()
                            .zip(speeds.iter())
                            .collect();

                        self.speed_sort(&mut all_active_with_speeds, |(_, speed)| PriorityItem {
                            order: None,
                            priority: 0,
                            speed: **speed as f64,
                            sub_order: 0,
                            effect_order: 0,
                            index: 0,
                        });

                        // JS: this.battle.speedOrder = allActive.map((a) => a.side.n * a.battle.sides.length + a.position);
                        // TODO: Rust doesn't have speedOrder field yet - add it if needed

                        // JS: this.battle.fieldEvent("SwitchIn", switchersIn);
                        self.field_event_switch_in(&switchers_in);

                        // JS: for (const poke of switchersIn) {
                        //         if (!poke.hp) continue;
                        //         poke.isStarted = true;
                        //         poke.draggedIn = null;
                        //     }
                        for (s_idx, p_idx) in switchers_in {
                            if let Some(pokemon) = self.sides[s_idx].pokemon.get_mut(p_idx) {
                                if pokemon.hp <= 0 {
                                    continue;
                                }
                                pokemon.is_started = true;
                                pokemon.dragged_in = None;
                            }
                        }
                    }
                    _ => {
                        // Other Pokemon actions (mega evo, terastallize, etc.)
                    }
                }
            }
        }

        // JS: if (this.gen >= 5 && action.choice !== "start") { this.eachEvent("Update"); }
        // Call Update event for all actions except "start" in Gen 5+
        let is_start_action = matches!(action, Action::Field(f) if matches!(f.choice, FieldActionType::Start));
        if self.gen >= 5 && !is_start_action {
            self.each_event("Update", None, None);
        }

        // JS: if (this.gen >= 8 && (this.queue.peek()?.choice === "move" || this.queue.peek()?.choice === "runDynamax")) {
        // JS:     this.updateSpeed();
        // JS:     for (const queueAction of this.queue.list) {
        // JS:         if (queueAction.pokemon) this.getActionSpeed(queueAction);
        // JS:     }
        // JS:     this.queue.sort();
        // JS: }
        if self.gen >= 8 {
            let should_sort = if let Some(next_action) = self.queue.peek() {
                let is_move_or_dynamax = match next_action {
                    Action::Move(_) => true,
                    Action::Pokemon(p) => matches!(p.choice, PokemonActionType::RunDynamax),
                    _ => false,
                };
                if is_move_or_dynamax {
                    eprintln!("[RUN_ACTION GEN8] Next action is move/dynamax, will sort queue");
                }
                is_move_or_dynamax
            } else {
                false
            };

            if should_sort {
                // JS: this.updateSpeed();
                self.update_speed();

                // JS: for (const queueAction of this.queue.list) {
                // JS:     if (queueAction.pokemon) this.getActionSpeed(queueAction);
                // JS: }
                let mut list = std::mem::take(&mut self.queue.list);
                for action in &mut list {
                    if action.has_pokemon() {
                        self.get_action_speed(action);
                    }
                }
                self.queue.list = list;

                // JS: this.queue.sort();
                self.sort_action_queue();
            }
        }

        // JS: if (!this.queue.peek() || (this.gen <= 3 && ['move', 'residual'].includes(this.queue.peek()!.choice))) {
        //         this.checkFainted();
        //     }
        // In gen 3 or earlier, switching in fainted pokemon is done after every move
        let should_check_fainted = self.queue.peek().is_none() || self.gen <= 3;
        if should_check_fainted {
            self.check_fainted();
        }

        // JS: const switches = this.sides.map(side => side.active.some(pokemon => pokemon && !!pokemon.switchFlag));
        // Build switches array - check if each side has any Pokemon with switchFlag set
        let mut switches: Vec<bool> = self.sides.iter().map(|side| {
            side.active.iter().any(|&opt_idx| {
                if let Some(poke_idx) = opt_idx {
                    if let Some(pokemon) = side.pokemon.get(poke_idx) {
                        return pokemon.switch_flag;
                    }
                }
                false
            })
        }).collect();

        // JS: for (let i = 0; i < this.sides.length; i++) {
        for i in 0..self.sides.len() {
            // JS: if (switches[i] && !this.canSwitch(this.sides[i])) {
            // Note: canSwitch returns number of possible switches, 0 is falsy in JS
            if switches[i] && self.can_switch(i) == 0 {
                // JS: for (const pokemon of this.sides[i].active) { pokemon.switchFlag = false; }
                for poke_idx in self.sides[i].pokemon.iter_mut() {
                    poke_idx.switch_flag = false;
                }
                // JS: if (!reviveSwitch) switches[i] = false;
                switches[i] = false;
            } else if switches[i] {
                // JS: for (const pokemon of this.sides[i].active) {
                //         if (pokemon.hp && pokemon.switchFlag && ... && !pokemon.skipBeforeSwitchOutEventFlag) {
                //             this.runEvent('BeforeSwitchOut', pokemon);
                //             pokemon.skipBeforeSwitchOutEventFlag = true;
                //             this.faintMessages();
                //             if (this.ended) return true;
                //             if (pokemon.fainted) {
                //                 switches[i] = this.sides[i].active.some(sidePokemon => sidePokemon && !!sidePokemon.switchFlag);
                //             }
                //         }
                //     }
                let active_positions: Vec<usize> = self.sides[i].active.iter()
                    .filter_map(|&opt_idx| opt_idx)
                    .collect();

                for poke_idx in active_positions {
                    let should_run_event = {
                        let pokemon = &self.sides[i].pokemon[poke_idx];
                        pokemon.hp > 0 && pokemon.switch_flag && !pokemon.skip_before_switch_out_event_flag
                    };

                    if should_run_event {
                        // Set the flag first
                        self.sides[i].pokemon[poke_idx].skip_before_switch_out_event_flag = true;

                        // Run BeforeSwitchOut event
                        self.run_event("BeforeSwitchOut", Some((i, poke_idx)), None, None, None);

                        // Check faint messages
                        self.faint_messages(false, false, true);

                        if self.ended {
                            return;
                        }

                        // Check if fainted and update switches
                        if self.sides[i].pokemon[poke_idx].fainted {
                            switches[i] = self.sides[i].active.iter().any(|&opt_idx| {
                                if let Some(idx) = opt_idx {
                                    if let Some(p) = self.sides[i].pokemon.get(idx) {
                                        return p.switch_flag;
                                    }
                                }
                                false
                            });
                        }
                    }
                }
            }
        }

        // JS: for (const playerSwitch of switches) {
        //         if (playerSwitch) {
        //             this.makeRequest('switch');
        //             return true;
        //         }
        //     }
        for player_switch in switches {
            if player_switch {
                self.make_request(Some(BattleRequestState::Switch));
                return;
            }
        }
    }
}
