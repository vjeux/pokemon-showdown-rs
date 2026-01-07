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
    pub fn on_source_modify_damage(
        battle: &mut Battle,
        _damage: i32,
        _source_pos: Option<(usize, usize)>,
        _target_pos: Option<(usize, usize)>,
        move_id: &str,
    ) -> EventResult {
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
            battle.chain_modify(2_f32); return EventResult::Continue;
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
    pub fn on_accuracy(
        _battle: &mut Battle,
        accuracy: i32,
        _target_pos: Option<(usize, usize)>,
        _source_pos: Option<(usize, usize)>,
        move_id: &str,
    ) -> EventResult {
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
            return EventResult::Boolean(true);
        }

        // return accuracy;
        EventResult::Number(accuracy)
    }

    /// onRestart() {
    ///     return null;
    /// }
    pub fn on_restart(
        _battle: &mut Battle,
        _pokemon_pos: (usize, usize),
    ) -> EventResult {
        // return null;
        // Returning null prevents the volatile from being restarted
        EventResult::Stop
    }
}
