use crate::battle_queue::Action;
use crate::battle_queue::BattleQueue;
use crate::battle_queue::PokemonAction;
use crate::battle_queue::PokemonActionType;

impl BattleQueue {

    /// Insert a runSwitch action for a pokemon
    /// This queues the switch-in effects to happen at the right time
    pub fn insert_run_switch(&mut self, side_index: usize, pokemon_index: usize) {
        let action = Action::Pokemon(PokemonAction {
            choice: PokemonActionType::RunSwitch,
            order: 101,
            priority: 0,
            speed: 1, // Speed doesn't matter for runSwitch
            pokemon_index,
            side_index,
            event: None,
        });
        self.list.push(action);
    }
}
