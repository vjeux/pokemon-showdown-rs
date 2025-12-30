use crate::side::*;

impl Side {


    /// Check if choice for this turn is complete
    /// Equivalent to side.ts isChoiceDone()
    //
    // 	isChoiceDone() {
    // 		if (!this.requestState) return true;
    // 		if (this.choice.forcedSwitchesLeft) return false;
    //
    // 		if (this.requestState === 'teampreview') {
    // 			return this.choice.actions.length >= this.pickedTeamSize();
    // 		}
    //
    // 		// current request is move/switch
    // 		this.getChoiceIndex(); // auto-pass
    // 		return this.choice.actions.length >= this.active.length;
    // 	}
    //
    pub fn is_choice_done(&self, picked_team_size: Option<usize>) -> bool {
        // if (!this.requestState) return true;
        if matches!(self.request_state, RequestState::None) {
            return true;
        }

        // if (this.choice.forcedSwitchesLeft) return false;
        if self.choice.forced_switches_left > 0 {
            return false;
        }

        match self.request_state {
            RequestState::TeamPreview => {
                // return this.choice.actions.length >= this.pickedTeamSize();
                let team_size = self.picked_team_size(picked_team_size);
                self.choice.actions.len() >= team_size
            }
            RequestState::Move | RequestState::Switch => {
                // this.getChoiceIndex(); // auto-pass
                // TODO: Call getChoiceIndex() for auto-pass
                // This would auto-add pass actions for fainted Pokemon
                // For now, just check actions length

                // return this.choice.actions.length >= this.active.length;
                self.choice.actions.len() >= self.active.len()
            }
            RequestState::None => true,
        }
    }
}
