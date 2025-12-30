use crate::*;
use crate::battle_queue::BattleQueue;
use crate::battle_queue::MoveAction;

impl BattleQueue {

    /// Check if a pokemon will move this turn (by Pokemon reference)
    /// JavaScript pattern: this.queue.willMove(target)
    pub fn will_move_pokemon(&self, pokemon: &crate::pokemon::Pokemon) -> Option<&MoveAction> {
        self.will_move(pokemon.side_index, pokemon.position)
    }
}
