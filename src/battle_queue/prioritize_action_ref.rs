use crate::battle_queue::Action;
use crate::battle_queue::BattleQueue;

impl BattleQueue {

    /// Prioritize an action by action itself (move to front with new order)
    pub fn prioritize_action_ref(&mut self, action: &Action) -> bool {
        let pos = self.list.iter().position(|a| {
            a.side_index() == action.side_index()
                && a.pokemon_index() == action.pokemon_index()
                && std::mem::discriminant(a) == std::mem::discriminant(action)
        });
        if let Some(i) = pos {
            let mut removed = self.list.remove(i);
            // Set order to 3 (high priority)
            match &mut removed {
                Action::Move(m) => m.order = 3,
                Action::Switch(s) => s.order = 3,
                _ => {}
            }
            self.list.insert(0, removed);
            true
        } else {
            false
        }
    }
}
