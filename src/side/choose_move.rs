use crate::side::*;
use crate::Battle;
use crate::Pokemon;

impl Side {

    /// Choose move action
    //
    // 	chooseMove(
    // 		moveText?: string | number,
    // 		targetLoc = 0,
    // 		event: 'mega' | 'megax' | 'megay' | 'zmove' | 'ultra' | 'dynamax' | 'terastallize' | '' = ''
    // 	) {
    // 		if (this.requestState !== 'move') {
    // 			return this.emitChoiceError(`Can't move: You need a ${this.requestState} response`);
    // 		}
    // 		const index = this.getChoiceIndex();
    // 		if (index >= this.active.length) {
    // 			return this.emitChoiceError(`Can't move: You sent more choices than unfainted Pokémon.`);
    // 		}
    // 		const autoChoose = !moveText;
    // 		const pokemon: Pokemon = this.active[index];
    //
    // 		// Parse moveText (name or index)
    // 		// If the move is not found, the action is invalid without requiring further inspection.
    //
    // 		const request = pokemon.getMoveRequestData();
    // 		let moveid = '';
    // 		let targetType = '';
    // 		if (autoChoose) moveText = 1;
    // 		if (typeof moveText === 'number' || (moveText && /^[0-9]+$/.test(moveText))) {
    // 			// Parse a one-based move index.
    // 			const moveIndex = Number(moveText) - 1;
    // 			if (moveIndex < 0 || moveIndex >= request.moves.length || !request.moves[moveIndex]) {
    // 				return this.emitChoiceError(`Can't move: Your ${pokemon.name} doesn't have a move ${moveIndex + 1}`);
    // 			}
    // 			moveid = request.moves[moveIndex].id;
    // 			targetType = request.moves[moveIndex].target!;
    // 		} else {
    // 			// Parse a move ID.
    // 			// Move names are also allowed, but may cause ambiguity (see client issue #167).
    // 			moveid = toID(moveText);
    // 			if (moveid.startsWith('hiddenpower')) {
    // 				moveid = 'hiddenpower';
    // 			}
    // 			for (const move of request.moves) {
    // 				if (move.id !== moveid) continue;
    // 				targetType = move.target || 'normal';
    // 				break;
    // 			}
    // 			if (!targetType && ['', 'dynamax'].includes(event) && request.maxMoves) {
    // 				for (const [i, moveRequest] of request.maxMoves.maxMoves.entries()) {
    // 					if (moveid === moveRequest.move) {
    // 						moveid = request.moves[i].id;
    // 						targetType = moveRequest.target;
    // 						event = 'dynamax';
    // 						break;
    // 					}
    // 				}
    // 			}
    // 			if (!targetType && ['', 'zmove'].includes(event) && request.canZMove) {
    // 				for (const [i, moveRequest] of request.canZMove.entries()) {
    // 					if (!moveRequest) continue;
    // 					if (moveid === toID(moveRequest.move)) {
    // 						moveid = request.moves[i].id;
    // 						targetType = moveRequest.target;
    // 						event = 'zmove';
    // 						break;
    // 					}
    // 				}
    // 			}
    // 			if (!targetType) {
    // 				if (moveid !== 'testfight') {
    // 					return this.emitChoiceError(`Can't move: Your ${pokemon.name} doesn't have a move matching ${moveid}`);
    // 				}
    // 			}
    // 		}
    //
    // 		const moves = pokemon.getMoves();
    // 		if (autoChoose) {
    // 			for (const [i, move] of request.moves.entries()) {
    // 				if (move.disabled) continue;
    // 				if (i < moves.length && move.id === moves[i].id && moves[i].disabled) continue;
    // 				moveid = move.id;
    // 				targetType = move.target!;
    // 				break;
    // 			}
    // 		}
    // 		const move = this.battle.dex.moves.get(moveid);
    //
    // 		// Z-move
    //
    // 		const zMove = event === 'zmove' ? this.battle.actions.getZMove(move, pokemon) : undefined;
    // 		if (event === 'zmove' && !zMove) {
    // 			return this.emitChoiceError(`Can't move: ${pokemon.name} can't use ${move.name} as a Z-move`);
    // 		}
    // 		if (zMove && this.choice.zMove) {
    // 			return this.emitChoiceError(`Can't move: You can't Z-move more than once per battle`);
    // 		}
    //
    // 		if (zMove) targetType = this.battle.dex.moves.get(zMove).target;
    //
    // 		// Dynamax
    // 		// Is dynamaxed or will dynamax this turn.
    // 		const maxMove = (event === 'dynamax' || pokemon.volatiles['dynamax']) ?
    // 			this.battle.actions.getMaxMove(move, pokemon) : undefined;
    // 		if (event === 'dynamax' && !maxMove) {
    // 			return this.emitChoiceError(`Can't move: ${pokemon.name} can't use ${move.name} as a Max Move`);
    // 		}
    //
    // 		if (maxMove) targetType = this.battle.dex.moves.get(maxMove).target;
    //
    // 		// Validate targeting
    //
    // 		if (autoChoose || moveid === 'testfight') {
    // 			targetLoc = 0;
    // 		} else if (this.battle.actions.targetTypeChoices(targetType)) {
    // 			if (!targetLoc && this.active.length >= 2) {
    // 				return this.emitChoiceError(`Can't move: ${move.name} needs a target`);
    // 			}
    // 			if (!this.battle.validTargetLoc(targetLoc, pokemon, targetType)) {
    // 				return this.emitChoiceError(`Can't move: Invalid target for ${move.name}`);
    // 			}
    // 		} else {
    // 			if (targetLoc) {
    // 				return this.emitChoiceError(`Can't move: You can't choose a target for ${move.name}`);
    // 			}
    // 		}
    //
    // 		const lockedMove = pokemon.getLockedMove();
    // 		if (lockedMove) {
    // 			let lockedMoveTargetLoc = pokemon.lastMoveTargetLoc || 0;
    // 			const lockedMoveID = toID(lockedMove);
    // 			if (pokemon.volatiles[lockedMoveID]?.targetLoc) {
    // 				lockedMoveTargetLoc = pokemon.volatiles[lockedMoveID].targetLoc;
    // 			}
    // 			if (pokemon.maybeLocked) this.choice.cantUndo = true;
    // 			this.choice.actions.push({
    // 				choice: 'move',
    // 				pokemon,
    // 				targetLoc: lockedMoveTargetLoc,
    // 				moveid: lockedMoveID,
    // 			});
    // 			return true;
    // 		} else if (!moves.length) {
    // 			// Override action and use Struggle if there are no enabled moves with PP
    // 			// Gen 4 and earlier announce a Pokemon has no moves left before the turn begins, and only to that player's side.
    // 			if (this.battle.gen <= 4) this.send('-activate', pokemon, 'move: Struggle');
    // 			if (pokemon.maybeLocked) this.choice.cantUndo = true;
    // 			this.choice.actions.push({
    // 				choice: 'move',
    // 				pokemon,
    // 				moveid: 'struggle',
    // 			});
    // 			return true;
    // 		} else if (moveid === 'testfight') {
    // 			// test fight button
    // 			if (!pokemon.maybeLocked) {
    // 				return this.emitChoiceError(`Can't move: ${pokemon.name}'s Fight button is known to be safe`);
    // 			}
    // 			this.updateRequestForPokemon(pokemon, req => this.updateDisabledRequest(pokemon, req));
    // 			this.emitRequest(this.activeRequest!, true);
    // 			this.choice.error = 'Hack to avoid sending error messages to the client :D';
    // 			return false;
    // 		} else if (maxMove) {
    // 			// Dynamaxed; only Taunt and Assault Vest disable Max Guard, but the base move must have PP remaining
    // 			if (pokemon.maxMoveDisabled(move)) {
    // 				return this.emitChoiceError(`Can't move: ${pokemon.name}'s ${maxMove.name} is disabled`);
    // 			}
    // 		} else if (!zMove) {
    // 			// Check for disabled moves
    // 			let isEnabled = false;
    // 			let disabledSource = '';
    // 			for (const m of moves) {
    // 				if (m.id !== moveid) continue;
    // 				if (!m.disabled) {
    // 					isEnabled = true;
    // 					break;
    // 				} else if (m.disabledSource) {
    // 					disabledSource = m.disabledSource;
    // 				}
    // 			}
    // 			if (!isEnabled) {
    // 				// Request a different choice
    // 				if (autoChoose) throw new Error(`autoChoose chose a disabled move`);
    // 				return this.emitChoiceError(`Can't move: ${pokemon.name}'s ${move.name} is disabled`, { pokemon, update: req => {
    // 					let updated = this.updateDisabledRequest(pokemon, req);
    // 					for (const m of req.moves) {
    // 						if (m.id === moveid) {
    // 							if (!m.disabled) {
    // 								m.disabled = true;
    // 								updated = true;
    // 							}
    // 							if (m.disabledSource !== disabledSource) {
    // 								m.disabledSource = disabledSource;
    // 								updated = true;
    // 							}
    // 							break;
    // 						}
    // 					}
    // 					return updated;
    // 				} });
    // 			}
    // 			// The chosen move is valid yay
    // 		}
    //
    // 		// Mega evolution
    //
    // 		const mixandmega = this.battle.format.mod === 'mixandmega';
    // 		const mega = (event === 'mega');
    // 		const megax = (event === 'megax');
    // 		const megay = (event === 'megay');
    // 		if (mega && !pokemon.canMegaEvo) {
    // 			return this.emitChoiceError(`Can't move: ${pokemon.name} can't mega evolve`);
    // 		}
    // 		if (megax && !pokemon.canMegaEvoX) {
    // 			return this.emitChoiceError(`Can't move: ${pokemon.name} can't mega evolve X`);
    // 		}
    // 		if (megay && !pokemon.canMegaEvoY) {
    // 			return this.emitChoiceError(`Can't move: ${pokemon.name} can't mega evolve Y`);
    // 		}
    // 		if ((mega || megax || megay) && this.choice.mega && !mixandmega) {
    // 			return this.emitChoiceError(`Can't move: You can only mega-evolve once per battle`);
    // 		}
    // 		const ultra = (event === 'ultra');
    // 		if (ultra && !pokemon.canUltraBurst) {
    // 			return this.emitChoiceError(`Can't move: ${pokemon.name} can't ultra burst`);
    // 		}
    // 		if (ultra && this.choice.ultra && !mixandmega) {
    // 			return this.emitChoiceError(`Can't move: You can only ultra burst once per battle`);
    // 		}
    // 		let dynamax = (event === 'dynamax');
    // 		const canDynamax = (this.activeRequest as MoveRequest)?.active[this.active.indexOf(pokemon)].canDynamax;
    // 		if (dynamax && (this.choice.dynamax || !canDynamax)) {
    // 			if (pokemon.volatiles['dynamax']) {
    // 				dynamax = false;
    // 			} else {
    // 				if (this.battle.gen !== 8) {
    // 					return this.emitChoiceError(`Can't move: Dynamaxing doesn't outside of Gen 8.`);
    // 				} else if (pokemon.side.canDynamaxNow()) {
    // 					return this.emitChoiceError(`Can't move: ${pokemon.name} can't Dynamax now.`);
    // 				} else if (pokemon.side.allySide?.canDynamaxNow()) {
    // 					return this.emitChoiceError(`Can't move: It's your partner's turn to Dynamax.`);
    // 				}
    // 				return this.emitChoiceError(`Can't move: You can only Dynamax once per battle.`);
    // 			}
    // 		}
    // 		const terastallize = (event === 'terastallize');
    // 		if (terastallize && !pokemon.canTerastallize) {
    // 			// Make this work properly
    // 			return this.emitChoiceError(`Can't move: ${pokemon.name} can't Terastallize.`);
    // 		}
    // 		if (terastallize && this.choice.terastallize) {
    // 			return this.emitChoiceError(`Can't move: You can only Terastallize once per battle.`);
    // 		}
    // 		if (terastallize && this.battle.gen !== 9) {
    // 			// Make this work properly
    // 			return this.emitChoiceError(`Can't move: You can only Terastallize in Gen 9.`);
    // 		}
    //
    // 		this.choice.actions.push({
    // 			choice: 'move',
    // 			pokemon,
    // 			targetLoc,
    // 			moveid,
    // 			mega: mega || ultra,
    // 			megax,
    // 			megay,
    // 			zmove: zMove,
    // 			maxMove: maxMove ? maxMove.id : undefined,
    // 			terastallize: terastallize ? pokemon.teraType : undefined,
    // 		});
    //
    // 		if (pokemon.maybeDisabled && (this.battle.gameType === 'singles' || (
    // 			this.battle.gen <= 3 && !this.battle.actions.targetTypeChoices(targetType)
    // 		))) {
    // 			this.choice.cantUndo = true;
    // 		}
    //
    // 		if (mega || megax || megay) this.choice.mega = true;
    // 		if (ultra) this.choice.ultra = true;
    // 		if (zMove) this.choice.zMove = true;
    // 		if (dynamax) this.choice.dynamax = true;
    // 		if (terastallize) this.choice.terastallize = true;
    //
    // 		return true;
    // 	}
    //
    // TODO: Verify move parameter type matches JavaScript's ActiveMove usage
    pub fn choose_move(
        &mut self,
        battle: &mut Battle,
        move_id: ID,
        target_loc: Option<i8>,
        mega: bool,
        zmove: Option<String>,
        max_move: Option<String>,
        terastallize: Option<String>,
    ) -> Result<(), String> {
        let index = self.get_choice_index(false);
        if index >= self.active.len() {
            return Err("You sent more choices than unfainted Pokemon".to_string());
        }

        if self.request_state != RequestState::Move {
            return Err(format!(
                "Can't move: You need a {:?} response",
                self.request_state
            ));
        }

        // CRITICAL: Check for locked moves BEFORE processing the choice
        // JS: const lockedMove = pokemon.getLockedMove();
        // JS: if (lockedMove) { ... queue locked move and return true ... }
        //
        // This is essential for two-turn moves like Sky Drop, Outrage, etc.
        // When a Pokemon is locked into a move, we must queue that locked move
        // instead of the requested move, to match JavaScript behavior exactly.
        //
        // Without this check, we would queue the requested move (like rockslide),
        // it would fail due to invulnerability, but PP would already be deducted.
        // JavaScript never even tries to execute the requested move - it immediately
        // queues the locked move instead.
        //
        // Get the pokemon position for the locked move check
        let pokemon_index_opt = self.active.get(index).copied();
        if let Some(Some(pokemon_index)) = pokemon_index_opt {
            let pokemon_pos = (self.n, pokemon_index);

            // Check for locked move using Battle
            let locked_move = Pokemon::get_locked_move(battle, pokemon_pos);

            if let Some(locked_move_id) = locked_move {
                // Pokemon is locked into a move - queue that move instead
                // JS: let lockedMoveTargetLoc = pokemon.lastMoveTargetLoc || 0;
                // JS: const lockedMoveID = toID(lockedMove);
                // JS: if (pokemon.volatiles[lockedMoveID]?.targetLoc) {
                // JS:   lockedMoveTargetLoc = pokemon.volatiles[lockedMoveID].targetLoc;
                // JS: }
                //
                // For now, use targetLoc=0 as default (we can enhance this later
                // to read from volatile state if needed)
                let locked_target_loc = Some(0);

                // Queue the locked move
                self.choice.actions.push(ChosenAction {
                    choice: ChoiceType::Move,
                    pokemon_index: index,
                    target_loc: locked_target_loc,
                    move_id: Some(locked_move_id),
                    move_action: None,
                    switch_index: None,
                    mega: false, // Locked moves don't allow mega/zmove/etc
                    megax: None,
                    megay: None,
                    zmove: None,
                    max_move: None,
                    terastallize: None,
                    priority: None,
                });

                // Return success - locked move queued
                return Ok(());
            }
        }

        // No locked move - proceed with normal move choice processing
        // Check mega/dynamax/tera restrictions
        if mega && self.choice.mega {
            return Err("You can only mega evolve once per battle".to_string());
        }
        if zmove.is_some() && self.choice.z_move {
            return Err("You can only use a Z-move once per battle".to_string());
        }
        if max_move.is_some() && self.choice.dynamax {
            return Err("You can only Dynamax once per battle".to_string());
        }
        if terastallize.is_some() && self.choice.terastallize {
            return Err("You can only Terastallize once per battle".to_string());
        }

        self.choice.actions.push(ChosenAction {
            choice: ChoiceType::Move,
            pokemon_index: index,
            target_loc,
            move_id: Some(move_id),
            move_action: None,
            switch_index: None,
            mega,
            megax: None,
            megay: None,
            zmove: zmove.clone(),
            max_move: max_move.clone(),
            terastallize: terastallize.clone(),
            priority: None,
        });

        if mega {
            self.choice.mega = true;
        }
        if zmove.is_some() {
            self.choice.z_move = true;
        }
        if max_move.is_some() {
            self.choice.dynamax = true;
        }
        if terastallize.is_some() {
            self.choice.terastallize = true;
        }

        Ok(())
    }
}
