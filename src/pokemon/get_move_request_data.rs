use crate::*;
use crate::event::EventResult;

impl Pokemon {

    /// Get move request data for protocol
    /// Equivalent to getMoveRequestData in pokemon.ts
    //
    // 	getMoveRequestData() {
    // 		let lockedMove = this.maybeLocked ? null : this.getLockedMove();
    //
    // 		// Information should be restricted for the last active Pokémon
    // 		const isLastActive = this.isLastActive();
    // 		const canSwitchIn = this.battle.canSwitch(this.side) > 0;
    // 		let moves = this.getMoves(lockedMove, isLastActive);
    //
    // 		if (!moves.length) {
    // 			moves = [{ move: 'Struggle', id: 'struggle' as ID, target: 'randomNormal', disabled: false }];
    // 			lockedMove = 'struggle' as ID;
    // 		}
    //
    // 		const data: PokemonMoveRequestData = {
    // 			moves,
    // 		};
    //
    // 		if (isLastActive) {
    // 			this.maybeDisabled = this.maybeDisabled && !lockedMove;
    // 			this.maybeLocked = this.maybeLocked || this.maybeDisabled;
    // 			if (this.maybeDisabled) {
    // 				data.maybeDisabled = this.maybeDisabled;
    // 			}
    // 			if (this.maybeLocked) {
    // 				data.maybeLocked = this.maybeLocked;
    // 			}
    // 			if (canSwitchIn) {
    // 				if (this.trapped === true) {
    // 					data.trapped = true;
    // 				} else if (this.maybeTrapped) {
    // 					data.maybeTrapped = true;
    // 				}
    // 			}
    // 		} else {
    // 			this.maybeDisabled = false;
    // 			this.maybeLocked = false;
    // 			if (canSwitchIn) {
    // 				// Discovered by selecting a valid Pokémon as a switch target and cancelling.
    // 				if (this.trapped) data.trapped = true;
    // 			}
    // 			this.maybeTrapped = false;
    // 		}
    //
    // 		if (!lockedMove) {
    // 			if (this.canMegaEvo) data.canMegaEvo = true;
    // 			if (this.canMegaEvoX) data.canMegaEvoX = true;
    // 			if (this.canMegaEvoY) data.canMegaEvoY = true;
    // 			if (this.canUltraBurst) data.canUltraBurst = true;
    // 			const canZMove = this.battle.actions.canZMove(this);
    // 			if (canZMove) data.canZMove = canZMove;
    //
    // 			if (this.getDynamaxRequest()) data.canDynamax = true;
    // 			if (data.canDynamax || this.volatiles['dynamax']) data.maxMoves = this.getDynamaxRequest(true);
    // 			if (this.canTerastallize) data.canTerastallize = this.canTerastallize;
    // 		}
    //
    // 		return data;
    // 	}
    //
    pub fn get_move_request_data(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
    ) -> serde_json::Value {
        let (side_idx, poke_idx) = pokemon_pos;

        // Update can_terastallize based on current state (item, mega evo, gen)
        // JavaScript: this.canTerastallize = this.battle.actions.canTerastallize(this);
        let new_can_terastallize = crate::battle_actions::can_terastallize(battle, pokemon_pos);
        {
            let pokemon = match battle.pokemon_at_mut(side_idx, poke_idx) {
                Some(p) => p,
                None => return serde_json::Value::Null,
            };
            pokemon.can_terastallize = new_can_terastallize;
        }

        // Update can_mega_evo based on current state
        // JavaScript: this.canMegaEvo = this.battle.actions.canMegaEvo(this);
        let new_can_mega_evo = crate::battle_actions::can_mega_evo(battle, side_idx, poke_idx);
        {
            let pokemon = match battle.pokemon_at_mut(side_idx, poke_idx) {
                Some(p) => p,
                None => return serde_json::Value::Null,
            };
            pokemon.can_mega_evo = new_can_mega_evo;
        }

        // Update can_ultra_burst based on current state
        // JavaScript: this.canUltraBurst = this.battle.actions.canUltraBurst(this);
        let new_can_ultra_burst = crate::battle_actions::can_ultra_burst(battle, side_idx, poke_idx);
        {
            let pokemon = match battle.pokemon_at_mut(side_idx, poke_idx) {
                Some(p) => p,
                None => return serde_json::Value::Null,
            };
            pokemon.can_ultra_burst = new_can_ultra_burst;
        }

        // JS: let lockedMove = this.maybeLocked ? null : this.getLockedMove();
        let locked_move: Option<ID> = {
            let pokemon = match battle.pokemon_at(side_idx, poke_idx) {
                Some(p) => p,
                None => return serde_json::Value::Null,
            };

            if pokemon.maybe_locked.unwrap_or(false) {
                None
            } else {
                Pokemon::get_locked_move(battle, pokemon_pos)
            }
        };

        // JS: const isLastActive = this.isLastActive();
        let is_last_active = Pokemon::is_last_active(battle, pokemon_pos);

        // JS: const canSwitchIn = this.battle.canSwitch(this.side) > 0;
        let can_switch_in = battle.can_switch(side_idx) > 0;

        // JS: let moves = this.getMoves(lockedMove, isLastActive);
        let mut moves: Vec<serde_json::Value> = {
            let pokemon = match battle.pokemon_at_mut(side_idx, poke_idx) {
                Some(p) => p,
                None => return serde_json::Value::Null,
            };
            pokemon.get_moves(locked_move.as_ref())
        };

        // Updated locked_move for Struggle fallback
        let mut final_locked_move = locked_move;

        // JS: if (!moves.length) {
        if moves.is_empty() {
            // JS: moves = [{ move: 'Struggle', id: 'struggle' as ID, target: 'randomNormal', disabled: false }];
            moves = vec![serde_json::json!({
                "move": "Struggle",
                "id": "struggle",
                "target": "randomNormal",
                "disabled": false
            })];
            // JS: lockedMove = 'struggle' as ID;
            final_locked_move = Some(ID::from("struggle"));
        }

        // JS: const data: PokemonMoveRequestData = { moves };
        let mut data = serde_json::json!({
            "moves": moves
        });

        // Handle isLastActive vs non-last-active branching
        // JS: if (isLastActive)
        if is_last_active {
            // Phase 1: Read current values and compute new values
            let (new_maybe_disabled, new_maybe_locked, maybe_trapped_val, trapped_val) = {
                let pokemon = match battle.pokemon_at(side_idx, poke_idx) {
                    Some(p) => p,
                    None => return data,
                };

                // JS: this.maybeDisabled = this.maybeDisabled && !lockedMove;
                let new_maybe_disabled = pokemon.maybe_disabled && final_locked_move.is_none();

                // JS: this.maybeLocked = this.maybeLocked || this.maybeDisabled;
                let new_maybe_locked = pokemon.maybe_locked.unwrap_or(false) || new_maybe_disabled;

                (new_maybe_disabled, new_maybe_locked, pokemon.maybe_trapped, pokemon.trapped)
            };

            // Phase 2: Update Pokemon fields
            {
                let pokemon = match battle.pokemon_at_mut(side_idx, poke_idx) {
                    Some(p) => p,
                    None => return data,
                };
                pokemon.maybe_disabled = new_maybe_disabled;
                pokemon.maybe_locked = Some(new_maybe_locked);
            }

            // JS: if (this.maybeDisabled) { data.maybeDisabled = this.maybeDisabled; }
            if new_maybe_disabled {
                data["maybeDisabled"] = serde_json::json!(true);
            }

            // JS: if (this.maybeLocked) { data.maybeLocked = this.maybeLocked; }
            if new_maybe_locked {
                data["maybeLocked"] = serde_json::json!(true);
            }

            // JS: if (canSwitchIn)
            if can_switch_in {
                // JS: if (this.trapped === true) { data.trapped = true; }
                if trapped_val.is_trapped() {
                    data["trapped"] = serde_json::json!(true);
                // JS: else if (this.maybeTrapped) { data.maybeTrapped = true; }
                } else if maybe_trapped_val {
                    data["maybeTrapped"] = serde_json::json!(true);
                }
            }
        } else {
            // JS: this.maybeDisabled = false;
            // JS: this.maybeLocked = false;
            // JS: this.maybeTrapped = false;
            {
                let pokemon = match battle.pokemon_at_mut(side_idx, poke_idx) {
                    Some(p) => p,
                    None => return data,
                };
                pokemon.maybe_disabled = false;
                pokemon.maybe_locked = Some(false);
                pokemon.maybe_trapped = false;
            }

            // JS: if (canSwitchIn) { if (this.trapped) data.trapped = true; }
            if can_switch_in {
                let trapped_val = {
                    let pokemon = match battle.pokemon_at(side_idx, poke_idx) {
                        Some(p) => p,
                        None => return data,
                    };
                    pokemon.trapped
                };

                if trapped_val.is_trapped() {
                    data["trapped"] = serde_json::json!(true);
                }
            }
        }

        // JS: if (!lockedMove)
        if final_locked_move.is_none() {
            // Read all the can* fields
            let (can_mega_evo, can_mega_evo_x, can_mega_evo_y, can_ultra_burst, can_terastallize, has_dynamax_volatile) = {
                let pokemon = match battle.pokemon_at(side_idx, poke_idx) {
                    Some(p) => p,
                    None => return data,
                };

                (
                    pokemon.can_mega_evo.clone(),
                    pokemon.can_mega_evo_x.clone(),
                    pokemon.can_mega_evo_y.clone(),
                    pokemon.can_ultra_burst.clone(),
                    pokemon.can_terastallize.clone(),
                    pokemon.has_volatile(&ID::from("dynamax")),
                )
            };

            // JS: if (this.canMegaEvo) data.canMegaEvo = true;
            if can_mega_evo.is_some() {
                data["canMegaEvo"] = serde_json::json!(true);
            }

            // JS: if (this.canMegaEvoX) data.canMegaEvoX = true;
            if can_mega_evo_x.is_some() {
                data["canMegaEvoX"] = serde_json::json!(true);
            }

            // JS: if (this.canMegaEvoY) data.canMegaEvoY = true;
            if can_mega_evo_y.is_some() {
                data["canMegaEvoY"] = serde_json::json!(true);
            }

            // JS: if (this.canUltraBurst) data.canUltraBurst = true;
            if can_ultra_burst.is_some() {
                data["canUltraBurst"] = serde_json::json!(true);
            }

            // JS: const canZMove = this.battle.actions.canZMove(this);
            // JS: if (canZMove) data.canZMove = canZMove;
            let can_z_move_result = crate::battle_actions::can_z_move(
                battle,
                side_idx,
                poke_idx,
            );
            if let Some(can_z_move_data) = can_z_move_result {
                data["canZMove"] = serde_json::to_value(can_z_move_data).unwrap();
            }

            // JS: if (this.getDynamaxRequest()) data.canDynamax = true;
            let dynamax_request = {
                let pokemon = match battle.pokemon_at(side_idx, poke_idx) {
                    Some(p) => p,
                    None => return data,
                };
                pokemon.get_dynamax_request(false)
            };

            if dynamax_request.is_some() {
                data["canDynamax"] = serde_json::json!(true);
            }

            // JS: if (data.canDynamax || this.volatiles['dynamax']) data.maxMoves = this.getDynamaxRequest(true);
            if dynamax_request.is_some() || has_dynamax_volatile {
                let max_moves = {
                    let pokemon = match battle.pokemon_at(side_idx, poke_idx) {
                        Some(p) => p,
                        None => return data,
                    };
                    pokemon.get_dynamax_request(true)
                };
                if let Some(max_moves_data) = max_moves {
                    data["maxMoves"] = max_moves_data;
                }
            }

            // JS: if (this.canTerastallize) data.canTerastallize = this.canTerastallize;
            if let Some(tera_type) = can_terastallize {
                data["canTerastallize"] = serde_json::json!(tera_type);
            }
        }

        // JS: return data;
        data
    }
}
