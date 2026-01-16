use crate::battle_queue::Action;
use crate::battle_queue::BattleQueue;
use crate::battle_queue::PokemonActionType;

impl BattleQueue {

    // ==========================================
    // Methods ported from battle-queue.ts
    // ==========================================

    /// Check if will act (has move/switch/shift/instaswitch action)
    /// TypeScript: willAct() - returns action or null
    /// JavaScript: for (const action of this.list) {
    ///              if (['move', 'switch', 'instaswitch', 'shift'].includes(action.choice)) {
    ///                return action;
    ///              }
    ///            }
    ///            return null;
    pub fn will_act(&self) -> Option<&Action> {
        debug_elog!("[WILL_ACT] Checking queue, list.len()={}", self.list.len());
        for (i, action) in self.list.iter().enumerate() {
            let desc = match action {
                Action::Move(m) => format!("Move({})", m.move_id.as_str()),
                Action::Switch(s) => format!("Switch(slot {})", s.pokemon_index),
                Action::Field(f) => format!("Field({:?})", f.choice),
                Action::Pokemon(p) => format!("Pokemon({:?})", p.choice),
                Action::Team(_) => "Team".to_string(),
            };
            debug_elog!("[WILL_ACT]   [{}] {}", i, desc);
        }

        for action in &self.list {
            match action {
                Action::Move(_) | Action::Switch(_) => {
                    debug_elog!("[WILL_ACT] Found action to return: Some");
                    return Some(action);
                }
                Action::Pokemon(p) if p.choice == PokemonActionType::Shift => {
                    debug_elog!("[WILL_ACT] Found Shift action to return: Some");
                    return Some(action);
                }
                _ => {}
            }
        }
        debug_elog!("[WILL_ACT] No action found, returning None");
        None
    }
}
