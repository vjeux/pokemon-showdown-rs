use crate::*;
use crate::battle::BattleRequestState;
use crate::side::RequestState;

impl Battle {

    /// Start the first turn (after team preview)
    /// Equivalent to TypeScript runAction() case 'start' (battle.ts:2629-2700)
    /// Note: In TS this is part of runAction switch statement, extracted to separate method in Rust
    pub fn start_battle(&mut self) {
        self.add("start", &[]);
        self.turn = 1;
        self.add("turn", &[self.turn.to_string().into()]);

        // Collect switch-in operations first to avoid borrow conflict
        let num_sides = self.sides.len();
        let active_per_half = self.active_per_half;
        let pokemon_counts: Vec<usize> = self.sides.iter().map(|s| s.pokemon.len()).collect();

        let mut switch_ops = Vec::new();
        for (side_idx, &pokemon_count) in pokemon_counts.iter().enumerate().take(num_sides) {
            for slot in 0..active_per_half {
                if slot < pokemon_count {
                    switch_ops.push((side_idx, slot, slot));
                }
            }
        }

        for (side_idx, slot, pokemon_idx) in &switch_ops {
            self.switch_in(*side_idx, *slot, *pokemon_idx, None, false);
        }

        // Trigger switch-in abilities after all Pokemon are on the field
        // (JavaScript: this.runEvent('SwitchIn', pokemon))
        for (side_idx, _slot, pokemon_idx) in switch_ops {
            self.run_event("SwitchIn", Some((side_idx, pokemon_idx)), None, None, None);
        }

        self.request_state = BattleRequestState::Move;
        for side in &mut self.sides {
            side.request_state = RequestState::Move;
        }
    }
}
