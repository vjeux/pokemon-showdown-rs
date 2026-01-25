use crate::*;
use crate::battle::BattleRequestState;
use crate::side::RequestState;

impl Battle {

    /// Make a request for player input
    /// Equivalent to battle.ts makeRequest()
    ///
    //
    // 	makeRequest(type?: RequestState) {
    // 		if (type) {
    // 			this.requestState = type;
    // 			for (const side of this.sides) {
    // 				side.clearChoice();
    // 			}
    // 		} else {
    // 			type = this.requestState;
    // 		}
    //
    // 		for (const side of this.sides) {
    // 			side.activeRequest = null;
    // 		}
    //
    // 		if (type === 'teampreview') {
    // 			// `pickedTeamSize = 6` means the format wants the user to select
    // 			// the entire team order, unlike `pickedTeamSize = undefined` which
    // 			// will only ask the user to select their lead(s).
    // 			const pickedTeamSize = this.ruleTable.pickedTeamSize;
    // 			this.add(`teampreview${pickedTeamSize ? `|${pickedTeamSize}` : ''}`);
    // 		}
    //
    // 		const requests = this.getRequests(type);
    // 		for (let i = 0; i < this.sides.length; i++) {
    // 			this.sides[i].activeRequest = requests[i];
    // 		}
    // 		this.sentRequests = false;
    //
    // 		if (this.sides.every(side => side.isChoiceDone())) {
    // 			throw new Error(`Choices are done immediately after a request`);
    // 		}
    // 	}
    //
    pub fn make_request(&mut self, request_type: Option<BattleRequestState>) {
        // JS: if (type) { this.requestState = type; ... } else { type = this.requestState; }
        let req_type = if let Some(rt) = request_type {
            self.request_state = rt;
            // JS: for (const side of this.sides) { side.clearChoice(); }
            for side in &mut self.sides {
                // Set temporary request_state for clearChoice
                // This will be updated below based on the actual request
                side.request_state = match rt {
                    BattleRequestState::Move => RequestState::Move,
                    BattleRequestState::Switch => RequestState::Switch,
                    BattleRequestState::TeamPreview => RequestState::TeamPreview,
                    BattleRequestState::None => RequestState::None,
                };
                side.clear_choice(rt);
            }
            rt
        } else {
            self.request_state
        };

        // JS: for (const side of this.sides) { side.activeRequest = null; }
        for side in &mut self.sides {
            side.active_request = None;
        }

        // JS: if (type === 'teampreview') { ... this.add(`teampreview${pickedTeamSize ? `|${pickedTeamSize}` : ''}`); }
        if matches!(req_type, BattleRequestState::TeamPreview) {
            // JS: const pickedTeamSize = this.ruleTable.pickedTeamSize;
            // JS: this.add(`teampreview${pickedTeamSize ? `|${pickedTeamSize}` : ''}`);
            if let Some(ref rule_table) = self.rule_table {
                if let Some(picked_team_size) = rule_table.picked_team_size {
                    self.add(
                        "teampreview",
                        &[Arg::String(picked_team_size.to_string())],
                    );
                } else {
                    self.add("teampreview", &[]);
                }
            } else {
                self.add("teampreview", &[]);
            }
        }

        // JS: const requests = this.getRequests(type);
        // JS: for (let i = 0; i < this.sides.length; i++) { this.sides[i].activeRequest = requests[i]; }
        let requests = self.get_requests();
        for i in 0..self.sides.len() {
            // Check if this is a wait request first
            // Wait requests have a simpler structure: {"wait": true, "side": ...}
            // and don't match the full BattleRequest struct
            if requests[i].get("wait").and_then(|v| v.as_bool()).unwrap_or(false) {
                // JS: if (!this.activeRequest || this.activeRequest.wait) return '';
                // Wait request means this side has RequestState::None (empty string in JS)
                self.sides[i].request_state = RequestState::None;
                self.sides[i].active_request = None;
                continue;
            }

            // Convert serde_json::Value to BattleRequest
            if let Ok(request) = serde_json::from_value::<crate::choice::BattleRequest>(requests[i].clone()) {
                // CRITICAL: In JavaScript, side.requestState is a GETTER that derives from activeRequest:
                //   get requestState(): RequestState {
                //       if (!this.activeRequest || this.activeRequest.wait) return '';
                //       if (this.activeRequest.teamPreview) return 'teampreview';
                //       if (this.activeRequest.forceSwitch) return 'switch';
                //       return 'move';
                //   }
                // We must replicate this logic here using request_type:
                use crate::choice::RequestType;
                let side_request_state = match request.request_type {
                    RequestType::Wait => RequestState::None,
                    RequestType::TeamPreview => RequestState::TeamPreview,
                    RequestType::Switch => RequestState::Switch,
                    RequestType::Move => RequestState::Move,
                };

                self.sides[i].request_state = side_request_state;
                self.sides[i].active_request = Some(request);
            }
        }

        // JS: this.sentRequests = false;
        self.sent_requests = false;

        // JS: if (this.sides.every(side => side.isChoiceDone())) { throw new Error(...); }
        // Safety check to prevent infinite loops
        let picked_team_size = self.rule_table.as_ref().and_then(|rt| rt.picked_team_size);
        if self
            .sides
            .iter_mut()
            .all(|side| side.is_choice_done(picked_team_size))
        {
            panic!("Choices are done immediately after a request");
        }
    }
}
