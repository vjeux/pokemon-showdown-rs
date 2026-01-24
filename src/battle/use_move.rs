//! Battle::use_move - Execute a move
//!
//! JavaScript equivalent: battle.actions.useMove()
//!
//! Provides Battle methods for executing moves, which is needed by move callbacks
//! like Copycat, Sleep Talk, Snatch, etc.

use crate::*;
use crate::battle::Effect;
use crate::dex::MoveData;

impl Battle {
    /// Use a move - wrapper for the standalone use_move function
    /// Equivalent to battle.actions.useMove()
    ///
    /// JavaScript equivalent (battle-actions.ts):
    /// ```javascript
    /// useMove(move: Move | string, pokemon: Pokemon, options?: {
    ///     target?: Pokemon | null, sourceEffect?: Effect | null,
    ///     zMove?: string, maxMove?: string,
    /// })
    /// ```
    ///
    /// This is called by move callbacks like:
    /// - Copycat (copies last move used)
    /// - Sleep Talk (uses random move while asleep)
    /// - Snatch (steals target's move)
    /// - Magic Coat/Magic Bounce (reflects move)
    ///
    /// Parameters:
    /// - move_data: The move data to use
    /// - pokemon_pos: Position of Pokemon using the move
    /// - target_pos: Optional target position
    /// - source_effect: Optional source effect (e.g., "sleeptalk", "copycat")
    /// - z_move: Optional Z-move variant
    /// - max_move: Optional Max move variant
    ///
    /// Returns: true if move succeeded, false otherwise
    pub fn use_move(
        &mut self,
        move_data: &MoveData,
        pokemon_pos: (usize, usize),
        target_pos: Option<(usize, usize)>,
        source_effect: Option<&Effect>,
        z_move: Option<&str>,
        max_move: Option<&str>,
    ) -> bool {
        crate::battle_actions::use_move::use_move(
            self,
            move_data,
            pokemon_pos,
            target_pos,
            source_effect,
            z_move,
            max_move,
        )
    }

    /// Use a move with bounced/reflected parameters
    /// Used by Magic Coat, Magic Bounce, and other reflection abilities
    ///
    /// JavaScript equivalent:
    /// ```javascript
    /// const newMove = this.dex.getActiveMove(move.id);
    /// newMove.hasBounced = true;
    /// newMove.pranksterBoosted = false;
    /// this.actions.useMove(newMove, target, { target: source });
    /// ```
    ///
    /// This is a convenience method that:
    /// 1. Marks the move as bounced
    /// 2. Optionally sets prankster boost status
    /// 3. Calls use_move with swapped source/target
    ///
    /// Parameters:
    /// - move_data: The move data being reflected
    /// - new_user_pos: Position of Pokemon now using the move (original target)
    /// - new_target_pos: Position of new target (original user)
    /// - has_bounced: Whether to mark move as bounced
    /// - prankster_boosted: Whether move should be prankster boosted
    ///
    /// Returns: true if move succeeded, false otherwise
    pub fn use_move_with_bounced(
        &mut self,
        move_data: &MoveData,
        new_user_pos: (usize, usize),
        new_target_pos: Option<(usize, usize)>,
        has_bounced: bool,
        prankster_boosted: bool,
    ) -> bool {
        // Set active move flags before using the move
        if let Some(ref active_move) = self.active_move {
            if has_bounced {
                active_move.borrow_mut().has_bounced = true;
            }
            active_move.borrow_mut().prankster_boosted = prankster_boosted;
        }

        // Use the move with the new user and target
        self.use_move(
            move_data,
            new_user_pos,
            new_target_pos,
            None, // sourceEffect is None for reflected moves
            None, // z_move
            None, // max_move
        )
    }
}
