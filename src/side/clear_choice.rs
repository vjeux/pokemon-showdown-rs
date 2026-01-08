use crate::side::*;

impl Side {

    /// Clear choice for the turn
    /// Equivalent to side.ts clearChoice()
    //
    // 	clearChoice() {
    // 		let forcedSwitches = 0;
    // 		let forcedPasses = 0;
    // 		if (this.battle.requestState === 'switch') {
    // 			const canSwitchOut = this.active.filter(pokemon => pokemon?.switchFlag).length;
    // 			const canSwitchIn = this.pokemon.slice(this.active.length).filter(pokemon => pokemon && !pokemon.fainted).length;
    // 			forcedSwitches = Math.min(canSwitchOut, canSwitchIn);
    // 			forcedPasses = canSwitchOut - forcedSwitches;
    // 		}
    // 		this.choice = {
    // 			cantUndo: false,
    // 			error: ``,
    // 			actions: [],
    // 			forcedSwitchesLeft: forcedSwitches,
    // 			forcedPassesLeft: forcedPasses,
    // 			switchIns: new Set(),
    // 			zMove: false,
    // 			mega: false,
    // 			ultra: false,
    // 			dynamax: false,
    // 			terastallize: false,
    // 		};
    // 	}
    //
    pub fn clear_choice(&mut self, battle_request_state: crate::battle::BattleRequestState) {
        let mut forced_switches = 0;
        let mut forced_passes = 0;

        if matches!(
            battle_request_state,
            crate::battle::BattleRequestState::Switch
        ) {
            // Count active Pokemon with switchFlag set
            let can_switch_out = self
                .active
                .iter()
                .filter_map(|&opt_idx| opt_idx.and_then(|idx| self.pokemon.get(idx)))
                .filter(|p| p.switch_flag.is_some())
                .count();

            // Count benched Pokemon that aren't fainted
            // Note: In JavaScript, pokemon array is physically reordered so active Pokemon
            // are at front. In Rust, we use position field instead, so we filter by position.
            let can_switch_in = self
                .pokemon
                .iter()
                .filter(|p| p.position >= self.active.len() && !p.is_fainted())
                .count();

            forced_switches = can_switch_out.min(can_switch_in);
            forced_passes = can_switch_out - forced_switches;
        }

        self.choice = Choice {
            cant_undo: false,
            error: String::new(),
            actions: Vec::new(),
            forced_switches_left: forced_switches,
            forced_passes_left: forced_passes,
            switch_ins: Vec::new(),
            z_move: false,
            mega: false,
            ultra: false,
            dynamax: false,
            terastallize: false,
        };
    }
}
