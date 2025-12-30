use crate::*;
use crate::battle::BattleRequestState;
use crate::side::RequestState;

impl Battle {

    /// Start the next turn
    pub fn next_turn(&mut self) {
        // Clear turn state
        for side in &mut self.sides {
            side.clear_turn_state();
        }

        self.turn += 1;
        self.add_log("turn", &[&self.turn.to_string()]);

        // Check if any side has fainted active Pokemon that need to be replaced
        let mut needs_switch = false;
        for side in &self.sides {
            // First check if this side has any available Pokemon to switch in
            let has_available_pokemon =
                side.pokemon.iter().any(|p| !p.is_fainted() && !p.is_active);

            if has_available_pokemon {
                // Only require switches if there are Pokemon available to switch in
                for &active_idx in &side.active {
                    // Check if slot is empty (Pokemon fainted) OR if the Pokemon in slot is fainted
                    if active_idx.is_none() {
                        needs_switch = true;
                        break;
                    } else if let Some(poke_idx) = active_idx {
                        if side
                            .pokemon
                            .get(poke_idx)
                            .map(|p| p.is_fainted())
                            .unwrap_or(false)
                        {
                            needs_switch = true;
                            break;
                        }
                    }
                }
            }
            if needs_switch {
                break;
            }
        }

        // Set up new request
        if needs_switch {
            // If there are fainted active Pokemon, request switches
            self.request_state = BattleRequestState::Switch;
            for side in &mut self.sides {
                side.request_state = RequestState::Switch;
            }
        } else {
            // Otherwise, request moves as normal
            self.request_state = BattleRequestState::Move;
            for side in &mut self.sides {
                side.request_state = RequestState::Move;
            }
        }
    }
}
