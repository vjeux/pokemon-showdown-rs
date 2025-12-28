//! Mud Sport Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;


pub mod condition {
    use super::*;

    /// onFieldStart(field, source) {
    ///     this.add('-fieldstart', 'move: Mud Sport', `[of] ${source}`);
    /// }
    pub fn on_field_start(battle: &mut Battle, _field_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>) -> EventResult {
        // this.add('-fieldstart', 'move: Mud Sport', `[of] ${source}`);
        if let Some(source) = source_pos {
            let source_arg = {
                let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                source_pokemon.get_slot()
            };

            battle.add("-fieldstart", &[
                "move: Mud Sport".into(),
                format!("[of] {}", source_arg).into(),
            ]);
        } else {
            battle.add("-fieldstart", &["move: Mud Sport".into()]);
        }

        EventResult::Continue
    }

    /// onBasePower(basePower, attacker, defender, move) {
    ///     if (move.type === 'Electric') {
    ///         this.debug('mud sport weaken');
    ///         return this.chainModify([1352, 4096]);
    ///     }
    /// }
    pub fn on_base_power(battle: &mut Battle, _base_power: i32, _pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
        

        // Get the move type
        let move_type = {
            let active_move = match &battle.active_move {
                Some(active_move) => &active_move.id,
                None => return EventResult::Continue,
            };
            let move_data = battle.dex.get_move_by_id(active_move);
            move_data.map(|m| m.move_type.clone())
        };

        // if (move.type === 'Electric') {
        if move_type.as_deref() == Some("Electric") {
            // this.debug('mud sport weaken');
            // (debug is typically not needed in Rust implementation)

            // return this.chainModify([1352, 4096]);
            return EventResult::Number(battle.chain_modify_fraction(1352, 4096));
        }

        EventResult::Continue
    }

    /// onFieldEnd() {
    ///     this.add('-fieldend', 'move: Mud Sport');
    /// }
    pub fn on_field_end(battle: &mut Battle) -> EventResult {
        // this.add('-fieldend', 'move: Mud Sport');
        battle.add("-fieldend", &["move: Mud Sport".into()]);

        EventResult::Continue
    }
}
