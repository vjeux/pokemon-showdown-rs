use crate::*;
use crate::battle::PriorityItem;

impl Battle {

    /// Run switch-in effects for a Pokemon
    /// 1:1 port of battle-actions.ts runSwitch()
    pub fn run_switch(&mut self, side_idx: usize, poke_idx: usize) {
        // Collect all switchers - consume all consecutive runSwitch actions
        let mut switchers_in: Vec<(usize, usize)> = vec![(side_idx, poke_idx)];

        // Collect any additional runSwitch actions from the queue
        while let Some(action) = self.queue.peek() {
            if action.is_run_switch() {
                if let Some((s, p)) = action.get_switch_target() {
                    switchers_in.push((s, p));
                }
                self.queue.shift();
            } else {
                break;
            }
        }

        // JS: const allActive = this.battle.getAllActive(true);
        // JS: this.battle.speedSort(allActive);
        // Collect all active Pokemon with their speeds
        let mut all_active: Vec<(usize, usize, i32)> = Vec::new();
        for (side_idx, side) in self.sides.iter().enumerate() {
            for poke_idx in side.active.iter().flatten() {
                if let Some(pokemon) = side.pokemon.get(*poke_idx) {
                    if !pokemon.fainted {
                        all_active.push((side_idx, *poke_idx, pokemon.speed));
                    }
                }
            }
        }

        // Sort all active Pokemon using speed_sort (which uses PRNG for ties)
        self.speed_sort(&mut all_active, |item| PriorityItem {
            order: None,
            priority: 0,
            speed: item.2,
            sub_order: 0,
            effect_order: 0,
            index: 0,
        });

        // Speed sort all active Pokemon
        self.update_speed();

        // Run SwitchIn field event for all switchers
        self.field_event_switch_in(&switchers_in);

        // Mark all switchers as started and clear draggedIn
        for (s_idx, p_idx) in &switchers_in {
            if self.sides[*s_idx].pokemon[*p_idx].hp == 0 {
                continue;
            }
            self.sides[*s_idx].pokemon[*p_idx].is_started = true;
            self.sides[*s_idx].pokemon[*p_idx].dragged_in = None;
        }
    }
}
