//! Water Sport Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

pub mod condition {
    use super::*;

    /// onFieldStart(field, source) {
    ///     this.add('-fieldstart', 'move: Water Sport', `[of] ${source}`);
    /// }
    pub fn on_field_start(
        battle: &mut Battle,
        _field_pos: Option<(usize, usize)>,
        source_pos: Option<(usize, usize)>,
    ) -> EventResult {
        let source = match source_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // this.add('-fieldstart', 'move: Water Sport', `[of] ${source}`);
        let source_slot = {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            source_pokemon.get_slot()
        };

        battle.add(
            "-fieldstart",
            &[
                crate::battle::Arg::from("move: Water Sport"),
                crate::battle::Arg::from(format!("[of] {}", source_slot)),
            ],
        );

        EventResult::Continue
    }

    /// onBasePower(basePower, attacker, defender, move) {
    ///     if (move.type === 'Fire') {
    ///         this.debug('water sport weaken');
    ///         return this.chainModify([1352, 4096]);
    ///     }
    /// }
    pub fn on_base_power(
        battle: &mut Battle,
        _base_power: i32,
        _pokemon_pos: (usize, usize),
        _target_pos: Option<(usize, usize)>,
    ) -> EventResult {
        // if (move.type === 'Fire') {
        //     this.debug('water sport weaken');
        //     return this.chainModify([1352, 4096]);
        // }
        let move_type = battle
            .active_move
            .as_ref()
            .map(|m| m.move_type.as_str())
            .unwrap_or("");

        if move_type == "Fire" {
            battle.debug("water sport weaken");
            return EventResult::Number(battle.chain_modify_fraction(1352, 4096));
        }

        EventResult::Continue
    }

    /// onFieldEnd() {
    ///     this.add('-fieldend', 'move: Water Sport');
    /// }
    pub fn on_field_end(battle: &mut Battle) -> EventResult {
        // this.add('-fieldend', 'move: Water Sport');
        battle.add("-fieldend", &[crate::battle::Arg::from("move: Water Sport")]);

        EventResult::Continue
    }
}
