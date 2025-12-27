//! Minimize Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;


pub mod condition {
    use super::*;

    /// onSourceModifyDamage(damage, source, target, move) {
    ///     const boostedMoves = [
    ///         'stomp', 'steamroller', 'bodyslam', 'flyingpress', 'dragonrush', 'heatcrash', 'heavyslam', 'maliciousmoonsault', 'supercellslam',
    ///     ];
    ///     if (boostedMoves.includes(move.id)) {
    ///         return this.chainModify(2);
    ///     }
    /// }
    pub fn on_source_modify_damage(battle: &mut Battle, damage: i32, source_pos: Option<(usize, usize)>, target_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
        // Moves that deal double damage to minimized targets
        let boosted_moves = [
            "stomp", "steamroller", "bodyslam", "flyingpress", "dragonrush",
            "heatcrash", "heavyslam", "maliciousmoonsault", "supercellslam",
        ];

        if boosted_moves.contains(&move_id) {
            EventResult::Number(damage * 2)
        } else {
            EventResult::Continue
        }
    }

    /// onAccuracy(accuracy, target, source, move) {
    ///     const boostedMoves = [
    ///         'stomp', 'steamroller', 'bodyslam', 'flyingpress', 'dragonrush', 'heatcrash', 'heavyslam', 'maliciousmoonsault', 'supercellslam',
    ///     ];
    ///     if (boostedMoves.includes(move.id)) {
    ///         return true;
    ///     }
    ///     return accuracy;
    /// }
    pub fn on_accuracy(battle: &mut Battle, accuracy: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
        // Boosted moves always hit minimized targets
        let boosted_moves = [
            "stomp", "steamroller", "bodyslam", "flyingpress", "dragonrush",
            "heatcrash", "heavyslam", "maliciousmoonsault", "supercellslam",
        ];

        if boosted_moves.contains(&move_id) {
            EventResult::Bool(true)
        } else {
            EventResult::Number(accuracy)
        }
    }
}
