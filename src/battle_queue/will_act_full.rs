use crate::*;
use crate::battle_queue::Action;
use crate::battle_queue::BattleQueue;
use crate::battle_queue::PokemonActionType;

impl BattleQueue {

    // ==========================================
    // Methods ported from battle-queue.ts
    // ==========================================

    /// Check if will act (has move/switch/shift action)
    pub fn will_act_full(&self) -> Option<&Action> {
        for action in &self.list {
            match action {
                Action::Move(_) | Action::Switch(_) => return Some(action),
                Action::Pokemon(p) if p.choice == PokemonActionType::Shift => return Some(action),
                _ => {}
            }
        }
        None
    }
}
