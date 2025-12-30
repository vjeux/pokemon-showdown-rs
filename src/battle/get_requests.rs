use crate::*;
use crate::battle::BattleRequestState;

impl Battle {

    /// Get requests for all players
    /// Equivalent to battle.ts getRequests()
    //
    // TODO: INCOMPLETE IMPLEMENTATION - ~20% of TypeScript logic
    // Missing from TypeScript version (battle.ts:1372-1424, 53 lines):
    // 1. Switch request: forceSwitch array with switchTable from pokemon.switchFlag
    // 2. TeamPreview request: maxChosenTeamSize from ruleTable.pickedTeamSize
    // 3. Move request: active array with getMoveRequestData() for each Pokemon
    // 4. Ally request data for multi-battle formats
    // 5. noCancel logic based on supportCancel and multiple requests
    // 6. Wait requests for sides with no Pokemon left
    // Current implementation only returns basic JSON with side/rqid/requestType
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
    pub fn get_requests(&self) -> Vec<serde_json::Value> {
        self.sides
            .iter()
            .map(|side| {
                serde_json::json!({
                    "side": side.id_str(),
                    "rqid": self.turn, // Use turn as request ID
                    "requestType": match self.request_state {
                        BattleRequestState::Move => "move",
                        BattleRequestState::Switch => "switch",
                        BattleRequestState::TeamPreview => "teampreview",
                        BattleRequestState::None => "none",
                    }
                })
            })
            .collect()
    }
}
