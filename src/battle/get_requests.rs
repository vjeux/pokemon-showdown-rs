use crate::*;
use crate::battle::BattleRequestState;

impl Battle {

    /// Get requests for all players
    /// Equivalent to battle.ts getRequests() (battle.ts:1372-1424, 53 lines)
    ///
    // Note: Changed from &self to &mut self because getMoveRequestData() modifies Pokemon state
    // (maybe_disabled, maybe_locked, maybe_trapped fields) which matches JavaScript behavior
    //
    // 	getRequests(type: RequestState) {
    // 		// default to no request
    // 		const requests: ChoiceRequest[] = Array(this.sides.length).fill(null);
    //
    // 		switch (type) {
    // 		case 'switch':
    // 			for (let i = 0; i < this.sides.length; i++) {
    // 				const side = this.sides[i];
    // 				if (!side.pokemonLeft) continue;
    // 				const switchTable = side.active.map(pokemon => !!pokemon?.switchFlag);
    // 				if (switchTable.some(Boolean)) {
    // 					requests[i] = { forceSwitch: switchTable, side: side.getRequestData() };
    // 				}
    // 			}
    // 			break;
    //
    // 		case 'teampreview':
    // 			for (let i = 0; i < this.sides.length; i++) {
    // 				const side = this.sides[i];
    // 				const maxChosenTeamSize = this.ruleTable.pickedTeamSize || undefined;
    // 				requests[i] = { teamPreview: true, maxChosenTeamSize, side: side.getRequestData() };
    // 			}
    // 			break;
    //
    // 		default:
    // 			for (let i = 0; i < this.sides.length; i++) {
    // 				const side = this.sides[i];
    // 				if (!side.pokemonLeft) continue;
    // 				const activeData = side.active.map(pokemon => pokemon?.getMoveRequestData());
    // 				requests[i] = { active: activeData, side: side.getRequestData() };
    // 				if (side.allySide) {
    // 					(requests[i] as MoveRequest).ally = side.allySide.getRequestData(true);
    // 				}
    // 			}
    // 			break;
    // 		}
    //
    // 		const multipleRequestsExist = requests.filter(Boolean).length >= 2;
    // 		for (let i = 0; i < this.sides.length; i++) {
    // 			if (requests[i]) {
    // 				if (!this.supportCancel || !multipleRequestsExist) requests[i].noCancel = true;
    // 			} else {
    // 				requests[i] = { wait: true, side: this.sides[i].getRequestData() };
    // 			}
    // 		}
    //
    // 		return requests;
    // 	}
    //
    pub fn get_requests(&mut self) -> Vec<serde_json::Value> {
        // JS: const requests: ChoiceRequest[] = Array(this.sides.length).fill(null);
        let mut requests: Vec<Option<serde_json::Value>> = vec![None; self.sides.len()];

        // JS: switch (type)
        match self.request_state {
            // JS: case 'switch':
            BattleRequestState::Switch => {
                // JS: for (let i = 0; i < this.sides.length; i++)
                for (i, side) in self.sides.iter().enumerate() {
                    // JS: if (!side.pokemonLeft) continue;
                    if side.pokemon_left == 0 {
                        continue;
                    }

                    // JS: const switchTable = side.active.map(pokemon => !!pokemon?.switchFlag);
                    let switch_table: Vec<bool> = side.active.iter().map(|poke_idx_opt| {
                        match poke_idx_opt {
                            Some(poke_idx) => {
                                side.pokemon.get(*poke_idx)
                                    .map(|p| p.switch_flag.is_some())
                                    .unwrap_or(false)
                            }
                            None => false,
                        }
                    }).collect();

                    // JS: if (switchTable.some(Boolean))
                    if switch_table.iter().any(|&flag| flag) {
                        // JS: requests[i] = { forceSwitch: switchTable, side: side.getRequestData() };
                        requests[i] = Some(serde_json::json!({
                            "forceSwitch": switch_table,
                            "side": side.get_request_data(false),
                        }));
                    }
                }
            }

            // JS: case 'teampreview':
            BattleRequestState::TeamPreview => {
                // JS: for (let i = 0; i < this.sides.length; i++)
                for (i, side) in self.sides.iter().enumerate() {
                    // JS: const maxChosenTeamSize = this.ruleTable.pickedTeamSize || undefined;
                    let max_chosen_team_size = self.rule_table.as_ref()
                        .and_then(|rt| rt.picked_team_size);

                    // JS: requests[i] = { teamPreview: true, maxChosenTeamSize, side: side.getRequestData() };
                    let mut request = serde_json::json!({
                        "teamPreview": true,
                        "side": side.get_request_data(false),
                    });
                    if let Some(size) = max_chosen_team_size {
                        request["maxChosenTeamSize"] = serde_json::json!(size);
                    }
                    requests[i] = Some(request);
                }
            }

            // JS: default (move requests):
            BattleRequestState::Move | BattleRequestState::None => {
                // JS: for (let i = 0; i < this.sides.length; i++)
                // Note: Can't use iter() here because get_move_request_data needs &mut self
                for i in 0..self.sides.len() {
                    let side = &self.sides[i];

                    // JS: if (!side.pokemonLeft) continue;
                    if side.pokemon_left == 0 {
                        continue;
                    }

                    // JS: const activeData = side.active.map(pokemon => pokemon?.getMoveRequestData());
                    // Collect active pokemon indices first to avoid borrow checker issues
                    let active_indices: Vec<Option<usize>> = side.active.clone();

                    let active_data: Vec<serde_json::Value> = active_indices.iter().map(|poke_opt| {
                        match poke_opt {
                            Some(poke_idx) => {
                                Pokemon::get_move_request_data(self, (i, *poke_idx))
                            }
                            None => serde_json::Value::Null,
                        }
                    }).collect();

                    // Re-fetch side reference after mutable calls
                    let side = &self.sides[i];

                    // JS: requests[i] = { active: activeData, side: side.getRequestData() };
                    let mut request = serde_json::json!({
                        "active": active_data,
                        "side": side.get_request_data(false),
                    });

                    // JS: if (side.allySide) { (requests[i] as MoveRequest).ally = side.allySide.getRequestData(true); }
                    // Rust: ally_index is Option<usize> pointing to allied side in multi battles
                    if let Some(ally_idx) = side.ally_index {
                        if let Some(ally_side) = self.sides.get(ally_idx) {
                            // Pass true to indicate this is ally data (use baseMoves instead of moves)
                            request["ally"] = ally_side.get_request_data(true);
                        }
                    }

                    requests[i] = Some(request);
                }
            }
        }

        // JS: const multipleRequestsExist = requests.filter(Boolean).length >= 2;
        let multiple_requests_exist = requests.iter().filter(|r| r.is_some()).count() >= 2;

        // JS: for (let i = 0; i < this.sides.length; i++)
        for (i, request_opt) in requests.iter_mut().enumerate() {
            // JS: if (requests[i])
            if let Some(ref mut request) = request_opt {
                // JS: if (!this.supportCancel || !multipleRequestsExist) requests[i].noCancel = true;
                if !self.support_cancel || !multiple_requests_exist {
                    request["noCancel"] = serde_json::json!(true);
                }
            } else {
                // JS: requests[i] = { wait: true, side: this.sides[i].getRequestData() };
                *request_opt = Some(serde_json::json!({
                    "wait": true,
                    "side": self.sides[i].get_request_data(false),
                }));
            }
        }

        // JS: return requests;
        // Convert Option<Value> to Value (unwrap since we filled all None with wait requests)
        requests.into_iter().map(|r| r.unwrap()).collect()
    }
}
