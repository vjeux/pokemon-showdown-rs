//! Get mutable reference to a move action from the queue
//!
//! Pokemon Showdown - http://pokemonshowdown.com/

use crate::battle_queue::{Action, BattleQueue, MoveAction};

impl BattleQueue {
    /// Returns a mutable reference to the move action for the specified pokemon if it exists in the queue
    ///
    /// # Arguments
    ///
    /// * `side_index` - The side index of the pokemon
    /// * `pokemon_index` - The pokemon index within the side
    ///
    /// # Returns
    ///
    /// `Some(&mut MoveAction)` if a move action for the pokemon exists, `None` otherwise
    pub fn will_move_mut(&mut self, side_index: usize, pokemon_index: usize) -> Option<&mut MoveAction> {
        for action in &mut self.list {
            if let Action::Move(move_action) = action {
                if move_action.side_index == side_index
                    && move_action.pokemon_index == pokemon_index
                {
                    return Some(move_action);
                }
            }
        }
        None
    }
}
