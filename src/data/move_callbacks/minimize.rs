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
        use crate::dex_data::ID;

        // const boostedMoves = [
        //     'stomp', 'steamroller', 'bodyslam', 'flyingpress', 'dragonrush', 'heatcrash', 'heavyslam', 'maliciousmoonsault', 'supercellslam',
        // ];
        let boosted_moves = [
            ID::from("stomp"),
            ID::from("steamroller"),
            ID::from("bodyslam"),
            ID::from("flyingpress"),
            ID::from("dragonrush"),
            ID::from("heatcrash"),
            ID::from("heavyslam"),
            ID::from("maliciousmoonsault"),
            ID::from("supercellslam"),
        ];

        let current_move_id = ID::from(move_id);

        // if (boostedMoves.includes(move.id)) {
        if boosted_moves.contains(&current_move_id) {
            // return this.chainModify(2);
            return EventResult::ChainModify(2);
        }

        EventResult::Continue
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
        use crate::dex_data::ID;

        // const boostedMoves = [
        //     'stomp', 'steamroller', 'bodyslam', 'flyingpress', 'dragonrush', 'heatcrash', 'heavyslam', 'maliciousmoonsault', 'supercellslam',
        // ];
        let boosted_moves = [
            ID::from("stomp"),
            ID::from("steamroller"),
            ID::from("bodyslam"),
            ID::from("flyingpress"),
            ID::from("dragonrush"),
            ID::from("heatcrash"),
            ID::from("heavyslam"),
            ID::from("maliciousmoonsault"),
            ID::from("supercellslam"),
        ];

        let current_move_id = ID::from(move_id);

        // if (boostedMoves.includes(move.id)) {
        if boosted_moves.contains(&current_move_id) {
            // return true;
            return EventResult::Bool(true);
        }

        // return accuracy;
        EventResult::Int(accuracy)
    }
}
